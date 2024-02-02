<script lang="ts">
    import ObjectButtonImage from "../place_menu/objects/ObjectButtonImage.svelte";
    import { loginData, selectedObject } from "../stores";
    import Image from "../components/Image.svelte";
    import * as wasm from "wasm-lib";
    import Loading from "../components/Loading.svelte";
    import { reportedUsers, reportUser } from "../firebase/report";

    $: hasReported =
        $selectedObject?.namePlaced != null
            ? $reportedUsers.reported.includes($selectedObject.namePlaced)
            : true;
</script>

{#if $selectedObject != null}
    <ul
        class="flex flex-col p-4 pt-5 text-lg text-white rounded-lg w-96 menu-panel flex-center pointer-events-all"
    >
        <li class="object-info-item">
            <span>Type:</span>

            <div class="w-9 h-9 max-w-9 max-h-9 flex-center">
                <ObjectButtonImage
                    id={$selectedObject?.id ?? 1}
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
                {#if $selectedObject?.mainColor.blending ?? false}
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
                {#if $selectedObject?.detailColor.blending ?? false}
                    <span class="font-pusab">B</span>
                {/if}
            </div>
        </li>
        <li class="object-info-item">
            <span>Z Layer:</span>

            <span
                >{wasm.z_layer_name(
                    $selectedObject?.zLayer ?? wasm.ZLayer.B1
                )}</span
            >
        </li>
        <li class="object-info-item">
            <span>Z Order:</span>

            <span>{$selectedObject?.zOrder ?? 0}</span>
        </li>
        <li class="object-info-item">
            <span>Placed by:</span>

            {#if $selectedObject?.namePlaced != null}
                <span>{$selectedObject?.namePlaced}</span>
            {:else}
                <div class="relative w-9 h-9 max-w-9 max-h-9">
                    <Loading darken={false} />
                </div>
            {/if}
        </li>

        {#if $selectedObject?.namePlaced != null}
            <div class="flex flex-col items-center justify-center pt-3">
                {#if $loginData.currentUserData != null}
                    {#if $loginData.currentUserData.placeData?.username != $selectedObject.namePlaced}
                        <div class="flex items-center justify-center gap-1">
                            <h1 class="w-full text-lg text-center">
                                Report User:
                            </h1>
                            <button
                                disabled={hasReported}
                                class="h-16 enabled:bounce-active disabled:opacity-40 disabled:cursor-not-allowed"
                                aria-label="Report User"
                                on:click={() => {
                                    if ($selectedObject?.namePlaced != null) {
                                        reportUser($selectedObject.namePlaced);
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
                            {#if hasReported}
                                You have already reported this user. Thanks!
                            {:else}
                                Please report inappropriate usernames or alt
                                accounts.
                                <b
                                    >Do not spam reports or falsely report
                                    users.</b
                                >
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
    </ul>
{/if}

<style lang="postcss">
    .object-info-item {
        @apply flex h-12 w-full items-center justify-between rounded-lg p-2 even:bg-white/10;
    }
</style>
