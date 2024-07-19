<script lang="ts">
    import { default as cx } from "classnames";
    import { onMount } from "svelte";
    import ToastContainer from "../components/ToastContainer.svelte";
    import Input from "../components/Input.svelte";
    import OnceButton from "../components/OnceButton.svelte";
    import Loading from "../components/Loading.svelte";
    import {
        MAX_GRADIENT_STOPS,
        VALID_KOFI_TRANSACTION_ID,
        VALID_KOFI_TRANSACTION_ID_CHARS,
        type KofiTxId,
    } from "shared-lib/kofi";
    import Cross from "../icons/cross.svg";
    import Check from "../icons/check.svg";
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
    import { SyncedCooldown } from "../utils/cooldown";
    import { GRADIENT_COOLDOWN_SECONDS } from "shared-lib/user";

    let modal: HTMLDialogElement;

    enum Page {
        SUBMIT_TX_ID,
        SELECT_GRADIENT,
    }

    let isInProgress = false;

    let currentPage: Page = Page.SUBMIT_TX_ID;

    $: {
        const hasDonated = $loginData.currentUserData?.placeData?.hasDonated;

        if (hasDonated == null || !hasDonated) {
            currentPage = Page.SUBMIT_TX_ID;
        } else {
            currentPage = Page.SELECT_GRADIENT;
        }
    }

    $: {
        if ($openMenu != ExclusiveMenus.Kofi) {
            modal?.close();
        } else if ($openMenu == ExclusiveMenus.Kofi) {
            modal.showModal();
        }
    }

    let allowClose = true;
    $: allowClose = !isInProgress;

    let kofiTxId: string = "";
    let isValidKofiTxId = false;
    $: isValidKofiTxId = VALID_KOFI_TRANSACTION_ID.test(kofiTxId);

    let nameGradientString: string;
    let nameGradientStops: number[] = $currentNameGradient.positions ?? [
        0, 50, 100,
    ];
    let nameGradientColors: string[] = $currentNameGradient.colors ?? [
        "#ff0000",
        "#00ff00",
        "#0000ff",
    ];

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
                "Transaction ID recieved successfully. Thanks for keeping this project going! <3"
            );
        } catch (e) {
            console.error(`Failed to submit tx id: ${e}`);
            Toast.showErrorToast(
                "There was an issue submitting transaction ID. Please try again."
            );
        }

        isInProgress = false;

        resetSubmitButton();
    };

    const gradientCooldown = new SyncedCooldown(
        `userData/${$loginData.currentUserData?.userData?.uid ?? ""}`,
        "epochNextGradient",
        GRADIENT_COOLDOWN_SECONDS
    );
    let {
        display: gradientCooldownDisplay,
        finished: gradientCooldownFinished,
    } = gradientCooldown;

    const onUpdateGradient = async () => {
        try {
            await changeNameGradient({
                grad: nameGradientString,
            });

            $currentNameGradient.positions = nameGradientStops;
            $currentNameGradient.colors = nameGradientColors;

            gradientCooldown.start();

            Toast.showSuccessToast("Successfully updated gradient!");
        } catch (e) {
            console.error(`Failed to update gradient: ${e}`);
            Toast.showErrorToast(
                "There was an updating your gradient. Please try again."
            );
        }

        resetGradientButton();
    };
</script>

<dialog
    aria-label="Change name color modal"
    class="overflow-visible pointer-events-auto dialog-panel"
    bind:this={modal}
>
    <ToastContainer />

    <div
        class={cx({
            "menu-panel xs:h-96 xs:w-80 w-[450px]": true,
            "h-96": currentPage === Page.SUBMIT_TX_ID,
            "h-[500px]": currentPage === Page.SELECT_GRADIENT,
        })}
    >
        {#if currentPage === Page.SUBMIT_TX_ID}
            <div
                class="grid grid-rows-[min-content_1fr] items-start h-full p-6 xs:p-4"
            >
                <hgroup class="flex flex-col items-center justify-center gap-1">
                    <h1
                        class="text-2xl text-center xs:text-xl font-pusab text-stroke"
                    >
                        Enter Kofi Transaction ID
                    </h1>
                    <p class="text-sm text-center xs:text-xs text-white/55">
                        EX.: <wbr />00000000-1111-2222-3333-444444444444
                    </p>
                    <!-- TODO: add turnstile + cooldown? -->
                </hgroup>
                <div class="flex-col h-full gap-2 flex-center">
                    <div class="w-full gap-2 flex-center">
                        {#if isValidKofiTxId}
                            <Check
                                class="text-[#47ff47] w-7 h-7 shrink-0 ml-auto"
                            />
                        {:else}
                            <Cross
                                class="text-[#ff4747] w-7 h-7 shrink-0 ml-auto"
                            />
                        {/if}
                        <form
                            class="w-full"
                            id="kofi-tx-id-form"
                            on:submit={e => e.preventDefault()}
                        >
                            <Input
                                class="w-[inherit] h-12 text-base xs:text-sm text-center rounded-lg outline-none font-pusab text-stroke bg-black/40 px-2"
                                maxLength={36}
                                hardValidInput={VALID_KOFI_TRANSACTION_ID_CHARS}
                                autoTrim
                                bind:value={kofiTxId}
                            />
                        </form>
                    </div>
                </div>
                <OnceButton
                    form="kofi-tx-id-form"
                    disabled={!isValidKofiTxId}
                    class="w-full p-2 h-min"
                    type="white"
                    on:click={onSubmitTxId}
                    bind:reset={resetSubmitButton}
                >
                    <p class="text-lg xs:text-base">Submit</p>
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
                        class="m-auto username-gradient w-min"
                        style={`
                            background-image: ${nameGradientString};
                        `}
                    >
                        {$loginData.currentUserData?.placeData?.username ?? ""}
                    </p>
                    <p
                        class="m-auto username-gradient w-min font-pusab"
                        style={`
                            background-image: ${nameGradientString};
                        `}
                    >
                        {$loginData.currentUserData?.placeData?.username ?? ""}
                    </p>
                </div>
                <div class="flex-col h-full gap-2 px-4 py-1">
                    <GradientPicker
                        maxStops={MAX_GRADIENT_STOPS}
                        bind:rotatedGradientString={nameGradientString}
                        bind:gradientStops={nameGradientStops}
                        bind:gradientColors={nameGradientColors}
                    ></GradientPicker>
                </div>
                <OnceButton
                    class="w-full p-2 h-min"
                    type="white"
                    disabled={!$gradientCooldownFinished}
                    on:click={onUpdateGradient}
                    bind:reset={resetGradientButton}
                >
                    <p class="text-lg xs:text-base">Update</p>
                </OnceButton>
                {#if !$gradientCooldownFinished}
                    <p
                        class="text-sm text-center transition duration-500 text-white/50 hover:text-white"
                    >
                        You changed your gradient recently! Please wait {$gradientCooldownDisplay}
                        before changing it again.
                    </p>
                {/if}
            </div>
        {/if}
        {#if isInProgress}
            <Loading class="top-0 rounded-xl" />
        {/if}
    </div>
    <div class="flex items-center h-12 text-white xs:h-10 flex-center -z-10">
        <div class="h-full">
            <button
                disabled={!allowClose}
                class={cx({
                    "flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200": true,
                    "text-disabled-white pointer-events-none": !allowClose,
                })}
                aria-label="Close"
                on:click={() => {
                    $openMenu = null;
                }}
            >
                <Cross alt="Close" class="w-full h-full"></Cross>
            </button>
        </div>
    </div>
</dialog>

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
