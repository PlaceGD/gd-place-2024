<script lang="ts">
    import * as wasm from "wasm-lib";

    import {
        bannedUsers,
        loginData,
        menuMinimized,
        menuSelectedObject,
        menuTabGroup,
        selectedObject,
        TabGroup,
    } from "../../stores";

    import OnceButton from "../../components/Buttons/OnceButton.svelte";
    import Toast from "../../utils/toast";
    import Image from "../../components/Image.svelte";
    import {
        banUser,
        getReportCooldown,
        reportUser,
    } from "../../firebase/cloud_functions";
    import ColoredName from "../../components/ColoredName.svelte";
    import Loading from "../../components/Loading.svelte";
    import { setClipboard } from "../../utils/clipboard";
    import { Cooldown } from "../../utils/cooldown";
    import { getCameraPos } from "../../level_view/view_controls";
    import { getNewTurnstileToken } from "../../utils/turnstile";
    import ObjectButtonImage from "../objects/ObjectButtonImage.svelte";
    import checker from "../assets/checker.png?url";
    import FadedScroll from "../../components/FadedScroll.svelte";
    import { onDestroy, onMount } from "svelte";
    import { db } from "../../firebase/firebase";
    import {
        SFX_TRIGGER,
        SFX_TRIGGER_SOUNDS,
        SONG_TRIGGER,
        SONG_TRIGGER_SONGS,
    } from "shared-lib/nexusgen";
    import DeclineButton from "../../components/Buttons/DeclineButton.svelte";
    import AcceptButton from "../../components/Buttons/AcceptButton.svelte";
    import IconButton from "../../components/Buttons/IconButton.svelte";
    import reportButtonimage from "../../moderator/assets/report_button.png?url";
    import type { FirebaseError } from "shared-lib/cloud_functions";
    import { SFX_ICONS, SONG_ICONS } from "../edit/sfx_tab";
    import { REPORT_COOLDOWN_SECONDS } from "shared-lib/user";

    export let state: wasm.State;

    const cooldown = new Cooldown(
        getReportCooldown,
        loginData,
        "lastReportTimestamp"
    );
    let { display: cooldownDisplay, finished: cooldownFinished } = cooldown;

    onDestroy(() => {
        cooldown.cleanup();
    });

    $: isYourself =
        $loginData.currentUserData?.userDetails?.username ==
        $selectedObject?.namePlaced;

    let resetReportButton: () => void;
    let resetBanButton: () => void;

    const report = async (name: string) => {
        try {
            // const token = await getNewTurnstileToken();

            console.log($selectedObject!.posX, $selectedObject!.posY);

            await reportUser({
                username: name,
                // turnstileResp: token,
                x: $selectedObject!.posX,
                y: $selectedObject!.posY,
            }).then(v => {
                cooldown.start(v.data.cooldown);
            });
        } catch (e: any) {
            console.error("Failed to report user", e.details.message);
            Toast.showErrorToast(`Failed to report user. (${e.details.code})`);
        }

        resetReportButton();
    };

    const ban = async (name: string) => {
        try {
            const reason = prompt(
                "Reason for banning (inappropriate username / alt account / etc):"
            );

            if (reason != null) {
                await banUser({
                    reason,
                    username: name,
                });
            }
        } catch (e: any) {
            console.error("Failed to ban user", e.details.message);
            Toast.showErrorToast(`Failed to ban user. (${e.details.code})`);
        }

        resetBanButton();
    };
</script>

{#if $selectedObject != null}
    <fieldset class="delete-tab-grid">
        <div
            class="grid h-full grid-rows-[min-content_1fr] min-w-60 max-w-60 md:min-w-40 md:max-w-40 sm:max-w-full sm:min-w-full sm:w-full sm:h-min sm:grid-rows-none sm:grid-cols-2 sm:gap-2"
        >
            <hgroup
                class="flex flex-col items-center self-start justify-between overflow-hidden text-center thin-scrollbar sm:self-center md:pt-4 sm:pt-0"
            >
                <h1
                    class="text-xl text-center md:text-base font-pusab text-stroke"
                >
                    Placed By:
                </h1>
                <FadedScroll orientation="horizontal" threshold={2}>
                    {#if $selectedObject.namePlaced != null}
                        <button
                            aria-label="Copy Username"
                            class="text-2xl font-bold md:text-base hover:underline"
                            on:click={() => {
                                if ($selectedObject != null) {
                                    setClipboard(
                                        $selectedObject.namePlaced ?? ""
                                    )
                                        .then(() =>
                                            Toast.showInfoToast(
                                                "Copied username!"
                                            )
                                        )
                                        .catch(e => {
                                            Toast.showErrorToast(
                                                `There was an issue copying the username! (${e})`
                                            );
                                        });
                                }
                            }}
                        >
                            <ColoredName
                                username={$selectedObject.namePlaced}
                            />
                        </button>
                    {:else}
                        <div class="flex w-full h-full flex-center">
                            <div
                                class="relative w-9 sm:w-7 h-9 sm:h-7 max-w-9 max-h-9"
                            >
                                <Loading darken={false} />
                            </div>
                        </div>
                    {/if}
                </FadedScroll>
            </hgroup>
            <div class="flex flex-col self-end w-full gap-2 sm:gap-1">
                {#if $loginData.currentUserData != null}
                    <OnceButton
                        userDisabled={!$cooldownFinished ||
                            isYourself ||
                            $selectedObject?.namePlaced == null}
                        bind:reset={resetReportButton}
                        let:disabled
                        let:click
                    >
                        <IconButton
                            {disabled}
                            class="w-full gap-2"
                            aria-label="Report User"
                            on:click={() => {
                                click();
                                if ($selectedObject?.namePlaced != null) {
                                    report($selectedObject.namePlaced);
                                }
                            }}
                        >
                            <Image
                                slot="left"
                                src={reportButtonimage}
                                class="w-6 h-6 my-2 xs:w-5 xs:h-5"
                            ></Image>
                            <span slot="children" class="text-base md:text-sm">
                                Report
                            </span>
                        </IconButton>
                    </OnceButton>
                {/if}
                {#if $loginData.currentUserData && $loginData.currentUserData.userDetails && $loginData.currentUserData.userDetails.moderator}
                    {#if $bannedUsers[$selectedObject.namePlaced?.toLowerCase() ?? ""]}
                        <p
                            class="text-sm text-center sm:text-xs hover-text-transition"
                        >
                            User is banned
                        </p>
                    {:else}
                        <OnceButton
                            userDisabled={isYourself ||
                                $selectedObject?.namePlaced == null}
                            let:click
                            let:disabled
                            bind:reset={resetBanButton}
                        >
                            <DeclineButton
                                {disabled}
                                on:click={() => {
                                    click();
                                    if ($selectedObject?.namePlaced != null) {
                                        ban($selectedObject.namePlaced);
                                    }
                                }}
                                class="w-full"
                            >
                                <span class="text-base md:text-sm">Ban</span>
                            </DeclineButton>
                        </OnceButton>
                    {/if}
                {:else}
                    <p
                        class="text-sm text-center sm:text-xs hover-text-transition"
                    >
                        {#if !isYourself}
                            {#if !$cooldownFinished}
                                Thanks for reporting!
                                <span class="proportional-nums"
                                    >{$cooldownDisplay}</span
                                >
                            {:else}
                                Only report rulebreakers.
                            {/if}
                        {:else}
                            You can't report yourself 😛
                        {/if}
                    </p>
                {/if}
            </div>
        </div>

        <span
            class="min-w-[1px] max-w-[1px] h-full sm:min-w-full sm:min-h-[1px] sm:max-h-[1px] bg-white/20 flex-1"
        ></span>

        <ul class="object-info-grid">
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <li
                class="object-info-item type sm:gap-1"
                on:click={() => {
                    $menuSelectedObject = $selectedObject.id ?? 1;
                    $menuTabGroup = TabGroup.Build;
                }}
            >
                <h1 class="text-xl md:text-base font-pusab text-stroke">
                    Type:
                </h1>
                <div
                    class="w-14 md:w-10 sm:w-8 aspect-square justify-self-center"
                >
                    {#key $selectedObject.id}
                        <ObjectButtonImage id={$selectedObject.id ?? 1} />
                    {/key}
                </div>
                <p class="text-sm text-center hover-text-transition">
                    ({$selectedObject.id ?? 1})
                </p>
            </li>

            <!-- {#if $selectedObject.id == SFX_TRIGGER}
                <li class="object-info-item colors">
                    <h1
                        class="text-xl md:text-base font-pusab text-stroke sm:whitespace-nowrap"
                    >
                        Speed:
                    </h1>
                    <span
                        class="font-pusab text-stroke text-[40px] md:text-[25px] sm:text-[20px]"
                    >
                        {$selectedObject.mainColor.g - 12}
                    </span>
                </li>
            {:else} -->
            <li class="object-info-item-colors colors">
                <h1 class="text-xl md:text-base font-pusab text-stroke">
                    {#if $selectedObject.id == SFX_TRIGGER}
                        Sound:
                    {:else if $selectedObject.id == SONG_TRIGGER}
                        Song:
                    {:else}
                        Colors:
                    {/if}
                </h1>
                <ul class="flex w-full gap-4 md:gap-2 flex-center">
                    <li
                        class="flex flex-col-reverse gap-1 flex-center"
                        aria-labelledby="main-color-label"
                    >
                        {#if $selectedObject.id == SFX_TRIGGER}
                            <div
                                class="rounded-md w-14 h-14 md:w-10 md:h-10 sm:w-8 sm:h-8 flex-center text-stroke"
                            >
                                <Image
                                    src={SFX_ICONS[
                                        SFX_TRIGGER_SOUNDS[
                                            $selectedObject.mainColor.r
                                        ]
                                    ]}
                                    lazyLoad
                                    class="object-contain w-full h-full"
                                />
                            </div>
                        {:else if $selectedObject.id == SONG_TRIGGER}
                            <div
                                class="rounded-md w-14 h-14 md:w-10 md:h-10 sm:w-8 sm:h-8 flex-center text-stroke"
                            >
                                <Image
                                    src={SONG_ICONS[
                                        SONG_TRIGGER_SONGS[
                                            $selectedObject.mainColor.r
                                        ]
                                    ]}
                                    lazyLoad
                                    class="object-contain w-full h-full"
                                />
                            </div>
                        {:else}
                            <div
                                class="rounded-md w-14 h-14 md:w-10 md:h-10 sm:w-8 sm:h-8 flex-center text-stroke"
                                style={`
                                    background: url(${checker});
                                    background-size: contain;
                                    box-shadow: 
                                        0px 0px 0px 2px white, 
                                        inset 0px 0px 0px 2px black, 
                                        ${
                                            $selectedObject != null
                                                ? `inset 0px 0px 0px 100px rgba(
                                                    ${$selectedObject.mainColor.r}, 
                                                    ${$selectedObject.mainColor.g}, 
                                                    ${$selectedObject.mainColor.b}, 
                                                    ${$selectedObject.mainColor.opacity / 255})`
                                                : ""
                                        };
                                `}
                            >
                                {#if $selectedObject.mainColor.blending ?? false}
                                    <span
                                        class="text-lg font-pusab"
                                        title="Blending"
                                    >
                                        B
                                    </span>
                                {/if}
                            </div>
                        {/if}
                    </li>
                    <li
                        class="flex flex-col-reverse gap-1 flex-center"
                        aria-labelledby="detail-color-label"
                    >
                        {#if $selectedObject.id == SFX_TRIGGER || $selectedObject.id == SONG_TRIGGER}
                            <div
                                class="rounded-md w-14 h-14 md:w-10 md:h-10 sm:w-8 sm:h-8 flex-center text-stroke"
                            >
                                <span
                                    class="font-pusab text-stroke text-[40px] md:text-[25px] sm:text-[20px]"
                                >
                                    {$selectedObject.mainColor.g - 12}
                                </span>
                            </div>
                        {:else}
                            <div
                                class="rounded-md w-14 h-14 md:w-10 md:h-10 sm:w-8 sm:h-8 flex-center text-stroke"
                                style={`
                                    background: url(${checker});
                                    background-size: contain;
                                    box-shadow: 
                                    0px 0px 0px 2px white, 
                                    inset 0px 0px 0px 2px black, 
                                    ${
                                        $selectedObject != null
                                            ? `inset 0px 0px 0px 100px rgba(
                                                ${$selectedObject.detailColor.r}, 
                                                ${$selectedObject.detailColor.g}, 
                                                ${$selectedObject.detailColor.b}, 
                                                ${$selectedObject.detailColor.opacity / 255})`
                                            : ""
                                    };
                            `}
                            >
                                {#if $selectedObject.detailColor.blending ?? false}
                                    <span
                                        class="text-lg font-pusab"
                                        title="Blending"
                                    >
                                        B
                                    </span>
                                {/if}
                            </div>
                        {/if}
                    </li>
                </ul>
                <div class="flex w-full gap-4 md:gap-2 flex-center sm:text-sm">
                    <h2 class="flex-1" id="main-color-label">
                        {#if $selectedObject.id == SFX_TRIGGER}
                            SFX
                        {:else if $selectedObject.id == SONG_TRIGGER}
                            Song
                        {:else}
                            Main
                        {/if}
                    </h2>
                    <h2 class="flex-1" id="detail-color-label">
                        {#if $selectedObject.id == SFX_TRIGGER || $selectedObject.id == SONG_TRIGGER}
                            Speed
                        {:else}
                            Detail
                        {/if}
                    </h2>
                </div>
            </li>
            <!-- {/if} -->
            <li class="object-info-item zlayer">
                <h1
                    class="text-xl md:text-base font-pusab text-stroke sm:whitespace-nowrap"
                >
                    Z Layer:
                </h1>
                <span
                    class="font-pusab text-stroke text-[40px] md:text-[25px] sm:text-[20px]"
                >
                    {wasm.z_layer_name($selectedObject.zLayer)}
                </span>
            </li>
            <li class="object-info-item zorder">
                <h1
                    class="text-xl md:text-base font-pusab text-stroke sm:whitespace-nowrap"
                >
                    Z Index:
                </h1>
                <span
                    class="font-pusab text-stroke text-[40px] md:text-[25px] sm:text-[20px]"
                >
                    {$selectedObject.zOrder ?? 0}
                </span>
            </li>
        </ul>
    </fieldset>
{:else}
    <div
        class="w-full h-full p-4 text-4xl text-center md:text-3xl sm:text-2xl xs:text-xl flex-center font-pusab text-stroke"
    >
        Select an object to delete it!
    </div>
{/if}

<style lang="postcss">
    .delete-tab-grid {
        @apply grid h-full w-full items-center gap-4 p-4 md:gap-2 md:p-2 sm:flex-col xs:gap-1;
        grid-template-columns: min-content min-content 1fr;
    }

    @media screen(sm) {
        .delete-tab-grid {
            grid-template-columns: unset;
            grid-template-rows: min-content min-content 1fr;
        }
    }

    .object-info-item {
        @apply grid h-full w-full items-center justify-center;
        grid-auto-rows: 1fr;
        grid-template-rows: 1fr min-content 1fr;
    }

    /* this one does not change, the others do */
    .object-info-item-colors {
        @apply grid h-full w-full items-center justify-center;
        grid-auto-rows: 1fr;
        grid-template-rows: 1fr 1fr 1fr;
    }

    .object-info-grid {
        @apply grid h-full max-w-[600px] flex-1 content-center gap-4 justify-self-center text-center sm:w-full sm:max-w-full sm:justify-around sm:gap-x-4 sm:gap-y-1 xs:justify-center;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        grid-template-rows: 1fr;
        grid-template-areas: "type colors zlayer zorder";
    }

    .type {
        grid-area: type;
    }
    .colors {
        grid-area: colors;
    }
    .zlayer {
        grid-area: zlayer;
    }
    .zorder {
        grid-area: zorder;
    }

    @media screen(sm) {
        .object-info-grid {
            grid-template-columns: min-content min-content;
            grid-template-rows: min-content min-content min-content;
            grid-template-areas:
                "type colors"
                "zlayer colors"
                "zorder colors";
        }

        .object-info-item {
            grid-auto-rows: unset;
            grid-template-rows: unset;
            grid-template-columns: 85px 1fr 50px; /* these hardcoded values pain me */
        }
    }
</style>
