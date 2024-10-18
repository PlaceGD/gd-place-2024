import { writable } from "svelte/store";
import {
    ExclusiveMenus,
    menuBuildTab,
    menuEditTab,
    menuMinimized,
    menuSelectedObject,
    menuTabGroup,
    openMenu,
    TabGroup,
} from "../stores";
import {
    ClickInteraction,
    EditorGuide,
    HighlightElement,
    Setup,
    WaitThen,
    type GuideAction,
} from "./guideActions";
import { LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from "shared-lib/nexusgen";
import { EditTab } from "../place_menu/edit/edit_tab";

export const GUIDE_ELEM_IDS = {
    placeMenu: "place-menu",
    placeMenuModes: "place-menu-modes",
    placeMenuTabs: "place-menu-tabs",
    objectsList: "objects-list",
    pdButton: "place-delete-button",
    transformTab: "transform-tab",
    layersTab: "layers-tab",
    colorsTab: "colors-tab",
} as const;

export const GUIDE_STEPS: GuideAction[] = [
    new Setup(
        {
            begin: async () => {
                // run any initial setup for the guide, like hiding stuff
                menuMinimized.set(true);
            },
        },
        new EditorGuide(
            "This is the beginning of the platformer level where the player will start. The idea is to build a course that will let the player reach the goal.",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            { x: 0, y: 0 }
        )
    ),
    new EditorGuide(
        "This is the goal of the level and when the player reaches this the level has been completed. Build your structures to help the player get here.",
        {
            x: LEVEL_WIDTH_UNITS,
            y: LEVEL_HEIGHT_UNITS,
            zoom: 12,
        },
        { x: 0, y: 0 }
    ),
    new Setup(
        {
            begin: async () => {
                menuMinimized.set(false);
            },
            end: async ({ direction }) => {
                // if we are going backwards we want the menu to be hidden
                // but not forwards
                if (direction === -1) {
                    menuMinimized.set(true);
                }
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenu,
            "This is the build and is the tool for placing and deleting objects."
        )
    ),
    new Setup(
        {
            end: async () => {
                menuTabGroup.set(TabGroup.Build);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuModes,
            `These are the different build tab modes. From top to bottom: Place, Edit, Delete. To place an object, first select the Place mode.`
        )
    ),

    new ClickInteraction(
        `[data-guide=${GUIDE_ELEM_IDS.placeMenuTabs}] > li *`,
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuTabs,
            "Next, select the kind of object you want from the list. (Click one of the objects)"
        )
    ),

    new Setup(
        {
            end: async () => {
                menuSelectedObject.set(83);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.objectsList,
            "Then, pick the object you want to place."
        )
    ),
    new Setup(
        {
            begin: async ({ state }) => {
                let prev = state.get_preview_object();
                prev.x_scale_exp = 0;
                prev.x_angle = 0;
                prev.y_scale_exp = 0;
                prev.y_angle = 18;
                prev.x = 75;
                prev.y = 75;
                state.set_preview_object(prev);
                state.set_preview_visibility(true);
            },
        },
        new EditorGuide(
            "Click anywhere on the level to view the preview of your object.",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            { x: 0, y: 0 }
        )
    ),
    new Setup(
        {
            begin: async () => {
                menuTabGroup.set(TabGroup.Edit);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuTabs,
            "Properties such as transform, layer and color can be changed from the Edit mode."
        )
    ),
    new Setup(
        {
            begin: async () => {
                menuEditTab.set(EditTab.Transform);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.transformTab,
            "Transform allows you to change the position of the object, flip it, rotate it, etc."
        )
    ),
    new Setup(
        {
            begin: async () => {
                menuEditTab.set(EditTab.Layers);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.layersTab,
            `The layers tab allows you to change how the object is drawn on top of other objects and the player. 
            An object with a higher Z layer and Z index than the one below it will show on top.
            The layers "T*" mean the object will show up on top of the player and "B*" means they will show below the player.
            `
        )
    ),
    new Setup(
        {
            begin: async () => {
                menuEditTab.set(EditTab.Colors);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.colorsTab,
            "The colors tab will change the color of the object. Some objects can have 2 colors - a main and a detail."
        )
    ),
    new Setup(
        {
            end: async ({ state }) => {
                state.set_preview_visibility(false);
                menuMinimized.set(false);
            },
        },
        new HighlightElement(
            GUIDE_ELEM_IDS.pdButton,
            "Finally, click the place button to place the object!"
        )
    ),
] as const;

export const isGuideActive = writable(false);

export const beginGuide = () => {
    isGuideActive.update(() => true);
};
