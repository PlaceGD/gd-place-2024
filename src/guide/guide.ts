import { writable } from "svelte/store";
import { ExclusiveMenus, openMenu } from "../stores";

export type Step = {
    id: string;
    text: string;
    beforeCb?: () => Promise<void>;
};

export const GUIDE_ELEM_IDS = {
    test: "test",
    zoom: "zoom",
    signup: "signup",
    hideGrid: "hideGrid",
} as const;

export const GUIDE_STEPS: Step[] = [
    // {
    //     id: GUIDE_ELEM_IDS.test,
    //     text: "Click this button to view settings.",
    // },
    // {
    //     id: GUIDE_ELEM_IDS.hideGrid,
    //     text: "Click this button to hide grid.",
    // },
    // {
    //     id: GUIDE_ELEM_IDS.zoom,
    //     text: "Click these button to change zoom. You can use keybind blah blag",
    // },
    // {
    //     id: GUIDE_ELEM_IDS.signup,
    //     text: "fuck me",
    // },
] as const;

export const isGuideActive = writable(false);
// export const guideElems = writable<Record<string, HTMLElement>>({});

// export const registerElem = (id: string, elem: HTMLElement) => {
//     guideElems.update(g => {
//         g[id] = elem;
//         return g;
//     });
// };
// export const unregisterElem = (id: string) => {
//     guideElems.update(g => {
//         delete g[id];
//         return g;
//     });
// };

export const beginGuide = () => {
    isGuideActive.update(() => true);
};

// export const beginGuide = () => {
//     for (const step of GUIDE_STEPS) {
//         const elem = document.getElementById(step.id)!
//             .firstChild! as HTMLElement;

//         console.log(elem);

//         elem.style.boxShadow = "0 0 0 max(100vh, 100vw) rgba(0, 0, 0, .5)";
//     }
// };
