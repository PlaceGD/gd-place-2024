<script lang="ts">
    import Image from "../components/Image.svelte";
    import { handleSignOut, type LoginData } from "./login";

    export let loginData: LoginData;
</script>

<button
    class="top-0 right-0 z-30 w-16 h-16"
    on:click={() => {
        if (loginData.isLoggedIn) {
            handleSignOut();
        } else {
            loginData.showLoginUI = true;
        }
    }}
>
    <Image
        src={`assets/ui/login/profile_${loginData.isLoggedIn ? "out" : "in"}.png`}
        class="object-contain aspect-square"
    ></Image>
</button>
