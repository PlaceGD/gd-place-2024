<script lang="ts">
    import Toast from "../../utils/toast";
    import { onMount } from "svelte";
    import { type SliderMethods, LoginMethod } from "../login";
    import TOS from "./TOS.svelte";
    import {
        signInGithub,
        signInGoogle,
        signInTwitter,
        type UserProperties,
    } from "../../firebase/auth";
    import Back from "../icons/back.svg";
    import { get, ref } from "firebase/database";
    import { db } from "../../firebase/firebase";

    export let slider: SliderMethods;
    export let method: LoginMethod;

    let currentUser: UserProperties | null = null;

    const logInSuccess = (user: any) => {
        get(ref(db, `userData/${user.user.uid}`))
            .then(snapshot => {
                currentUser = snapshot.val();

                Toast.showSuccessToast("Signed in successfully!");

                if (currentUser === null) {
                } else {
                    // todo
                }
            })
            .catch(err => {
                Toast.showErrorToast(`Failed to get user data! (${err})`);
                slider.previous();
            });
    };

    const logInFailed = (err: any) => {
        console.error(err);
        Toast.showErrorToast(`Failed to login!`);

        slider.previous();
    };

    onMount(() => {
        slider.setInteractability(false);

        switch (method) {
            case LoginMethod.Google:
                signInGoogle().then(logInSuccess).catch(logInFailed);
                break;
            case LoginMethod.GitHub:
                signInGithub().then(logInSuccess).catch(logInFailed);
                break;
            case LoginMethod.X:
                signInTwitter().then(logInSuccess).catch(logInFailed);
                break;
        }
    });
</script>

<div class="w-full h-full flex-center text-center">
    <div
        class="relative flex flex-col items-center justify-between w-full h-full p-6 shrink-0 text-white"
    >
        <h1 class="text-3xl font-pusab text-stroke">{method}</h1>

        <p>
            A popup should open to continue the sign-in process. If it does not,
            please refresh.
        </p>

        {#if localStorage.getItem("hasAgreedToTOS") !== "1"}
            <p class="text-sm">
                You must read and agree to the
                <button
                    class="underline hover:decoration-dashed"
                    aria-label="Terms of Service"
                    on:click={() =>
                        slider.addSlideAndMove({
                            component: TOS,
                            showCloseButton: false,
                        })}
                >
                    TOS
                </button>
                before continuing.
            </p>
        {/if}
    </div>
</div>
