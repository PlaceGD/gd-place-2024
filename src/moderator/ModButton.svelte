<script lang="ts">
    import Image from "../components/Image.svelte";
    import { ExclusiveMenus, loginData, newReports, openMenu } from "../stores";
</script>

{#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.currentUserData.placeData.moderator}
    <div class="relative gap-2 flex-center">
        <button
            class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
            id="mod-button"
            on:click={() => {
                if ($openMenu != ExclusiveMenus.Moderator) {
                    $openMenu = ExclusiveMenus.Moderator;
                } else {
                    $openMenu = null;
                }
            }}
        >
            <Image
                src="/assets/ui/report/report_button.png"
                class="object-contain aspect-square"
            />
        </button>
        {#if $newReports}
            <Image
                src="/assets/ui/report/new_report.png"
                class="absolute z-30 object-contain w-6 translate-x-1/2 translate-y-1/2 pointer-events-none top-1/2 left-1/2 aspect-square"
            />
        {/if}
    </div>
{/if}
