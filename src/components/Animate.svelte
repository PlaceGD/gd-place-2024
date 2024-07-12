<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import {
        Motion,
        useAnimation,
        useReducedMotion,
        type VariantLabels,
    } from "svelte-motion";
    import type {
        Easing,
        MakeCustomValueType,
        Target,
        TargetAndTransition,
    } from "svelte-motion/types/types";

    const dispatcher = createEventDispatcher();

    // used to convert JS css properties (camel case) to actual css properites (snake with `-`)
    const camelToSnake = (prop: string): string => {
        return prop.replace(/[A-Z]/g, a => {
            return "-" + a.toLowerCase();
        });
    };

    let child: HTMLElement;

    const fakeMotion = (motion_og: (node: HTMLElement) => void) => {
        return (node: HTMLElement) => {
            child = node;
            motion_og(node);
        };
    };

    export let easing: Easing = "linear";
    export let duration: number = 0;

    export let initial: Target | VariantLabels | string[] = {};
    let motionInitial:
        | { [key: string]: string }
        | string
        | MakeCustomValueType<unknown> = {};

    const setIntial = () => {
        if (Array.isArray(initial)) {
            let mappedPrts: { [key: string]: string } = {};

            initial.map((prt): void => {
                let styles = window.getComputedStyle(child);

                mappedPrts[prt] = styles.getPropertyValue(camelToSnake(prt));
            });

            motionInitial = mappedPrts;
        } else {
            motionInitial = initial;
        }
    };

    export const resetIntialStyles = () => {
        child.removeAttribute("style");
        setIntial();
        animation.set(motionInitial);
    };

    onMount(() => {
        setIntial();
    });

    type ConditionalTarget = { [key: string]: TargetAndTransition };

    export let from: ConditionalTarget | TargetAndTransition | undefined =
        undefined;
    export let to: ConditionalTarget | TargetAndTransition = {};
    export let conditions: { [key: string]: boolean } = {};

    const animation = useAnimation();

    const reducedMotion = useReducedMotion();

    // TODO: reduced motion setting
    const reducedMotionTransition = $reducedMotion
        ? {
              ease: "linear",
              duration: 0,
          }
        : {
              easing,
              duration,
          };

    $: {
        Object.keys(to).forEach(k => {
            if (conditions[k] != undefined) {
                if (conditions[k]) {
                    animation.start(
                        (to as ConditionalTarget)[k],
                        reducedMotionTransition
                    );
                } else {
                    animation.start(
                        from || motionInitial,
                        reducedMotionTransition
                    );
                }
            } else {
                animation.start(
                    to as TargetAndTransition,
                    reducedMotionTransition
                );
            }
        });
    }
</script>

<Motion
    initial={motionInitial}
    {...$$restProps}
    animate={animation}
    let:motion
    let:props
    onAnimationStart={() => dispatcher("start", {})}
    onAnimationComplete={() => dispatcher("end", {})}
>
    <slot motion={fakeMotion(motion)} />
</Motion>
