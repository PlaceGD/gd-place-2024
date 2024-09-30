<script lang="ts">
    import { default as cx } from "classnames";
    import { createEventDispatcher } from "svelte";

    export let src: string | null = null;
    export let alt: string = "";
    export let skeleton: boolean = false;
    export let lazyLoad: boolean = false;

    let imgElement: HTMLImageElement;

    const dispatcher = createEventDispatcher();

    let hasBeenVisible = false;
    let hasLoaded = false;

    const options = {
        root: null,
        rootMargin: "0px",
        threshold: 0,
    };

    let observer: IntersectionObserver;
    $: if (hasBeenVisible) {
        observer.unobserve(imgElement);
    }
    $: if (src != null && hasBeenVisible) {
        imgElement.src = src;
    }

    const useLazyLoad = (image: HTMLImageElement) => {
        observer = new IntersectionObserver(entries => {
            if (entries[0].isIntersecting) {
                dispatcher("visible");
                hasBeenVisible = true;
            }
        }, options);
        observer.observe(image);

        return {};
    };

    // ignores the source for the lazy loading otherwise ...$$restProps will
    // give a src to the image before the lazy loading
    const { src: _src, ...restProps } = $$restProps;
</script>

{#if lazyLoad}
    <span class="relative w-full h-full flex-center">
        <img
            bind:this={imgElement}
            use:useLazyLoad
            on:load={() => (hasLoaded = true)}
            {alt}
            draggable="false"
            style:visibility={hasLoaded ? "visible" : "hidden"}
            class="absolute"
            aria-label={alt}
            {...restProps}
        />
        {#if skeleton && !hasLoaded}
            <div class="absolute w-2/3 h-2/3 placeholder" />
        {/if}
    </span>
{:else}
    <img {src} {alt} aria-label={alt} {...$$restProps} draggable="false" />
{/if}

<style>
    .placeholder {
        border-radius: 4px;
        background: linear-gradient(
            135deg,
            rgba(255, 255, 255, 0.3) 0%,
            rgba(255, 255, 255, 0.3) 33%,
            rgba(187, 187, 187, 0.3) 33%,
            rgba(187, 187, 187, 0.3) 66%,
            rgba(255, 255, 255, 0.3) 66%
        );
    }
</style>
