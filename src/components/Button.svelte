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
        [`gap-2 rounded-lg flex-center ${$$restProps["class"]}`]: true,
        "white-button": type !== "plain",
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
