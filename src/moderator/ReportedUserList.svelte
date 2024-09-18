<script lang="ts">
    import { default as cx } from "classnames";
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
    import AcceptButton from "../components/Buttons/AcceptButton.svelte";
    import DeclineButton from "../components/Buttons/DeclineButton.svelte";
    import { menuHeight } from "../utils/transitions";

    export let state: wasm.State;
    export let editorFocused: boolean;

    $: {
        if ($openMenu == ExclusiveMenus.Moderator && editorFocused) {
            $openMenu = null;
        }
    }

    $: isOpen = $openMenu == ExclusiveMenus.Moderator;

    let reportedUsers: Record<
        string,
        {
            timestamp: number;
            username: string;
            avg_x: number; // used to get average report position
            avg_y: number;
            count: number;
        }
    > = {};
    // const sortReported = () => {
    //     reportedUsers.sort((a, b) => (a.timestamp - b.timestamp));

    //     reportedUsers = reportedUsers;
    // };

    onMount(() => {
        db.ref("reportedUsers").on("child_removed", data => {
            delete reportedUsers[data.key ?? ""];
            reportedUsers = reportedUsers;
        });

        db.ref("reportedUsers")
            .orderByChild("timestamp")
            // .startAfter(Date.now() - 15 * 60 * 1000)
            .on("child_added", data => {
                if (data.key != undefined) {
                    reportedUsers[data.key] = data.val();
                    reportedUsers = reportedUsers;
                    $newReports = true;
                }
            });

        db.ref("reportedUsers")
            .orderByChild("timestamp")
            // .startAfter(Date.now() - 15 * 60 * 1000)
            .on("child_changed", data => {
                if (data.key != undefined) {
                    reportedUsers[data.key] = data.val();
                    reportedUsers = reportedUsers;
                }
            });

        db.ref("bannedUsers").on("child_removed", data => {
            delete $bannedUsers[data.val().username];
            $bannedUsers = $bannedUsers;
        });

        db.ref("bannedUsers").on("child_added", data => {
            $bannedUsers[data.val().username] = true;
        });
    });

    $: {
        if ($openMenu == ExclusiveMenus.Moderator) {
            $newReports = false;
        }
    }

    let currentIdx = -1;

    const userOp = async (
        op: "ignore" | "ban",
        userId: string,
        reason: string,
        idx: number
    ) => {
        try {
            currentIdx = idx;

            await reportedUserOperation({
                reason,
                operation: op,
                reportedUserUid: userId,
            });
        } catch (e: any) {
            console.error("Failed to perform operation", e.details.message);
            Toast.showErrorToast(
                `Failed to perform operation. (${e.details.code})`
            );
            currentIdx = -1;
        }
    };
</script>

{#if isOpen}
    <fieldset
        class="z-50 flex flex-col py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel flex-center max-h-[75%] pointer-events-auto"
        disabled={!isOpen}
        transition:menuHeight={{ duration: 200 }}
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
                    class="flex flex-col w-full gap-2 px-4 overflow-y-auto rounded-lg xs:px-2"
                >
                    {#each Object.entries(reportedUsers) as [uid, user], idx}
                        <li
                            class="relative flex-col w-full gap-2 p-2 rounded-lg flex-center even:bg-white/5 odd:bg-black/15"
                        >
                            <div class="relative flex w-full flex-center">
                                <span class="text-base xs:text-sm">
                                    {user.username} (x{user.count})
                                </span>

                                <div
                                    class="absolute right-0 w-8 h-8 xs:w-7 xs:h-7"
                                >
                                    <button
                                        title="View average report location"
                                        class="w-full h-full"
                                        on:click={() => {
                                            moveCamera(
                                                state,
                                                user.avg_x,
                                                user.avg_y
                                            );
                                        }}
                                    >
                                        <Back class="rotate-180 stroke-[1.5]"
                                        ></Back>
                                    </button>
                                </div>
                            </div>
                            <div class="flex w-full h-10 gap-2 xs:h-9">
                                <DeclineButton
                                    class="w-full"
                                    on:click={() => {
                                        userOp("ignore", uid, "", idx);
                                    }}
                                    disabled={currentIdx == idx}
                                >
                                    <span class="text-sm">Ignore</span>
                                </DeclineButton>
                                <AcceptButton
                                    class="w-full"
                                    on:click={() => {
                                        const reason = prompt(
                                            "Reason for banning (inappropriate username / alt account / etc):"
                                        );

                                        if (reason != null) {
                                            userOp("ban", uid, reason, idx);
                                        }
                                    }}
                                    disabled={currentIdx == idx}
                                >
                                    <span class="text-sm">Ban</span>
                                </AcceptButton>
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
    </fieldset>
{/if}
