<script lang="ts">
    import {
        deleteCooldown as deleteCooldownTime,
        placeCooldown as placeCooldownTime,
    } from "../firebase/cooldowns";
    import { addObject, removeObject } from "../firebase/object";
    import { GUIDE_ELEM_IDS, walmart } from "../guide/guide";
    import {
        applyPreviewColor,
        canPlacePreview,
        loginData,
        menuMinimized,
        menuTabGroup,
        songPlayingIsPreview,
        TabGroup,
    } from "../stores";
    import { playSound, transferSoundChannel } from "../utils/audio";
    import { Cooldown } from "../utils/cooldown";
    import * as wasm from "wasm-lib";
    import deleteTimerFinishedSoundUrl from "./assets/sounds/delete_timer_finished.mp3?url";
    import placeTimerFinishedSoundUrl from "./assets/sounds/place_timer_finished.mp3?url";
    import { default as cx } from "classnames";
    import Loading from "../components/Loading.svelte";
    import { scale } from "svelte/transition";
    import {
        getDeleteCooldown,
        getPlaceCooldown,
    } from "../firebase/cloud_functions";

    export let state: wasm.State;
    export let exportPlaceCooldownRemaining: number;
    export let exportDeleteCooldownRemaining: number;

    const placeCooldown = new Cooldown();
    getPlaceCooldown().then(v => placeCooldown.setCooldown(v.data));
    let {
        display: placeCooldownDisplay,
        finished: placeCooldownFinished,
        remaining: placeCooldownRemaining,
    } = placeCooldown;

    const deleteCooldown = new Cooldown();
    getDeleteCooldown().then(v => deleteCooldown.setCooldown(v.data));
    let {
        display: deleteCooldownDisplay,
        finished: deleteCooldownFinished,
        remaining: deleteCooldownRemaining,
    } = deleteCooldown;

    $: {
        exportPlaceCooldownRemaining = $placeCooldownRemaining;
        exportDeleteCooldownRemaining = $deleteCooldownRemaining;
    }

    let pdButtonDisabled = false;
    $: pdButtonDisabled =
        $menuTabGroup != TabGroup.Delete
            ? !$placeCooldownFinished
            : !$deleteCooldownFinished;

    let playPlaceCooldownSound = !$placeCooldownFinished;
    $: {
        if ($placeCooldownFinished) {
            $walmart.hasPlaceCooldown = false;

            if (playPlaceCooldownSound) {
                playSound({ url: placeTimerFinishedSoundUrl });
            } else {
                playPlaceCooldownSound = true;
            }
        } else {
            $walmart.hasPlaceCooldown = true;
        }
    }
    let playDeleteCooldownSound = !$deleteCooldownFinished;
    $: {
        if ($deleteCooldownFinished) {
            $walmart.hasDeleteCooldown = false;

            if (playDeleteCooldownSound) {
                playSound({ url: deleteTimerFinishedSoundUrl });
            } else {
                playDeleteCooldownSound = true;
            }
        } else {
            $walmart.hasDeleteCooldown = true;
        }
    }
</script>

<button
    class={cx({
        "self-end overflow-hidden pd-button cursor-pointer": true,
        "place-bttn-place": $menuTabGroup != TabGroup.Delete,
        "place-bttn-delete": $menuTabGroup == TabGroup.Delete,
        "bounce-active": !pdButtonDisabled,
    })}
    aria-label={`${$menuTabGroup != TabGroup.Delete ? "Place" : "Delete"} Button`}
    data-minimised={+$menuMinimized}
    on:click={() => {
        if ($menuTabGroup != TabGroup.Delete) {
            addObject(state.get_preview_object(), (k, c) => {
                canPlacePreview.set(true);
                state.set_preview_visibility(false);
                applyPreviewColor();
                transferSoundChannel("preview song", "song");
                songPlayingIsPreview.set(false);

                if (c != null) {
                    placeCooldown.setCooldown(c);
                } else {
                    pdButtonDisabled = false;
                }

                // state.add_object(k, state.get_preview_object());
            });
            // state.set_preview_visibility(false);
            pdButtonDisabled = true;
            $canPlacePreview = false;
        } else {
            let k = state.get_selected_object_key();
            let coord = state.get_selected_object_chunk();
            if (k != null && coord != null) {
                pdButtonDisabled = true;
                removeObject(k, [coord.x, coord.y], c => {
                    deleteCooldown.setCooldown(c);
                });
            }
        }
    }}
    disabled={pdButtonDisabled || $menuMinimized}
    data-guide={GUIDE_ELEM_IDS.pdButton}
>
    <div
        class="flex flex-col w-full h-full gap-1 py-4 text-5xl sm:flex-row sm:gap-2 flex-center md:text-4xl sm:text-4xl text-stroke"
    >
        <h1 class="font-pusab tab-text">
            {#if $menuTabGroup != TabGroup.Delete}
                Place
            {:else}
                Delete
            {/if}
        </h1>
        {#if pdButtonDisabled}
            {@const isTimerWaiting =
                ($menuTabGroup != TabGroup.Delete && $placeCooldownDisplay) ==
                    "--:--" ||
                ($menuTabGroup == TabGroup.Delete && $deleteCooldownDisplay) ==
                    "--:--"}
            <span
                class={cx({
                    "relative flex w-full h-12 text-center sm:h-full proportional-numsflex-center": true,
                    "w-full h-12 sm:w-12 sm:h-full": isTimerWaiting,
                    "sm:w-max h-max": !isTimerWaiting,
                })}
                style={isTimerWaiting ? "opacity: 0.5" : ""}
            >
                {#if isTimerWaiting}
                    <Loading darken={false} />
                {:else if $menuTabGroup != TabGroup.Delete}
                    <span
                        class="flex w-full flex-center sm:h-full font-pusab"
                        transition:scale
                    >
                        {$placeCooldownDisplay}
                    </span>
                {:else}
                    <span
                        class="flex w-full flex-center sm:h-full font-pusab"
                        transition:scale
                    >
                        {$deleteCooldownDisplay}
                    </span>
                {/if}
            </span>
        {/if}
    </div>
</button>

<style lang="postcss">
    .pd-button {
        width: 256px;
    }
    .pd-button[data-minimised="0"] {
        height: 256px;
    }
    .pd-button[data-minimised="1"] {
        height: 0px;
    }

    .place-bttn-place {
        border-radius: 16px;
        background: #7ade2d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #49851b inset,
            4px 4px 0px 8px #c6f249 inset;
    }
    .place-bttn-place:not(:disabled):active {
        border-radius: 16px;
        background: #61b91d;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #3a6a16 inset,
            4px 4px 0px 8px #b2eb11 inset;
    }
    .place-bttn-place:disabled {
        cursor: not-allowed;
    }
    .place-bttn-place:disabled .tab-text {
        @apply text-xl;
        opacity: 0.5;
    }

    .place-bttn-delete {
        border-radius: 16px;
        background: #de2d30;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #851b1d inset,
            4px 4px 0px 8px #f24980 inset;
    }
    .place-bttn-delete:not(:disabled):active {
        border-radius: 16px;
        background: #b91d20;
        box-shadow:
            0px 0px 0px 4px #fff inset,
            0px 0px 0px 8px #000 inset,
            -4px -4px 0px 8px #6a1617 inset,
            4px 4px 0px 8px #eb1158 inset;
    }
    .place-bttn-delete:disabled {
        cursor: not-allowed;
    }
    .place-bttn-delete:disabled .tab-text {
        @apply text-xl;
        opacity: 0.5;
    }

    @media screen(md) {
        .pd-button {
            width: 180px;
        }
    }

    @media screen(sm) {
        .pd-button {
            width: 100%;
        }
        .pd-button[data-minimised="0"] {
            height: 64px;
        }
    }
</style>
