<script lang="ts">
    import Image from "../components/Image.svelte";
    import { ExclusiveMenus, openMenu } from "../stores";

    import settingsButtonUrl from "./assets/settings.png?url";
</script>

<button
    class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
    on:click={() => {
        $openMenu =
            $openMenu == ExclusiveMenus.Settings
                ? null
                : ExclusiveMenus.Settings;
    }}
>
    <Image src={settingsButtonUrl} class="object-contain aspect-square"></Image>
</button>
