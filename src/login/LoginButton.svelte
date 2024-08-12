<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import Image from "../components/Image.svelte";
    import { getUsernameColor } from "../firebase/donations";
    import {
        currentUserColor,
        ExclusiveMenus,
        loginData,
        openMenu,
    } from "../stores";
    import { handleSignOut } from "./login";

    import profileInUrl from "./assets/profile_in.png";
    import profileOutUrl from "./assets/profile_out.png";
</script>

<div class="gap-4 xs:gap-2 flex-center">
    {#if $loginData.currentUserData != null && $loginData.currentUserData.userDetails != null}
        <h1 class="z-30 text-3xl text-white font-pusab sm:text-2xl xs:text-xl">
            <ColoredName
                username={$loginData.currentUserData.userDetails.username}
                colorOverride={$currentUserColor}
            />
        </h1>
    {/if}
    <button
        class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
        on:click={() => {
            $openMenu = null;

            if ($loginData.currentUserData?.userDetails != null) {
                $openMenu = null;

                handleSignOut();
            } else {
                $openMenu = ExclusiveMenus.Login;
            }
        }}
    >
        <Image
            src={$loginData.currentUserData?.userDetails != null
                ? profileOutUrl
                : profileInUrl}
            class="object-contain aspect-square"
        ></Image>
    </button>
</div>
