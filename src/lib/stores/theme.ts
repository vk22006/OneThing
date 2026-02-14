import { writable } from "svelte/store";
import { browser } from "$app/environment";

export type Theme = "light" | "dark";

const defaultTheme: Theme = "light";

const initialTheme: Theme =
    browser
        ? (localStorage.getItem("theme") as Theme) ?? defaultTheme
        : defaultTheme;

export const theme = writable<Theme>(initialTheme);

// save only in browser
if (browser) {
    theme.subscribe(value => {
        localStorage.setItem("theme", value);
    });
}
