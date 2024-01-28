<script lang="ts">
    import Image from "./components/Image.svelte";

    import Cross from "./icons/cross.svg";
    import Check from "./icons/check.svg";

    let showReadMore = false;
</script>

<div class="absolute bottom-0 z-50 w-full h-auto p-4 flex-center">
    <div
        class="flex-col w-1/2 gap-3 p-3 text-center text-white rounded-lg shadow-lg sm:text-sm sm:w-full flex-center bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
    >
        <p>
            <b>
                This site uses cookies to protect from spam and abuse, and
                analytics to collect data about the event.
            </b>
        </p>
        <button
            class="p-2 text-lg rounded-lg xs:text-lg white-button"
            aria-label="Read More"
            on:click={() => (showReadMore = true)}
        >
            Read More
        </button>
    </div>
</div>

{#if showReadMore}
    <div class="absolute z-[60] flex flex-col w-full h-full gap-2 flex-center">
        <div
            class="flex flex-col p-4 overflow-hidden text-white rounded-lg shadow-lg w-96 aspect-square xs:w-80 bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
        >
            <p>sfdgfdfdgdfg</p>
            <div class="flex w-full gap-4">
                <button
                    class="w-[inherit] gap-2 p-1 rounded-lg flex-center white-button"
                >
                    <Check class="text-[#47ff47] w-11 h-11 xs:w-8 h-8" />
                    <p class="xs:text-sm w-min">Continue</p>
                </button>
                <button
                    class="w-[inherit] gap-2 p-1 rounded-lg flex-center white-button"
                >
                    <Cross class="text-[#ff4747] w-11 h-11 xs:w-8 h-8" />

                    <p class="xs:text-sm w-min">Disable Analytics</p>
                </button>
            </div>
        </div>
    </div>
    <div
        tabindex="-1"
        class="absolute z-50 w-full h-full backdrop-blur-lg brightness-30"
    ></div>
{/if}
