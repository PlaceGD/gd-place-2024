import type { ComponentType } from "svelte";
import HighlightElementComp from "./HighlightElement.svelte";

type Props<C> =
    C extends ComponentType<infer X> ? Partial<X["$$prop_def"]> : never;

export abstract class GuideAction {
    component?: ComponentType;
    props?(): Props<ComponentType>;

    constructor(public description: string) {}
}

export class HighlightElement extends GuideAction {
    override component = HighlightElementComp;

    override props(): Props<typeof HighlightElementComp> {
        return {
            target: this.target,
        };
    }

    constructor(
        private target: string,
        description: string
    ) {
        super(description);
    }
}
