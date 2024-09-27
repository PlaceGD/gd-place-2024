if (typeof window !== "undefined") {
    window.consoleErrors = [];

    window.clearAllTheStuff = () => {
        localStorage.clear();
        indexedDB.deleteDatabase("PlaceDB");
        window.location.reload();
    };

    const oldError = window.console.error;
    window.console.error = function (...args) {
        window.consoleErrors.push(args.join(" "));

        return oldError(...args);
    };
}
