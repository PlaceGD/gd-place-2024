<script lang="ts">
    import Image from "../components/Image.svelte";
    import { loginData, showModeratorOptions } from "../stores";
</script>

{#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.currentUserData.placeData.moderator}
    <div class="gap-2 flex-center relative">
        <button
            class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
            id="mod-button"
            on:click={() =>
                ($showModeratorOptions.show = !$showModeratorOptions.show)}
        >
            <Image
                src="/assets/ui/report/report_button.png"
                class="object-contain aspect-square"
            />
        </button>
        {#if $showModeratorOptions.newReports}
            <Image
                src="/assets/ui/report/new_report.png"
                class="absolute translate-x-1/2 translate-y-1/2 top-1/2 left-1/2 object-contain aspect-square z-30 w-6 w-6 pointer-events-none"
            />
        {/if}
    </div>
{/if}
