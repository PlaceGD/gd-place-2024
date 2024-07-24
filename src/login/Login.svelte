<script lang="ts">
    import { default as cx } from "classnames";
    import Image from "../components/Image.svelte";
    import Loading from "../components/Loading.svelte";
    import { LoginMethod, handleSignIn } from "./login";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import Cross from "../icons/cross.svg";
    import Check from "../icons/check.svg";
    import Toast from "../utils/toast";
    import Input from "../components/Input.svelte";
    import { VALID_USERNAME, VALID_USERNAME_CHARS } from "shared-lib/user";
    import { initUserData } from "../firebase/auth";
    import { ref, get } from "firebase/database";
    import { db } from "../firebase/firebase";
    import { Turnstile } from "svelte-turnstile";
    import FadedScroll from "../components/FadedScroll.svelte";
    import Button from "../components/Button.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import ToastContainer from "../components/ToastContainer.svelte";
    import ScreenModal from "../components/ScreenModal.svelte";

    let twitter = false;
    document.addEventListener("keydown", e => {
        // if (e.key == "Escape" && allowClose) {
        //     $openMenu = null;
        // } else {
        //     twitter = e.shiftKey;
        // }
        twitter = e.shiftKey;
    });
    // document.addEventListener("keyup", e => {
    //     twitter = e.shiftKey;
    // });

    // let modal: HTMLDialogElement;

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
    // $: allowClose = currentPage == Page.LOGIN_METHOD ? !isInProgress : false;

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
                    let maybeData = await get(
                        ref(
                            db,
                            `userData/${$loginData.currentUserData.userData.uid}`
                        )
                    );

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

<Image tabindex="-1" src="/assets/ui/login/twitter.svg" class="hidden" alt="" />
<ScreenModal
    label="Login or Sign Up Modal"
    state={isInProgress ? "loading" : "default"}
    canClose={!isInProgress && currentPage !== Page.SHOW_TOS}
    {isOpen}
    hasCloseButton={true}
    on:close={() => ($openMenu = null)}
>
    <!-- LOGIN METHOD -->
    <div
        class="login-page"
        style:display={currentPage == Page.LOGIN_METHOD ? "flex" : "none"}
    >
        <div
            class="flex flex-col items-center justify-between w-full h-full p-6 xs:p-4"
        >
            <h1 class="text-3xl xs:text-2xl font-pusab text-stroke">
                Login or Sign Up
            </h1>
            <ul class="w-full h-24 gap-4 xs:h-20 flex-center">
                <li class="h-full aspect-square max-w-max">
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        aria-label="Login with Twitter"
                        on:click={() => signInWith(LoginMethod.Google)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src="/assets/ui/login/google.svg"
                                alt="Login with Google"
                                class="flex-1 object-contain w-max"
                            />
                            <p>Google</p>
                        </span>
                    </Button>
                </li>
                <li class="h-full shadow-lg aspect-square max-w-max">
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        aria-label="Login with GitHub"
                        on:click={() => signInWith(LoginMethod.GitHub)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src="/assets/ui/login/github.svg"
                                alt="Login with GitHub"
                                class="flex-1 object-contain w-max"
                            />
                            <p>GitHub</p>
                        </span>
                    </Button>
                </li>
                <li class="h-full aspect-square max-w-max">
                    <Button
                        type="white"
                        class="flex-col w-full h-full p-2"
                        on:click={() => signInWith(LoginMethod.X)}
                    >
                        <span class="flex flex-col h-full gap-2 flex-center">
                            <Image
                                src="/assets/ui/login/{twitter
                                    ? 'twitter'
                                    : 'x'}.svg"
                                alt="Login with X (Twitter)"
                                class="flex-1 object-contain w-max"
                            />
                            <p>{twitter ? "Twitter" : "X"}</p>
                        </span>
                    </Button>
                </li>
            </ul>
            <p class="text-sm">
                Don't forget to the read the
                <button
                    class="underline hover:decoration-dashed"
                    aria-label="Terms of Service"
                    on:click={() => {
                        previousPage = currentPage;
                        currentPage = Page.SHOW_TOS;
                    }}
                >
                    TOS
                </button>
                !
            </p>
        </div>
    </div>
    <!-- TERMS OF SERVICE -->
    <div
        class="flex flex-col gap-4 login-page"
        style:display={currentPage == Page.SHOW_TOS ? "flex" : "none"}
    >
        <FadedScroll bind:reachedBottom={hasScrolledToBottomOfTos}>
            <section>
                <h1><u><strong>Terms of Service</strong></u></h1>

                <h2>Notice</h2>

                <p>
                    Welcome to GD Place! By accessing or using our platform, you
                    agree to be bound by these Terms of Service. Please read
                    them carefully before proceeding.
                </p>

                <h2>User Conduct</h2>

                <ul class="bulleted-list">
                    <li>
                        <strong>Inappropriate Usernames:</strong> Users must not
                        use inappropriate or offensive usernames when registering
                        or using our platform.
                    </li>
                    <li>
                        <strong>Prohibited Activities:</strong> Botting or the use
                        of alternate accounts (alt accounts) is strictly prohibited.
                        Any violations may result in the termination of your account.
                    </li>
                </ul>

                <h2>Reporting Violations</h2>

                <p>
                    Users who notice rule violations are encouraged to report
                    them. You can report violations within the app or by
                    contacting us via our contact information provided below.
                </p>

                <h2>Account Registration</h2>

                <p>
                    Users can sign up using one of three external platforms. By
                    signing up, users also agree to the Terms of Service of
                    those platforms.
                </p>

                <h2>Ownership of User Content</h2>

                <p>
                    By using our platform, you grant us (the creators)
                    permission to use any content created by you on the site
                    without requiring explicit permission.
                </p>

                <h2>External Services</h2>

                <p>
                    Donations and merchandise are handled by external sites. We
                    are not responsible for transactions made on these external
                    platforms.
                </p>

                <h2>Termination</h2>

                <p>
                    We reserve the right to terminate accounts for any
                    violations of these Terms of Service.
                </p>

                <h2>Contact Information</h2>

                <p>
                    For any inquiries or concerns about these Terms of Service,
                    please contact us:
                </p>
                <ul class="bulleted-list">
                    <li>
                        Twitter: <a href="https://twitter.com/<twitter_handle>"
                            >@<twitter_handle></twitter_handle></a
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
                    Our Privacy Policy is presented as a popup on our website.
                    By using our platform, you agree to our Privacy Policy.
                </p>
            </section>
        </FadedScroll>
        <div class="flex w-full gap-4">
            <Button
                class="w-full h-full"
                type="decline"
                disabled={!hasScrolledToBottomOfTos}
                on:click={() => {
                    currentPage = Page.LOGIN_METHOD;
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
    <div
        class="flex-col items-center justify-between text-center login-page"
        style:display={currentPage == Page.CREATE_USER ? "flex" : "none"}
    >
        <!-- CREATE USER -->
        <!-- {:else if currentPage == Page.CREATE_USER} -->

        <h1 class="text-3xl xs:text-2xl font-pusab text-stroke">
            Enter a Username
        </h1>
        <div class="flex-col gap-2 flex-center">
            <div class="w-full gap-2 flex-center">
                {#if isValidUsername}
                    <Check class="text-[#47ff47] w-7 h-7 shrink-0 ml-auto" />
                {:else}
                    <Cross class="text-[#ff4747] w-7 h-7 shrink-0 ml-auto" />
                {/if}
                <form
                    class="w-full"
                    id="username-form"
                    on:submit={e => e.preventDefault()}
                >
                    <Input
                        class="p-2 w-[inherit] text-2xl xs:text-lg text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
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
        <!-- <div id="cf-turnstile" bind:this={cfTurnstile}></div> -->
        <Turnstile
            siteKey={SITE_KEY}
            bind:reset={turnstileReset}
            on:turnstile-callback={e => (turnstileToken = e.detail.token)}
            on:turnstile-error={e =>
                Toast.showErrorToast(
                    `There was an error with the Turnstile. (${e})`
                )}
            on:turnstile-expired={() => turnstileReset && turnstileReset()}
        />
        <div class="w-full gap-2 flex-center">
            {#if hasAgreedToTOS}
                <Check class="text-[#47ff47] w-7 h-7 shrink-0 ml-auto" />
            {:else}
                <Cross class="text-[#ff4747] w-7 h-7 shrink-0 ml-auto" />
            {/if}
            <p class="text-base shrink-1 grow-0">
                I have read and agreed to the
                <button
                    class="underline hover:decoration-dashed text-nowrap"
                    aria-label="Terms of Service"
                    on:click={() => {
                        previousPage = currentPage;
                        currentPage = Page.SHOW_TOS;
                    }}
                >
                    Terms of Service
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
</ScreenModal>

<style lang="postcss">
    .login-page {
        @apply h-full w-full overflow-hidden p-6 xs:p-4;
    }
</style>
