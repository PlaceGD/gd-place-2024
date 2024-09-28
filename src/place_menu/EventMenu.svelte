<script lang="ts">
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { ExclusiveMenus, openMenu } from "../stores";

    export let kind: "pre-event" | "login-to-place";
</script>

<div
    class="absolute flex flex-col justify-end w-full pointer-events-none pre-event-menu"
>
    <div
        class="flex flex-col items-center menu-panel justify-evenly w-full h-[200px]"
    >
        {#if kind === "pre-event"}
            SPUT PUT UR STUFF HERE (make it responsive if u can 🥺)
        {:else if kind === "login-to-place"}
            <div class="flex flex-col gap-2 pointer-events-auto">
                <h1
                    class="text-white w-full h-full p-4 text-4xl text-center md:text-3xl sm:text-2xl xs:text-xl flex-center font-pusab text-stroke"
                >
                    Login to participate!
                </h1>
                <button
                    class="text-white text-2xl md:text-xl sm:text-lg xs:text-base text-center underline hover:decoration-dashed cursor-pointer"
                    on:click={() => {
                        $openMenu = ExclusiveMenus.Login;
                    }}
                >
                    Sign me up!
                </button>
            </div>{/if}
    </div>
</div>

<style lang="postcss">
    .pre-event-menu {
        height: 100svh;
        padding: 8px;
    }
</style>
