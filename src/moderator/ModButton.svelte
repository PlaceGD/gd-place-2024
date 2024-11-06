<script lang="ts">
    import Image from "../components/Image.svelte";
    import { ExclusiveMenus, loginData, newReports, openMenu } from "../stores";

    import newReportUrl from "./assets/new_report.png?url";
    import reportButtonUrl from "./assets/report_button.png?url";
</script>

{#if $loginData.currentUserData && $loginData.currentUserData.userDetails && $loginData.currentUserData.userDetails.moderator}
    <div class="relative gap-2 flex-center">
        <button
            class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12 shrink-0"
            id="mod-button"
            on:click={() => {
                $openMenu =
                    $openMenu == ExclusiveMenus.Moderator
                        ? null
                        : ExclusiveMenus.Moderator;
            }}
        >
            <Image src={reportButtonUrl} class="object-contain aspect-square" />
        </button>
        {#if $newReports}
            <Image
                src={newReportUrl}
                class="absolute z-30 object-contain w-6 translate-x-1/2 translate-y-1/2 pointer-events-none top-1/2 left-1/2 aspect-square"
            />
        {/if}
    </div>
{/if}
