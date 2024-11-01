<script lang="ts">
    import { default as cx } from "classnames";
    import FadedScroll from "../components/FadedScroll.svelte";
    import Radio from "../components/Radio.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import {
        editorSettings,
        eventStarted,
        ExclusiveMenus,
        loginData,
        openMenu,
    } from "../stores";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { menuHeight } from "../utils/transitions";
    import { onMount } from "svelte";
    import { GUIDE_ELEM_IDS } from "../guide/guide";
    import GuidePopup from "../guide/GuidePopup.svelte";
    import { fade } from "svelte/transition";

    $: isOpen = $openMenu == ExclusiveMenus.Settings;

    const KOFI_ID = "Z8Z410GRY2";

    let widgetString = "";
    onMount(() => {
        window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);
        widgetString = window.kofiwidget2
            .getHTML()
            .replace("Support me on ko-fi.com", "Support us!");
    });

    const settings: {
        bind: Exclude<keyof typeof $editorSettings, "quality">;
        name: string;
        guide?: string;
    }[] = [
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
            name: "Show who Deleted an Object",
            // description: "Hide the text that appears when an object is deleted",
            bind: "showDeleteTextI",
        },
        {
            name: "Show who Placed an Object",
            // description: "Hide the text that appears when an object is deleted",
            bind: "showPlacedTextI",
        },
    ];

    // just here so the faded scroll correctly updates with the animating height
    let transitionVal = 0;
</script>

{#if isOpen}
    <fieldset
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center h-[50%] pointer-events-auto"
        inert={!isOpen}
        transition:menuHeight={{ duration: 200 }}
        style={`max-height: ${settings.length * 70 + 140 + 70}px;`}
        on:introend={() => (transitionVal = 1)}
        on:outroend={() => (transitionVal = 0)}
    >
        <div
            class="grid-rows-[minmax(0,_1fr)_min-content] grid gap-2 px-2 py-1 divide-y divide-white/10 w-full h-full overflow-hidden thin-scrollbar"
        >
            <!-- Faded scroll just for fanciness -->
            <FadedScroll
                update={transitionVal}
                scrollY={transitionVal === 1
                    ? "overflow-y-auto"
                    : "overflow-y-hidden"}
            >
                <ul class="flex flex-col gap-2 xs:gap-1 alternating-bg">
                    {#each settings as setting}
                        <li
                            class="flex flex-col gap-1 p-3 rounded-lg li-alternating"
                            data-guide={setting.guide ?? ""}
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
                class="grid grid-cols-[min-content_1fr] gap-2 p-2 pb-0 flex-center"
            >
                <div class="flex flex-col justify-center">
                    <p class="whitespace-nowrap xs:whitespace-normal">
                        Canvas Resolution
                    </p>
                </div>
                <div
                    class="flex gap-6 justify-self-center xs:gap-4 flex-center w-min"
                >
                    <label
                        for="low-quality"
                        class="flex flex-col gap-2 cursor-pointer flex-center"
                    >
                        <Radio
                            id="low-quality"
                            name="quality"
                            on:click={() => ($editorSettings.quality = "low")}
                            checked={$editorSettings.quality === "low"}
                        />
                        <span class="text-sm text-center">Low</span>
                    </label>
                    <label
                        for="medium-quality"
                        class="flex flex-col gap-2 cursor-pointer flex-center"
                    >
                        <Radio
                            id="medium-quality"
                            name="quality"
                            on:click={() =>
                                ($editorSettings.quality = "medium")}
                            checked={$editorSettings.quality === "medium"}
                        />
                        <span class="text-sm text-center">Medium</span>
                    </label>
                    <label
                        for="high-quality"
                        class="flex flex-col gap-2 cursor-pointer flex-center"
                    >
                        <Radio
                            id="high-quality"
                            name="quality"
                            on:click={() => ($editorSettings.quality = "high")}
                            checked={($editorSettings.quality ?? "high") ===
                                "high"}
                        />
                        <span class="text-sm text-center">High</span>
                    </label>
                </div>
            </div>

            {#if $eventStarted && $loginData?.currentUserData?.userDetails != null}
                <div class="p-2 pb-0">
                    <GuidePopup></GuidePopup>
                </div>
            {/if}

            <div
                class="flex flex-col items-center justify-center gap-2 p-2 pb-0 text-center"
                data-guide={GUIDE_ELEM_IDS.settingsMenuDonate}
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
                {:else if $loginData?.currentUserData?.userDetails != null}
                    <p class="text-sm">
                        Get a colored name by making a donation (any amount)!
                    </p>
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
                        <span title="Support us">
                            {#if isOpen}
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html widgetString}
                            {/if}
                        </span>
                    </button>
                {/if}
            </div>
        </div>
    </fieldset>
{:else if transitionVal == 0}
    <span
        class="text-white z-50 font-bold mr-4 opacity-40 xs:opacity-60 text-sm xs:text-xs text-right"
        in:fade={{ duration: 500 }}
    >
        {#each [$editorSettings.showCollidable ? "showing only collidable objects" : null, $editorSettings.hideTriggers ? "hiding triggers" : null].filter(v => v != null) as t}
            <span>⚙️ {t}</span><br />
        {/each}
    </span>
{/if}
