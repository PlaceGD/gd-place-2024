<script lang="ts">
    import { scale } from "svelte/transition";
    import { eventStatus, timeLeft } from "../stores";

    let hours = 0;
    let minutes = 0;
    let seconds = 0;

    $: {
        hours = Math.floor($timeLeft / 3600);
        minutes = Math.floor(($timeLeft % 3600) / 60);
        seconds = Math.floor($timeLeft % 60);
    }
</script>

{#if $timeLeft > 0 && hours < 10 && $eventStatus != "before"}
    <div
        class="z-50 p-4 m-4 text-white menu-panel sm:p-2 xs:m-2 tabular-nums"
        transition:scale={{
            duration: 200,
            delay: 0,
            start: 0.5,
        }}
        style={$timeLeft < 600
            ? "color: #ff0000"
            : $timeLeft < 3600
              ? "color: #ffff00"
              : ""}
    >
        {hours.toString().padStart(2, "0")}:{minutes
            .toString()
            .padStart(2, "0")}:{seconds.toString().padStart(2, "0")}
    </div>
{/if}
