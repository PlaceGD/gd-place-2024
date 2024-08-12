<script lang="ts">
    import { onMount } from "svelte";
    import FadedScroll from "../components/FadedScroll.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import {
        editorSettings,
        ExclusiveMenus,
        loginData,
        openMenu,
    } from "../stores";
    import Button from "../components/Button.svelte";

    export let editorFocused: boolean;

    $: {
        if ($openMenu == ExclusiveMenus.Settings && editorFocused) {
            $openMenu = null;
        }
    }

    const KOFI_ID = "Z8Z410GRY2";

    onMount(() => {
        window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);
    });
</script>

{#if $openMenu == ExclusiveMenus.Settings}
    <div
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
    >
        <div
            class="grid-rows-[minmax(0,_1fr)_min-content] grid gap-2 px-2 py-1 divide-y divide-white/10 w-full h-full overflow-hidden thin-scrollbar"
        >
            <!-- Faded scroll just for fanciness -->
            <FadedScroll>
                <ul class="flex flex-col xs:gap-1 gap-2 alternating-bg">
                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-danger"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Show Collidable Objects</span>
                                <span class="text-xs text-white/50"
                                    >Highlights the objects in the level that
                                    you can collide with.</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-danger"
                                    bind:isToggled={$editorSettings.showCollidable}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>

                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-triggers"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Hide Triggers</span>
                                <span class="text-xs text-white/50"
                                    >Hide triggers in the editor</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-triggers"
                                    bind:isToggled={$editorSettings.hideTriggers}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>
                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-triggers"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Hide Grid</span>
                                <span class="text-xs text-white/50"
                                    >Hide the editor grid</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-triggers"
                                    bind:isToggled={$editorSettings.hideGrid}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>
                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-triggers"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Hide Ground</span>
                                <span class="text-xs text-white/50"
                                    >Hides the ground</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-triggers"
                                    bind:isToggled={$editorSettings.hideGround}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>
                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-triggers"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Hide Selection Outline</span>
                                <span class="text-xs text-white/50"
                                    >Hide the object selection outline</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-triggers"
                                    bind:isToggled={$editorSettings.hideOutline}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>
                    <li
                        class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                    >
                        <label
                            for="show-triggers"
                            class="grid w-full items-center grid-cols-[1fr_min-content]"
                        >
                            <div class="flex flex-col">
                                <span>Hide Delete Text</span>
                                <span class="text-xs text-white/50"
                                    >Hides the text that appears when an object
                                    is deleted</span
                                >
                            </div>
                            <span>
                                <ToggleSwitch
                                    id="show-triggers"
                                    bind:isToggled={$editorSettings.hideDeleteText}
                                ></ToggleSwitch>
                            </span>
                        </label>
                    </li>
                </ul>
            </FadedScroll>
            <!-- TODO: get stream link from database -->
            {#if false}
                <div class="flex flex-col items-center justify-center p-2">
                    <p class="text-base">Watch the official stream live!</p>

                    <a href="sd" title="Support us"> STREAM LINKT</a>
                </div>
            {/if}
            <div
                class="flex flex-col items-center justify-center gap-2 p-2 pb-0 text-center"
            >
                {#if $loginData.currentUserData?.userDetails?.hasDonated}
                    <Button
                        type="white"
                        class="p-2 text-base xs:text-sm"
                        on:click={() => {
                            $openMenu = ExclusiveMenus.Kofi;
                        }}>Update Name Gradient</Button
                    >

                    <a
                        href={`https://www.ko-fi.com/${KOFI_ID}`}
                        target="_blank"
                        rel="noreffer"
                        class="text-sm text-center underline transition duration-500 text-white/50 hover:text-white hover:decoration-dashed"
                    >
                        Donate!
                    </a>
                {:else}
                    <p class="text-base">
                        Want to stand out with a colored name and support the
                        project? Make a small donation to our kofi!
                    </p>

                    <span title="Support us">
                        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                        {@html window.kofiwidget2
                            .getHTML()
                            .replace(
                                "Support me on ko-fi.com",
                                "Support us!"
                            )}</span
                    >
                    <Button
                        type="plain"
                        class="p-1 text-sm text-center underline transition duration-500 text-white/50 hover:text-white hover:decoration-dashed"
                        on:click={() => {
                            $openMenu = ExclusiveMenus.Kofi;
                        }}
                    >
                        Submit Transaction Id
                    </Button>
                {/if}
            </div>
        </div>
    </div>
{/if}
