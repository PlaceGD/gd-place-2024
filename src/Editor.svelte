<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import PlaceMenu from "./place_menu/PlaceMenu.svelte";
    import * as wasm from "wasm-lib";
    import ViewControls from "./level_view/ViewControls.svelte";
    import ModButton from "./moderator/ModButton.svelte";
    import ReportedUserList from "./moderator/ReportedUserList.svelte";
    import Login from "./login/Login.svelte";
    import LoginButton from "./login/LoginButton.svelte";
    import SettingsButton from "./settings/SettingsButton.svelte";
    import SettingsOptions from "./settings/SettingsOptions.svelte";
    import {
        canPlaceEditDelete,
        eventEndTime,
        eventStartTime,
        eventStatus,
        nowStore,
        openMenu,
        scheduleFor,
        timeLeft,
        viewingLevelAfterEvent,
    } from "./stores";
    import NameGradient from "./name_gradient/NameGradient.svelte";
    import ZoomButton from "./level_view/ZoomButton.svelte";
    import MetaButton from "./meta/MetaButton.svelte";
    import MetaMenu from "./meta/MetaMenu.svelte";
    import EventMenu from "./place_menu/EventMenu.svelte";
    import SongStopButton from "./level_view/SongStopButton.svelte";
    import EndCountdown from "./level_view/EndCountdown.svelte";
    import Guide from "./guide/Guide.svelte";
    import { playSound } from "./utils/audio";

    import SharedEnding from "./ending/SharedEnding.svelte";
    import ViewLevelButton from "./ending/ViewLevelButton.svelte";
    import { onMount } from "svelte";
    import endingAmbientUrl from "./assets/ending_ambient_bg.mp3?url";
    import endingSequenceAmbientUrl from "./assets/ending_sequence_sound_ending_idk_idfk.mp3?url";
    import bigTickUrl from "./assets/big_tick.mp3?url";
    import tickUrl from "./assets/tick.mp3?url";
    import { runtTimelapse, timelapsetime } from "./timelapse";
    import { derived } from "svelte/store";
    import { LEVEL_NAME_DELAY } from "shared-lib/ending";

    // const dick = (v: wasm.Gliberal) => {
    //     v.doink
    // }

    export let wasmLoaded: boolean;

    let editorFocused: boolean = false;

    $: {
        if (editorFocused) {
            $openMenu = null;
        }
    }

    let state: wasm.State | null = null;

    let canvas: HTMLCanvasElement;
    let canvasWidth: number;
    let canvasHeight: number;

    $: showEnding =
        ($eventStatus == "name set" || $eventStatus == "fully done") &&
        $nowStore >= $eventEndTime + LEVEL_NAME_DELAY * 1000;

    //$: console.log($nowStore, $eventEndTime + LEVEL_NAME_DELAY * 1000);

    $: if ($timeLeft > 0.0 && $timeLeft <= 10.0) {
        playSound({
            url: $timeLeft < 1.0 ? bigTickUrl : tickUrl,
            volume: 1.0,
            exclusiveChannel: `tick${Math.round($timeLeft)}`,
        });
    }

    const loopSound = () => {
        if ($eventStatus !== "fully done") {
            playSound({
                url: endingAmbientUrl,
                endCb: () => {
                    loopSound();
                },
                volume: 0.5,
                exclusiveChannel: "ending-ambient",
            });
        }
    };
    let scheduled = false;
    $: if ($eventStatus !== "loading" && !scheduled) {
        scheduleFor(loopSound, eventEndTime, {
            runIfNegative: true,
            delay: 36000,
        });
        scheduled = true;
    }

    // for (let i = 0; i < 11; i++) {
    //     scheduleFor(() => {}, eventEndTime, { delay: -i * 1000 });
    // }

    let endSoundScheduled = false;
    $: if ($nowStore >= 0 && !endSoundScheduled) {
        scheduleFor(
            () => {
                playSound({
                    url: endingSequenceAmbientUrl,
                    volume: 2.0,
                    exclusiveChannel: "ending-sequence",
                });
            },
            eventEndTime,
            {}
        );
        endSoundScheduled = true;
    }

    let seconds_left = 0;
    let minutes_left = 0;
    let hours_left = 0;

    let clock = "00:00";
    let ampm = "";

    $: {
        let time_left =
            (825379786 + 50 * 60 * 60 * 1000 - $timelapsetime) / 1000;
        seconds_left = Math.floor(time_left % 60);
        minutes_left = Math.floor((time_left / 60) % 60);
        hours_left = Math.floor(time_left / 60 / 60);

        let date = new Date($timelapsetime + 1730871820288);
        let hours = date.getHours();
        let minutes = date.getMinutes();
        ampm = hours >= 12 ? "PM" : "AM";
        hours = hours % 12;
        hours = hours ? hours : 12;
        clock = `${hours < 10 ? "0" : ""}${hours}:${minutes < 10 ? "0" : ""}${minutes}`;
    }

    const DISPLAY: string = "time";
</script>

<!-- <button
    class="absolute z-50 text-xl text-white bg-black"
    on:click={() => ($DEBUG_ENDING_VISIBILITY = !$DEBUG_ENDING_VISIBILITY)}
    >SHOW/HIDE ENDING</button
> -->

<div class="absolute w-full h-full">
    {#if DISPLAY == "time"}
        <div
            class="absolute top-0 right-0 flex flex-col items-start w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-row justify-center items-center p-5 xs:gap-2 menu-panel z-50 w-[45%] h-30 gap-5"
                style="border-radius: 0 0 4rem 0;"
            >
                <div class="text-white text-[6rem] opacity-50">Time left:</div>
                <div class="text-white text-[8rem] font-bold tabular-nums">
                    {String(hours_left).padStart(2, "0")}:{String(
                        minutes_left
                    ).padStart(2, "0")}:{String(seconds_left).padStart(2, "0")}
                </div>
            </div>
        </div>
    {/if}

    {#if DISPLAY == "clock"}
        <div
            class="absolute top-0 right-0 flex flex-col items-start w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-row justify-center items-center p-1 xs:gap-2 menu-panel z-50 w-[30%] h-60 gap-3"
                style="border-radius: 0 0 4rem 0; border-right: 3px solid #152b1b; border-bottom: 3px solid #152b1b;"
            >
                <div
                    class="text-[15rem] font-digital tabular-nums italic"
                    style="color: #d3ffcf; text-shadow: 0 0 10px #9fed98;"
                >
                    {clock}
                </div>
                <div class="flex flex-col items-center gap-1">
                    <span
                        class="text-[4rem] font-digital"
                        style="color: #9fed98; text-shadow: 0 0 10px #9fed98; opacity: 0.5; transform: translateY(-2rem);"
                        >{ampm}</span
                    >
                    <span
                        class="text-[1rem]"
                        style="color: #9fed98; text-shadow: 0 0 10px #9fed98; opacity: 0.5;"
                        >GMT+1</span
                    >
                </div>
            </div>
        </div>
    {/if}
    {#if state != null && $eventStatus != "loading"}
        <div
            class="absolute top-0 right-0 flex flex-col items-end w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-row-reverse justify-end gap-4 p-2 xs:gap-2 pointer-events-all"
            >
                {#if $eventStatus == "during" || $eventStatus == "before"}
                    <SettingsButton />
                {/if}
                <ModButton />
                <MetaButton />
                {#if $eventStatus == "during" || $eventStatus == "before"}
                    <LoginButton />
                {/if}
            </div>
            <Login />

            
            <SettingsOptions />
        </div>
    {/if}
    {#if wasmLoaded}
        <!-- {#if showEnding && state != null && !$viewingLevelAfterEvent}
            <SharedEnding bind:state />
        {/if} -->
        {#if $eventStatus != "loading"}
            <LevelView
                bind:state
                bind:canvas
                bind:canvasHeight
                bind:canvasWidth
            />
        {/if}
    {/if}
    {#if state != null && $eventStatus != "loading"}
        <ViewControls bind:state bind:canvas bind:isFocused={editorFocused} />
        {#if $nowStore >= $eventStartTime}
            <div style:display={"contents"}>
                <PlaceMenu bind:state />
            </div>
        {/if}
        {#if $nowStore < $eventStartTime}
            <EventMenu kind="pre-event" bind:state />
        {/if}

        <!-- {#if $eventStatus == "before" || $eventStatus == "during"}
            <div
                style:display={$eventStatus == "during" && $canPlaceEditDelete
                    ? "contents"
                    : "none"}
            >
                <PlaceMenu bind:state />
            </div>

            {#if $eventStatus == "before"}
                <EventMenu kind="pre-event" bind:state />
            {:else if !$canPlaceEditDelete}
                <EventMenu kind="login-to-place" bind:state />
            {/if}
        {/if} -->
    {/if}
</div>
