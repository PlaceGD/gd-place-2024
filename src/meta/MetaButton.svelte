<script lang="ts">
    import { DEV_UIDS } from "shared-lib/cloud_functions";
    import Image from "../components/Image.svelte";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import metaButtonUrl from "./assets/meta_button.png";

    $: canShow = __DEBUG
        ? true
        : DEV_UIDS.includes($loginData.currentUserData?.user.uid ?? "");
</script>

{#if $loginData.currentUserData}
    {#if canShow}
        <div class="relative gap-2 flex-center">
            <button
                class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
                id="mod-button"
                on:click={() => {
                    $openMenu =
                        $openMenu == ExclusiveMenus.Meta
                            ? null
                            : ExclusiveMenus.Meta;
                }}
            >
                <Image
                    src={metaButtonUrl}
                    class="object-contain aspect-square"
                />
            </button>
        </div>
    {/if}
{/if}
