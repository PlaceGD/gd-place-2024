<script lang="ts">
    import { Motion, useReducedMotion } from "svelte-motion";
    import { default as cx } from "classnames";

    export let layoutId: string;

    const shouldReducedMotion = useReducedMotion();
</script>

<Motion
    let:motion
    {layoutId}
    transition={{
        duration: $shouldReducedMotion ? 0 : 0.15,
    }}
    animate={{}}
>
    <span
        class={cx({
            "absolute w-full h-full sliding-selector": true,
            layoutId: true,
        })}
        use:motion
    >
    </span>
</Motion>
