import type { ComponentType, EventDispatcher } from "svelte";
import * as wasm from "wasm-lib";
import HighlightElementComp from "./HighlightElement.svelte";
import { moveCamera } from "../level_view/view_controls";
import { get, type Unsubscriber, type Writable } from "svelte/store";

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
    getRequiresInteraction(): boolean {
        return false;
    }

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
        private tooltiopPos: EditorGuidePosition
    ) {
        super(description);
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

        this.action = action;
    }

    override getRequiresInteraction() {
        return this.action.getRequiresInteraction();
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

        this.action = action;
    }

    override getRequiresInteraction() {
        return this.action.getRequiresInteraction();
    }

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
        private action: T
    ) {
        super(action.description);

        this.action = action;
    }

    override getRequiresInteraction() {
        return true;
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
                props.nextStep();
            }
        };
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        await this.action?.onBeginAction?.(props);

        this.handlerFn = this.onClickHandler(props) as any;

        document.addEventListener("click", this.handlerFn);
    }

    override async onEndAction(props: ActionBeginEndProps): Promise<void> {
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
        private action: T
    ) {
        super(action.description);

        this.action = action;
    }

    override getRequiresInteraction() {
        return true;
    }

    override getComponent() {
        return this.action?.getComponent?.();
    }
    override getProps() {
        return this.action?.getProps?.();
    }

    override async onBeginAction(props: ActionBeginEndProps): Promise<void> {
        await this.action?.onBeginAction?.(props);

        let isInitial = true;
        this.unsubscribe = this.store.subscribe(v => {
            if (!isInitial && v[this.storeMember]) {
                props.nextStep();
            }
            isInitial = false;
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
