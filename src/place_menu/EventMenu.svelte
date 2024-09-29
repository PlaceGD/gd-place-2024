<script lang="ts">
    import { get } from "svelte/store";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { eventElapsed, ExclusiveMenus, openMenu, userCount } from "../stores";

    export let kind: "pre-event" | "login-to-place";

    import profileInUrl from "../login/assets/profile_in.png";
    import loadingAnimSvg from "./assets/loadinganimcss.svg";
    import Image from "../components/Image.svelte";

    let seconds_left = 0;
    let switch_hours = "";
    let switch_minutes = "";
    let switch_seconds = "";

    import * as wasm from "wasm-lib";
    export let state: wasm.State;

    let creator_names = "unknown";

    $: {
        seconds_left = -($eventElapsed / 1000);

        const names = state.get_countdown_creator_names();

        if (seconds_left < 3600 * 24) {
            creator_names = `${names[1]}, ${names[2]}, and ${names[3]}`;
        } else {
            creator_names = `${names[0]}, ${names[1]}, ${names[2]}, and ${names[3]}`;
        }

        
        
        // if u change this also change it in the wasm :3
        const next_switch = (seconds_left - 1800) % 3600;

        if (Number.isNaN(next_switch) || next_switch < 0) {
            switch_hours = "00";
            switch_minutes = "00";
            switch_seconds = "00";
        } else {
            switch_hours = Math.floor(next_switch / 3600).toFixed(0).padStart(2, "0");
            switch_minutes = Math.floor((next_switch % 3600) / 60).toFixed(0).padStart(2, "0");
            switch_seconds = Math.floor(next_switch % 60).toFixed(0).padStart(2, "0");
        }
    }
</script>

<div
    class="absolute flex flex-col justify-end items-center w-full pointer-events-none pre-event-menu"
>
    
    {#if kind === "pre-event"}

        <div class="flex flex-col items-center menu-panel justify-evenly w-full h-[200px] max-w-[90rem]">
            <div class="grid flex-row grid-cols-4 justify-evenly items-center w-full h-full sm:flex-col px-5">
                <!-- temporary loading animation -->
                <!-- <div class="loading_icon sm:hidden max-h-[170px] max-w-sm">
                    <Image
                        src={loadingAnimSvg}
                        class="justify-center items-center col-span-1"
                        style="max-height: inherit;"
                    ></Image>
                </div> -->
                <div class="flex flex-row justify-center items-center gap-5 col-span-3 sm:col-span-4">
                    <h1 class="text-white text-6xl xs:text-4xl">
                        {$userCount}
                    </h1>
                    <h1 class="text-white text-3xl xs:text-xl">creators have signed up</h1>
                </div>

                <div 
                    class="flex flex-col items-center gap-5 w-full h-[90%] menu-panel p-6 sm:w-[90%] sm:h-full sm:p-6 col-span-1 sm:hidden max-h-min"
                >
                    <h1 class="text-white text-xl md:text-xl font-bold">
                        Countdown designs by:
                    </h1>
                    
                    <div class="text-white opacity-80 text-lg md:text-md text-center">
                        {creator_names}
                    </div>
                    {#if seconds_left > 1800}
                        <div class="text-white opacity-50 text-md md:text-sm text-center italic">
                            Next design switch: {switch_hours}:{switch_minutes}:{switch_seconds}
                        </div>
                    {/if}
                </div>
            </div>
        </div>

    
    {:else if kind === "login-to-place"}
        <div class="flex flex-col items-center justify-center w-full h-[200px] pointer-events-none">
            <div class="flex flex-col items-center menu-panel justify-evenly w-2/3 sm:w-3/4 h-[150px]">
                <div class="flex flex-col gap-2 pointer-events-auto h-full">
                    <button
                        class="flex flex-row justify-items-evenly items-center gap-5 sm:gap-2 text-white w-full h-full p-4 text-4xl text-center md:text-3xl sm:text-2xl xs:text-xl flex-center font-pusab"
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
        -webkit-mask-image: linear-gradient(
            90deg,
            #00000000 15%,
            #fff 50%,
            #00000000 80%
        );
        mask-image: linear-gradient(
            90deg,
            #00000000 15%,
            #fff 50%,
            #00000000 80%
        );
    }
</style>
