<script lang="ts">
    import Input from "./Input.svelte";

    type Validator = ((value: string) => boolean) | RegExp | null;

    export let maxLength: number;
    export let hardValidInput: Validator = null;
    export let softValidInput: Validator = null;
    export let defaultValue = "";
    export let tabIndex: number = 0;
    export let autoTrim: boolean = false;

    export let value: any = defaultValue;
</script>

<Input
    {maxLength}
    {hardValidInput}
    {softValidInput}
    {defaultValue}
    {tabIndex}
    {autoTrim}
    bind:value
    {...$$restProps}
    class="p-2 text-center text-base rounded-lg outline-none text-stroke xs:p-1 bg-black/40 w-full outline-2 outline outline-white/20 -outline-offset-2 {$$restProps[
        'class'
    ]}"
/>
