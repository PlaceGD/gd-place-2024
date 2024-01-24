import type { SvelteComponent } from "svelte";

export type LoginData = {
    isLoggedIn: boolean;
    showLoginUI: boolean;
};

export enum LoginMethod {
    Google,
    GitHub,
    X,
}

export const handleLogout = () => {};

export type Component = new (...args: any[]) => SvelteComponent;
export type ComponentWithProps = { component: Component; props?: any };

export type SliderMethods = {
    previous: () => void;
    addSlideAndMove: (slide: ComponentWithProps) => void;
};
