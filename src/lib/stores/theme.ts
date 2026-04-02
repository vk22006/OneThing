import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { LazyStore } from '@tauri-apps/plugin-store';

export type Theme = 'light' | 'dark' | 'baroque-blue' | 'forest' | 'celestial-night' | 'earthy' | 'charcoal';

const defaultTheme: Theme = 'light';

function getInitialTheme(): Theme {
	if (typeof window === 'undefined') return defaultTheme;
	try {
		const stored = localStorage.getItem('theme');
		const validThemes = ['light', 'dark', 'baroque-blue', 'forest', 'celestial-night', 'earthy', 'charcoal'];
		if (stored && validThemes.includes(stored)) return stored as Theme;
		const lower = stored?.toLowerCase();
		if (lower && validThemes.includes(lower)) return lower as Theme;
	} catch {
		// Ignore localStorage error
	}
	return defaultTheme;
}

const initialTheme = getInitialTheme();
export const theme = writable<Theme>(initialTheme);

function applyThemeToDOM(value: string) {
	let debugText =
		'applyThemeToDOM called with: ' + value + ' | has_doc: ' + (typeof document !== 'undefined');

	if (typeof document === 'undefined') {
		return;
	}

	try {
		const safeVal = (value || 'light').toLowerCase();
		document.documentElement.setAttribute('data-theme', safeVal);
		document.body.classList.remove('light', 'dark', 'Light', 'Dark', 'baroque-blue', 'forest', 'celestial-night', 'earthy', 'charcoal');
		document.body.classList.add(safeVal);
		debugText += ' | success setting ' + safeVal;
	} catch (e) {
		console.error('Theme DOM update error', e);
		debugText += ' | ERROR';
	}

	// Direct DOM write to debugger div
	const debugEl = document.getElementById('tauri-debug');
	if (debugEl) {
		const p = document.createElement('p');
		p.style.color = 'yellow';
		p.innerText = 'Trace: ' + debugText;
		debugEl.appendChild(p);
	}
}

theme.subscribe(applyThemeToDOM);

export async function initThemeStore() {
	if (typeof window === 'undefined') return;

	const isTauri = '__TAURI_INTERNALS__' in window || '__TAURI_IPC__' in window || '__TAURI__' in window;
	
	if (isTauri) {
		try {
			const store = new LazyStore('settings.json');
			let saved = await store.get<string>('theme');
			if (saved) {
				const norm = saved.toLowerCase() as Theme;
				if (['light', 'dark', 'baroque-blue', 'forest', 'celestial-night', 'earthy', 'charcoal'].includes(norm)) {
					theme.set(norm);
				}
			}

			theme.subscribe((value) => {
				try {
					localStorage.setItem('theme', value);
				} catch {}
				store
					.set('theme', value)
					.then(() => store.save())
					.catch(console.error);
			});
			return;
		} catch (e) {
			console.error('Tauri store error', e);
		}
	}

	// LocalStorage fallback
	theme.subscribe((value) => {
		try {
			localStorage.setItem('theme', value);
		} catch {}
	});
}
