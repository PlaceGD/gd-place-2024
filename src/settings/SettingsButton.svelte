<script lang="ts">
    import Image from "../components/Image.svelte";
    import { showSettingsOptions } from "../stores";
</script>

<button
    class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
    on:click={() => {
        $showSettingsOptions = !$showSettingsOptions;
    }}
>
    <Image src="/assets/ui/settings.png" class="object-contain aspect-square"
    ></Image>
</button>
