<script lang="ts">
    import { default as cx } from "classnames";
    import { createEventDispatcher } from "svelte";
    import Check from "../icons/check.svg";
    import Cross from "../icons/cross.svg";

    export let type: "accept" | "decline" | "white" | "plain" = "white";
    export let iconClass = "";
    export let disabled = false;

    const dispatcher = createEventDispatcher();

    const { class: _class, disabled: _dis, ...restProps } = $$restProps;
</script>

<button
    class={cx({
        [`rounded-lg relative flex flex-center outline-2 outline outline-white/20 -outline-offset-2 ${$$restProps["class"]}`]: true,
        "white-button": type !== "plain",
        "grid-cols-[min-content_1fr]": type === "accept" || type === "decline",
        "grid-cols-[1fr]": type === "plain" || type === "white",
    })}
    on:click={() => dispatcher("click")}
    {disabled}
    {...restProps}
>
    {#if type == "accept"}
        <Check class="text-[#47ff47] h-full p-1 {iconClass}" />
    {:else if type == "decline"}
        <Cross class="text-[#ff4747] h-full p-1 {iconClass}" />
    {/if}

    <slot />
</button>
