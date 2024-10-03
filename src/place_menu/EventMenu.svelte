<script lang="ts">
    import { get } from "svelte/store";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import {
        eventElapsed,
        ExclusiveMenus,
        openMenu,
        userCount,
    } from "../stores";

    export let kind: "pre-event" | "login-to-place";

    import profileInUrl from "../login/assets/profile_in.png";
    import loadingAnimSvgUrl from "./assets/loading_anim.svg?url";
    import Image from "../components/Image.svelte";

    let seconds_left = 0;
    let switch_hours = "";
    let switch_minutes = "";
    let switch_seconds = "";

    import * as wasm from "wasm-lib";
    import { moveCamera } from "../level_view/view_controls";
    export let state: wasm.State;

    let creator_names = "";

    $: {
        if ($eventElapsed != Number.NEGATIVE_INFINITY) {
            seconds_left = -($eventElapsed / 1000);

            const names = state.get_countdown_creator_names();
            if (
                !(
                    names[0] === "Spu7Nix" &&
                    names[1] === "Spu7Nix" &&
                    names[2] === "Spu7Nix" &&
                    names[3] === "Spu7Nix"
                )
            ) {
                if (seconds_left < 3600 * 24) {
                    creator_names = `${names[1]}, ${names[2]}, and ${names[3]}`;
                } else {
                    creator_names = `${names[0]}, ${names[1]}, ${names[2]}, and ${names[3]}`;
                }
            }

            // if u change this also change it in the wasm :3
            const next_switch = (seconds_left - 600) % 1200;

            if (Number.isNaN(next_switch) || next_switch < 0) {
                switch_hours = "00";
                switch_minutes = "00";
                switch_seconds = "00";
            } else {
                switch_hours = Math.floor(next_switch / 3600)
                    .toFixed(0)
                    .padStart(2, "0");
                switch_minutes = Math.floor((next_switch % 3600) / 60)
                    .toFixed(0)
                    .padStart(2, "0");
                switch_seconds = Math.floor(next_switch % 60)
                    .toFixed(0)
                    .padStart(2, "0");
            }
        }
    }
</script>

<div
    class="absolute flex flex-col items-center justify-end w-full pointer-events-none pre-event-menu"
>
    {#if kind === "pre-event"}
        <div
            class="pointer-events-auto flex flex-col items-center menu-panel justify-evenly w-full h-[200px] max-w-[90rem]"
        >
            <div
                class="grid overflow-hidden grid-cols-[min-content_1fr_min-content] sm:grid-rows-[1fr_min-content] sm:grid-cols-none justify-evenly items-center w-full h-full p-4 xs:p-2 gap-8 sm:gap-4"
            >
                <div class="h-full pl-20 md:pl-4 w-max loading_icon sm:hidden">
                    <Image src={loadingAnimSvgUrl} class="h-full max-h-max" />
                </div>

                <h1
                    class="flex flex-wrap gap-2 px-4 text-3xl text-center text-white md:text-2xl flex-center xs:text-xl"
                >
                    <span class="text-6xl md:text-5xl xs:text-4xl">
                        {$userCount}
                    </span>
                    <span>creators have signed up</span>
                </h1>

                <button
                    class="flex flex-col items-center justify-around h-full gap-1 p-2 text-center sm:flex-row menu-panel w-96 md:w-80 sm:w-full justify-self-end"
                    style={`
                        opacity: ${creator_names ? 1 : 0};
                        scale: ${creator_names ? 1 : 0.7};
                        transition: opacity 0.5s, scale 0.5s;
                        background-color: #ffffff06;
                    `}
                    on:click={() => {
                        moveCamera(
                            state,
                            450 + 30 * 20.5,
                            seconds_left < 3600 * 24 ? 300 : 450
                        );
                    }}
                >
                    <h1
                        class="text-xl font-bold text-white md:text-lg sm:text-base xs:text-sm"
                    >
                        Countdown designs by:
                    </h1>

                    <div
                        class="text-lg text-white opacity-80 md:text-base sm:text-sm xs:text-xs"
                    >
                        {creator_names}
                    </div>

                    {#if seconds_left > 600}
                        <div
                            class="italic text-center text-white opacity-50 text-md md:text-sm sm:text-xs xs:text-xs tabular-nums"
                        >
                            Next design switch: {switch_hours}:{switch_minutes}:{switch_seconds}
                        </div>
                    {/if}

                    <div
                        class="text-xs text-white opacity-40 md:text-xxs sm:text-xxxs xs:hidden"
                    >
                        (colons made by GD Colon)
                    </div>
                </button>
            </div>
        </div>
    {:else if kind === "login-to-place"}
        <div
            class="flex flex-col items-center justify-center w-full h-[200px] pointer-events-none"
        >
            <div
                class="pointer-events-auto flex flex-col items-center menu-panel justify-evenly w-2/3 sm:w-3/4 h-[150px]"
            >
                <div class="flex flex-col h-full gap-2">
                    <button
                        class="flex flex-row items-center w-full h-full gap-5 p-4 text-4xl text-center text-white justify-items-evenly sm:gap-2 md:text-3xl sm:text-2xl xs:text-xl flex-center font-pusab"
                        on:click={() => {
                            $openMenu = ExclusiveMenus.Login;
                        }}
                    >
                        <Image
                            src={profileInUrl}
                            class="object-contain aspect-square h-3/4"
                        ></Image>

                        Sign in to participate!
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>

<style lang="postcss">
    .pre-event-menu {
        height: 100svh;
        padding: 8px;
    }

    .loading_icon {
        mask-image: linear-gradient(
            transparent 0%,
            black 10%,
            black 90%,
            transparent 100%
        );
    }
</style>
