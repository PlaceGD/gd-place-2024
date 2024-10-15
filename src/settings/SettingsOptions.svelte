<script lang="ts">
    import { default as cx } from "classnames";
    import FadedScroll from "../components/FadedScroll.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import {
        editorSettings,
        ExclusiveMenus,
        loginData,
        openMenu,
    } from "../stores";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { menuHeight } from "../utils/transitions";
    import { onMount } from "svelte";

    $: isOpen = $openMenu == ExclusiveMenus.Settings;

    const KOFI_ID = "Z8Z410GRY2";

    let widgetString = "";
    onMount(() => {
        window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);
        widgetString = window.kofiwidget2
            .getHTML()
            .replace("Support me on ko-fi.com", "Support us!");
    });

    const settings: { bind: keyof typeof $editorSettings; name: string }[] = [
        {
            name: "Show Collidable Objects",
            // description:
            //     "Highlights the objects in the level that you can collide with.",
            bind: "showCollidable",
        },
        {
            name: "Hide Triggers",
            // description: "Hide triggers in the editor",
            bind: "hideTriggers",
        },
        {
            name: "Hide Grid",
            // description: "Hide the editor grid",
            bind: "hideGrid",
        },
        {
            name: "Hide Ground",
            // description: "Hides the ground",
            bind: "hideGround",
        },
        {
            name: "Hide Selection Outline",
            // description: "Hide the object selection outline",
            bind: "hideOutline",
        },
        {
            name: "Hide Delete Text",
            // description: "Hide the text that appears when an object is deleted",
            bind: "hideDeleteText",
        },
        {
            name: "Hide Hover Tooltip",
            // description: "Hide the text that appears when an object is deleted",
            bind: "hidePlacedTooltip",
        },
    ];

    // just here so the faded scroll correctly updates with the animating height
    let transitionVal = 0;
</script>

{#if isOpen}
    <fieldset
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center h-[50%] pointer-events-auto"
        disabled={!isOpen}
        transition:menuHeight={{ duration: 200 }}
        on:introend={() => (transitionVal = 1)}
        on:outroend={() => (transitionVal = 0)}
    >
        <div
            class="grid-rows-[minmax(0,_1fr)_min-content] grid gap-2 px-2 py-1 divide-y divide-white/10 w-full h-full overflow-hidden thin-scrollbar"
        >
            <!-- Faded scroll just for fanciness -->
            <FadedScroll update={transitionVal} scrollY="overflow-y-scroll">
                <ul class="flex flex-col gap-2 xs:gap-1 alternating-bg">
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
                                    <!-- <span class="text-xs text-white/50"
                                    >{setting.description}</span
                                > -->
                                </div>
                                <span>
                                    <ToggleSwitch
                                        id={setting.bind}
                                        bind:isToggled={$editorSettings[
                                            setting.bind
                                        ]}
                                    ></ToggleSwitch>
                                </span>
                            </label>
                        </li>
                    {/each}
                </ul>
            </FadedScroll>
            <div
                class="flex flex-col items-center justify-center gap-2 p-2 pb-0 text-center"
            >
                {#if $loginData.currentUserData?.userDetails?.hasDonated}
                    <WhiteButton
                        class="text-base xs:text-sm"
                        on:click={() => {
                            $openMenu = ExclusiveMenus.Kofi;
                        }}
                    >
                        Update Name Gradient
                    </WhiteButton>

                    <a
                        href={`https://www.ko-fi.com/${KOFI_ID}`}
                        target="_blank"
                        rel="noreffer"
                        class="text-sm text-center underline hover-text-transition hover:decoration-dashed"
                    >
                        Donate!
                    </a>
                {:else}
                    <p class="text-sm">
                        Get a colored name by making a donation (any amount)!
                    </p>

                    {#if $loginData?.currentUserData?.userDetails != null}
                        <span title="Support us">
                            {#if isOpen}
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html widgetString}
                            {/if}
                        </span>
                        <button
                            class="p-1 text-sm text-center underline hover-text-transition hover:decoration-dashed"
                            on:click={() => {
                                $openMenu = ExclusiveMenus.Kofi;
                            }}
                        >
                            Choose your username colors
                        </button>
                    {:else}
                        <button
                            class="p-1 text-sm text-center underline hover-text-transition hover:decoration-dashed"
                            on:click={() => {
                                $openMenu = ExclusiveMenus.Login;
                            }}
                        >
                            You must login first!
                        </button>
                    {/if}
                {/if}
            </div>
        </div>
    </fieldset>
{/if}
