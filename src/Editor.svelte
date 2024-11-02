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
        eventElapsedContinuous,
        eventEndTime,
        eventStartTime,
        eventStatus,
        loginData,
        nowStore,
        openMenu,
    } from "./stores";
    import NameGradient from "./name_gradient/NameGradient.svelte";
    import ZoomButton from "./level_view/ZoomButton.svelte";
    import MetaButton from "./meta/MetaButton.svelte";
    import MetaMenu from "./meta/MetaMenu.svelte";
    import EventMenu from "./place_menu/EventMenu.svelte";
    import SongStopButton from "./level_view/SongStopButton.svelte";
    import EndCountdown from "./level_view/EndCountdown.svelte";
    import Guide from "./guide/Guide.svelte";
    import EndingNameInput from "./ending/EndingNameInput.svelte";
    import { playSound } from "./utils/audio";
    import boomUrl from "./assets/boom.mp3?url";
    import endingAmbientUrl from "./assets/ending_ambient_bg.mp3?url";
    import { LEVEL_NAME_DELAY } from "./ending/ending";
    import { scheduleFor } from "shared-lib/util";

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

    $: showEndingNameInput =
        $eventStatus == "name set" &&
        $nowStore >= $eventEndTime + LEVEL_NAME_DELAY * 1000;

    // $: console.log("penis$eventEndTime);

    $: {
        if ($eventStatus == "name set") {
            const loopSound = () => {
                playSound({
                    url: endingAmbientUrl,
                    endCb: () => {
                        if (true) {
                            // change this to check if the enter level name thing is still running :3
                            loopSound();
                        }
                    },
                    volume: 0.5,
                });
            };
            loopSound();

            scheduleFor(() => {
                playSound({
                    url: boomUrl,
                });
            }, $eventEndTime + 11000);
        }
    }
</script>

<div class="absolute w-full h-full">
    {#if state != null}
        <Guide {state} />

        <div
            class="absolute top-0 right-0 flex flex-col items-end w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-row-reverse justify-end gap-4 p-2 xs:gap-2 pointer-events-all"
            >
                <SettingsButton />

                {#if state != null}
                    <ModButton />
                {/if}
                <MetaButton />
                <LoginButton />
            </div>
            <Login />
            <NameGradient />

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
        {#if showEndingNameInput}
            <EndingNameInput />
        {/if}
        <LevelView bind:state bind:canvas bind:canvasHeight bind:canvasWidth />
    {/if}
    {#if state != null}
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
