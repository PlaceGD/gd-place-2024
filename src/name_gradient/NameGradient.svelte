<script lang="ts">
    import { default as cx } from "classnames";
    import { onMount } from "svelte";
    import ToastContainers from "../components/ToastContainers.svelte";
    import OnceButton from "../components/Buttons/OnceButton.svelte";
    import Loading from "../components/Loading.svelte";
    import {
        MAX_GRADIENT_STOPS,
        VALID_KOFI_TRANSACTION_ID,
        VALID_KOFI_TRANSACTION_ID_CHARS,
        type KofiTxId,
    } from "shared-lib/kofi";
    import Cross from "../icons/Cross.svelte";
    import Check from "../icons/Check.svelte";
    import {
        changeNameGradient,
        submitKofiTxId,
    } from "../firebase/cloud_functions";
    import Toast from "../utils/toast";
    import {
        currentNameGradient,
        ExclusiveMenus,
        loginData,
        openMenu,
    } from "../stores";
    import GradientPicker from "../components/GradientPicker.svelte";
    import Button from "../components/Button.svelte";
    import { complement } from "../utils/gradient";
    import { GRADIENT_COOLDOWN_SECONDS } from "shared-lib/user";
    import ScreenModal from "../components/ScreenModal.svelte";
    import DarkInput from "../components/DarkInput.svelte";
    import { SyncedCooldown } from "../utils/cooldown";
    import type { Readable } from "svelte/store";
    import type { UserData } from "../firebase/auth";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { getRandomGradientColors } from "../utils/gradient";
    import IconButton from "../components/Buttons/IconButton.svelte";
    import MagicWand from "../icons/MagicWand.svelte";

    enum Page {
        SUBMIT_TX_ID,
        SELECT_GRADIENT,
    }

    let isInProgress = false;
    $: isOpen = $openMenu == ExclusiveMenus.Kofi;

    let currentPage: Page = Page.SUBMIT_TX_ID;

    $: {
        const hasDonated = $loginData.currentUserData?.userDetails?.hasDonated;

        if (hasDonated == null || !hasDonated) {
            currentPage = Page.SUBMIT_TX_ID;
        } else {
            currentPage = Page.SELECT_GRADIENT;
        }
    }

    let kofiTxId: string = "";
    let isValidKofiTxId = false;
    $: isValidKofiTxId = VALID_KOFI_TRANSACTION_ID.test(kofiTxId);

    let nameGradientString: string;
    let nameGradientStops: number[] = $currentNameGradient.positions ?? [
        0, 50, 100,
    ];
    let nameGradientColors: string[] =
        $currentNameGradient.colors ?? getRandomGradientColors();
    let nameGradientIDs: number[] = $currentNameGradient.ids ?? [69, 420, 1337];

    let resetSubmitButton: () => void;
    let resetGradientButton: () => void;

    const onSubmitTxId = async () => {
        isInProgress = true;

        try {
            await submitKofiTxId({
                txId: kofiTxId as KofiTxId,
            });

            currentPage = Page.SELECT_GRADIENT;

            Toast.showSuccessToast(
                "Donation recieved successfully. Thanks for keeping this project going! <3"
            );
        } catch (e: any) {
            console.error("Failed to submit donation code", e.details.message);
            Toast.showErrorToast(
                `Failed to submit donation code. (${e.details.code})`
            );
        }

        isInProgress = false;

        resetSubmitButton();
    };

    let gradientCooldown: SyncedCooldown | null = null;
    let gradientCooldownDisplay: Readable<string> | null = null;
    let gradientCooldownFinished: Readable<boolean> | null = null;

    const onUserData = (data: UserData | null) => {
        if (gradientCooldown != null) {
            gradientCooldown.unsub();
        }
        if (data != null) {
            gradientCooldown = SyncedCooldown.new(
                `userDetails/${data?.user?.uid ?? ""}/epochNextGradient`
            );
            gradientCooldownDisplay = gradientCooldown.display;
            gradientCooldownFinished = gradientCooldown.finished;
        } else {
            gradientCooldown = null;
            gradientCooldownDisplay = null;
            gradientCooldownFinished = null;
        }
    };

    $: {
        onUserData($loginData.currentUserData);
    }
    // const gradientCooldown = SyncedCooldown.new(
    //     `userDetails/${$loginData.currentUserData?.user?.uid ?? ""}/epochNextGradient`
    // );
    // let {
    //     display: gradientCooldownDisplay,
    //     finished: gradientCooldownFinished,
    // } = gradientCooldown;

    const onUpdateGradient = async () => {
        isInProgress = true;

        try {
            await changeNameGradient({
                grad: nameGradientString,
            });

            $currentNameGradient.positions = nameGradientStops;
            $currentNameGradient.colors = nameGradientColors;
            $currentNameGradient.ids = nameGradientIDs;

            Toast.showSuccessToast("Successfully updated colors!");
        } catch (e: any) {
            console.error("Failed to update name colors", e.details.message);
            Toast.showErrorToast(
                `Failed to update colors. (${e.details.code})`
            );
        }

        isInProgress = false;

        resetGradientButton();
    };
</script>

<ScreenModal
    hasCloseButton={true}
    state={isInProgress ? "loading" : "default"}
    {isOpen}
    canClose={!isInProgress}
    size="max-w-[500px] h-[600px] max-h-[650px]"
>
    {#if currentPage === Page.SUBMIT_TX_ID}
        <div
            class="grid grid-rows-[min-content_1fr] items-start h-full p-6 xs:p-4"
        >
            <hgroup class="flex flex-col items-center justify-center gap-1">
                <h1
                    class="text-2xl text-center xs:text-xl font-pusab text-stroke"
                >
                    Enter Donation Code
                </h1>
                <p class="text-sm text-center xs:text-xs text-white/55">
                    You will get an E-Mail with a code after donating (check
                    spam folder)
                </p>
            </hgroup>
            <div class="flex-col h-full gap-2 flex-center">
                <div class="w-full gap-2 flex-center">
                    {#if isValidKofiTxId}
                        <Check
                            class="text-[#47ff47] xs:w-7 xs:h-7 w-8 h-8 shrink-0 ml-auto stroke-[1.5]"
                        />
                    {:else}
                        <Cross
                            class="text-[#ff4747] xs:w-7 xs:h-7 w-8 h-8 shrink-0 ml-auto stroke-[1.5]"
                        />
                    {/if}
                    <form
                        class="w-full"
                        id="kofi-tx-id-form"
                        on:submit={e => e.preventDefault()}
                    >
                        <DarkInput
                            class="w-[inherit] h-12 text-lg sm:text-md xs:text-sm"
                            maxLength={36}
                            hardValidInput={VALID_KOFI_TRANSACTION_ID_CHARS}
                            autoTrim
                            bind:value={kofiTxId}
                        />
                    </form>
                </div>
            </div>
            <OnceButton
                userDisabled={!isValidKofiTxId}
                let:click
                let:disabled
                bind:reset={resetSubmitButton}
            >
                <WhiteButton
                    {disabled}
                    form="kofi-tx-id-form"
                    class="w-full"
                    on:click={click}
                    on:click={onSubmitTxId}
                >
                    <p class="text-lg xs:text-base">Submit</p>
                </WhiteButton>
            </OnceButton>
        </div>
    {:else if currentPage === Page.SELECT_GRADIENT}
        <div
            class="grid items-start h-full gap-2 px-6 py-4 select-gradient xs:p-2"
        >
            <hgroup class="flex flex-col items-center justify-center gap-1">
                <h1
                    class="text-3xl text-center xs:text-2xl font-pusab text-stroke"
                >
                    Select Name Color
                </h1>
            </hgroup>
            <div
                class="z-30 flex self-center w-full gap-2 p-1 overflow-x-scroll text-2xl text-white font-pusab usernames thin-scrollbar"
            >
                <p
                    class="m-auto text-2xl username-gradient w-min xs:text-xl"
                    style={`background-image: ${nameGradientString};`}
                >
                    {$loginData.currentUserData?.userDetails?.username ?? ""}
                </p>
                <p
                    class="m-auto text-2xl username-gradient w-min font-pusab xs:text-xl"
                    style={`background-image: ${nameGradientString};`}
                >
                    {$loginData.currentUserData?.userDetails?.username ?? ""}
                </p>
            </div>
            <div class="flex-col h-full gap-2 px-4 py-1 xs:px-2">
                <GradientPicker
                    maxStops={MAX_GRADIENT_STOPS}
                    bind:rotatedGradientString={nameGradientString}
                    bind:gradientStops={nameGradientStops}
                    bind:gradientColors={nameGradientColors}
                    bind:gradientIDs={nameGradientIDs}
                ></GradientPicker>
            </div>
            <div class="grid grid-cols-[0.3fr_0.7fr] w-full gap-2">
                <button
                    class="flex w-full gap-1 px-2 underline flex-center hover:decoration-dashed"
                    on:click={() => {
                        nameGradientColors = getRandomGradientColors();
                        nameGradientStops = nameGradientColors.map(
                            (_, idx) =>
                                (idx / (nameGradientColors.length - 1)) * 100
                        );
                        nameGradientIDs = nameGradientColors.map(_ =>
                            Math.random()
                        );
                    }}
                >
                    <p class="text-lg xs:text-base">Random</p>
                    <MagicWand
                        stroke-width={1}
                        class="p-1 xs:w-7 xs:h-7 w-9 h-9"
                    />
                </button>
                <OnceButton
                    userDisabled={!($gradientCooldownFinished ?? true)}
                    let:click
                    let:disabled
                    bind:reset={resetGradientButton}
                >
                    <WhiteButton
                        {disabled}
                        class="w-full"
                        on:click={click}
                        on:click={onUpdateGradient}
                    >
                        <p class="text-lg xs:text-base">Set Colors</p>
                    </WhiteButton>
                </OnceButton>
            </div>
            {#if !($gradientCooldownFinished ?? false)}
                <p class="text-sm text-center transition hover-text-transition">
                    You changed your gradient recently! Please wait <span
                        class="proportional-nums"
                        >{$gradientCooldownDisplay ?? "--:--"}</span
                    >
                    before changing it again.
                </p>
            {/if}
        </div>
    {/if}
</ScreenModal>

<style lang="postcss">
    .select-gradient {
        grid-template-rows: min-content min-content minmax(0, 1fr);
    }

    .usernames {
        -webkit-mask-image: linear-gradient(
            to right,
            transparent,
            black 20%,
            black 80%,
            transparent
        );
        mask-image: linear-gradient(
            to right,
            transparent,
            black 20%,
            black 80%,
            transparent
        );
    }
    .usernames::before,
    .usernames::after {
        content: "";
    }

    .username-gradient {
        -webkit-text-fill-color: rgba(255, 255, 255, 0.1) !important;
        background-clip: text !important;
        -webkit-background-clip: text !important;
    }
</style>
