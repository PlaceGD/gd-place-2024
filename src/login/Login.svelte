<script lang="ts">
    import { default as cx } from "classnames";
    import Image from "../components/Image.svelte";
    import Loading from "../components/Loading.svelte";
    import { LoginMethod, handleSignIn } from "./login";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import { IconX as Cross } from "@tabler/icons-svelte";
    import { IconCheck as Check } from "@tabler/icons-svelte";
    import Toast from "../utils/toast";
    import { VALID_USERNAME, VALID_USERNAME_CHARS } from "shared-lib/user";
    import { initUserData } from "../firebase/auth";
    import { db } from "../firebase/firebase";
    import { Turnstile } from "svelte-turnstile";
    import FadedScroll from "../components/FadedScroll.svelte";
    import Button from "../components/Button.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import ToastContainer from "../components/ToastContainer.svelte";
    import ScreenModal from "../components/ScreenModal.svelte";
    import DarkInput from "../components/DarkInput.svelte";

    import twitterIconUrl from "./assets/twitter.svg?url";
    import googleIconUrl from "./assets/google.svg?url";
    import githubIconUrl from "./assets/github.svg?url";
    import xIconUrl from "./assets/x.svg?url";

    let twitter = false;
    document.addEventListener("keydown", e => {
        twitter = e.shiftKey;
    });
    document.addEventListener("keyup", e => {
        twitter = e.shiftKey;
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
            // modal?.close();
        } else if ($openMenu == ExclusiveMenus.Login) {
            isOpen = true;
            // modal.showModal();
        }
    }

    const signInWith = (method: LoginMethod) => {
        isInProgress = true;
        handleSignIn(method).then(async isOK => {
            if (isOK) {
                if ($loginData.currentUserData != null) {
                    let maybeData = await db
                        .ref(
                            `userData/${$loginData.currentUserData.userData.uid}`
                        )
                        .get();
                    let maybePlaceData = maybeData.val();

                    if (maybePlaceData != null) {
                        $loginData.currentUserData.placeData = maybePlaceData;
                        $loginData.isLoggedIn = true;
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
        });
    };

    const initNewUser = () => {
        isInProgress = true;
        if ($loginData.currentUserData != null) {
            initUserData(
                $loginData.currentUserData.userData.uid,
                userName,
                turnstileToken! // cant submit unless the token is not null
            )
                .then(() => {
                    isInProgress = false;
                    Toast.showSuccessToast(
                        "User successfully created! Thanks for participating!"
                    );
                    $openMenu = null;
                    $loginData.isLoggedIn = true;
                })
                .catch(e => {
                    isInProgress = false;
                    console.error(e);
                    Toast.showErrorToast("Username already taken!");
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

    let turnstileToken: string | null = null;
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
    on:close={() => ($openMenu = null)}
>
    <!-- LOGIN METHOD -->
    {#if currentPage == Page.LOGIN_METHOD}
        <div
            class="flex flex-col items-center justify-between gap-2 modal-panel"
        >
            <h1 class="text-3xl text-center xs:text-2xl font-pusab text-stroke">
                Login or Sign Up
            </h1>
            <ul class="grid w-full grid-cols-3 gap-4 xs:gap-2">
                <li>
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        aria-label="Login with Twitter"
                        on:click={() => signInWith(LoginMethod.Google)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src={googleIconUrl}
                                alt="Login with Google"
                                class="w-11 xs:w-10 aspect-square"
                                tabindex="-1"
                            />
                            <p>Google</p>
                        </span>
                    </Button>
                </li>
                <li>
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        aria-label="Login with GitHub"
                        on:click={() => signInWith(LoginMethod.GitHub)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src={githubIconUrl}
                                alt="Login with GitHub"
                                class="w-11 xs:w-10 aspect-square"
                                tabindex="-1"
                            />
                            <p>GitHub</p>
                        </span>
                    </Button>
                </li>
                <li>
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        on:click={() => signInWith(LoginMethod.X)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src={twitter ? twitterIconUrl : xIconUrl}
                                alt="Login with X (Twitter)"
                                class="w-11 xs:w-10 aspect-square"
                                tabindex="-1"
                            />
                            <p>
                                {twitter ? "Twitter" : "the everything app"}
                            </p>
                        </span>
                    </Button>
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
                </button>
                !
            </p>
        </div>
    {/if}
    <!-- TERMS OF SERVICE -->
    {#if currentPage == Page.SHOW_TOS}
        <div
            class="grid gap-4 modal-panel grid-rows-[minmax(0,_1fr)_min-content]"
        >
            <FadedScroll bind:reachedBottom={hasScrolledToBottomOfTos}>
                <section class="text">
                    <!-- svelte-ignore a11y-autofocus -->
                    <button aria-hidden="true" tabindex="-1" autofocus
                        ><h1><u><strong>rules</strong></u></h1></button
                    >

                    <h2>Notice</h2>

                    <p>
                        Welcome to GD Place! By accessing or using our platform,
                        you agree to be bound by these Terms of Service. Please
                        read them carefully before proceeding.
                    </p>

                    <h2>User Conduct</h2>

                    <ul class="bulleted-list">
                        <li>
                            <strong>Inappropriate Usernames:</strong> Users must
                            not use inappropriate or offensive usernames when registering
                            or using our platform.
                        </li>
                        <li>
                            <strong>Prohibited Activities:</strong> Botting or the
                            use of alternate accounts (alt accounts) is strictly
                            prohibited. Any violations may result in the termination
                            of your account.
                        </li>
                    </ul>

                    <h2>Reporting Violations</h2>

                    <p>
                        Users who notice rule violations are encouraged to
                        report them. You can report violations within the app or
                        by contacting us via our contact information provided
                        below.
                    </p>

                    <h2>Account Registration</h2>

                    <p>
                        Users can sign up using one of three external platforms.
                        By signing up, users also agree to the rules of those
                        platforms.
                    </p>

                    <h2>Ownership of User Content</h2>

                    <p>
                        By using our platform, you grant us (the creators)
                        permission to use any content created by you on the site
                        without requiring explicit permission.
                    </p>

                    <h2>External Services</h2>

                    <p>
                        Donations and merchandise are handled by external sites.
                        We are not responsible for transactions made on these
                        external platforms.
                    </p>

                    <h2>Termination</h2>

                    <p>
                        We reserve the right to terminate accounts for any
                        violations of these Terms of Service.
                    </p>

                    <h2>Contact Information</h2>

                    <p>
                        For any inquiries or concerns about these Terms of
                        Service, please contact us:
                    </p>
                    <ul class="bulleted-list">
                        <li>
                            Twitter: <a
                                href="https://twitter.com/<twitter_handle>"
                                >@TODO</a
                            >
                        </li>
                        <li>
                            Email: <a href="mailto:geometrydash.place@gmail.com"
                                >geometrydash.place@gmail.com</a
                            >
                        </li>
                    </ul>

                    <h2>Acceptance of Privacy Policy</h2>

                    <p>
                        Our Privacy Policy is presented as a popup on our
                        website. By using our platform, you agree to our Privacy
                        Policy.
                    </p>
                </section>
            </FadedScroll>

            <div class="flex w-full gap-4">
                <Button
                    class="w-full h-full"
                    type="decline"
                    disabled={!hasScrolledToBottomOfTos}
                    on:click={() => {
                        hasAgreedToTOS = false;
                        $openMenu = null;
                    }}
                >
                    <p class="xs:text-sm">Disagree</p>
                </Button>
                <Button
                    class="w-full h-full"
                    type="accept"
                    disabled={!hasScrolledToBottomOfTos}
                    on:click={() => {
                        hasAgreedToTOS = true;
                        currentPage = previousPage;
                    }}
                >
                    <p class="xs:text-sm">Agree</p>
                </Button>
            </div>
        </div>
    {/if}
    <!-- CREATE USER -->
    {#if currentPage == Page.CREATE_USER}
        <div
            class="flex flex-col items-center justify-between text-center modal-panel"
        >
            <h1 class="text-3xl xs:text-2xl font-pusab text-stroke">
                Enter a Username
            </h1>
            <div class="flex-col gap-2 flex-center">
                <div class="w-full gap-2 flex-center">
                    {#if isValidUsername}
                        <Check
                            class="text-[#47ff47] xs:w-7 xs:h-7 w-8 h-8 shrink-0 ml-auto stroke-1"
                        />
                    {:else}
                        <Cross
                            class="text-[#ff4747] xs:w-7 xs:h-7 w-8 h-8 shrink-0 ml-auto stroke-1"
                        />
                    {/if}
                    <form
                        class="w-full"
                        id="username-form"
                        on:submit={e => e.preventDefault()}
                    >
                        <DarkInput
                            class="w-[inherit] text-2xl xs:text-lg font-pusab"
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
            <span
                class="flex items-center justify-center w-full h-auto xs:scale-90"
            >
                <Turnstile
                    siteKey={SITE_KEY}
                    bind:reset={turnstileReset}
                    on:turnstile-callback={e =>
                        (turnstileToken = e.detail.token)}
                    on:turnstile-error={e => {
                        console.error(e);
                        Toast.showErrorToast(
                            `There was an error with the Turnstile. (${e})`
                        );
                    }}
                    on:turnstile-expired={() =>
                        turnstileReset && turnstileReset()}
                />
            </span>
            <div class="flex w-full gap-2">
                {#if hasAgreedToTOS}
                    <Check
                        class="text-[#47ff47] xs:w-7 xs:h-7 w-8 h-8 shrink-0 stroke-1"
                    />
                {:else}
                    <Cross
                        class="text-[#ff4747] xs:w-7 xs:h-7 w-8 h-8 shrink-0 stroke-1"
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
            <Button
                form="username-form"
                disabled={!hasAgreedToTOS ||
                    !isValidUsername ||
                    turnstileToken == null}
                class="w-full p-2 h-min"
                on:click={initNewUser}
                type="white"
            >
                <p class="text-lg xs:text-base">Submit</p>
            </Button>
        </div>
    {/if}
</ScreenModal>
