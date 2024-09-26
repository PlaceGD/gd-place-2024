import { writable } from "svelte/store";

export let DEBUG = writable(__DEBUG ?? false); // in development mode it is set to true by default
