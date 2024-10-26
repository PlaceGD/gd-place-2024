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
    EditorGuidePosition,
} from "./guideActions";
import { LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from "shared-lib/nexusgen";
import { EditTab } from "../place_menu/edit/edit_tab";

export const walmart = writable({
    hasDeleteSelection: false,
    hasPlacedObject: false,
    hasEditedObject: false,
});

export const GUIDE_ELEM_IDS = {
    placeMenu: "place-menu",
    placeMenuEditButton: "place-menu-edit",
    placeMenuDeleteButton: "place-menu-delete",
    settingsMenuDonate: "settings-menu-donate",
    pdButton: "place-delete-button",
    placeMenuModes: null,
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
            "The goal of this project is to work a working platformer level - together! This is the part of the level where the player will spawn in... ",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            EditorGuidePosition.Bottom
        )
    ),
    new EditorGuide(
        "...and when the player touches this <b>END trigger</b>, they have completed the level! Your job is to build everything in between.",
        {
            x: LEVEL_WIDTH_UNITS - 70,
            y: LEVEL_HEIGHT_UNITS - 70,
            zoom: 12,
        },
        EditorGuidePosition.Bottom
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
            "manual-next",
            new HighlightElement(
                GUIDE_ELEM_IDS.placeMenu,
                "Let's place an object! Select an object from this menu.",
                true
            )
        )
    ),
    new FlagStoreChange(
        walmart,
        "hasPlacedObject",
        new EditorGuide(
            `Click where you want your object to be placed in the level.`,
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            EditorGuidePosition.Top
        )
    ),
    new Setup(
        {
            begin: async () => {
                menuMinimized.set(false);
            },
        },
        new ClickInteraction(
            "#edit-mode *",
            "auto-next",
            new HighlightElement(
                GUIDE_ELEM_IDS.placeMenuEditButton,
                "Go to the <b>edit tab</b> to adjust your object.",
                true
            )
        )
    ),
    new EditorGuide(
        "Move, scale, rotate, or change the color of your object!",
        null,
        EditorGuidePosition.Top,
        true
    ),
    new ClickInteraction(
        ".pd-button *",
        "auto-next",
        new HighlightElement(
            GUIDE_ELEM_IDS.pdButton,
            "Click the place button to place the object in the level! (After you have done this, you need to wait some time before you can place another one.)",
            true
        )
    ),
    new EditorGuide(
        "<b>Congratulations!</b> You have placed your first object.",
        null,
        EditorGuidePosition.Top
    ),
    new ClickInteraction(
        "#delete-mode *",
        "auto-next",
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuDeleteButton,
            "Let's delete an object >:) Go to the <b>delete tab</b>.",
            true
        )
    ),
    new FlagStoreChange(
        walmart,
        "hasDeleteSelection",
        new EditorGuide(
            "Click on an object in the level you wish to delete!",
            null,
            EditorGuidePosition.Top
        )
    ),
    new ClickInteraction(
        ".pd-button *",
        "auto-next",
        new HighlightElement(
            GUIDE_ELEM_IDS.pdButton,
            "Click the delete button to delete the object in the level! (After you have done this, you need to wait some time before you can delete another one.)",
            true
        )
    ),
    new EditorGuide(
        "<b>Congratulations!</b> You have deleted your first object! You can only delete one object every few minutes, but the delete timer is separate from the place timer.",
        null,
        EditorGuidePosition.Top
    ),
    new Setup(
        {
            begin: async () => {
                menuMinimized.set(true);
                openMenu.set(ExclusiveMenus.Settings);
            },
            end: async () => {
                openMenu.set(null);
                menuMinimized.set(false);
            },
        },

        new WaitThen(
            new HighlightElement(
                GUIDE_ELEM_IDS.settingsMenuDonate,
                "This project took a lot of passionate work to make, so if you enjoy it, consider donating! If you donate (any amount) you also get to choose the colors of your username (which makes you really cool)!"
            ),
            500
        )
    ),
] as const;

export const isGuideActive = writable(false);

export const beginGuide = () => {
    isGuideActive.update(() => true);
};
