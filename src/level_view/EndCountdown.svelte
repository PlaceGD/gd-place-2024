<script lang="ts">
    import { scale } from "svelte/transition";
    import { timeLeft } from "../stores";

    let hours = 0;
    let minutes = 0;
    let seconds = 0;

    $: {
        const secs = $timeLeft / 1000;
        hours = Math.floor(secs / 3600);
        minutes = Math.floor((secs % 3600) / 60);
        seconds = Math.floor(secs % 60);
    }
</script>

{#if $timeLeft && $timeLeft > 0 && hours < 10}
    <div
        class="menu-panel z-50 text-white p-4 m-4 sm:p-2 xs:m-2 tabular-nums"
        transition:scale={{
            duration: 200,
            delay: 0,
            start: 0.5,
        }}
        style={$timeLeft < 600 ? "color: #ff0000" : ""}
    >
        {hours.toString().padStart(2, "0")}:{minutes
            .toString()
            .padStart(2, "0")}:{seconds.toString().padStart(2, "0")}
    </div>
{/if}
