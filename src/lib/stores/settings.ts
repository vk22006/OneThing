import { LazyStore } from '@tauri-apps/plugin-store';

// A single unified store instance to avoid split-brain memory cache overwrites
export const settingsStore = new LazyStore('settings.json', { autoSave: true, defaults: {} });
