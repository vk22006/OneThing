import { writable } from 'svelte/store';

/**
 * When set to true, the Tasks page should focus the new-task input field.
 * Reset it back to false after responding so it can be fired again.
 */
export const focusTaskInput = writable(false);
