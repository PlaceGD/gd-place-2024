<script lang="ts">
    import { getUsernameColor } from "../firebase/donations";

    export let username: string;
</script>

{#await getUsernameColor(username) then color}
    <span
        style={`
            font-family: inherit;
            background-image: ${color};
            -webkit-text-fill-color: rgba(255, 255, 255, 0.1);
            background-clip: text;
            -webkit-background-clip: text;
        `}
    >
        {username}
    </span>
{/await}
