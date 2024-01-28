<script lang="ts">
    import { default as cx } from "classnames";
    import Image from "../components/Image.svelte";
    import Loading from "../components/Loading.svelte";
    import { LoginMethod, handleSignIn } from "./login";
    import { loginData } from "../stores";
    import Cross from "../icons/cross.svg";
    import Check from "../icons/check.svg";
    import Toast from "../utils/toast";
    import Input from "../components/Input.svelte";
    import { VALID_USERNAME, VALID_USERNAME_CHARS } from "shared-lib";
    import { initUserData } from "../firebase/auth";
    import { ref, get } from "firebase/database";
    import { db } from "../firebase/firebase";

    let twitter = false;
    document.addEventListener("keydown", e => {
        if (e.key == "Escape" && allowClose) {
            $loginData.showLoginUI = false;
        } else {
            twitter = e.shiftKey;
        }
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
    let userName = "";

    let isInProgress = false;
    $: allowClose = currentPage == Page.LOGIN_METHOD ? !isInProgress : false;

    const signInWith = (method: LoginMethod) => {
        isInProgress = true;
        handleSignIn(method).then(async isOK => {
            if (isOK) {
                if ($loginData.currentUserData !== null) {
                    let maybeData = await get(
                        ref(
                            db,
                            `userData/${$loginData.currentUserData.userData.uid}`
                        )
                    );

                    let maybePlaceData = maybeData.val();

                    if (maybePlaceData !== null) {
                        $loginData.currentUserData.placeData = maybePlaceData;
                        $loginData.isLoggedIn = true;
                        $loginData.showLoginUI = false;
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
        if ($loginData.currentUserData !== null) {
            initUserData($loginData.currentUserData.userData.uid, userName)
                .then(() => {
                    isInProgress = false;
                    Toast.showSuccessToast(
                        "User successfully created! Thanks for participating!"
                    );
                    $loginData.showLoginUI = false;
                    $loginData.isLoggedIn = true;
                })
                .catch(e => {
                    isInProgress = false;
                    console.error(e);
                    Toast.showErrorToast("Username already taken!");
                });
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
</script>

{#if $loginData.showLoginUI}
    <Image
        tabindex="-1"
        src="/assets/ui/login/twitter.svg"
        class="hidden"
        alt=""
    />
    <section
        class="absolute z-40 flex-col w-full h-full text-white flex-center"
        aria-label="Login or Sign Up"
    >
        <div class="flex flex-col w-auto h-auto gap-2">
            <div
                class="flex overflow-hidden rounded-lg shadow-lg w-96 aspect-square xs:w-80 bg-menu-gray/90 shadow-black/40 backdrop-blur-md"
            >
                <div class="w-full h-full">
                    <!-- LOGIN METHOD -->
                    {#if currentPage == Page.LOGIN_METHOD}
                        <div
                            class="absolute flex flex-col items-center justify-between w-full h-full p-6"
                        >
                            <h1
                                class="text-3xl xs:text-2xl font-pusab text-stroke"
                            >
                                Login or Sign Up
                            </h1>
                            <ul class="w-full h-24 gap-4 xs:h-20 flex-center">
                                <li class="h-full aspect-square max-w-max">
                                    <button
                                        class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center white-button"
                                        aria-label="Login with Twitter"
                                        on:click={() =>
                                            signInWith(LoginMethod.Google)}
                                    >
                                        <Image
                                            src="/assets/ui/login/google.svg"
                                            alt="Login with Google"
                                            class="flex-1 object-contain w-max"
                                        />
                                        <p>Google</p>
                                    </button>
                                </li>
                                <li
                                    class="h-full shadow-lg aspect-square max-w-max"
                                >
                                    <button
                                        class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center white-button"
                                        aria-label="Login with GitHub"
                                        on:click={() =>
                                            signInWith(LoginMethod.GitHub)}
                                    >
                                        <Image
                                            src="/assets/ui/login/github.svg"
                                            alt="Login with GitHub"
                                            class="flex-1 object-contain w-max"
                                        />
                                        <p>GitHub</p>
                                    </button>
                                </li>
                                <li class="h-full aspect-square max-w-max">
                                    <button
                                        class="flex-col w-full h-full gap-2 p-2 rounded-lg flex-center white-button"
                                        aria-label="Login with X (Twitter)"
                                        on:click={() =>
                                            signInWith(LoginMethod.X)}
                                    >
                                        <Image
                                            src="/assets/ui/login/{twitter
                                                ? 'twitter'
                                                : 'x'}.svg"
                                            alt="Login with X (Twitter)"
                                            class="flex-1 object-contain w-max"
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
                    {:else if currentPage == Page.SHOW_TOS}
                        <div
                            class="absolute w-full h-full text-center flex-center"
                        >
                            <p>HERE IS THE TOS PLEASE READ</p>
                            <button
                                on:click={() => {
                                    hasAgreedToTOS = true;
                                    currentPage = previousPage;
                                }}
                            >
                                Ok
                            </button>
                        </div>
                    {:else if currentPage == Page.CREATE_USER}
                        <div
                            class="absolute flex flex-col items-center justify-between w-full h-full p-6 text-center xs:p-4"
                        >
                            <h1
                                class="text-3xl xs:text-2xl font-pusab text-stroke"
                            >
                                Enter a Username
                            </h1>
                            <div class="flex-col gap-2 flex-center">
                                <div class="w-full gap-2 flex-center">
                                    {#if VALID_USERNAME.test(userName)}
                                        <Check
                                            class="text-[#47ff47] w-7 h-7 shrink-0 ml-auto"
                                        />
                                    {:else}
                                        <Cross
                                            class="text-[#ff4747] w-7 h-7 shrink-0 ml-auto"
                                        />
                                    {/if}
                                    <Input
                                        class="p-2 w-[inherit] text-2xl xs:text-lg text-center rounded-lg outline-none font-pusab text-stroke bg-black/40"
                                        maxLength={16}
                                        hardValidInput={VALID_USERNAME_CHARS}
                                        autoTrim
                                        bind:value={userName}
                                    />
                                </div>
                                <p
                                    class="text-xs transition duration-500 text-white/50 hover:text-white"
                                >
                                    Usernames can only be 3 to 16 characters in
                                    length, and only contain alphanumeric
                                    characters, - and _. Usernames are case
                                    insensitive.
                                </p>
                            </div>
                            <div class="w-full gap-2 flex-center">
                                {#if hasAgreedToTOS}
                                    <Check
                                        class="text-[#47ff47] w-7 h-7 shrink-0 ml-auto"
                                    />
                                {:else}
                                    <Cross
                                        class="text-[#ff4747] w-7 h-7 shrink-0 ml-auto"
                                    />
                                {/if}
                                <p class="text-md shrink-1 grow-0">
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
                            <button
                                class={cx({
                                    "text-lg xs:text-md p-2 rounded-lg bg-white/10": true,
                                    "hover:bg-white/20 active:bg-white/30":
                                        hasAgreedToTOS && userName.length > 0,
                                    "text-disabled-white cursor-not-allowed":
                                        !hasAgreedToTOS ||
                                        userName.length === 0,
                                })}
                                on:click={initNewUser}
                            >
                                Submit
                            </button>
                        </div>
                    {/if}
                </div>
                {#if isInProgress}
                    <Loading />
                {/if}
            </div>
            <div class="flex items-center h-12 text-white flex-center">
                <div class="h-full">
                    <button
                        disabled={!allowClose}
                        class={cx({
                            "flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200": true,
                            "text-disabled-white pointer-events-none":
                                !allowClose,
                        })}
                        aria-label="Close"
                        on:click={() => {
                            $loginData.showLoginUI = false;
                        }}
                    >
                        <Cross alt="Close" class="w-full h-full"></Cross>
                    </button>
                </div>
            </div>
        </div>
    </section>

    <div
        tabindex="-1"
        class="absolute z-30 w-full h-full backdrop-blur-lg brightness-30"
    ></div>
{/if}
