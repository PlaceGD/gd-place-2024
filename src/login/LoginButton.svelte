<script lang="ts">
    import Image from "../components/Image.svelte";
    import { getUsernameColor } from "../firebase/donations";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import { handleSignOut } from "./login";

    // TODO: make user gradient update immediately when changed
</script>

<div class="gap-2 flex-center">
    {#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.isLoggedIn}
        {#await getUsernameColor($loginData.currentUserData.placeData.username) then color}
            <h1
                class="z-30 text-2xl text-white font-pusab"
                style={`
                    background-image: ${color};
                    -webkit-text-fill-color: rgba(255, 255, 255, 0.1);
                    background-clip: text;
                    -webkit-background-clip: text;
                `}
            >
                {$loginData.currentUserData.placeData.username}
            </h1>
        {/await}
    {/if}
    <button
        class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
        on:click={() => {
            if ($loginData.isLoggedIn) {
                $openMenu = null;

                handleSignOut();
            } else {
                $openMenu = ExclusiveMenus.Login;
            }
        }}
    >
        <Image
            src="/assets/ui/login/profile_{$loginData.isLoggedIn
                ? 'out'
                : 'in'}.png"
            class="object-contain aspect-square"
        ></Image>
    </button>
</div>
