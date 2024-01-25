<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import { default as cx } from "classnames";
    import Image from "../components/Image.svelte";
    import LoginMethodComp from "./slides/LoginMethod.svelte";
    import {
        type LoginData,
        type SliderMethods,
        LoginMethod,
        type ComponentWithProps,
        handleSignIn,
    } from "./login";

    import TOS from "./slides/TOS.svelte";

    import Close from "./icons/close.svg";
    import Back from "./icons/back.svg";
    import { cubicInOut } from "svelte/easing";
    import { tweened } from "svelte/motion";
    import { currentUserData } from "../firebase/auth";
    import Toast from "../utils/toast";

    export let loginData: LoginData;

    let twitter = false;
    document.addEventListener("keydown", e => {
        if (e.key === "Escape") {
            closeSlides();
        } else {
            twitter = e.shiftKey;
        }
    });
    document.addEventListener("keyup", e => {
        twitter = e.shiftKey;
    });

    let slideParent: HTMLDivElement;

    let userSetCanInteract = true;
    let transitionCanInteract = userSetCanInteract;

    const slideTween = tweened(0, {
        duration: 300,
        easing: cubicInOut,
    });

    const closeSlides = () => {
        userSetCanInteract = true;
        transitionCanInteract = true;
        $slideTween = 0;
        slides = [];
        loginData.showLoginUI = false;
    };

    let slides: ComponentWithProps[] = [];

    const addSlideAndMove = (slide: ComponentWithProps) => {
        transitionCanInteract = false;
        slides = [...slides, slide];

        let oldVal: number = $slideTween;
        let unsub = slideTween.subscribe(v => {
            if (v == oldVal - slideParent.clientWidth) {
                transitionCanInteract = true;
                unsub();
            }
        });

        $slideTween -= slideParent.clientWidth;
    };
    const previousSlide = () => {
        transitionCanInteract = false;
        showBackButton = slides[slides.length - 2]?.showBackButton;
        showCloseButton = slides[slides.length - 2]?.showCloseButton ?? true;

        let oldVal: number = $slideTween;
        let unsub = slideTween.subscribe(v => {
            if (v == oldVal + slideParent.clientWidth) {
                transitionCanInteract = true;
                --slides.length;
                unsub();
            }
        });

        $slideTween += slideParent.clientWidth;
    };

    const sliderMethods: SliderMethods = {
        previous: previousSlide,
        addSlideAndMove: addSlideAndMove,
        setInteractability: (interact: boolean) =>
            (userSetCanInteract = interact),
    };

    $: showBackButton = slides[slides.length - 1]?.showBackButton;
    $: showCloseButton = slides[slides.length - 1]?.showCloseButton ?? true;

    $: canInteract = userSetCanInteract && transitionCanInteract;

    /////////////////////////

    const signInMaybeSlideNext = (method: LoginMethod) => {
        userSetCanInteract = false;
        handleSignIn(LoginMethod.Google).then(isOK => {
            if (isOK) {
                if ($currentUserData !== null) {
                    if ($currentUserData.placeData !== null) {
                        closeSlides();
                    } else {
                        // next slide
                    }
                } else {
                    console.error(
                        "login OK (isOk == true) but `currentUserData` still null"
                    );
                    Toast.showErrorToast(
                        "There was an issue signing in. Please try again."
                    );
                }
            }
        });
    };
</script>

{#if loginData.showLoginUI}
    <!-- preloads the twitter svg for the secret :3 -->
    <Image
        tabindex="-1"
        src="assets/ui/login/twitter.svg"
        class="hidden"
        alt=""
    />
    <section
        class={cx({
            "absolute z-40 flex-col w-full h-full text-white flex-center": true,
            "pointer-events-none": !canInteract,
        })}
        aria-label="Login or Sign Up"
    >
        <div class="flex flex-col w-auto h-auto gap-2">
            <div
                class="flex overflow-hidden rounded-lg shadow-lg w-96 h-96 bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
                bind:this={slideParent}
            >
                <div
                    class="flex flex-col items-center justify-between w-full h-full p-6 shrink-0"
                    style:transform="translateX({$slideTween}px)"
                >
                    <h1 class="text-3xl font-pusab text-stroke">
                        Login or Sign Up
                    </h1>
                    <ul class="w-full h-24 gap-4 flex-center">
                        <li class="h-full aspect-square max-w-max">
                            <button
                                class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center bg-white/10 hover:bg-white/20 active:bg-white/30"
                                aria-label="Login with Twitter"
                                on:click={() => {
                                    signInMaybeSlideNext(LoginMethod.Google);
                                }}
                            >
                                <Image
                                    src="assets/ui/login/google.svg"
                                    alt="Login with Google"
                                    class="flex-1 object-contain w-max"
                                />
                                <p>Google</p>
                            </button>
                        </li>
                        <li class="h-full shadow-lg aspect-square max-w-max">
                            <button
                                class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center bg-white/10 hover:bg-white/20 active:bg-white/30"
                                aria-label="Login with GitHub"
                                on:click={() =>
                                    addSlideAndMove({
                                        component: LoginMethodComp,
                                        props: {
                                            method: LoginMethod.GitHub,
                                        },
                                        showBackButton: true,
                                    })}
                            >
                                <Image
                                    src="assets/ui/login/github.svg"
                                    alt="Login with GitHub"
                                    class="flex-1 object-contain w-max"
                                />
                                <p>GitHub</p>
                            </button>
                        </li>
                        <li class="h-full aspect-square max-w-max">
                            <button
                                class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center bg-white/10 hover:bg-white/20 active:bg-white/30"
                                aria-label="Login with X (Twitter)"
                                on:click={() =>
                                    addSlideAndMove({
                                        component: LoginMethodComp,
                                        props: { method: LoginMethod.X },
                                        showBackButton: true,
                                    })}
                            >
                                <Image
                                    src="assets/ui/login/{twitter
                                        ? 'twitter'
                                        : 'x'}.svg"
                                    alt="Login with X (Twitter)"
                                    class="flex-1 object-contain w-max"
                                />
                                <p>{twitter ? "Twitter" : "X"}</p>
                            </button>
                        </li>
                    </ul>
                    <p class="text-sm">
                        Don't forget to the read the
                        <button
                            class="underline hover:decoration-dashed"
                            aria-label="Terms of Service"
                            on:click={() =>
                                addSlideAndMove({
                                    component: TOS,
                                    showCloseButton: false,
                                })}
                        >
                            TOS
                        </button>
                        !
                    </p>
                </div>
                {#each slides as slide}
                    <div
                        class="w-full h-full shrink-0"
                        style:transform="translateX({$slideTween}px)"
                    >
                        <svelte:component
                            this={slide.component}
                            {...slide.props}
                            slider={sliderMethods}
                        />
                    </div>
                {/each}
            </div>
            <div
                class={cx({
                    "flex items-center slides-buttons h-12": true,
                    "text-white": canInteract,
                    "text-disabled-white cursor-not-allowed": !canInteract,
                })}
            >
                {#if showBackButton}
                    <div class="h-full back">
                        <button
                            disabled={!canInteract}
                            class="flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200 aspect-square"
                            aria-label="Back"
                            on:click={() => previousSlide()}
                        >
                            <Back alt="Back" class="w-full h-full"></Back>
                        </button>
                    </div>
                {/if}
                {#if showCloseButton}
                    <div class="h-full close">
                        <button
                            disabled={!canInteract}
                            class="flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200"
                            aria-label="Close"
                            on:click={closeSlides}
                        >
                            <Close alt="Close" class="w-full h-full"></Close>
                        </button>
                    </div>
                {/if}
            </div>
        </div>
    </section>

    <div
        tabindex="-1"
        class="absolute z-30 w-full h-full backdrop-blur-lg brightness-30"
    ></div>
{/if}

<style>
    .slides-buttons {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        grid-template-areas: "back close .";
    }

    .back {
        @apply flex h-full justify-start;
        grid-area: back;
    }

    .close {
        @apply flex justify-center;
        grid-area: close;
    }

    :global(.splide__list) {
        @apply flex h-full w-full;
    }
</style>
