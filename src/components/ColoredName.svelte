<script lang="ts">
    import { getUsernameColor } from "../firebase/donations";

    export let username: string;
    export let colorOverride: string | null = null;

    const getOrOverride = async (
        username: string,
        override: string | null
    ): Promise<string> => {
        if (override != null) {
            return override;
        }
        return await getUsernameColor(username);
    };
</script>

<span
    class="grid grid-cols-1 grid-rows-1"
    style={`
        font-family: inherit;
        font-weight: inherit;
    `}
>
    <span
        class="block h-full col-start-1 row-start-1 text-stroke fix-stroke"
        style={`
                font-family: inherit;
                font-weight: inherit;
                color: black;
            `}
    >
        {username}
    </span>
    {#await getOrOverride(username, colorOverride)}
        <span
            class="block h-full col-start-1 row-start-1"
            style={`
                font-family: inherit;
                font-weight: inherit;
            `}
        >
            {username}
        </span>
    {:then color}
        <span
            class="block h-full col-start-1 row-start-1"
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
    {/await}
</span>

<style>
    /* firefox */
    .fix-stroke {
        animation: 0.1s animate-fade;
    }

    @keyframes animate-fade {
        0% {
            margin: 1px;
        }
        100% {
            margin: 0;
        }
    }
</style>
