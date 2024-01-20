<script lang="ts">
    import Image from "../components/Image.svelte";
    import type { LoginData } from "./Login";

    export let loginData: LoginData;
</script>

{#if loginData.showLoginUI}
    <section class="absolute z-40 w-full h-full" aria-label="Login or Sign Up">
        <button
            class="text-white bg-red"
            on:click={() => {
                loginData.showLoginUI = false;
            }}
        >
            CLOSE !!!!!!!!
        </button>
    </section>

    <div
        tabindex="-1"
        class="absolute z-30 w-full h-full backdrop-blur-lg brightness-50"
    ></div>
{/if}
