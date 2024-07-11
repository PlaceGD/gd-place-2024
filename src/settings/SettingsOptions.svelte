<script lang="ts">
    import FadedScroll from "../components/FadedScroll.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import { ExclusiveMenus, openMenu } from "../stores";

    export let editorFocused: boolean;

    $: {
        if ($openMenu == ExclusiveMenus.Settings && editorFocused) {
            $openMenu = null;
        }
    }
</script>

{#if $openMenu == ExclusiveMenus.Settings}
    <div
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-lg text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
    >
        <!-- Faded scroll just for fanciness -->
        <FadedScroll>
            <ul class="px-2 py-1">
                <!-- TODO(sputnix) add the settings here -->
                <li class="flex flex-col gap-1 li-alternating p-2 rounded-lg">
                    <label
                        for="show-danger"
                        class="grid w-full items-center grid-cols-[1fr_min-content]"
                    >
                        <div class="flex flex-col">
                            <span>Show Danger Objects</span>
                            <span class="text-white/50 text-xs"
                                >Highlights the objects in the level that can
                                kill you.</span
                            >
                        </div>
                        <span>
                            <ToggleSwitch id="show-danger" isToggled={false}
                            ></ToggleSwitch>
                        </span>
                    </label>
                </li>
            </ul>
        </FadedScroll>
    </div>
{/if}
