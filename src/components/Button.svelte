<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Check from "../icons/check.svg";
    import Cross from "../icons/cross.svg";

    export let type: "accept" | "decline" | "plain" = "plain";
    export let iconClass = "w-11 h-11 xs:w-8 xs:h-8";
    export let disabled = false;

    const dispatcher = createEventDispatcher();

    const { class: _class, disabled: _dis, ...restProps } = $$restProps;
</script>

<button
    class="w-full h-full gap-2 p-2 rounded-lg flex-center white-button {$$restProps.class}"
    on:click={() => dispatcher("click")}
    {disabled}
    {...restProps}
>
    {#if type == "accept"}
        <Check class="text-[#47ff47] {iconClass}" />
    {:else if type == "decline"}
        <Cross class="text-[#ff4747] {iconClass}" />
    {/if}

    <slot />
</button>
