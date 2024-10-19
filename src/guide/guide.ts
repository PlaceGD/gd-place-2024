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
    FlagStoreChange,
    WaitThen,
    type GuideAction,
} from "./guideActions";
import { LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from "shared-lib/nexusgen";
import { EditTab } from "../place_menu/edit/edit_tab";

export const walmart = writable({
    hasDeleteSelection: false,
    hasPlacedObject: false,
});

export const GUIDE_ELEM_IDS = {
    placeMenu: "place-menu",
    placeMenuEditButton: "place-menu-edit",
    placeMenuDeleteButton: "place-menu-delete",
    settingsMenuDonate: "settings-menu-donate",
    pdButton: "place-delete-button",
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
            "TODO start of level",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            { x: 0, y: 0 }
        )
    ),
    new EditorGuide(
        "TODO end of level",
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
                menuTabGroup.set(TabGroup.Build);
            },
        },
        new ClickInteraction(
            ".object-grid-container > li *",
            new HighlightElement(
                GUIDE_ELEM_IDS.placeMenu,
                "TODO choose an object",
                true
            )
        )
    ),
    new FlagStoreChange(
        walmart,
        "hasPlacedObject",
        new EditorGuide(
            "TODO place object",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            { x: 0, y: 0 }
        )
    ),
    new ClickInteraction(
        "#edit-mode *",
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuEditButton,
            "TODO choose edit to edit object",
            true
        )
    ),
    new HighlightElement(
        GUIDE_ELEM_IDS.placeMenu,
        "TODO edit the object props",
        true
    ),
    new ClickInteraction(
        ".pd-button *",
        new HighlightElement(
            GUIDE_ELEM_IDS.pdButton,
            "TODO click place button",
            true
        )
    ),
    new EditorGuide("TODO WOW OBJECT", null, { x: 0, y: 0 }),
    new ClickInteraction(
        "#delete-mode *",
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuDeleteButton,
            "TODO choose delete to delete object",
            true
        )
    ),
    new FlagStoreChange(
        walmart,
        "hasDeleteSelection",
        new EditorGuide("TODO click on an object to select it", null, {
            x: 0,
            y: 0,
        })
    ),
    new Setup(
        {
            begin: async () => {
                openMenu.set(ExclusiveMenus.Settings);
            },
        },

        new WaitThen(
            new HighlightElement(
                GUIDE_ELEM_IDS.settingsMenuDonate,
                "TODO donate"
            ),
            500
        )
    ),
] as const;

export const isGuideActive = writable(false);

export const beginGuide = () => {
    isGuideActive.update(() => true);
};
