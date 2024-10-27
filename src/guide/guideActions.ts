import type { ComponentType, EventDispatcher } from "svelte";
import * as wasm from "wasm-lib";
import HighlightElementComp from "./HighlightElement.svelte";
import { moveCamera } from "../level_view/view_controls";
import {
    derived,
    get,
    writable,
    type Unsubscriber,
    type Writable,
} from "svelte/store";

type Props<C> =
    C extends ComponentType<infer X> ? Partial<X["$$prop_def"]> : never;

type ActionPropsBase = {
    state: wasm.State;
    tooltipSize: { width: number; height: number };
};
type ActionBeginEndProps = ActionPropsBase & {
    nextStep: () => Promise<void>;
};

export abstract class GuideAction {
    public requiresInteraction: Writable<boolean> = writable(false);
    public canInteract: Writable<boolean> = writable(false);

    getComponent?(): ComponentType | undefined;
    getProps?(): Props<ComponentType> | undefined;

    constructor(public description: string) {}

    // used when a component is not provided
    getTooltipPos?(props: ActionPropsBase): [number, number] | undefined;

    async onBeginAction?(props: ActionBeginEndProps): Promise<void>;
    async onEndAction?(props: ActionBeginEndProps): Promise<void>;
}

export class HighlightElement extends GuideAction {
    override getComponent() {
        return HighlightElementComp;
    }

    override getProps(): Props<typeof HighlightElementComp> {
        return {
            target: this.target,
            allowClicking: this.allowClickingInRegion,
        };
    }

    constructor(
        private target: string,
        description: string,
        public allowClickingInRegion: boolean = false
    ) {
        super(description);
    }
}

export enum EditorGuidePosition {
    Top,
    Bottom,
}

export class EditorGuide extends GuideAction {
    constructor(
        description: string,
        private cameraPos: { x: number; y: number; zoom: number } | null,
        private tooltiopPos: EditorGuidePosition,
        canInteract: boolean = false
    ) {
        super(description);

        this.canInteract.set(canInteract);
    }

    override getTooltipPos(props: ActionPropsBase): [number, number] {
        if (this.tooltiopPos === EditorGuidePosition.Top) {
            return [window.innerWidth / 2 - props.tooltipSize.width / 2, 60];
        } else {
            return [
                window.innerWidth / 2 - props.tooltipSize.width / 2,
                window.innerHeight - props.tooltipSize.height,
            ];
        }
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        if (this.cameraPos) {
            moveCamera(props.state, this.cameraPos.x, this.cameraPos.y);
            props.state.set_zoom(this.cameraPos.zoom);
        }
    }
}

export class WaitThen<T extends GuideAction> extends GuideAction {
    constructor(
        private action: T,
        private delay: number
    ) {
        super(action.description);
        this.requiresInteraction = action.requiresInteraction;
        this.canInteract = action.canInteract;

        this.action = action;
    }

    override getComponent() {
        return this.action?.getComponent?.();
    }

    override getProps() {
        return this.action?.getProps?.();
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        await new Promise(res => setTimeout(res, this.delay));
        await this.action?.onBeginAction?.(props);
    }

    override async onEndAction(props: ActionBeginEndProps): Promise<void> {
        await this.action?.onEndAction?.(props);
    }

    override getTooltipPos?(
        props: ActionPropsBase
    ): [number, number] | undefined {
        return this.action?.getTooltipPos?.(props);
    }
}

export class Setup<T extends GuideAction> extends GuideAction {
    constructor(
        private setup: {
            begin?: (props: ActionBeginEndProps) => Promise<void>;
            end?: (props: ActionBeginEndProps) => Promise<void>;
        },
        private action: T
    ) {
        super(action.description);
        this.requiresInteraction = action.requiresInteraction;
        this.canInteract = action.canInteract;

        this.action = action;
    }

    // override getRequiresInteraction() {
    //     return this.action.getRequiresInteraction();
    // }

    override getComponent() {
        return this.action?.getComponent?.();
    }

    override getProps() {
        return this.action?.getProps?.();
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        await this.setup?.begin?.(props);
        await this.action?.onBeginAction?.(props);
    }

    override async onEndAction(props: ActionBeginEndProps): Promise<void> {
        await this.setup?.end?.(props);
        await this.action?.onEndAction?.(props);
    }

    override getTooltipPos?(
        props: ActionPropsBase
    ): [number, number] | undefined {
        return this.action?.getTooltipPos?.(props);
    }
}

export class ClickInteraction<T extends GuideAction> extends GuideAction {
    private handlerFn: any | null = null;

    constructor(
        private selector: string,
        private mode: "auto-next" | "manual-next",
        private action: T,
        private canSkip: () => boolean = () => false
    ) {
        super(action.description);
        this.requiresInteraction = action.requiresInteraction;
        this.canInteract = action.canInteract;
        this.requiresInteraction.set(!canSkip());
        this.canInteract.set(true);

        this.action = action;
    }

    override getComponent() {
        return this.action?.getComponent?.();
    }
    override getProps() {
        return this.action?.getProps?.();
    }

    private onClickHandler(props: ActionBeginEndProps) {
        return (e: PointerEvent & { target: HTMLElement }) => {
            if (e.button === 0 && e.target.matches(this.selector)) {
                if (this.mode == "auto-next") {
                    props.nextStep();
                } else {
                    this.requiresInteraction.set(false);
                }
            }
        };
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        this.requiresInteraction.set(!this.canSkip());
        await this.action?.onBeginAction?.(props);

        this.handlerFn = this.onClickHandler(props) as any;

        document.addEventListener("click", this.handlerFn);
    }

    override async onEndAction(props: ActionBeginEndProps): Promise<void> {
        this.requiresInteraction.set(true);

        if (this.handlerFn) {
            document.removeEventListener("click", this.handlerFn);
        }

        await this.action?.onEndAction?.(props);
    }

    override getTooltipPos?(
        props: ActionPropsBase
    ): [number, number] | undefined {
        return this.action?.getTooltipPos?.(props);
    }
}

export class FlagStoreChange<
    T extends GuideAction,
    U extends Writable<{ [key: string]: any }>,
> extends GuideAction {
    private unsubscribe: Unsubscriber | null = null;

    constructor(
        private store: U,
        private storeMember: U extends Writable<infer X> ? keyof X : never,
        private mode: "auto-next" | "manual-next",
        private action: T
    ) {
        super(action.description);

        this.requiresInteraction = action.requiresInteraction;
        this.canInteract = action.canInteract;
        this.requiresInteraction.set(true);
        this.canInteract.set(true);

        this.action = action;
    }

    override getComponent() {
        return this.action?.getComponent?.();
    }
    override getProps() {
        return this.action?.getProps?.();
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        await this.action?.onBeginAction?.(props);

        this.unsubscribe = this.store.subscribe(v => {
            if (v[this.storeMember]) {
                if (this.mode == "auto-next") {
                    props.nextStep();
                } else {
                    this.requiresInteraction.set(false);
                }
            }
        });
    }
    override async onEndAction(props: ActionBeginEndProps): Promise<void> {
        this?.unsubscribe?.();
        await this.action?.onEndAction?.(props);
    }
    override getTooltipPos?(
        props: ActionPropsBase
    ): [number, number] | undefined {
        return this.action?.getTooltipPos?.(props);
    }
}
