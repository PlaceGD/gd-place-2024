<script lang="ts">
    import FadedScroll from "./components/FadedScroll.svelte";
    import Button from "./components/Button.svelte";
    import { analytics } from "./stores";
    import ScreenModal from "./components/ScreenModal.svelte";

    let hasScrolledToBottom = false;

    let isOpen = false;

    let hidePopup = $analytics != null ? true : false;
</script>

{#if !hidePopup}
    <div
        class="absolute bottom-0 z-50 w-1/2 h-auto p-4 transform -translate-x-1/2 xs:w-full left-1/2"
    >
        <div
            class="flex-col w-full gap-3 p-3 text-center text-white rounded-lg shadow-lg sm:text-sm sm:w-full flex-center bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
        >
            <p>
                <strong class="text-xl sm:text-base">
                    This site uses analytics to collect interesting data about
                    the event.
                </strong>
            </p>

            <div class="flex-row w-full gap-3 text-center flex-center">
                <Button
                    type="accept"
                    class="px-2 w-max"
                    aria-label="Fine by me"
                    on:click={() => {
                        isOpen = false;
                        hidePopup = true;
                        $analytics = true;
                    }}
                >
                    Fine by me
                </Button>
                <Button
                    type="plain"
                    class="w-max"
                    aria-label="Read More"
                    on:click={() => {
                        isOpen = true;
                    }}
                >
                    <u>Read More</u>
                </Button>
            </div>
        </div>
    </div>
{/if}

<ScreenModal
    label="Privacy Policy Modal"
    {isOpen}
    canClose={false}
    size="max-w-[600px] max-h-[400px]"
>
    <div class="grid gap-4 modal-panel grid-rows-[minmax(0,_1fr)_min-content]">
        <section class="text">
            <h2>Analytics</h2>

            <p>
                <strong>
                    We use Cloudflare Web analytics to collect some data about
                    the people participating in the event.
                </strong>
            </p>

            <p>
                This includes what country you're from, what operating system
                you are using, etc. This is literally just out of our own nerdy
                interest, we probably don't even really need this warning, but
                it's better to be safe than to get sued by the EU I guess.
            </p>

            <p>
                <b>
                    You can read more about the analytics we collect <a
                        class="underline hover:decoration-dashed"
                        href="https://developers.cloudflare.com/analytics/web-analytics/understanding-web-analytics/dimensions/"
                        >here</a
                    >
                    and
                    <a
                        class="underline hover:decoration-dashed"
                        href="https://developers.cloudflare.com/analytics/web-analytics/understanding-web-analytics/high-level-metrics/"
                        >here</a
                    >.
                </b>
            </p>
        </section>
        <div class="flex w-full gap-4 xs:gap-2 h-11 xs:h-10">
            <Button
                type="decline"
                class="w-full"
                on:click={() => {
                    isOpen = false;
                    hidePopup = true;
                    $analytics = true;
                }}
            >
                <p class="xs:text-sm">Disable Analytics</p>
            </Button>
            <Button
                type="accept"
                class="w-full"
                on:click={() => {
                    isOpen = false;
                    hidePopup = true;
                    $analytics = false;
                }}
            >
                <p class="xs:text-sm">Continue</p>
            </Button>
        </div>
    </div>
</ScreenModal>
