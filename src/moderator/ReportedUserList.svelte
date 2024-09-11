<script lang="ts">
    import { onMount } from "svelte";
    import {
        bannedUsers,
        ExclusiveMenus,
        newReports,
        openMenu,
    } from "../stores";
    import Loading from "../components/Loading.svelte";

    import { db } from "../firebase/firebase";
    import Toast from "../utils/toast";
    import type { DatabaseSchema } from "shared-lib/database";
    import Button from "../components/Button.svelte";
    import FadedScroll from "../components/FadedScroll.svelte";
    import { reportedUserOperation } from "../firebase/cloud_functions";
    import Back from "../icons/Back.svelte";
    import { moveCamera } from "../level_view/view_controls";
    import * as wasm from "wasm-lib";

    export let state: wasm.State;
    export let editorFocused: boolean;

    $: {
        if ($openMenu == ExclusiveMenus.Moderator && editorFocused) {
            $openMenu = null;
        }
    }

    let reportedUsers: {
        uid: string;
        timestamp: number;
        username: string;
        sumX: number; // used to get average report position
        sumY: number;
        count: number;
    }[] = [];

    onMount(() => {
        db.ref("reportedUsers").on("child_removed", data => {
            reportedUsers = reportedUsers.filter(
                user => data.val().uid != user.uid
            );
        });

        db.ref("reportedUsers")
            .orderByChild("timestamp")
            .startAfter(Date.now() - 15 * 60 * 1000)
            .on("child_added", data => {
                const reportData = data.val();

                const user = reportedUsers.find(c => c.uid == reportData.uid);

                if (user != undefined) {
                    user.count += 1;
                    user.sumX += reportData.x;
                    user.sumY += reportData.y;
                } else {
                    $newReports = true;

                    reportedUsers.push({
                        count: 1,
                        username: reportData.username,
                        sumX: reportData.x,
                        sumY: reportData.y,
                        timestamp: reportData.timestamp,
                        uid: reportData.uid,
                    });

                    reportedUsers.sort((a, b) =>
                        a.timestamp > b.timestamp ? 1 : -1
                    );
                }
            });

        // onChildAdded(ref(db, "bannedUsers"), data => {
        //     bannedUsers.update(users => {
        //         if (data.key) {
        //             users = [...users, data.key]; // key will be the username
        //         }
        //         return users;
        //     });
        // });
    });

    $: {
        if ($openMenu == ExclusiveMenus.Moderator) {
            $newReports = false;
        }
    }

    let currentIdx = -1;

    const userOp = (op: "ignore" | "ban", userId: string, idx: number) => {
        // currentIdx = idx;
        // reportedUserOperation({
        //     operation: op,
        //     reportedUserUid: userId,
        // })
        //     .then(() => {
        //         delete reportedUsers[userId];
        //         currentIdx = -1;
        //     })
        //     .catch(e => {
        //         Toast.showErrorToast(`Failed to perform operation! (${e})`);
        //         currentIdx = -1;
        //     });
    };
</script>


<div
    class="z-50 flex flex-col py-2 gap-2 mr-6 text-lg text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
    style={`
            height: ${$openMenu != ExclusiveMenus.Moderator ? "0" : "50vh"};
            opacity: ${$openMenu != ExclusiveMenus.Moderator ? "0" : "1"};
            transition: height 0.2s ease, opacity 0.2s ease-out;
        `}
>
    <h1
        class="text-2xl text-center sm:text-xl xs:text-lg font-pusab text-stroke"
    >
        Reported Users:
    </h1>
    {#if reportedUsers != null && Object.keys(reportedUsers).length == 0}
        <p class="text-lg sm:text-sm xs:text-sm">
            No users have been reported!
        </p>
    {:else if reportedUsers != null}
        <FadedScroll>
            <ul
                class="flex flex-col w-full gap-2 px-6 overflow-y-auto rounded-lg xs:px-4"
            >
                {#each Object.values(reportedUsers) as user, idx}
                    <li
                        class="relative flex-col w-full gap-2 p-2 rounded-lg flex-center even:bg-white/5 odd:bg-black/15"
                    >
                        <div
                            class="grid items-center justify-center w-full grid-cols-3 grid-rows-1"
                        >
                            <p class="col-start-2">
                                <span class="text-base xs:text-sm"
                                    >{user.username}</span
                                >
                                <span class="text-sm xs:text-xs"
                                    >(x{user.count})</span
                                >
                            </p>
                            <div
                                class="w-8 h-8 col-start-3 justify-self-end"
                            >
                                <Button
                                    title="View average report location"
                                    class="w-full h-full"
                                    type="plain"
                                    on:click={() => {
                                        const data = reportedUsers[idx];

                                        moveCamera(
                                            state,
                                            data.sumX / data.count,
                                            data.sumY / data.count
                                        );
                                    }}
                                >
                                    <Back class="rotate-180 stroke-[1.5]"
                                    ></Back>
                                </Button>
                            </div>
                        </div>
                        <div class="flex w-full h-10 gap-2 xs:h-9">
                            <Button
                                type="decline"
                                class="w-full h-full"
                                on:click={() => {
                                    userOp("ignore", user.uid, idx);
                                }}
                                disabled={currentIdx == idx}
                            >
                                <span class="text-sm">Ignore</span>
                            </Button>
                            <Button
                                type="accept"
                                class="w-full h-full"
                                on:click={() => {
                                    userOp("ban", user.uid, idx);
                                }}
                                disabled={currentIdx == idx}
                            >
                                <span class="text-sm">Ban</span>
                            </Button>
                        </div>
                        {#if currentIdx == idx}
                            <Loading class="rounded-lg" />
                        {/if}
                    </li>
                {/each}
            </ul>
        </FadedScroll>
        <!-- {:else}
        <div class="relative w-12 h-12">
            <Loading darken={false} />
        </div> -->
    {/if}
</div>

