if (typeof window !== "undefined") {
    window.consoleErrors = [];

    window.clearAllTheStuff = () => {
        localStorage.clear();
        indexedDB.deleteDatabase("PlaceDB");
        window.location.reload();
    };

    const oldError = window.console.error;
    window.console.error = function (...args) {
        const message = args.join(" ");

        if (!message.includes("recursive use of an object")) {
            window.consoleErrors.push(message);
        }

        return oldError(...args);
    };
}
