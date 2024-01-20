<script lang="ts">
    import Editor from "./Editor.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    import Logo from "./components/Logo.svelte";
    import Toast from "./utils/Toast";
    import { alertHasDarkReader } from "./utils/Document";
    import Login from "./login/Login.svelte";
    import LoginButton from "./login/LoginButton.svelte";
    import type { LoginData } from "./login/Login";

    import { wasmProgress, initWasm } from "./LoadWasm";

    let loginData: LoginData = {
        isLoggedIn: false,
        showLoginUI: false,
    };

    alertHasDarkReader();

    if (localStorage.getItem("didErrorOccur") == "1") {
        localStorage.setItem("didErrorOccur", "0");
        Toast.showErrorToast(
            "An error occured and the page had to be refreshed."
        );
    }

    initWasm();
</script>

<SvelteToast options={{ duration: 6000, intro: { y: -64 } }} />

<div class="relative w-screen h-screen overflow-hidden">
    <div class="absolute top-0 right-0 flex gap-4 p-2 w-max h-max">
        <LoginButton bind:loginData></LoginButton>
    </div>
    <Login bind:loginData></Login>
    <Editor bind:wasmLoaded={$wasmProgress.hasLoaded} />
    {#if !$wasmProgress.hasLoaded}
        <div class="absolute">
            <input
                type="range"
                min={0}
                max={$wasmProgress.max}
                bind:value={$wasmProgress.progress}
                aria-label="WASM download progress"
                aria-valuetext={`${($wasmProgress.progress / $wasmProgress.max) * 100}%`}
                tabindex="-1"
            />
        </div>
    {/if}
</div>

<style lang="postcss">
    :global(._toastContainer) {
        font-family: Saira;
        color: white;
        border-radius: 8px;
    }

    @media screen(lg) {
        :global(._toastContainer) {
            font-size: 14px;
        }
    }

    @media screen(sm) {
        :global(._toastContainer) {
            font-size: 12px;
        }
    }
</style>
