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

    window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);

    // onMount(() => {
    //     window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);
    // });

    const settings = [
        {
            name: "Show Collidable Objects",
            description: "Highlights the objects in the level that you can collide with.",
            bind: "showCollidable",
        },
        {
            name: "Hide Triggers",
            description: "Hide triggers in the editor",
            bind: "hideTriggers",
        },
        {
            name: "Hide Grid",
            description: "Hide the editor grid",
            bind: "hideGrid",
        },
        {
            name: "Hide Ground",
            description: "Hides the ground",
            bind: "hideGround",
        },
        {
            name: "Hide Selection Outline",
            description: "Hide the object selection outline",
            bind: "hideOutline",
        },
        {
            name: "Hide Delete Text",
            description: "Hide the text that appears when an object is deleted",
            bind: "hideDeleteText",
        },
    ];
</script>


    <div
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
        style={`
                height: ${$openMenu != ExclusiveMenus.Settings ? "0" : "50vh"};
                opacity: ${$openMenu != ExclusiveMenus.Settings ? "0" : "1"};
                transition: height 0.2s ease, opacity 0.2s ease-out;
            `}
    >
        <div
            class="grid-rows-[minmax(0,_1fr)_min-content] grid gap-2 px-2 py-1 divide-y divide-white/10 w-full h-full overflow-hidden thin-scrollbar"
            style={`
                opacity: ${$openMenu != ExclusiveMenus.Settings ? "0" : "1"};
                transition: opacity 0.1s;
                transition-delay: 0.1s;
            `}
        >
            <!-- Faded scroll just for fanciness -->
            <FadedScroll>
                <ul class="flex flex-col xs:gap-1 gap-2 alternating-bg">

                    {#each settings as setting}
                        <li
                            class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                        >
                            <label
                                for={setting.bind}
                                class="grid w-full items-center grid-cols-[1fr_min-content]"
                            >
                                <div class="flex flex-col">
                                    <span>{setting.name}</span>
                                    <span class="text-xs text-white/50">{setting.description}</span>
                                </div>
                                <span>
                                    <ToggleSwitch
                                        id={setting.bind}
                                        bind:isToggled={$editorSettings[setting.bind]}
                                    ></ToggleSwitch>
                                </span>
                            </label>
                        </li>
                    {/each}
                    
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
                        Get a colored name by making a donation!
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
                        Choose your username colors
                    </Button>
                {/if}
            </div>
        </div>
    </div>

