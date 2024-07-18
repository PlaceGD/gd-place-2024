<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import Image from "../components/Image.svelte";
    import { getUsernameColor } from "../firebase/donations";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import { handleSignOut } from "./login";

    // TODO: make user gradient update immediately when changed
</script>

<div class="gap-2 flex-center">
    {#if $loginData.currentUserData && $loginData.currentUserData.placeData && $loginData.isLoggedIn}
        <h1 class="font-pusab text-2xl xs:text-xl z-30 text-white">
            <ColoredName
                username={$loginData.currentUserData.placeData.username}
            />
        </h1>
    {/if}
    <button
        class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
        on:click={() => {
            $openMenu = null;

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
