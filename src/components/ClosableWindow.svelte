<script lang="ts">
    import { scale, blur } from "svelte/transition";
    export let name;

    let hidden = localStorage.getItem(name) === "true";
</script>

{#if !hidden}
    <div
        class="relative bg-menu-gray/30 rounded-xl shadow-lg backdrop-blur-md flex flex-col border-4 border-white/80 pointer-events-none"
        transition:blur={{
            duration: 150,
            delay: 0,
        }}
    >
        <div
            class="flex flex-row border-b-2 border-b-white/80 bg-menu-gray/80 rounded-t-xl"
        >
            <div class="flex-grow"></div>
            <button
                class="cursor-pointer pointer-events-auto"
                on:click={() => {
                    hidden = true;
                    localStorage.setItem(name, "true");
                }}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-12 w-12"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="white"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12"
                    />
                </svg>
            </button>
        </div>
        <div class="p-6 xs:p-3">
            <slot />
        </div>
    </div>
{/if}
