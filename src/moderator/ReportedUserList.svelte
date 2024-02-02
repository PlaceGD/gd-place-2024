<script lang="ts">
    import { onMount } from "svelte";
    import { showModeratorOptions } from "../stores";
    import Loading from "../components/Loading.svelte";
    import { get, ref } from "firebase/database";
    import { db } from "../firebase/firebase";
    import Toast from "../utils/toast";
    import type { DatabaseSchema } from "shared-lib";
    import Button from "../components/Button.svelte";
    import FadedScroll from "../components/FadedScroll.svelte";
    import { reportedUserOperation } from "../firebase/cloud_functions";

    let container: HTMLDivElement | null = null;

    $: {
        if (container != null) {
            container.focus();

            container.addEventListener("blur", () => {
                console.log("here");
            });
        }
    }

    $: {
        if ($showModeratorOptions) {
            get(ref(db, "reportedUsers"))
                .then(d => {
                    if (d.hasChildren()) {
                        reportedUsers = d.val();
                    } else {
                        reportedUsers = [];
                    }
                })
                .catch(e => {
                    Toast.showErrorToast(
                        `Failed to get reported users! (${e})`
                    );
                });
        }
    }

    let reportedUsers: null | DatabaseSchema["reportedUsers"][""][] = null;
</script>

{#if $showModeratorOptions}
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
                    class="flex flex-col w-full gap-2 px-6 overflow-y-auto xs:px-4"
                >
                    {#each Object.entries(reportedUsers) as [uid, user]}
                        <li
                            class="flex-col w-full gap-2 p-2 rounded-lg flex-center even:bg-white/5 odd:bg-black/15"
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
                                        reportedUserOperation({
                                            operation: "ignore",
                                            reportedUserUid: uid,
                                        });
                                    }}
                                >
                                    <span class="text-sm">Ignore</span>
                                </Button>
                                <Button
                                    type="accept"
                                    iconClass="w-8 h-8"
                                    on:click={() => {
                                        reportedUserOperation({
                                            operation: "ban",
                                            reportedUserUid: uid,
                                        });
                                    }}
                                >
                                    <span class="text-sm">Ban</span>
                                </Button>
                            </div>
                        </li>
                    {/each}
                </ul>
            </FadedScroll>
        {:else}
            <div class="relative w-12 h-12">
                <Loading darken={false} />
            </div>
        {/if}
    </div>
{/if}
