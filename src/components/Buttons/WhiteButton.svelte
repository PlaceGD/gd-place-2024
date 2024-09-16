<script lang="ts">
    import { default as cx } from "classnames";
    import IconButton from "./IconButton.svelte";
</script>

<IconButton
    {...$$restProps}
    class={cx({
        "py-2": true,
        [$$restProps["class"]]: true,
    })}
    on:click
>
    <slot slot="children" />
</IconButton>
