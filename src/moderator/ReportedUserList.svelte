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
    import { fly } from "svelte/transition";
    import ScreenModal from "../components/ScreenModal.svelte";
    import type { ReportedUserOperationReq } from "shared-lib/cloud_functions";
    import MapPinExclamation from "../icons/MapPinExclamation.svelte";
    import DarkInput from "../components/DarkInput.svelte";
    import OnceButton from "../components/Buttons/OnceButton.svelte";
    import { banUser as singleBanUser } from "../firebase/cloud_functions";

    export let state: wasm.State;

    $: isOpen = $openMenu == ExclusiveMenus.Moderator;

    let viewingReporters: string | null = null;

    // by user
    let reportedUsers: Record<
        string,
        {
            username: string;
            list: Record<
                string,
                {
                    samaritanUsername: string;
                    samaritanID: string;

                    x: number;
                    y: number;

                    timestamp: number;
                }
            >;
        }
    > = {};
    // const sortReported = () => {
    //     reportedUsers.sort((a, b) => (a.timestamp - b.timestamp));

    //     reportedUsers = reportedUsers;
    // };

    onMount(() => {
        db.ref("reports").on("child_removed", data => {
            if (data.key == null) {
                return;
            }

            let val = data.val();
            delete reportedUsers[val.badUserID].list[data.key];
            if (Object.keys(reportedUsers[val.badUserID].list).length == 0) {
                delete reportedUsers[val.badUserID];
            }
            reportedUsers = reportedUsers;
        });

        db.ref("reports")
            .orderByChild("timestamp")
            // .startAfter(Date.now() - 15 * 60 * 1000)
            .on("child_added", data => {
                if (data.key != undefined) {
                    let val = data.val();

                    let uid = val.badUserID;
                    if (reportedUsers[uid] == undefined) {
                        reportedUsers[uid] = {
                            username: val.badUsername,
                            list: {},
                        };
                    }

                    reportedUsers[uid].list[data.key] = {
                        samaritanUsername: val.samaritanUsername,
                        samaritanID: val.samaritanID,
                        x: val.x,
                        y: val.y,
                        timestamp: val.timestamp,
                    };
                    reportedUsers = reportedUsers;
                    $newReports = true;
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

    const userOp = async (op: ReportedUserOperationReq, idx: number) => {
        try {
            currentIdx = idx;

            await reportedUserOperation(op);

            currentIdx = -1;
        } catch (e: any) {
            console.error("Failed to perform operation", e.details.message);
            Toast.showErrorToast(
                `Failed to perform operation. (${e.details.code})`
            );
            currentIdx = -1;
        }
    };

    let reportListPosIdx = 0;

    const banUser = (uid: string, idx: number, keys: string[]) => {
        const reason = prompt(
            "Reason for banning (inappropriate username / alt account / etc):"
        );

        if (reason != null) {
            userOp(
                {
                    operation: "ban",
                    userUid: uid,
                    reason,
                    reportKeys: keys,
                },
                idx
            ).then(() => {
                Toast.showInfoToast("Successfully banned user!");
            });
        }
    };

    let banUsername = "";
    let resetUsernameBanButton: () => void;
</script>

<ScreenModal
    hasCloseButton={true}
    isOpen={viewingReporters != null}
    on:close={() => {
        viewingReporters = null;
    }}
    size="max-w-[500px] h-[600px] max-h-[650px]"
>
    {#if reportedUsers != null && viewingReporters != null && reportedUsers[viewingReporters] != null}
        <div
            class="flex flex-col w-full h-full gap-4 p-4 overflow-hidden xs:p-2 xs:gap-2"
        >
            <h1
                class="text-2xl text-center sm:text-xl xs:text-lg font-pusab text-stroke"
            >
                Reports for {reportedUsers[viewingReporters].username}
            </h1>
            <FadedScroll update={viewingReporters}>
                <ul
                    class="flex flex-col w-full gap-2 px-4 overflow-y-auto rounded-lg xs:px-2"
                >
                    {#each Object.entries(reportedUsers[viewingReporters].list) as [key, data], idx}
                        <li
                            class="relative flex flex-col gap-2 p-2 rounded-md flex-center even:bg-white/5 odd:bg-black/15"
                        >
                            {data.samaritanUsername}
                            <div class="flex w-full h-10 gap-2 xs:h-9">
                                <DeclineButton
                                    class="w-full"
                                    on:click={() => {
                                        userOp(
                                            {
                                                operation: "ignore",
                                                reportKeys: [key],
                                            },
                                            idx
                                        );
                                    }}
                                    disabled={currentIdx == idx}
                                >
                                    <span class="text-sm">Ignore</span>
                                </DeclineButton>
                                <AcceptButton
                                    class="w-full"
                                    on:click={() => {
                                        banUser(data.samaritanID, idx, [key]);
                                    }}
                                    disabled={currentIdx === idx}
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
        </div>
    {/if}
    <!-- {#each  as }
    
{/each} -->
</ScreenModal>

{#if isOpen}
    <fieldset
        class="z-50 py-2 gap-2 mr-6 text-white rounded-lg sm:mr-4 w-96 xs:w-80 menu-panel h-[50%] pointer-events-auto"
        inert={!isOpen}
        transition:menuHeight={{ duration: 200 }}
    >
        <div class="flex flex-col w-full h-full overflow-hidden flex-center">
            <h1
                class="text-2xl text-center sm:text-xl xs:text-lg font-pusab text-stroke"
            >
                Reported Users:
            </h1>
            <div class="w-full h-full overflow-auto flex-center">
                {#if reportedUsers != null && Object.keys(reportedUsers).length == 0}
                    <p class="text-lg sm:text-sm xs:text-sm">
                        No users have been reported!
                    </p>
                {:else if reportedUsers != null}
                    <FadedScroll>
                        <ul
                            class="flex flex-col w-full gap-2 px-4 overflow-y-auto rounded-lg xs:px-2"
                        >
                            {#each Object.entries(reportedUsers) as [uid, { username, list }], idx}
                                {@const keys = Object.keys(list)}
                                <li
                                    class="relative flex-col w-full gap-2 p-2 rounded-lg flex-center even:bg-white/5 odd:bg-black/15"
                                >
                                    <div
                                        class="relative grid grid-cols-[min-content_1fr_min-content] gap-4 xs:gap-2 w-full justify-items-center items-center"
                                    >
                                        <button
                                            class="text-sm text-center text-white xs:text-sm"
                                            on:click={() => {
                                                viewingReporters = uid;
                                            }}
                                        >
                                            View Reporters
                                        </button>

                                        <span class="text-base xs:text-sm">
                                            {username} (x{keys.length})
                                        </span>

                                        <div class="w-8 h-8 xs:w-7 xs:h-7">
                                            <button
                                                title="Cycle through report locations"
                                                class="w-full h-full"
                                                on:click={() => {
                                                    reportListPosIdx =
                                                        (reportListPosIdx + 1) %
                                                        keys.length;
                                                    let v =
                                                        list[
                                                            keys[
                                                                reportListPosIdx
                                                            ]
                                                        ];
                                                    moveCamera(state, v.x, v.y);
                                                    // moveCamera(
                                                    //     state,
                                                    //     list.avg_x,
                                                    //     list.avg_y
                                                    // );
                                                }}
                                            >
                                                <MapPinExclamation
                                                    class="stroke-[1] w-full h-full"
                                                />
                                            </button>
                                        </div>
                                    </div>
                                    <div class="flex w-full h-10 gap-2 xs:h-9">
                                        <DeclineButton
                                            class="w-full"
                                            on:click={() => {
                                                userOp(
                                                    {
                                                        operation: "ignore",
                                                        reportKeys: keys,
                                                    },
                                                    idx
                                                );
                                            }}
                                            disabled={currentIdx == idx}
                                        >
                                            <span class="text-sm">Ignore</span>
                                        </DeclineButton>
                                        <AcceptButton
                                            class="w-full"
                                            on:click={() => {
                                                banUser(uid, idx, keys);
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
            </div>
            <div
                class="flex flex-col self-end gap-2 px-4 pt-4 text-center flex-center xs:p-2"
            >
                <div class="flex gap-4 xs:gap-2">
                    <p class="text-base xs:text-sm">Ban By Username</p>
                    <DarkInput maxLength={16} bind:value={banUsername} />
                </div>
                <OnceButton
                    let:click
                    let:disabled
                    bind:reset={resetUsernameBanButton}
                >
                    <DeclineButton
                        class="w-full"
                        on:click={() => {
                            click();
                            const reason = prompt(
                                "Reason for banning (inappropriate username / alt account / etc):"
                            );
                            if (reason != null && banUsername != "") {
                                singleBanUser({
                                    username: banUsername,
                                    reason,
                                })
                                    .then(() => {
                                        Toast.showInfoToast(
                                            "Successfully banned user!"
                                        );
                                        resetUsernameBanButton();
                                    })
                                    .catch(e => {
                                        console.log("Failed to ban user", e);
                                        Toast.showErrorToast(
                                            "Failed to ban user!"
                                        );
                                        resetUsernameBanButton();
                                    });
                            }
                        }}
                        {disabled}
                    >
                        Ban
                    </DeclineButton>
                </OnceButton>
            </div>
        </div>
    </fieldset>
{/if}
