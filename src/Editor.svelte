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
        eventElapsed,
        eventStarted,
        eventStartTime,
        loginData,
        openMenu,
    } from "./stores";
    import NameGradient from "./name_gradient/NameGradient.svelte";
    import ZoomButton from "./level_view/ZoomButton.svelte";
    import MetaButton from "./meta/MetaButton.svelte";
    import MetaMenu from "./meta/MetaMenu.svelte";
    import EventMenu from "./place_menu/EventMenu.svelte";

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
</script>

<div class="absolute w-full h-full">
    {#if state != null}
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
        <div
            class="absolute top-0 right-0 flex flex-col items-start w-full h-full gap-4 pointer-events-none sm:gap-2"
        >
            <div
                class="flex flex-col justify-end gap-4 p-2 xs:gap-2 pointer-events-all"
            >
                <ZoomButton zoom="in" {canvas} />
                <ZoomButton zoom="out" {canvas} />
            </div>
        </div>
    {/if}
    {#if wasmLoaded}
        <LevelView bind:state bind:canvas bind:canvasHeight bind:canvasWidth />
    {/if}
    {#if state != null}
        <ViewControls bind:state bind:canvas bind:isFocused={editorFocused} />

        {#if !$eventStarted}
            <EventMenu kind="pre-event" />
        {:else if $canPlaceEditDelete}
            <PlaceMenu bind:state />
        {:else}
            <EventMenu kind="login-to-place" />
        {/if}
    {/if}
</div>
