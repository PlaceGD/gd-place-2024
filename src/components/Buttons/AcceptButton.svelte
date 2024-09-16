<script lang="ts">
    import Check from "../../icons/Check.svelte";
    import IconButton from "./IconButton.svelte";
</script>

<IconButton {...$$restProps} on:click>
    <Check
        slot="left"
        class="stroke-[1] text-[#47ff47] xs:w-7 xs:h-7 sm:w-9 sm:h-9 w-10 h-10"
    />
    <slot slot="children" />
</IconButton>
