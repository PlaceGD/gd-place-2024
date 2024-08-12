<script lang="ts">
    import { getUsernameColor } from "../firebase/donations";

    export let username: string;
    // export let class: string = "";
</script>

{#await getUsernameColor(username) then color}
    <span
        class="relative"
        style={`
        font-family: inherit;
        font-weight: inherit;
    `}
    >
        <span
            class="absolute text-stroke"
            style={`
                font-family: inherit;
                font-weight: inherit;
            `}
        >
            {username}
        </span>
        <span
            class="relative"
            style={`
                font-family: inherit;
                font-weight: inherit;
                background-image: ${color};
                ${color !== "white" ? "-webkit-text-fill-color: rgba(255, 255, 255, 0.1);" : ""}
                background-clip: text;
                -webkit-background-clip: text;
            `}
        >
            {username}
        </span>
    </span>
{/await}
