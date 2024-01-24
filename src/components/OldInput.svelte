<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";

    export let maxLength: number;
    export let validInput: RegExp | ((value: string) => boolean) | any;
    export let defaultValue: string = "";
    export let tabIndex: number = 0;

    export let value: any = defaultValue;

    const dispatcher = createEventDispatcher();

    let prevValidInputData: string = defaultValue;

    const validLayerInput = (s: string): boolean => {
        if (s.trim() == "") {
            return true;
        }
        if (typeof validInput == "function") {
            return validInput(s);
        } else if (validInput instanceof RegExp) {
            return validInput.test(s);
        } else {
            return s == validInput;
        }
    };

    let inputElement: HTMLInputElement;

    const enterIfValid = (e: any): boolean => {
        if (!validLayerInput(e.currentTarget.value)) {
            e.currentTarget.value = prevValidInputData;
            return false;
        } else {
            prevValidInputData = e.currentTarget.value;
            return true;
        }
    };

    export const setValue = (v: any) => {
        value = v;
    };

    onMount(() => {
        inputElement.value = prevValidInputData;
    });
</script>

<input
    type="text"
    maxlength={maxLength}
    bind:value
    on:input={e => {
        if (enterIfValid(e)) {
            dispatcher("input", prevValidInputData);
        }
    }}
    bind:this={inputElement}
    tabindex={tabIndex}
    {...$$restProps}
/>
