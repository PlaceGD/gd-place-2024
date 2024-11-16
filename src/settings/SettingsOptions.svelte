<script lang="ts">
    import { default as cx } from "classnames";
    import FadedScroll from "../components/FadedScroll.svelte";
    import Radio from "../components/Radio.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import {
        editorSettings,
        eventStatus,
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
    import { KOFI_ID } from "../utils/misc";
    import KofiButton from "../components/KofiButton.svelte";

    $: isOpen = $openMenu == ExclusiveMenus.Settings;

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
            name: "Select Dangerous Objects",
            bind: "selectDangerous",
        },
        {
            name: "Hide Triggers",
            // description: "Hide triggers in the editor",
            bind: "hideTriggers",
        },
        {
            name: "No Rotating Objects",
            // description: "Hide triggers in the editor",
            bind: "noRotatingObjects",
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
            bind: "showDeleteText",
        },
        {
            name: "Show who Placed an Object",
            // description: "Hide the text that appears when an object is deleted",
            bind: "showPlacedText",
        },
    ];

    // just here so the faded scroll correctly updates with the animating height
    let transitionVal = 0;
</script>

{#if isOpen}
    <fieldset
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center h-[55%] pointer-events-auto"
        inert={!isOpen}
        transition:menuHeight={{ duration: 200 }}
        style={`max-height: ${settings.length * 70 + 100 + 140 + 70}px;`}
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
                    <li
                        class="grid grid-cols-[min-content_1fr] p-3 rounded-lg li-alternating flex-center"
                    >
                        <div class="flex flex-col justify-center">
                            <p class="whitespace-nowrap xs:whitespace-normal">
                                Resolution
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
                                    on:click={() =>
                                        ($editorSettings.quality = "low")}
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
                                    checked={$editorSettings.quality ===
                                        "medium"}
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
                                    on:click={() =>
                                        ($editorSettings.quality = "high")}
                                    checked={($editorSettings.quality ??
                                        "high") === "high"}
                                />
                                <span class="text-sm text-center">High</span>
                            </label>
                        </div>
                    </li>
                </ul>
            </FadedScroll>

            {#if $eventStatus == "during" && $loginData?.currentUserData?.userDetails != null}
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
                    {#if isOpen}
                        <KofiButton />
                    {/if}
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
                        {#if isOpen}
                            <KofiButton />
                        {/if}
                    </button>
                {/if}
            </div>
        </div>
    </fieldset>
{:else if transitionVal == 0}
    <span
        class="z-50 mr-4 text-sm font-bold text-right text-white opacity-40 xs:opacity-60 xs:text-xs"
        in:fade={{ duration: 500 }}
    >
        {#each [$editorSettings.showCollidable ? "showing only collidable objects" : null, $editorSettings.selectDangerous ? "only selecting dangerous objects" : null, $editorSettings.hideTriggers ? "hiding triggers" : null, $editorSettings.noRotatingObjects ? "no rotating objects" : null].filter(v => v != null) as t}
            <span>⚙️ {t}</span><br />
        {/each}
    </span>
{/if}
