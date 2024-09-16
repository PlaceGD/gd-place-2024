<script lang="ts">
    import { Turnstile } from "svelte-turnstile";
    import Toast from "./utils/toast";
    import {
        setTurnstileResetFunction,
        setTurnstileToken,
        TokenStatus,
    } from "./utils/turnstile";
    import { reportUser } from "./firebase/cloud_functions";

    const SITE_KEY = __TURNSTILE_GENERAL_KEY;
    let turnstileReset: () => void;

    $: {
        if (turnstileReset !== undefined) {
            setTurnstileResetFunction(turnstileReset);
        }
    }

    console.log(SITE_KEY);
</script>

<Turnstile
    siteKey={SITE_KEY}
    on:callback={e => {
        setTurnstileToken(e.detail.token);
    }}
    on:error={e => {
        Toast.showErrorToast(
            `There was an error with the Turnstile. Code: ${e.detail.code}`
        );

        setTurnstileToken(TokenStatus.Error);
    }}
    on:expired={() => {
        setTurnstileToken(TokenStatus.NoToken);
    }}
    on:timeout={() => {
        setTurnstileToken(TokenStatus.NoToken);
    }}
    on:unsupported={() => {
        Toast.showErrorToast(
            "Your browser does not support Cloudflare Turnstiles. Please try with a different browser."
        );
    }}
    bind:reset={turnstileReset}
    retry="never"
/>
