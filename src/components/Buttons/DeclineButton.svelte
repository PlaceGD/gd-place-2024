<script lang="ts">
    import Cross from "../../icons/Cross.svelte";
    import IconButton from "./IconButton.svelte";
</script>

<IconButton {...$$restProps} on:click>
    <Cross
        slot="right"
        class="stroke-[1] text-[#ff4747] xs:w-7 xs:h-7 sm:w-9 sm:h-9 w-10 h-10"
    />
    <slot slot="children" />
</IconButton>
