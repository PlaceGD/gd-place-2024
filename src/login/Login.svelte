<script lang="ts">
    import { default as cx } from "classnames";
    import Image from "../components/Image.svelte";
    import Loading from "../components/Loading.svelte";
    import { LoginMethod, handleSignIn } from "./login";
    import {
        ExclusiveMenus,
        hasLoggedInBefore,
        loginData,
        openMenu,
    } from "../stores";
    import Cross from "../icons/Cross.svelte";
    import Check from "../icons/Check.svelte";
    import Toast from "../utils/toast";
    import { VALID_USERNAME, VALID_USERNAME_CHARS } from "shared-lib/user";
    import { initUserData } from "../firebase/auth";
    import { db } from "../firebase/firebase";
    // import { Turnstile } from "svelte-turnstile";
    import FadedScroll from "../components/FadedScroll.svelte";
    import Button from "../components/Button.svelte";
    import { SvelteToast, toast } from "@zerodevx/svelte-toast";
    import ToastContainers from "../components/ToastContainers.svelte";
    import ScreenModal from "../components/ScreenModal.svelte";
    import DarkInput from "../components/DarkInput.svelte";

    import twitterIconUrl from "./assets/twitter.svg?url";
    import googleIconUrl from "./assets/google.svg?url";
    import githubIconUrl from "./assets/github.svg?url";
    import xIconUrl from "./assets/x.svg?url";
    import AcceptButton from "../components/Buttons/AcceptButton.svelte";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";

    let twitter = true;
    document.addEventListener("keydown", e => {
        twitter = !e.shiftKey;
    });
    document.addEventListener("keyup", e => {
        twitter = !e.shiftKey;
    });

    enum Page {
        LOGIN_METHOD,
        CREATE_USER,
        SHOW_TOS,
    }

    let currentPage = Page.LOGIN_METHOD;
    let previousPage = Page.LOGIN_METHOD;

    let hasAgreedToTOS = false;

    let isValidUsername = false;
    $: isValidUsername = VALID_USERNAME.test(userName);
    let userName = "";

    let isInProgress = false;
    let isOpen = false;
    $: canClose = currentPage == Page.LOGIN_METHOD ? !isInProgress : false;

    let hasScrolledToBottomOfTos = false;

    $: {
        if ($openMenu != ExclusiveMenus.Login) {
            currentPage = Page.LOGIN_METHOD;
            isOpen = false;
        } else if ($openMenu == ExclusiveMenus.Login) {
            isOpen = true;
        }
    }

    const signInWith = (method: LoginMethod) => {
        isInProgress = true;
        handleSignIn(method)
            .then(async isOK => {
                if (isOK) {
                    if ($loginData.currentUserData != null) {
                        let maybeData = await db
                            .ref(
                                `userDetails/${$loginData.currentUserData.user.uid}`
                            )
                            .get();

                        let maybePlaceData = maybeData.val();

                        if (maybePlaceData != null) {
                            $loginData.currentUserData.userDetails =
                                maybePlaceData;
                            $openMenu = null;
                        } else {
                            previousPage = currentPage;
                            currentPage = Page.CREATE_USER;
                        }

                        isInProgress = false;
                    } else {
                        isInProgress = false;
                        console.error(
                            "login OK (isOk == true) but `currentUserData` still null"
                        );
                        Toast.showErrorToast(
                            "There was an issue signing in. Please try again."
                        );
                    }
                } else {
                    isInProgress = false;
                }
            })
            .catch(e => {
                console.error("handleSignIn error", e);
                Toast.showErrorToast(
                    "There was an issue signing in. Please try again."
                );
            });
    };

    const initNewUser = () => {
        isInProgress = true;
        if ($loginData.currentUserData != null) {
            initUserData(
                $loginData.currentUserData.user.uid,
                userName,
                turnstileToken! // cant submit unless the token is not null
            )
                .then(() => {
                    isInProgress = false;
                    Toast.showSuccessToast(
                        "User successfully created! Thanks for participating!"
                    );
                    $openMenu = null;

                    hasLoggedInBefore.set(true);
                })
                .catch(e => {
                    isInProgress = false;

                    console.error("Failed to create user", e.details.message);

                    if (e.details.code === 300 || e.details.code === 301) {
                        Toast.showErrorToast("Username is taken!");
                    } else if (e.details.code === 100) {
                        Toast.showErrorToast("Invalid username!");
                    } else {
                        Toast.showErrorToast(
                            `Failed to create user. (${e.details.code})`
                        );
                    }
                });

            turnstileReset && turnstileReset();
        } else {
            isInProgress = false;
            console.error(
                "User data set before user was created! (`initNewUser`)"
            );
            Toast.showErrorToast(
                "There was an issue signing in. Please try again."
            );
        }
    };

    let turnstileToken: string | null = "";
    const SITE_KEY = __TURNSTILE_LOGIN_SITE_KEY;
    let turnstileReset: () => void | undefined;
</script>

<Image tabindex="-1" src={twitterIconUrl} class="hidden" alt="" />
<ScreenModal
    label="Login or Sign Up Modal"
    state={isInProgress ? "loading" : "default"}
    {canClose}
    {isOpen}
    hasCloseButton={true}
    on:close={() => {
        $openMenu = null;
    }}
>
    <!-- LOGIN METHOD -->
    <div
        class="absolute flex flex-col items-center justify-between gap-2 modal-panel"
        style:visibility={currentPage === Page.LOGIN_METHOD
            ? "visible"
            : "hidden"}
    >
        <h1 class="text-3xl text-center xs:text-2xl font-pusab text-stroke">
            Login or Sign Up
        </h1>
        <ul class="grid w-full grid-cols-3 gap-4 xs:gap-2">
            <li>
                <WhiteButton
                    class="flex-col w-full h-full gap-2"
                    aria-label="Login with Twitter"
                    on:click={() => signInWith(LoginMethod.Google)}
                >
                    <Image
                        src={googleIconUrl}
                        alt="Login with Google"
                        class="w-11 xs:w-10 aspect-square"
                        tabindex="-1"
                    />
                    <p>Google</p>
                </WhiteButton>
            </li>
            <li>
                <WhiteButton
                    class="flex-col w-full h-full gap-2"
                    aria-label="Login with GitHub"
                    on:click={() => signInWith(LoginMethod.GitHub)}
                >
                    <Image
                        src={githubIconUrl}
                        alt="Login with GitHub"
                        class="w-11 xs:w-10 aspect-square"
                        tabindex="-1"
                    />
                    <p>GitHub</p>
                </WhiteButton>
            </li>
            <li>
                <WhiteButton
                    class="flex-col w-full h-full gap-2"
                    on:click={() => signInWith(LoginMethod.X)}
                >
                    <Image
                        src={twitter ? twitterIconUrl : xIconUrl}
                        alt="Login with X (Twitter)"
                        class="w-11 xs:w-10 aspect-square"
                        tabindex="-1"
                    />
                    <p>
                        {twitter ? "Twitter" : "the everything app"}
                    </p>
                </WhiteButton>
            </li>
        </ul>
        <p class="text-sm text-center">
            Don't forget to the read the
            <button
                class="underline hover:decoration-dashed"
                aria-label="Terms of Service"
                on:click={() => {
                    previousPage = currentPage;
                    currentPage = Page.SHOW_TOS;
                }}
            >
                rules
            </button>!
        </p>
    </div>

    <!-- TERMS OF SERVICE -->
    <div
        class="grid gap-4 modal-panel grid-rows-[minmax(0,_1fr)_min-content] absolute"
        style:visibility={currentPage === Page.SHOW_TOS ? "visible" : "hidden"}
    >
        <FadedScroll
            bind:reachedBottom={hasScrolledToBottomOfTos}
            update={currentPage}
        >
            <section class="text">
                <h1>Rules</h1>

                <ul class="bulleted-list">
                    <li>Only use one account per person.</li>
                    <li>Do not create inappropriate imagery (or usernames).</li>
                    <li>Do not exercise hate speech (please).</li>
                    <li>
                        Only report people who are breaking the rules.
                        <!-- <i style="font-size: small; opacity: 0.5;">
                            (unless you and another user are both reporting each
                            other for breaking this rule, in which case one of
                            you should break one other rule so that the other
                            one is no longer breaking a rule)
                        </i> -->
                    </li>
                </ul>

                <strong>
                    Breaking any of these rules can get your account banned
                    without notice.
                </strong>
            </section>
        </FadedScroll>

        <div class="flex w-full gap-4">
            <AcceptButton
                class="w-full"
                on:click={() => {
                    hasAgreedToTOS = true;
                    currentPage = previousPage;
                }}
            >
                <p class="xs:text-sm">Agree</p>
            </AcceptButton>
        </div>
    </div>

    <!-- CREATE USER -->
    <div
        class="absolute flex flex-col items-center justify-between text-center modal-panel"
        style:visibility={currentPage === Page.CREATE_USER
            ? "visible"
            : "hidden"}
    >
        <h1 class="text-3xl xs:text-2xl font-pusab text-stroke">
            Enter a Username
        </h1>
        <div class="flex-col gap-2 flex-center">
            <div class="w-full gap-2 flex-center">
                {#if isValidUsername}
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
                    id="username-form"
                    on:submit={e => e.preventDefault()}
                >
                    <DarkInput
                        class="w-[inherit] text-2xl sm:text-xl xs:text-base font-pusab"
                        maxLength={16}
                        hardValidInput={VALID_USERNAME_CHARS}
                        autoTrim
                        bind:value={userName}
                    />
                </form>
            </div>
            <p
                class="text-xs transition duration-500 text-white/50 hover:text-white"
            >
                Usernames can only be 3 to 16 characters in length, and only
                contain alphanumeric characters, - and _. Usernames are case
                insensitive.
            </p>
        </div>
        <div class="flex w-full gap-2">
            {#if hasAgreedToTOS}
                <Check
                    class="text-[#47ff47] xs:w-7 xs:h-7 w-8 h-8 shrink-0 stroke-[1.5]"
                />
            {:else}
                <Cross
                    class="text-[#ff4747] xs:w-7 xs:h-7 w-8 h-8 shrink-0 stroke-[1.5]"
                />
            {/if}
            <p class="flex-auto m-auto text-base xs:text-sm">
                I have read and agreed to the
                <button
                    class="underline hover:decoration-dashed text-nowrap"
                    aria-label="Terms of Service"
                    on:click={() => {
                        previousPage = currentPage;
                        currentPage = Page.SHOW_TOS;
                    }}
                >
                    rules
                </button>
            </p>
        </div>
        <WhiteButton
            form="username-form"
            disabled={!hasAgreedToTOS ||
                !isValidUsername ||
                turnstileToken == null}
            class="w-full"
            on:click={initNewUser}
        >
            <p class="text-lg xs:text-base">Submit</p>
        </WhiteButton>
    </div>
</ScreenModal>
