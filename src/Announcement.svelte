<script lang="ts">
    import { db } from "./firebase/firebase";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import Toast from "./utils/toast";

    db.ref("announcement").on("value", s => {
        Toast.showAnnouncementToast(s.val());
    });

    //
</script>

<!-- for announcements only -->
<div
    class="announcement-wrapper"
    style="--toastContainerTop: 2rem; --toastContainerRight: 50%;"
>
    <SvelteToast target="announcement" options={{ intro: { y: -50 } }} />
</div>
