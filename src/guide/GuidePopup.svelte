<script lang="ts">
    import { toast } from "@zerodevx/svelte-toast";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import { beginGuide } from "./guide";

    export let toastId: number | null = null;
</script>

<div class="flex flex-row justify-center gap-2 flex-center">
    <div class="font-bold text-white">Need help?</div>
    <WhiteButton
        class="h-8"
        on:click={() => {
            if (toastId !== null) {
                toast.pop(toastId);
            }
            beginGuide();
            localStorage.setItem("dontShowGuidePopup", true);
        }}>Click here for a guide!</WhiteButton
    >
</div>
