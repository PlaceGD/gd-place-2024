<script lang="ts">
    import { onMount } from "svelte";
    import { showModeratorOptions } from "../stores";
    import Loading from "../components/Loading.svelte";
    import { get, ref, onChildAdded, onChildRemoved } from "firebase/database";
    import { db } from "../firebase/firebase";
    import Toast from "../utils/toast";
    import type { DatabaseSchema } from "shared-lib";
    import Button from "../components/Button.svelte";
    import FadedScroll from "../components/FadedScroll.svelte";
    import { reportedUserOperation } from "../firebase/cloud_functions";
    import Editor from "../Editor.svelte";

    let container: HTMLDivElement | null = null;

    $: {
        if (container != null) {
            container.focus();

            container.addEventListener("blur", () => {
                console.log("here");
            });
        }
    }

    onMount(() => {
        onChildRemoved(ref(db, "reportedUsers"), data => {
            const idx = reportedUsers.findIndex(u => u.uid == data.key);
            if (idx > -1) {
                reportedUsers?.splice(idx, 1);
                reportedUsers = reportedUsers;
            }
        });

        onChildAdded(ref(db, "reportedUsers"), data => {
            $showModeratorOptions.newReports = true;
            localStorage.setItem("newReports", "1");
            reportedUsers.push({
                uid: data.key!,
                ...data.val(),
            });
        });
    });

    $: {
        if ($showModeratorOptions.show) {
            localStorage.setItem("newReports", "0");
            $showModeratorOptions.newReports = false;
        }
    }

    let currentIdx = -1;

    type ReportedUser = {
        uid: string;
    } & DatabaseSchema["reportedUsers"][""];

    let reportedUsers: ReportedUser[] = [];

    const userOp = (op: "ignore" | "ban", user: ReportedUser, idx: number) => {
        currentIdx = idx;
        reportedUserOperation({
            operation: op,
            reportedUserUid: user.uid,
        })
            .then(() => {
                reportedUsers?.splice(idx, 1);
                reportedUsers = reportedUsers;
                currentIdx = -1;
            })
            .catch(e => {
                Toast.showErrorToast(`Failed to perform operation! (${e})`);
                currentIdx = -1;
            });
    };
</script>

{#if $showModeratorOptions.show}
    <div
        bind:this={container}
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-lg text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center pointer-events-all max-h-[75%]"
    >
        <h1
            class="text-2xl text-center sm:text-xl xs:text-lg font-pusab text-stroke"
        >
            Reported Users:
        </h1>
        {#if reportedUsers != null && reportedUsers.length == 0}
            <p class="text-lg sm:text-sm xs:text-sm">
                No users have been reported!
            </p>
        {:else if reportedUsers != null}
            <FadedScroll>
                <ul
                    class="flex flex-col w-full gap-2 px-6 overflow-y-auto xs:px-4 rounded-lg"
                >
                    {#each Object.values(reportedUsers) as user, idx}
                        <li
                            class="flex-col w-full gap-2 p-2 rounded-lg flex-center even:bg-white/5 odd:bg-black/15 relative"
                        >
                            <h2>
                                <span class="text-base xs:text-sm"
                                    >{user.username}</span
                                >
                                <span class="text-sm xs:text-xs"
                                    >(x{user.count})</span
                                >
                            </h2>
                            <div class="flex w-full h-10 gap-2 xs:h-9">
                                <Button
                                    type="decline"
                                    iconClass="w-8 h-8"
                                    on:click={() => {
                                        userOp("ignore", user, idx);
                                    }}
                                    disabled={currentIdx == idx}
                                >
                                    <span class="text-sm">Ignore</span>
                                </Button>
                                <Button
                                    type="accept"
                                    iconClass="w-8 h-8"
                                    on:click={() => {
                                        userOp("ban", user, idx);
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
{/if}
