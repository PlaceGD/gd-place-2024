<script lang="ts">
    import Button from "./Button.svelte";
    import { createEventDispatcher } from "svelte";

    const dispatcher = createEventDispatcher();

    let isDisabled = false;

    export let disabled = false;

    export const reset = () => {
        isDisabled = false;
    };
</script>

<Button
    {...$$props}
    disabled={disabled || isDisabled}
    on:click={() => {
        isDisabled = true;
        dispatcher("click");
    }}
>
    <slot {reset} />
</Button>
