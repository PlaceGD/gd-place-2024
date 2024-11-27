import { db } from "../firebase/firebase";
import Toast from "../utils/toast";
import { lastClosedAnnouncement } from "../stores";
import { get } from "svelte/store";

let lastClosed = 0;
lastClosedAnnouncement.subscribe(v => (lastClosed = v));

// db.ref("announcement").on("value", s => {
//     const data = s.val();

//     if (data == null || data.text.length === 0 || data.time < lastClosed) {
//         return;
//     }

//     if (typeof window != "undefined") {
//         Toast.showAnnouncementToast(data.text, () => {
//             lastClosedAnnouncement.set(Date.now());
//         });
//     }
// });
