<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import Image from "../components/Image.svelte";
    import { getUsernameColor } from "../firebase/donations";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import { handleSignOut } from "./login";

    import profileInUrl from "./assets/profile_in.png";
    import profileOutUrl from "./assets/profile_out.png";

    // TODO: make user gradient update immediately when changed
</script>

<div class="gap-4 flex-center">
    {#if $loginData.currentUserData != null && $loginData.currentUserData.userDetails != null}
        <h1 class="z-30 text-3xl text-white font-pusab xs:text-2xl">
            <ColoredName
                username={$loginData.currentUserData.userDetails.username}
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
