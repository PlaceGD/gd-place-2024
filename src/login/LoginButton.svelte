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
            class="absolute top-0 z-20 flex-col flex-center hover-anim opacity-70"
        >
            <Image src={upArrowUrl} class="w-12 h-12"></Image>
            <h1
                class="top-0 text-2xl font-bold text-center text-white pointer-events-none sm:text-xl xs:text-lg"
            >
                Sign up now!
            </h1>
        </div>
    {/if}
</div>

<style lang="postcss">
    @keyframes hover-anim {
        0% {
            transform: translateY(5rem);
        }
        50% {
            transform: translateY(5.5rem);
        }
        100% {
            transform: translateY(5rem);
        }
    }

    /* change this if theres a nicer way to do it */
    @keyframes hover-anim-small {
        0% {
            transform: translateY(4rem);
        }
        50% {
            transform: translateY(4.3rem);
        }
        100% {
            transform: translateY(4rem);
        }
    }

    .hover-anim {
        animation: hover-anim 2.5s infinite ease-in-out;
    }

    @media (max-width: 750px) {
        .hover-anim {
            animation: hover-anim-small 2.5s infinite ease-in-out;
        }
    }
</style>
