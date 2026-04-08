import { writable } from 'svelte/store';
import { settingsStore } from './settings';

export type Theme =
	| 'light'
	| 'dark'
	| 'baroque-blue'
	| 'forest'
	| 'celestial-night'
	| 'earthy'
	| 'charcoal';

const VALID_THEMES: Theme[] = [
	'light',
	'dark',
	'baroque-blue',
	'forest',
	'celestial-night',
	'earthy',
	'charcoal'
];

const defaultTheme: Theme = 'light';

// Start with default — the real persisted value is loaded async in initThemeStore()
export const theme = writable<Theme>(defaultTheme);

function applyThemeToDOM(value: string) {
	if (typeof document === 'undefined') return;
	try {
		const safeVal = (value || 'light').toLowerCase();
		document.documentElement.setAttribute('data-theme', safeVal);
		document.body.classList.remove(...VALID_THEMES, 'Light', 'Dark');
		document.body.classList.add(safeVal);
	} catch (e) {
		console.error('Theme DOM update error', e);
	}
}

// Keep the DOM in sync whenever the store value changes
theme.subscribe(applyThemeToDOM);

// Track whether we've already attached the persist-subscriber to avoid duplicates
let persistSubscribed = false;

/**
 * Must be called once from +layout.svelte onMount.
 * Loads the persisted theme from the Tauri store (or localStorage fallback)
 * and wires up a subscriber that persists future changes.
 */
export async function initThemeStore() {
	if (typeof window === 'undefined') return;

	const isTauri =
		'__TAURI_INTERNALS__' in window || '__TAURI_IPC__' in window || '__TAURI__' in window;

	if (isTauri) {
		try {
			const store = settingsStore;

			// Load persisted theme; fall back to default if missing/invalid
			const saved = await store.get<string>('theme');
			const resolved =
				saved && VALID_THEMES.includes(saved.toLowerCase() as Theme)
					? (saved.toLowerCase() as Theme)
					: defaultTheme;

			// Apply if different from the current store value
			theme.update((current) => (current !== resolved ? resolved : current));

			// Attach the persist-subscriber only once per app lifetime
			if (!persistSubscribed) {
				persistSubscribed = true;
				theme.subscribe((value) => {
					store.set('theme', value).catch(console.error);
				});
			}
			return;
		} catch (e) {
			console.error('Tauri store error, falling back to localStorage:', e);
		}
	}

	// ----- localStorage fallback (browser dev mode) -----
	try {
		const stored = localStorage.getItem('theme');
		if (stored && VALID_THEMES.includes(stored.toLowerCase() as Theme)) {
			theme.set(stored.toLowerCase() as Theme);
		}
	} catch {}

	if (!persistSubscribed) {
		persistSubscribed = true;
		theme.subscribe((value) => {
			try {
				localStorage.setItem('theme', value);
			} catch {}
		});
	}
}
