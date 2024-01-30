<script lang="ts">
    import Image from "../components/Image.svelte";
    import { loginData, showModeratorOptions } from "../stores";
</script>

{#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.currentUserData.placeData.moderator}
    <div class="gap-2 flex-center">
        <button
            class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
            on:click={() => ($showModeratorOptions = true)}
        >
            <Image
                src="/assets/ui/report/report_button.png"
                class="object-contain aspect-square"
            ></Image>
        </button>
    </div>
{/if}
