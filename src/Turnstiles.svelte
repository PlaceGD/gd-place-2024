<script lang="ts">
    import { Turnstile } from "svelte-turnstile";
    import Toast from "./utils/toast";
    import {
        setTurnstileResetFunction,
        setTurnstileToken,
        TokenStatus,
    } from "./utils/turnstile";
    import { reportUser } from "./firebase/cloud_functions";

    const SITE_KEY = __TURNSTILE_REPORT_SITE_KEY;
    let turnstileReset: () => void;

    $: {
        if (turnstileReset !== undefined) {
            setTurnstileResetFunction(turnstileReset);
        }
    }
</script>

<Turnstile
    siteKey={SITE_KEY}
    on:turnstile-callback={e => {
        console.log("HERE", e);

        reportUser({
            username: "Fow",
            turnstileResp: e.detail.token,
            x: 0,
            y: 0,
        });
        // setTurnstileToken(e.detail.token);
        // turnstileToken = e.detail.token;
    }}
    on:turnstile-error={() => {
        Toast.showErrorToast(`There was an error with the Turnstile`);
        setTurnstileToken(TokenStatus.Error);
    }}
    on:turnstile-expired={() => {
        setTurnstileToken(TokenStatus.NoToken);
    }}
    bind:reset={turnstileReset}
/>
