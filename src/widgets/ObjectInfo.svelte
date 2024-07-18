<script lang="ts">
    import ObjectButtonImage from "../place_menu/objects/ObjectButtonImage.svelte";
    import { bannedUsers, loginData, selectedObject } from "../stores";
    import Image from "../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import Loading from "../components/Loading.svelte";
    import { banUser, reportUser } from "../firebase/cloud_functions";
    import Toast from "../utils/toast";
    import { setClipboard } from "../utils/clipboard";
    import { Turnstile } from "svelte-turnstile";
    import { SyncedCooldown } from "../utils/cooldown";
    import { REPORT_COOLDOWN_SECONDS } from "shared-lib/user";
    import { getCameraPos } from "../level_view/view_controls";
    import OnceButton from "../components/OnceButton.svelte";
    import ColoredName from "../components/ColoredName.svelte";

    export let state: wasm.StateWrapper;

    const cooldown = new SyncedCooldown(
        `userData/${$loginData.currentUserData!.userData.uid}`,
        "epochNextPlaced",
        REPORT_COOLDOWN_SECONDS
    );
    let { display: cooldownDisplay, finished: cooldownFinished } = cooldown;

    $: isYourself =
        $loginData.currentUserData?.placeData?.username ==
        $selectedObject?.namePlaced;

    let resetReportButton: () => void;
    let resetBanButton: () => void;

    const report = async (name: string) => {
        try {
            const cameraPos = getCameraPos(state);
            await reportUser({
                username: name,
                turnstileResp: turnstileToken as string,
                x: cameraPos[0],
                y: cameraPos[1],
            });

            cooldown.start();
        } catch (e) {
            Toast.showErrorToast(`Failed to report user. (${e})`);
        }

        resetReportButton();

        turnstileToken = TokenStatus.Used;
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

    let objButtonSize = 0;
</script>

{#if $selectedObject != null}
    <ul
        class="relative flex flex-col gap-2 p-4 text-lg text-white rounded-lg w-96 menu-panel flex-center pointer-events-all"
    >
        <li
            class="absolute opacity-0 object-info-item li-alternating"
            bind:offsetHeight={objButtonSize}
        ></li>
        <li class="object-info-item li-alternating">
            <span>Type:</span>

            <div class="pr-3">
                <ObjectButtonImage
                    id={$selectedObject.id ?? 1}
                    {objButtonSize}
                />
            </div>
        </li>
        <li class="object-info-item li-alternating">
            <span>Main color:</span>

            <div
                class="h-full text-xl rounded-md aspect-square flex-center text-stroke"
                style={`
                    background: url("assets/ui/checker.png");
                    background-size: contain;
                    box-shadow: 0px 0px 0px 2px white, inset 0px 0px 0px 2px black, ${$selectedObject != null ? `inset 0px 0px 0px 100px rgba(${$selectedObject.mainColor.r}, ${$selectedObject.mainColor.g}, ${$selectedObject.mainColor.b}, ${$selectedObject.mainColor.opacity / 255})` : ""};
                `}
            >
                {#if $selectedObject.mainColor.blending ?? false}
                    <span class="text-sm font-pusab" title="Blending">B</span>
                {/if}
            </div>
        </li>
        <li class="object-info-item li-alternating">
            <span>Detail color:</span>

            <div
                class="h-full text-xl rounded-md aspect-square flex-center text-stroke"
                style={`
                background: url("assets/ui/checker.png");
                background-size: contain;
                box-shadow: 0px 0px 0px 2px white, inset 0px 0px 0px 2px black, ${$selectedObject != null ? `inset 0px 0px 0px 100px rgba(${$selectedObject.detailColor.r}, ${$selectedObject.detailColor.g}, ${$selectedObject.detailColor.b}, ${$selectedObject.detailColor.opacity / 255})` : ""};
            `}
            >
                {#if $selectedObject.detailColor.blending ?? false}
                    <span class="text-sm font-pusab">B</span>
                {/if}
            </div>
        </li>
        <li class="object-info-item li-alternating">
            <span>Z Layer:</span>

            <span
                >{wasm.z_layer_name(
                    $selectedObject.zLayer ?? wasm.ZLayer.B1
                )}</span
            >
        </li>
        <li class="object-info-item li-alternating">
            <span>Z Order:</span>

            <span>{$selectedObject.zOrder ?? 0}</span>
        </li>
        <li class="object-info-item li-alternating">
            <span>Placed by:</span>

            {#if $selectedObject.namePlaced != null}
                <button
                    aria-label="Copy Username"
                    class="hover:underline"
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
                    }}
                >
                    <ColoredName username={$selectedObject.namePlaced} />
                </button>
            {:else}
                <div class="relative w-9 h-9 max-w-9 max-h-9">
                    <Loading darken={false} />
                </div>
            {/if}
        </li>

        {#if $selectedObject?.namePlaced != null}
            <div class="flex flex-col items-center justify-center">
                {#if $loginData.currentUserData != null}
                    {#if (turnstileToken == TokenStatus.NoToken || turnstileToken == TokenStatus.Used) && $cooldownFinished && !isYourself}
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
                        <div class="relative w-9 h-9 max-w-9 max-h-9">
                            <Loading darken={false} />
                        </div>
                    {:else if !isYourself}
                        <div class="flex items-center justify-center gap-1">
                            <h1 class="w-full text-lg text-center">
                                Report User:
                            </h1>
                            <OnceButton
                                type="plain"
                                disabled={!$cooldownFinished}
                                class="h-16 enabled:bounce-active disabled:opacity-40 disabled:cursor-not-allowed"
                                aria-label="Report User"
                                on:click={() => {
                                    if ($selectedObject?.namePlaced != null) {
                                        report($selectedObject.namePlaced);
                                    }
                                }}
                                bind:reset={resetReportButton}
                            >
                                <Image
                                    src="/assets/ui/report/report_button.png"
                                    class="object-contain w-auto h-auto max-w-full max-h-full"
                                />
                            </OnceButton>
                        </div>
                        <p
                            class="text-sm text-center transition duration-500 text-white/50 hover:text-white"
                        >
                            {#if !$cooldownFinished}
                                Thanks for reporting!
                                {$cooldownDisplay}
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
                <div class="w-full h-10">
                    <OnceButton
                        type="decline"
                        class="w-full h-full"
                        on:click={() => {
                            if ($selectedObject?.namePlaced != null) {
                                ban($selectedObject.namePlaced);
                            }
                        }}
                        bind:reset={resetBanButton}>Ban User</OnceButton
                    >
                </div>
            {:else}
                <p class="text-sm">This user has already been banned.</p>
            {/if}
        {/if}
    </ul>
{/if}

<style lang="postcss">
    .object-info-item {
        @apply flex h-10 w-full items-center justify-between rounded-lg p-2;
    }
</style>
