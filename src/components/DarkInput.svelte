<script lang="ts">
    import { onDestroy } from "svelte";
    import Input from "./Input.svelte";

    type Validator = ((value: string) => boolean) | RegExp | null;

    export let maxLength: number;
    export let hardValidInput: Validator = null;
    export let softValidInput: Validator = null;
    export let defaultValue = "";
    export let tabIndex: number = 0;
    export let autoTrim: boolean = false;

    export let value: any = defaultValue;

    onDestroy(() => {
        console.log("Uh penis");
    });
</script>

<Input
    {maxLength}
    {hardValidInput}
    {softValidInput}
    {defaultValue}
    {tabIndex}
    {autoTrim}
    on:change
    on:focus
    on:blur
    bind:value
    {...$$restProps}
    class="p-2 text-center rounded-lg outline-none text-stroke xs:p-1 bg-black/40 outline-2 outline outline-white/20 -outline-offset-2 {$$restProps[
        'class'
    ]}"
/>
