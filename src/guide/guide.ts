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
            "The goal of this project is to work a working platformer level - together! This is the part of the level where the player will spawn in... ",
            {
                x: 0,
                y: 0,
                zoom: 12,
            },
            { x: 80, y: -10 }
        )
    ),
    new EditorGuide(
        "...and when the player touches this <b>END trigger</b>, they have completed the level! Your job is to build everything in between.",
        {
            x: LEVEL_WIDTH_UNITS - 70,
            y: LEVEL_HEIGHT_UNITS - 70,
            zoom: 12,
        },
        { x: LEVEL_WIDTH_UNITS + 30, y: LEVEL_HEIGHT_UNITS - 80 }
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
            { x: 0, y: 0 }
        )
    ),
    new ClickInteraction(
        "#edit-mode *",
        new HighlightElement(
            GUIDE_ELEM_IDS.placeMenuEditButton,
            "Go to the <b>edit tab</b> to adjust your object.",
            true
        )
    ),
    new HighlightElement(
        GUIDE_ELEM_IDS.placeMenu,
        "Move, scale, rotate, or change the color of your object!",
        true
    ),
    new ClickInteraction(
        ".pd-button *",
        new HighlightElement(
            GUIDE_ELEM_IDS.pdButton,
            "Click the place button to place the object in the level! (After you have done this, you need to wait a couple of minutes before you can place another one.)",
            true
        )
    ),
    new EditorGuide(
        "<b>Congratulations!</b> You have placed your first object.",
        null,
        { x: 0, y: 0 }
    ),
    new ClickInteraction(
        "#delete-mode *",
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
            {
                x: 0,
                y: 0,
            }
        )
    ),
    new EditorGuide(
        "<b>Congratulations!</b> You have deleted your first object! You can only delete one object every few minutes, but the delete timer is separate from the place timer.",
        null,
        { x: 0, y: 0 }
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
