<script lang="ts">
    import * as wasm from "wasm-lib";

    import {
        bannedUsers,
        loginData,
        menuMinimized,
        selectedObject,
    } from "../../stores";

    import OnceButton from "../../components/OnceButton.svelte";
    import Toast from "../../utils/toast";
    import { banUser, reportUser } from "../../firebase/cloud_functions";
    import ColoredName from "../../components/ColoredName.svelte";
    import Loading from "../../components/Loading.svelte";
    import { setClipboard } from "../../utils/clipboard";
    import { SyncedCooldown } from "../../utils/cooldown";
    import { getCameraPos } from "../../level_view/view_controls";
    import { getNewTurnstileToken } from "../../utils/turnstile";
    import ObjectButtonImage from "../objects/ObjectButtonImage.svelte";
    import checker from "../assets/checker.png?url";

    export let state: wasm.State;

    const cooldown = SyncedCooldown.new(
        `userDetails/${$loginData.currentUserData!.user.uid}/epochNextReport`
    );
    let { display: cooldownDisplay, finished: cooldownFinished } = cooldown;

    $: isYourself =
        $loginData.currentUserData?.userDetails?.username ==
        $selectedObject?.namePlaced;

    let resetReportButton: () => void;
    let resetBanButton: () => void;

    const report = async (name: string) => {
        try {
            const cameraPos = getCameraPos(state);

            const token = await getNewTurnstileToken();

            await reportUser({
                username: name,
                turnstileResp: token,
                x: cameraPos[0],
                y: cameraPos[1],
            });
        } catch (e) {
            Toast.showErrorToast(`Failed to report user. (${e})`);
        }

        resetReportButton();
    };

    const ban = async (name: string) => {
        try {
            await banUser({
                username: name,
            });
        } catch (e) {
            Toast.showErrorToast(`Failed to ban user! (${e})`);
        }

        resetBanButton();
    };
</script>

{#if $selectedObject != null}
    <fieldset
        class="flex items-center w-full h-full gap-4 p-4 sm:p-2 sm:flex-col sm:gap-2"
        disabled={$menuMinimized}
    >
        <div
            class="grid h-full grid-rows-2 min-w-60 max-w-60 md:min-w-40 md:max-w-40 sm:max-w-full sm:min-w-full sm:w-full sm:h-min sm:grid-rows-1 sm:grid-cols-2 sm:gap-2"
        >
            <hgroup
                class="flex flex-col items-center self-start justify-between overflow-x-scroll overflow-y-hidden thin-scrollbar"
            >
                <h1
                    class="text-xl text-center md:text-base font-pusab text-stroke"
                >
                    Placed By:
                </h1>
                <p>
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
                        <div class="relative w-9 h-9 max-w-9 max-h-9">
                            <Loading darken={false} />
                        </div>
                    {/if}
                </p>
            </hgroup>
            <div class="flex flex-col self-end w-full gap-2">
                <div class="flex flex-col items-center justify-center">
                    {#if $loginData.currentUserData != null}
                        <OnceButton
                            type="decline"
                            disabled={!$cooldownFinished || isYourself}
                            class="w-full text-base md:text-sm"
                            aria-label="Report User"
                            on:click={() => {
                                if ($selectedObject?.namePlaced != null) {
                                    report($selectedObject.namePlaced);
                                }
                            }}
                            bind:reset={resetReportButton}
                        >
                            Report
                        </OnceButton>
                    {/if}
                </div>

                {#if $loginData.currentUserData && $loginData.currentUserData.userDetails && $loginData.currentUserData.userDetails.moderator}
                    <OnceButton
                        type="decline"
                        class="w-full text-base md:text-sm"
                        disabled={$bannedUsers.includes(
                            $selectedObject.namePlaced?.toLowerCase() ?? ""
                        ) || isYourself}
                        on:click={() => {
                            if ($selectedObject?.namePlaced != null) {
                                ban($selectedObject.namePlaced);
                            }
                        }}
                        bind:reset={resetBanButton}
                    >
                        Ban
                    </OnceButton>
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
                            You can't report yourself 😜
                        {/if}
                    </p>
                {/if}
            </div>
        </div>

        <span
            class="min-w-[1px] max-w-[1px] h-full sm:min-w-full sm:min-h-[1px] sm:max-h-[1px] bg-white/20 flex-1"
        ></span>

        <ul
            class="max-w-[600px] sm:max-w-full sm:w-full flex-1 text-center h-full object-info-grid"
        >
            <li class="object-info-item type">
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
            <li class="object-info-item-colors colors">
                <h1 class="text-xl md:text-base font-pusab text-stroke">
                    Colors:
                </h1>
                <ul class="flex w-full gap-4 md:gap-2 flex-center">
                    <li
                        class="flex flex-col-reverse gap-1 flex-center"
                        aria-labelledby="main-color-label"
                    >
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
                    </li>
                    <li
                        class="flex flex-col-reverse gap-1 flex-center"
                        aria-labelledby="detail-color-label"
                    >
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
                    </li>
                </ul>
                <div class="flex w-full gap-4 md:gap-2 flex-center sm:text-sm">
                    <h2 class="flex-1" id="main-color-label">Main</h2>
                    <h2 class="flex-1" id="detail-color-label">Detail</h2>
                </div>
            </li>
            <li class="object-info-item zlayer">
                <h1
                    class="text-xl md:text-base font-pusab text-stroke sm:whitespace-nowrap"
                >
                    Z Layer:
                </h1>
                <span
                    class="font-pusab text-stroke text-[40px] md:text-[25px] sm:text-[20px]"
                >
                    {wasm.z_layer_name(
                        $selectedObject.zLayer ?? wasm.ZLayer.B1
                    )}
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
    .object-info-item {
        @apply grid h-full w-full items-center justify-center;
        grid-auto-rows: 1fr;
        grid-template-rows: 1fr 1fr 1fr;
    }

    /* this one does not change, the others do */
    .object-info-item-colors {
        @apply grid h-full w-full items-center justify-center;
        grid-auto-rows: 1fr;
        grid-template-rows: 1fr 1fr 1fr;
    }

    .object-info-grid {
        @apply grid gap-4 md:gap-2 sm:gap-0;
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
            grid-auto-columns: 1fr;
            grid-template-columns: 1fr 1fr 1fr;
        }
    }
</style>
