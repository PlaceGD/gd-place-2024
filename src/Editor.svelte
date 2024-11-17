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
</script>

<!-- <button
    class="absolute z-50 text-xl text-white bg-black"
    on:click={() => ($DEBUG_ENDING_VISIBILITY = !$DEBUG_ENDING_VISIBILITY)}
    >SHOW/HIDE ENDING</button
> -->

<div class="absolute w-full h-full">
    {#if state != null && $eventStatus != "loading"}
        <Guide {state} />

        <div
            class="absolute top-0 right-0 flex flex-col items-end w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-row-reverse justify-end gap-4 p-2 xs:gap-2 pointer-events-all"
            >
                {#if $eventStatus == "fully done" && !$viewingLevelAfterEvent}
                    <ViewLevelButton bind:state />
                {/if}

                {#if $eventStatus == "during" || $eventStatus == "before"}
                    <SettingsButton />
                {/if}

                {#if state != null && $eventStatus == "during"}
                    <ModButton />
                {/if}
                <MetaButton />
                {#if $eventStatus == "during" || $eventStatus == "before"}
                    <LoginButton />
                {/if}
            </div>
            <Login />

            {#if $canPlaceEditDelete}
                <NameGradient />
            {/if}

            {#if state != null}
                <ReportedUserList bind:state />
            {/if}
            <SettingsOptions />
            <MetaMenu />
        </div>
        {#if $eventStatus == "during" || $eventStatus == "before"}
            <div
                class="absolute top-0 right-0 flex flex-row items-start w-full h-full gap-4 pointer-events-none xs:flex-col sm:gap-2"
            >
                <div
                    class="flex flex-col justify-end gap-4 p-2 xs:gap-2 pointer-events-all"
                    data-guide="zoom"
                >
                    <ZoomButton zoom="in" {canvas} />
                    <ZoomButton zoom="out" {canvas} />
                    <SongStopButton />
                </div>
                <EndCountdown />
            </div>
        {/if}
    {/if}
    {#if wasmLoaded}
        {#if showEnding && state != null && !$viewingLevelAfterEvent}
            <SharedEnding bind:state />
        {/if}
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
        {#if $eventStatus == "before" || $eventStatus == "during"}
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
        {/if}
    {/if}
</div>
