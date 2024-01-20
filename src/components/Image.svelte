<script lang="ts">
    import { default as cx } from "classnames";

    export let src: string;
    export let alt: string = "";
    export let skeleton: boolean = false;
    export let lazyLoad: boolean = false;

    let hasLoaded = false;

    const options = {
        root: null,
        rootMargin: "0px",
        threshold: 0,
    };

    const useLazyLoad = (image: HTMLImageElement) => {
        const loaded = () => {
            hasLoaded = true;
        };

        const observer = new IntersectionObserver(entries => {
            if (entries[0].isIntersecting) {
                image.src = src;
                if (image.complete) {
                    loaded();
                } else {
                    image.addEventListener("load", loaded);
                }
            }
        }, options);
        observer.observe(image);

        return {
            destroy() {
                image.removeEventListener("load", loaded);
            },
        };
    };

    // ignores the source for the lazy loading otherwise ...$$restProps will
    // give a src to the image before the lazy loading
    const { src: _src, ...restProps } = $$restProps;
</script>

{#if lazyLoad}
    <img
        use:useLazyLoad
        {alt}
        draggable="false"
        style={cx({
            hidden: !hasLoaded,
        })}
        {...restProps}
    />
    {#if skeleton && !hasLoaded}
        <div class="w-2/3 h-2/3 placeholder" />
    {/if}
{:else}
    <img {src} {alt} {...$$restProps} draggable="false" />
{/if}

<style>
    .placeholder {
        border-radius: 4px;
        /* background: rgb(255, 255, 255); */
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
