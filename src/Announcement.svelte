<script lang="ts">
    import { db } from "./firebase/firebase";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import Toast from "./utils/toast";
    import { lastClosedAnnouncement } from "./stores";

    db.ref("announcement").on("value", s => {
        const data = s.val();

        if (
            data == null ||
            data.text.length === 0 ||
            data.time < $lastClosedAnnouncement
        ) {
            return;
        }

        Toast.showAnnouncementToast(data.text, () => {
            $lastClosedAnnouncement = Date.now();
        });
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
