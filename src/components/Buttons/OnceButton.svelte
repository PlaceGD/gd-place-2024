<script lang="ts">
    let isDisabled = false;

    export let userDisabled = false;

    export let click = () => {
        isDisabled = true;
    };

    export const reset = () => {
        isDisabled = false;
    };
</script>

<slot disabled={isDisabled || userDisabled} {click} {reset} />
