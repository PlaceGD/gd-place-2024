<script lang="ts">
    import { default as cx } from "classnames";
    import Image from "./components/Image.svelte";

    import Cross from "./icons/cross.svg";
    import Check from "./icons/check.svg";

    import Warning from "./icons/warning.svg";
    import Analytics from "./icons/analytics.svg";
    import { onMount } from "svelte";
    import { map } from "shared-lib";

    let showReadMore = false;
    let hasScrolledToBottom = false;

    let hidePopup = localStorage.getItem("analytics") != null ? true : false;

    let notice: HTMLDivElement | null = null;

    let threshold = 20;
    let scrollTop = 0;
    let noticeBottom = threshold + 1;

    const onScrollNotice = () => {
        if (notice == null) return;

        noticeBottom = notice.scrollHeight - notice.offsetHeight;
        scrollTop = notice.scrollTop;
        if (scrollTop === notice.scrollHeight - notice.offsetHeight) {
            hasScrolledToBottom = true;
        }
    };

    $: {
        if (notice != null && showReadMore) {
            notice.addEventListener("scroll", onScrollNotice);
        } else if (notice != null) {
            notice.removeEventListener("scroll", onScrollNotice);
        }
    }

    $: topThreshold = scrollTop >= threshold ? 10 : 0;
    $: bottomThreshold = scrollTop >= noticeBottom - threshold ? 100 : 90;
</script>

{#if !hidePopup}
    <div
        class="absolute bottom-0 z-50 w-1/2 h-auto p-4 left-1/2 transform -translate-x-1/2"
    >
        <div
            class="flex-col w-full gap-3 p-3 text-center text-white rounded-lg shadow-lg sm:text-sm sm:w-full flex-center bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
        >
            <p>
                <b>
                    This site uses cookies to protect from spam and abuse, and
                    analytics to collect data about the event.
                </b>
            </p>
            <button
                class="p-2 text-lg rounded-lg xs:text-lg white-button"
                aria-label="Read More"
                on:click={() => (showReadMore = true)}
            >
                Read More
            </button>
        </div>
    </div>
{/if}

{#if showReadMore}
    <div
        class="absolute z-[60] flex flex-col w-full h-full gap-2 flex-center"
        style="--gradient: linear-gradient(
            transparent 0%,
            black {topThreshold}%,
            black {bottomThreshold ?? 80}%,
            transparent 100%
        )"
    >
        <div
            class="flex flex-col p-6 overflow-hidden text-white rounded-lg shadow-lg w-[500px] aspect-square xs:w-80 bg-menu-gray/90 shadow-black/40 backdrop-blur-md gap-4"
        >
            <div
                class="overflow-y-scroll fadeout xs:text-sm"
                bind:this={notice}
            >
                <p class="mb-4">We use cookies to:</p>
                <ul>
                    <li
                        class="flex items-center pl-6 gap-4 bg-black/20 p-1 rounded-md"
                    >
                        <!-- <div class="w-10 h-10"> -->
                        <Warning class="w-10 h-10 xs:w-9 xs:h-9" />
                        <!-- </div> -->
                        <span>Protect against spam and abuse.</span>
                    </li>
                </ul>
                <p class="my-4">
                    The exact breakdown of our cookie usage is as follows:
                </p>
                <ul class="list-disc">
                    <li class="pl-6">
                        <a
                            href="https://www.google.com/recaptcha/about/"
                            class="underline hover:decoration-dashed"
                        >
                            reCAPTCHA
                        </a>
                        <ul class="list-disc">
                            <li class="pl-6">
                                <code>_GRECAPTCHA</code>: Required cookie for
                                <a
                                    href="https://www.google.com/recaptcha/about/"
                                    class="underline hover:decoration-dashed"
                                >
                                    reCAPTCHA
                                </a> analysis.
                            </li>
                        </ul>
                    </li>
                </ul>
                <p class="my-4">We use analytics to:</p>
                <ul class="list-disc">
                    <li
                        class="flex items-center pl-6 gap-4 bg-black/20 p-1 rounded-md"
                    >
                        <!-- <div class="w-10 h-10"> -->
                        <Analytics class="w-10 h-10 xs:w-9 xs:h-9" />
                        <!-- </div> -->
                        <span>Collect data about the event.</span>
                    </li>
                </ul>
                <p class="my-4">
                    The exact breakdown of our analytics usage is as follows:
                </p>
                <ul class="list-disc">
                    <li class="pl-6">
                        <p>
                            Cloudflare Web Analytics (<a
                                class="underline hover:decoration-dashed"
                                href="https://developers.cloudflare.com/analytics/web-analytics/understanding-web-analytics/dimensions/"
                                >Dimensions</a
                            >,
                            <a
                                class="underline hover:decoration-dashed"
                                href="https://developers.cloudflare.com/analytics/web-analytics/understanding-web-analytics/high-level-metrics/"
                                >High-level metrics</a
                            >)
                        </p>
                        <ul class="list-disc">
                            <li class="pl-6">Country</li>
                            <li class="pl-6">
                                Host (the domain, so <code>.gd</code>)
                            </li>
                            <li class="pl-6">
                                Path (different links you visit, so none for
                                this event)
                            </li>
                            <li class="pl-6">
                                Referer (how you found our site)
                            </li>
                            <li class="pl-6">Device Type</li>
                            <li class="pl-6">Browser</li>
                            <li class="pl-6">Operating System</li>
                            <li class="pl-6">Site (<code>place.gd</code>)</li>
                            <li class="pl-6">
                                Visits (a page view originating from a different
                                website)
                            </li>
                            <li class="pl-6">Page Views</li>
                            <li class="pl-6">Page Load Time</li>
                            <li class="pl-6">
                                Core Web Vitals (data about the site to
                                understand the user experience better)
                            </li>
                        </ul>
                        <p>Firebase</p>
                        <ul class="list-disc">
                            <li class="pl-6">...</li>
                        </ul>
                    </li>
                </ul>
                <p class="my-4">
                    If you choose to '<span class="text-[#47ff47]"
                        >Continue</span
                    >' we will continue to send analytics data.
                </p>
                <p class="my-4">
                    If you choose to '<span class="text-[#ff4747]"
                        >Disable Analytics</span
                    >' we will disable both Cloudflare and Firebase analytics.
                </p>
            </div>
            <div class="flex w-full gap-4">
                <button
                    class="w-[inherit] gap-2 p-1 rounded-lg flex-center white-button"
                    disabled={!hasScrolledToBottom}
                    on:click={() => {
                        showReadMore = false;
                        hidePopup = true;
                        localStorage.setItem("analytics", "0");
                    }}
                >
                    <Cross class="text-[#ff4747] w-11 h-11 xs:w-8 xs:h-8" />

                    <p class="xs:text-sm w-min">Disable Analytics</p>
                </button>
                <button
                    class="w-[inherit] gap-2 p-1 rounded-lg flex-center white-button"
                    disabled={!hasScrolledToBottom}
                    on:click={() => {
                        showReadMore = false;
                        hidePopup = true;
                        localStorage.setItem("analytics", "1");
                    }}
                >
                    <Check class="text-[#47ff47] w-11 h-11 xs:w-8 xs:h-8" />
                    <p class="xs:text-sm w-min">Continue</p>
                </button>
            </div>
        </div>
    </div>
    <div
        tabindex="-1"
        class="absolute z-50 w-full h-full backdrop-blur-lg brightness-30"
    ></div>
{/if}

<style>
    .fadeout {
        mask-image: var(--gradient);
        -webkit-mask-image: var(--gradient);
    }
</style>
