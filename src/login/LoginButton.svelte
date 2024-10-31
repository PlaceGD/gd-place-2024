<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import Image from "../components/Image.svelte";
    import { getUsernameColor } from "../firebase/donations";
    import {
        currentUserColor,
        eventStarted,
        ExclusiveMenus,
        hasLoggedInBefore,
        loginData,
        openMenu,
    } from "../stores";
    import { handleSignOut } from "./login";

    import profileInUrl from "./assets/profile_in.png";
    import profileOutUrl from "./assets/profile_out.png";
    import upArrowUrl from "./assets/arrow_up.svg?url";

    $hasLoggedInBefore = !!localStorage.getItem("has_previosly_logged_in");
    hasLoggedInBefore.subscribe(v => {
        localStorage.setItem("hasPreviouslyLoggedIn", v.toString());
    });
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
    <div class="relative flex flex-col">
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

        {#if $loginData.currentUserData?.userDetails == null && !$hasLoggedInBefore}
            <div
                class="absolute bottom-0 z-20 flex flex-col left-1/2 flex-center opacity-70 hover-anim"
            >
                <Image
                    src={upArrowUrl}
                    class="w-10 h-10"
                    style="filter: drop-shadow(0px 0px 5px rgba(0, 0, 0, 0.7))"
                />
                <h1
                    class="text-2xl font-bold text-center text-white pointer-events-none sm:text-xl xs:text-lg"
                    style="text-shadow: 0px 0px 5px rgba(0, 0, 0, 0.7);"
                >
                    Sign&nbsp;up now!
                </h1>
            </div>
        {/if}
    </div>
</div>

<style lang="postcss">
    @keyframes hover-anim {
        0% {
            transform: translate(-50%, 100%);
        }
        50% {
            transform: translate(-50%, 110%);
        }
        100% {
            transform: translate(-50%, 100%);
        }
    }

    /* change this if theres a nicer way to do it */
    /* i have :DD */
    .hover-anim {
        animation: hover-anim 2.5s infinite ease-in-out;
    }
</style>
