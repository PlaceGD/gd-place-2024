<script lang="ts">
    import { scale, blur } from "svelte/transition";
    export let name;
    export let open = true;

    let hidden = localStorage.getItem(name) === "true";
</script>

{#if !hidden}
    <div
        class="relative flex flex-col overflow-hidden border-4 shadow-lg shrink-0 bg-menu-gray/30 rounded-xl backdrop-blur-md border-white/80"
        
        style={`transform: scale(${open ? 1 : 0.8}); opacity: ${open ? 1 : 0}; pointer-events: none; transition: transform 300ms, opacity 300ms;`}
    >
        <div
            class="flex flex-row border-b-4 border-b-white/80 bg-menu-gray/80 rounded-t-xl"
        >
            <div class="flex-grow"></div>
            <button
                class="cursor-pointer pointer-events-auto bg-[#a22] hover:bg-[#e33] m-1 rounded-md"
                on:click={() => {
                    hidden = true;
                    localStorage.setItem(name, "true");
                }}
                tabindex="-1"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="w-12 h-12"
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
        <div class="p-6 pointer-events-none xs:p-3">
            <slot />
        </div>
    </div>
{/if}
