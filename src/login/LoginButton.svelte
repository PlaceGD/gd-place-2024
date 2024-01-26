<script lang="ts">
    import Image from "../components/Image.svelte";
    import { loginData } from "../stores";
    import { handleSignOut } from "./login";
</script>

<div class="gap-2 flex-center">
    {#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.isLoggedIn}
        <h1 class="font-pusab text-white text-2xl z-30">
            {$loginData.currentUserData.placeData.username}
        </h1>
    {/if}
    <button
        class="top-0 right-0 z-30 w-16 h-16"
        on:click={() => {
            if ($loginData.isLoggedIn) {
                handleSignOut();
            } else {
                $loginData.showLoginUI = true;
            }
        }}
    >
        <Image
            src={`assets/ui/login/profile_${$loginData.isLoggedIn ? "out" : "in"}.png`}
            class="object-contain aspect-square"
        ></Image>
    </button>
</div>
