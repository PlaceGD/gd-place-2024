<script lang="ts">
    export let id: string;
    export let name: string;

    export let checked: boolean = false;
</script>

<input type="radio" {id} {name} class="radio" on:click {checked} />

<style lang="postcss">
    .radio {
        @apply relative m-0 aspect-square h-6 w-6 appearance-none rounded-full bg-black/40 shadow-black/10 outline outline-2 -outline-offset-2 outline-white/20;

        &:disabled {
            @apply cursor-not-allowed; /*TODO*/
        }

        &:not(:disabled) {
            @apply cursor-pointer outline;
        }

        &::before {
            @apply absolute h-full w-full scale-50 rounded-full;
            content: "";
        }
        &:not(:enabled)::before {
            @apply bg-white/10;
        }
        &:enabled:not(:checked)::before {
            @apply bg-white;
        }
        &:checked::before {
            @apply bg-button-green;
        }
    }
</style>
