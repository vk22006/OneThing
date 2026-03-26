import { writable } from 'svelte/store';
import { LazyStore } from '@tauri-apps/plugin-store';
import { browser } from '$app/environment';

type NotificationSettings = {
	enabled: boolean;
	sound: boolean;
	focusMode: boolean;
	focusInterval: number;
};

const DEFAULT_SETTINGS: NotificationSettings = {
	enabled: true,
	sound: true,
	focusMode: false,
	focusInterval: 25
};

export const notificationSettings = writable({
	...DEFAULT_SETTINGS
});
let hydrated = false;
let initializing = false;

async function hydrateSettings() {
	if (!browser || hydrated || initializing) return;
	initializing = true;

	const store = new LazyStore('settings.json');
	const saved = await store.get<NotificationSettings>('notificationSettings');
	if (saved) {
		notificationSettings.set({ ...DEFAULT_SETTINGS, ...saved });
	}
	hydrated = true;
	initializing = false;
}

export async function loadSettings() {
	await hydrateSettings();
}

notificationSettings.subscribe(async (value) => {
	if (!browser) return;
	if (!hydrated) return;

	const store = new LazyStore('settings.json');
	await store.set('notificationSettings', value);
	await store.save();
});

if (browser) {
	void hydrateSettings();
}
