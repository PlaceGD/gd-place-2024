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
    } from "./login";

    import TOS from "./slides/TOS.svelte";

    import Close from "./icons/close.svg";
    import { cubicInOut } from "svelte/easing";
    import { tweened } from "svelte/motion";

    export let loginData: LoginData;

    let twitter = false;
    document.addEventListener("keydown", e => {
        twitter = e.shiftKey;
    });
    document.addEventListener("keyup", e => {
        twitter = e.shiftKey;
    });

    let self: any;
    let slideParent: HTMLDivElement;

    let canInteract = true;

    const slideTween = tweened(0, {
        duration: 300,
        easing: cubicInOut,
    });

    let slides: ComponentWithProps[] = [];
    const addSlideAndMove = (slide: ComponentWithProps) => {
        canInteract = false;
        slides = [...slides, slide];

        let oldVal: number = $slideTween;
        let unsub = slideTween.subscribe(v => {
            if (v == oldVal - slideParent.clientWidth) {
                canInteract = true;
                unsub();
            }
        });

        $slideTween -= slideParent.clientWidth;
    };

    let previousSlide = () => {
        canInteract = false;

        let oldVal: number = $slideTween;
        let unsub = slideTween.subscribe(v => {
            if (v == oldVal + slideParent.clientWidth) {
                canInteract = true;
                slides.pop();
                unsub();
            }
        });

        $slideTween += slideParent.clientWidth;
    };

    const sliderMethods: SliderMethods = {
        previous: previousSlide,
        addSlideAndMove: addSlideAndMove,
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
            "absolute z-40 flex-col w-full h-full gap-2 text-white flex-center": true,
            "pointer-events-none": !canInteract,
        })}
        aria-label="Login or Sign Up"
    >
        <div
            class="rounded-lg shadow-lg w-96 h-96 bg-menu-gray/90 shadow-black/40 backdrop-blur-md overflow-hidden flex"
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
                            on:click={() =>
                                addSlideAndMove({
                                    component: LoginMethodComp,
                                    props: { method: LoginMethod.Google },
                                })}
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
                        >
                            <Image
                                src="assets/ui/login/{twitter
                                    ? 'twitter'
                                    : 'x'}.svg"
                                alt="Login with X (Twitter)"
                                class={cx({
                                    "flex-1 object-contain p-1": true,
                                    "bg-white rounded-lg w-max": !twitter,
                                })}
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
                        on:click={() => addSlideAndMove({ component: TOS })}
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
        <button
            class="flex-col w-auto h-auto p-1 text-white rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200"
            aria-label="Close"
            on:click={() => {
                loginData.showLoginUI = false;
            }}
        >
            <Close src="assets/ui/login/close.svg" alt="Close"></Close>
        </button>
    </section>

    <div
        tabindex="-1"
        class="absolute z-30 w-full h-full backdrop-blur-lg brightness-30"
    ></div>
{/if}

<style>
    :global(.splide__list) {
        width: 100%;
        height: 100%;
        display: flex;
    }
</style>
