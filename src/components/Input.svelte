<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatcher = createEventDispatcher();

    type Validator = ((value: string) => boolean) | RegExp | null;

    export let maxLength: number;
    export let hardValidInput: Validator = null;
    export let softValidInput: Validator = null;
    export let defaultValue = "";
    export let tabIndex: number = 0;
    export let autoTrim: boolean = false;

    export let value: any = defaultValue;

    let innerValue = value;
    $: innerValue = value;
    let prevTypeableValue = value;

    let handleChange = () => {
        if (autoTrim) {
            innerValue = innerValue.trim();
        }
        if (checkIfValid(innerValue, hardValidInput)) {
            if (checkIfValid(innerValue, softValidInput)) {
                value = innerValue;
                dispatcher("change");
            }
            prevTypeableValue = innerValue;
        } else {
            innerValue = prevTypeableValue;
        }
    };

    $: {
        innerValue;
        handleChange();
    }

    const checkIfValid = (v: string, validator: Validator) => {
        if (validator == null) {
            return true;
        }
        if (typeof validator == "function") {
            return validator(v);
        } else {
            return validator.test(v);
        }
    };
    // const dispatcher = createEventDispatcher();
</script>

<input
    type="text"
    maxlength={maxLength}
    bind:value={innerValue}
    tabindex={tabIndex}
    {...$$restProps}
/>
