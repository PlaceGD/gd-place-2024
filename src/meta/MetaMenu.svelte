<script lang="ts">
    import { onMount } from "svelte";
    import FadedScroll from "../components/FadedScroll.svelte";
    import ToggleSwitch from "../components/ToggleSwitch.svelte";
    import { ExclusiveMenus, loginData, openMenu } from "../stores";
    import Button from "../components/Button.svelte";
    import { menuHeight } from "../utils/transitions";
    import DarkInput from "../components/DarkInput.svelte";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { setMeta } from "../firebase/cloud_functions";
    import type { MetaReq } from "shared-lib/cloud_functions";
    import Toast from "../utils/toast";
    import { db } from "../firebase/firebase";

    $: isOpen = $openMenu === ExclusiveMenus.Meta;

    let inputValues = {
        placeTimer: 0,
        deleteTimer: 0,
        eventStart: 0,
        eventEnd: 0,
        postponeStart: 0,
        postponeEnd: 0,
        modChangeUsername: "",
        unbanUsername: "",
        usernameOrID: "",
        donatorID: "",
        announcementText: "",
    };

    const meta = (data: MetaReq["op"]) => {
        setMeta({ op: data }).catch(e => {
            console.error("meta failed", e.message);
            Toast.showErrorToast(
                `Failed meta operation ${data.type} (${e.details.code})`
            );
        });
    };
</script>

{#if isOpen}
    <fieldset
        class="z-50 flex flex-col gap-4 p-4 mr-6 overflow-x-hidden overflow-y-scroll text-white rounded-lg pointer-events-auto sm:mr-4 w-96 xs:w-80 h-[50%] menu-panel thin-scrollbar"
        inert={!isOpen}
        transition:menuHeight={{ duration: 200 }}
    >
        <div class="flex flex-col w-full gap-2 flex-center">
            <div class="flex w-full gap-2 flex-center">
                <h1 class="font-pusab text-stroke">Announce</h1>
                <DarkInput
                    maxLength={5000}
                    class="w-full"
                    bind:value={inputValues.announcementText}
                ></DarkInput>
            </div>
            <div class="flex flex-row w-full">
                <WhiteButton
                    class="w-full"
                    on:click={() => {
                        meta({
                            type: "announcement",
                            text: inputValues.announcementText,
                        });
                    }}>Send</WhiteButton
                >
                <WhiteButton
                    class="w-full"
                    on:click={() => {
                        meta({
                            type: "clear_announcement",
                        });
                    }}>Clear</WhiteButton
                >
            </div>
        </div>
        <div class="w-full min-h-[1px] bg-white/50" />
        <div class="flex flex-col gap-2 flex-center">
            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Place</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.placeTimer}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "place_timer",
                            to: inputValues.placeTimer,
                        })}
                >
                    Set
                </WhiteButton>
            </div>
            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Delete</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.deleteTimer}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "delete_timer",
                            to: inputValues.deleteTimer,
                        })}
                >
                    Set
                </WhiteButton>
            </div>
            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Event Start</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.eventStart}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "event_start",
                            to: inputValues.eventStart,
                        })}
                >
                    Set
                </WhiteButton>
            </div>
            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Event End</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.eventEnd}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "event_end",
                            to: inputValues.eventEnd,
                        })}
                >
                    Set
                </WhiteButton>
            </div>

            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Postpone start</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.postponeStart}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "postpone_start",
                            secs: inputValues.postponeStart,
                        })}
                >
                    Set
                </WhiteButton>
            </div>
            <div class="flex w-full gap-2 flex-center">
                <h1 class="w-32 font-pusab text-stroke">Postpone end</h1>
                <DarkInput
                    maxLength={1000}
                    bind:value={inputValues.postponeEnd}
                    hardValidInput={/^-?\d*$/}
                ></DarkInput>
                <WhiteButton
                    on:click={() =>
                        meta({
                            type: "postpone_end",
                            secs: inputValues.postponeEnd,
                        })}
                >
                    Set
                </WhiteButton>
            </div>

            <WhiteButton
                on:click={() =>
                    meta({
                        type: "event_end",
                        to: Date.now() + 5000,
                    })}
            >
                Run End Sequence
            </WhiteButton>

            <div class="w-full min-h-[1px] bg-white/50" />

            <div class="flex flex-col w-full gap-2 flex-center">
                <div class="flex w-full gap-2 flex-center">
                    <h1 class="font-pusab text-stroke">username</h1>
                    <DarkInput
                        maxLength={16}
                        class="w-full"
                        bind:value={inputValues.modChangeUsername}
                    ></DarkInput>
                </div>
                <div class="flex w-full gap-2">
                    <WhiteButton
                        class="w-full"
                        on:click={() => {
                            meta({
                                type: "change_mod_status",
                                user: inputValues.modChangeUsername,
                                to: "mod",
                            });
                        }}>Mod</WhiteButton
                    >
                    <WhiteButton
                        class="w-full"
                        on:click={() => {
                            meta({
                                type: "change_mod_status",
                                user: inputValues.modChangeUsername,
                                to: "unmod",
                            });
                        }}>Unmod</WhiteButton
                    >
                </div>
            </div>
            <div class="w-full min-h-[1px] bg-white/50" />
            <div class="flex flex-col w-full gap-2 flex-center">
                <div class="flex w-full gap-2 flex-center">
                    <h1 class="font-pusab text-stroke">username</h1>
                    <DarkInput
                        maxLength={16}
                        class="w-full"
                        bind:value={inputValues.unbanUsername}
                    ></DarkInput>
                </div>

                <WhiteButton
                    class="w-full"
                    on:click={() => {
                        meta({
                            type: "unban",
                            user: inputValues.unbanUsername,
                        });
                    }}>Unban</WhiteButton
                >
            </div>
            <div class="w-full min-h-[1px] bg-white/50" />

            <div class="flex flex-col w-full gap-2 flex-center">
                <div class="flex w-full gap-2 flex-center">
                    <h1 class="font-pusab text-stroke">UID</h1>
                    <DarkInput
                        maxLength={100}
                        class="w-full"
                        bind:value={inputValues.donatorID}
                    ></DarkInput>
                </div>
                <div class="flex w-full gap-2">
                    <WhiteButton
                        class="w-full"
                        on:click={async () => {
                            meta({
                                type: "log_donation",
                                uid: inputValues.donatorID,
                            });
                        }}>Make Donator</WhiteButton
                    >
                </div>
            </div>
            <div class="w-full min-h-[1px] bg-white/50" />

            <div class="flex flex-col w-full gap-2 flex-center">
                <div class="flex w-full gap-2 flex-center">
                    <h1 class="font-pusab text-stroke">Username or ID</h1>
                    <DarkInput
                        maxLength={100}
                        class="w-full"
                        bind:value={inputValues.usernameOrID}
                    ></DarkInput>
                </div>

                <div class="flex w-full gap-2">
                    <WhiteButton
                        class="w-full"
                        on:click={async () => {
                            const id =
                                (
                                    await db
                                        .ref(
                                            `userName/${inputValues.usernameOrID.toLowerCase()}/uid`
                                        )
                                        .get()
                                ).val() ?? "<unknown>";
                            inputValues.usernameOrID = id;
                            inputValues.donatorID = id;
                        }}
                    >
                        To ID
                    </WhiteButton>
                    <WhiteButton
                        class="w-full"
                        on:click={async () => {
                            const username =
                                (
                                    await db
                                        .ref(
                                            `userDetails/${inputValues.usernameOrID}/username`
                                        )
                                        .get()
                                ).val() ?? "<unknown>";
                            inputValues.usernameOrID = username;
                            inputValues.unbanUsername = username;
                            inputValues.modChangeUsername = username;
                        }}>To username</WhiteButton
                    >
                </div>
            </div>
        </div>
    </fieldset>
{/if}
