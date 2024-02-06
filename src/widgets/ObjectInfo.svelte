<script lang="ts">
    import ObjectButtonImage from "../place_menu/objects/ObjectButtonImage.svelte";
    import {
        bannedUsers,
        loginData,
        reportTimer,
        selectedObject,
    } from "../stores";
    import Image from "../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import Loading from "../components/Loading.svelte";
    import Button from "../components/Button.svelte";
    import { banUser, reportUser } from "../firebase/cloud_functions";
    import Toast from "../utils/toast";
    import { setClipboard } from "../utils/clipboard";
    import { Turnstile } from "svelte-turnstile";

    // $: hasReported =
    //     $selectedObject?.namePlaced != null
    //         ? $reportedUsers.reported.includes($selectedObject.namePlaced)
    //         : true

    let { display: reportTimerDisplay, finished: reportTimerFinished } =
        reportTimer;

    $: isYourself =
        $loginData.currentUserData?.placeData?.username ==
        $selectedObject?.namePlaced;

    let isReporting = false;
    const report = async (name: string) => {
        reportTimer.start(3);
        // isReporting = true;

        // try {
        //     await reportUser({
        //         username: name,
        //         turnstileResp: turnstileToken as string,
        //         x: 0,
        //         y: 0,
        //     });

        // } catch (e) {
        //     Toast.showErrorToast(`Failed to report user. (${e})`);
        // }

        // isReporting = false;

        // turnstileToken = TokenStatus.Used;
    };

    let isBanning = false;
    const ban = async (name: string) => {
        try {
            await banUser({
                username: name,
            });
        } catch (e) {
            Toast.showErrorToast(`Failed to ban user! (${e})`);
        }
    };

    enum TokenStatus {
        NoToken,
        Used,
    }

    let turnstileToken: string | TokenStatus = TokenStatus.NoToken;
    const SITE_KEY = __TURNSTILE_REPORT_SITE_KEY;
    let turnstileReset: () => void | undefined;

    $: {
        if ($selectedObject == null && turnstileToken == TokenStatus.Used)
            turnstileToken = TokenStatus.NoToken;
    }
</script>

{#if $selectedObject != null}
    <ul
        class="flex flex-col p-4 pt-5 text-lg text-white rounded-lg w-96 menu-panel flex-center pointer-events-all"
    >
        <li class="object-info-item">
            <span>Type:</span>

            <div class="w-9 h-9 max-w-9 max-h-9 flex-center">
                <ObjectButtonImage
                    id={$selectedObject.id ?? 1}
                    objButtonSize={36 * 1.45}
                />
            </div>
        </li>
        <li class="object-info-item">
            <span>Main color:</span>

            <div
                class="text-xl rounded-md w-9 h-9 flex-center text-stroke"
                style={`
                    background: url("assets/ui/checker.png");
                    box-shadow: 0px 0px 0px 2px white, inset 0px 0px 0px 2px black, ${$selectedObject != null ? `inset 0px 0px 0px 100px rgba(${$selectedObject.mainColor.r}, ${$selectedObject.mainColor.g}, ${$selectedObject.mainColor.b}, ${$selectedObject.mainColor.opacity / 255})` : ""};
                `}
            >
                {#if $selectedObject.mainColor.blending ?? false}
                    <span class="font-pusab">B</span>
                {/if}
            </div>
        </li>
        <li class="object-info-item">
            <span>Detail color:</span>

            <div
                class="text-xl rounded-md w-9 h-9 flex-center text-stroke"
                style={`
                background: url("assets/ui/checker.png");
                box-shadow: 0px 0px 0px 2px white, inset 0px 0px 0px 2px black, ${$selectedObject != null ? `inset 0px 0px 0px 100px rgba(${$selectedObject.detailColor.r}, ${$selectedObject.detailColor.g}, ${$selectedObject.detailColor.b}, ${$selectedObject.detailColor.opacity / 255})` : ""};
            `}
            >
                {#if $selectedObject.detailColor.blending ?? false}
                    <span class="font-pusab">B</span>
                {/if}
            </div>
        </li>
        <li class="object-info-item">
            <span>Z Layer:</span>

            <span
                >{wasm.z_layer_name(
                    $selectedObject.zLayer ?? wasm.ZLayer.B1
                )}</span
            >
        </li>
        <li class="object-info-item">
            <span>Z Order:</span>

            <span>{$selectedObject.zOrder ?? 0}</span>
        </li>
        <li class="object-info-item">
            <span>Placed by:</span>

            {#if $selectedObject.namePlaced != null}
                <button
                    aria-label="Copy Username"
                    on:click={() => {
                        if ($selectedObject != null) {
                            setClipboard($selectedObject.namePlaced ?? "")
                                .then(() =>
                                    Toast.showInfoToast("Copied username!")
                                )
                                .catch(e => {
                                    Toast.showErrorToast(
                                        `There was an issue copying the username! (${e})`
                                    );
                                });
                        }
                    }}>{$selectedObject.namePlaced}</button
                >
            {:else}
                <div class="relative w-9 h-9 max-w-9 max-h-9">
                    <Loading darken={false} />
                </div>
            {/if}
        </li>

        {#if $selectedObject?.namePlaced != null}
            <div class="flex flex-col items-center justify-center pt-3">
                {$reportTimerDisplay}
                {#if $loginData.currentUserData != null}
                    {#if (turnstileToken == TokenStatus.NoToken || turnstileToken == TokenStatus.Used) && $reportTimerFinished && !isYourself}
                        <Turnstile
                            siteKey={SITE_KEY}
                            on:turnstile-callback={e => {
                                turnstileToken = e.detail.token;
                            }}
                            on:turnstile-error={() =>
                                Toast.showErrorToast(
                                    `There was an error with the Turnstile`
                                )}
                            on:turnstile-expired={() => {
                                // expritation is essentially the same as being used
                                turnstileToken = TokenStatus.Used;
                            }}
                            bind:reset={turnstileReset}
                        />
                        <div class="relative relative w-9 h-9 max-w-9 max-h-9">
                            <Loading darken={false} />
                        </div>
                    {:else if !isYourself}
                        <div class="flex items-center justify-center gap-1">
                            <h1 class="w-full text-lg text-center">
                                Report User:
                            </h1>
                            <button
                                disabled={$reportTimerFinished || isReporting}
                                class="h-16 enabled:bounce-active disabled:opacity-40 disabled:cursor-not-allowed"
                                aria-label="Report User"
                                on:click={() => {
                                    if ($selectedObject?.namePlaced != null) {
                                        report($selectedObject.namePlaced);
                                    }
                                }}
                            >
                                <Image
                                    src="/assets/ui/report/report_button.png"
                                    class="object-contain w-auto h-auto max-w-full max-h-full"
                                />
                            </button>
                        </div>
                        <p
                            class="text-sm text-center transition duration-500 text-white/50 hover:text-white"
                        >
                            {#if $reportTimerFinished}
                                TODO
                            {:else}
                                Please report inappropriate usernames or alt
                                accounts.
                                <b>Do not falsely report users.</b>
                            {/if}
                        </p>
                    {:else}
                        <p class="text-sm">You can't report yourself 😜</p>
                    {/if}
                {:else}
                    <p class="text-sm">
                        Please log in or sign up to report users!
                    </p>
                {/if}
            </div>
        {/if}

        {#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.currentUserData.placeData.moderator && $selectedObject.namePlaced != null}
            {#if !$bannedUsers.includes($selectedObject.namePlaced.toLowerCase())}
                <div class="h-12 w-full pt-3">
                    <Button
                        type="decline"
                        iconClass="w-8 h-8"
                        bind:disabled={isBanning}
                        on:click={() => {
                            isBanning = true;
                            if ($selectedObject?.namePlaced != null) {
                                ban($selectedObject.namePlaced);
                            }
                        }}>Ban User</Button
                    >
                </div>
            {:else}
                <p class="text-sm pt-3">This user has already been banned.</p>
            {/if}
        {/if}
    </ul>
{/if}

<style lang="postcss">
    .object-info-item {
        @apply flex h-12 w-full items-center justify-between rounded-lg p-2 odd:bg-black/15 even:bg-white/10;
    }
</style>
