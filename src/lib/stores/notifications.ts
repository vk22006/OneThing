import { writable } from 'svelte/store';
import { settingsStore } from './settings';
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

	const store = settingsStore;
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

	const store = settingsStore;
	await store.set('notificationSettings', value);
});

if (browser) {
	void hydrateSettings();
}
