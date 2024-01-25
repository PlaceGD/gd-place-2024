import type { SvelteComponent } from "svelte";

export type LoginData = {
    isLoggedIn: boolean;
    showLoginUI: boolean;
};

export enum LoginMethod {
    Google = "Google",
    GitHub = "GitHub",
    X = "X",
}

export const handleLogout = () => {};

export type Component = new (...args: any[]) => SvelteComponent;
export type ComponentWithProps = {
    component: Component;
    props?: any;
    showBackButton?: boolean;
    showCloseButton?: boolean;
};

export type SliderMethods = {
    previous: () => void;
    addSlideAndMove: (slide: ComponentWithProps) => void;
    setInteractability: (interact: boolean) => void;
};
