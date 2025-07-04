<script lang="ts">
    import {
        SFX_TRIGGER_SOUNDS,
        SONG_TRIGGER_SONGS,
    } from "shared-lib/nexusgen";
    import {
        menuMinimized,
        menuSelectedSFX,
        menuSelectedSong,
        menuSpeed,
        songPlaying,
    } from "../../stores";
    import Image from "../../components/Image.svelte";
    import { playSound } from "../../utils/audio";
    import RangeSlider from "svelte-range-slider-pips";
    import DarkInput from "../../components/DarkInput.svelte";
    import FadedScroll from "../../components/FadedScroll.svelte";
    import { semitonesToFactor } from "shared-lib/util";
    import { onDestroy, onMount } from "svelte";
    import { SFX_ICONS, SFX_SOUNDS, SONG_ICONS, SONG_SOUNDS } from "./sfx_tab";
    import { notNaNAnd } from "../../utils/misc";

    export let tabType: "sfx" | "song";

    let SOUNDS = tabType == "sfx" ? SFX_SOUNDS : SONG_SOUNDS;
    let TRIGGER_AUDIO =
        tabType == "sfx" ? SFX_TRIGGER_SOUNDS : SONG_TRIGGER_SONGS;
    let ICONS = tabType == "sfx" ? SFX_ICONS : SONG_ICONS;

    let isMounted = false;
    onMount(() => (isMounted = true));
    onDestroy(() => (isMounted = false));

    const HARD_VALID_INPUT = /^-?\d*$/;
    const SOFT_VALID_INPUT = (s: string) => {
        return notNaNAnd(s, n => -12 <= n && n <= 12);
    };

    const playTheSound = () => {
        playSound({
            url: SOUNDS[
                TRIGGER_AUDIO[
                    tabType == "sfx" ? $menuSelectedSFX : $menuSelectedSong
                ]
            ],
            exclusiveChannel: `preview ${tabType}`,
            speed: semitonesToFactor($menuSpeed),
            endCb:
                tabType == "song"
                    ? () => {
                          songPlaying.set(false);
                      }
                    : () => {},
            loadCb: () => {
                if (tabType == "song") {
                    songPlaying.set(true);
                }
            },
        });
    };
</script>

<div class="flex flex-col w-full h-full p-4 xs:p-2">
    <div class="flex w-full gap-4 pb-2 xs:gap-2 h-min flex-center">
        <label
            for="speed-slider"
            class="text-xl text-center h-min font-pusab text-stroke md:text-lg sm:text-base"
        >
            Speed
        </label>
        <RangeSlider
            min={-12}
            max={12}
            step={1}
            hoverable={false}
            id="speed-slider"
            pips
            ariaLabels={[tabType == "sfx" ? "SFX Speed" : "Song Speed"]}
            values={[$menuSpeed]}
            on:change={e => {
                $menuSpeed = e.detail.value;
                playTheSound();
            }}
        />
        <DarkInput
            class="w-20 text-2xl font-pusab sm:text-xl xs:text-base"
            maxLength={3}
            hardValidInput={HARD_VALID_INPUT}
            softValidInput={SOFT_VALID_INPUT}
            aria-label={tabType == "sfx"
                ? "SFX Speed Input"
                : "Song Speed Input"}
            bind:value={$menuSpeed}
            on:change={() => {
                playTheSound();
            }}
        ></DarkInput>
    </div>

    <FadedScroll update={isMounted} threshold={1}>
        <ul class="rounded-lg sfx-grid-container">
            {#each TRIGGER_AUDIO as audio_name, id}
                <li class="relative w-16 h-16 md:w-12 md:h-12 xs:w-10 xs:h-10">
                    <button
                        class={"absolute w-full h-full p-3 md:p-2 xs:p-1 z-20"}
                        tabindex={$menuMinimized ? -1 : 0}
                        on:click={() => {
                            if (tabType == "sfx") {
                                $menuSelectedSFX = id;
                            } else {
                                $menuSelectedSong = id;
                            }
                            playTheSound();
                        }}
                    >
                        <Image
                            src={ICONS[audio_name]}
                            lazyLoad
                            class="object-contain w-full h-full"
                        />
                    </button>
                    {#if (tabType == "sfx" ? $menuSelectedSFX : $menuSelectedSong) == id}
                        <span class="absolute w-full h-full sliding-selector"
                        ></span>
                    {/if}
                </li>
            {/each}
        </ul>
    </FadedScroll>

    {#if tabType == "song"}
        <div class="w-full text-sm text-center hover-text-transition">
            <a href="https://incompetech.com" target="_blank">
                Music by Kevin MacLeod
            </a>
        </div>
    {/if}
</div>

<style lang="postcss">
    .sfx-grid-container {
        @apply grid justify-between;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    :global(#speed-slider) {
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        --range-pip: #ffffff4a;
        --range-pip-active: #ffffff4a;
        @apply m-0 w-full;
    }
    :global(#speed-slider .rangeHandle) {
        @apply top-1/2 h-full w-auto rounded-full border-2 border-black p-2;
    }
    :global(#speed-slider .rangeHandle::before) {
        @apply h-full;
    }
    :global(#speed-slider .rangePips .pip.selected) {
        height: 0.4em;
    }

    @media screen(sm) {
        .sfx-grid-container {
            grid-template-columns: repeat(auto-fill, 56px);
        }
    }

    @media screen(xs) {
        .sfx-grid-container {
            grid-template-columns: repeat(auto-fill, 48px);
        }
    }
</style>
