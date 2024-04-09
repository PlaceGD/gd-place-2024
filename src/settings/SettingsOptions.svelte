<script lang="ts">
    import FadedScroll from "../components/FadedScroll.svelte";
    import { showSetttingsOptions } from "../stores";

    export let editorFocused: boolean;

    $: {
        if ($showSetttingsOptions && editorFocused) {
            $showSetttingsOptions = false;
        }
    }

</script>

{#if $showSetttingsOptions}
    <div
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-lg text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
    >
        <!-- Faded scroll just for fanciness -->
        <FadedScroll>
            <ul>
                <!-- TODO(sputnix) add the settings here -->
            </ul>
        </FadedScroll>
    </div>
{/if}
