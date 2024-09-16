<script lang="ts">
    import { default as cx } from "classnames";

    export let align: "left" | "right" = "left";
</script>

<button
    {...$$restProps}
    class={cx({
        "rounded-lg relative flex flex-center white-button px-2 gap-1": true,
        [$$restProps["class"]]: true,
        "grid-cols-[min-content_1fr]": align === "left",
        "grid-cols-[1fr_min-content]": align === "right",
    })}
    on:click
>
    {#if align === "left"}
        <slot name="left" />
    {/if}
    <slot name="children" />
    {#if align === "right"}
        <slot name="right" />
    {/if}
</button>
