<script lang="ts">
    import { onMount } from "svelte";
    import type { SliderMethods, LoginMethod } from "../login";
    import TOS from "./TOS.svelte";

    import Back from "../icons/back.svg";

    export let slider: SliderMethods;
    export let method: LoginMethod;

    onMount(() => {
        // do thing
        console.log("mounted");
    });
</script>

<div class="relative w-full h-full flex-center text-center">
    <button
        class="absolute top-0 left-0 p-2"
        on:click={() => slider.previous()}
    >
        <Back></Back>
    </button>
    <div
        class="flex flex-col items-center justify-between w-full h-full p-6 shrink-0"
    >
        <h1 class="text-3xl font-pusab text-stroke">Google</h1>

        <p class="text-sm">
            You must read and agree to the
            <button
                class="underline hover:decoration-dashed"
                aria-label="Terms of Service"
                on:click={() => slider.addSlideAndMove({ component: TOS })}
            >
                TOS
            </button>
            before continuing.
        </p>
    </div>
</div>
