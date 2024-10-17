import { writable } from "svelte/store";
import { ExclusiveMenus, openMenu } from "../stores";
import { HighlightElement, type GuideAction } from "./guideActions";

export const GUIDE_ELEM_IDS = {
    test: "test",
    zoom: "zoom",
    signup: "signup",
    hideGrid: "hideGrid",
} as const;

export const GUIDE_STEPS: GuideAction[] = [
    new HighlightElement(
        GUIDE_ELEM_IDS.test,
        "Click this button to view settings."
    ),
] as const;

export const isGuideActive = writable(false);

export const beginGuide = () => {
    isGuideActive.update(() => true);
};
