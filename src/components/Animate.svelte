<script lang="ts">
    import { onMount } from 'svelte'
    import { Motion, useAnimation, useReducedMotion, } from 'svelte-motion';
    import type { Easing, Target, TargetAndTransition } from 'svelte-motion/types/types'

    export let easing: Easing = "linear";
    export let duration: number = 0;

    export let from: Target = {};
    
    type ConditionalTarget = { [key: string]: TargetAndTransition };
    
    export let definition: ConditionalTarget | TargetAndTransition = {};
    export let to: ConditionalTarget | TargetAndTransition = {};
    
    export let conditions: { [key: string]: boolean } = {}

    const animation = useAnimation();

    const reducedMotion = useReducedMotion();

    // TODO: reduced motion setting
    const reducedMotionTransition = $reducedMotion ? {
        ease: "linear",
        duration: 0,
    } : {
        easing,
        duration
    };

    $: {
        Object.keys(definition).forEach((k) => {
            if (conditions[k] !== undefined) {
                if (conditions[k]) {
                    animation.start(
                        (definition as ConditionalTarget)[k],
                        reducedMotionTransition   
                    );
                } else {
                    animation.start(
                        $$props.initial || to,
                        reducedMotionTransition   
                    );
                }
            } else {
                animation.start(
                    definition as TargetAndTransition,  
                    reducedMotionTransition
                );
            }
        })
    }


</script>
<Motion  {...$$restProps} animate={animation} initial={from} let:motion let:props>
    <slot {...props} motion={motion} />
</Motion>