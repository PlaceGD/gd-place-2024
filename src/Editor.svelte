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
    import { loginData } from "./stores";
    import NameGradient from "./name_gradient/NameGradient.svelte";

    export let wasmLoaded: boolean;

    let editorFocused: boolean;

    let state: wasm.State | null = null;

    let canvas: HTMLCanvasElement;
    let canvasWidth: number;
    let canvasHeight: number;
</script>

<div class="absolute w-full h-full">
    <div
        class="absolute top-0 right-0 flex flex-col items-end w-full h-full gap-4 pointer-events-none sm:gap-2"
    >
        <div
            class="flex flex-row-reverse justify-end gap-4 p-2 pointer-events-all"
        >
            <SettingsButton />
            {#if state != null}
                <ModButton />
            {/if}
            <LoginButton />
        </div>
        <Login />
        <NameGradient />

        {#if state != null}
            <ReportedUserList bind:state {editorFocused} />
        {/if}
        <SettingsOptions {editorFocused}/>
    </div>
    {#if wasmLoaded}
        <LevelView bind:state bind:canvas bind:canvasHeight bind:canvasWidth />
    {/if}
    {#if state != null}
        <ViewControls bind:state bind:canvas bind:isFocused={editorFocused} />
        <PlaceMenu bind:state />
    {/if}
</div>
