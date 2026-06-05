<script lang="ts">
	import Header from '../../../lib/layout/Header.svelte';
	import { theme, type Theme } from '$lib/stores/theme';
	import { notificationSettings } from '$lib/stores/notifications';
	import { onMount } from 'svelte';
	import { loadSettings } from '$lib/stores/notifications';
	import { NotificationManager } from '$lib/notifications/notificationManager';
	import { invoke } from '@tauri-apps/api/core';
	import { settingsStore } from '$lib/stores/settings';

	let pageTitle = $state('Settings');
	type SettingsState = {
		enabled: boolean;
		sound: boolean;
		focusMode: boolean;
		focusInterval: number;
	};

	let currentTheme = $state<Theme>('light');
	let settingsState = $state<SettingsState>({
		enabled: true,
		sound: true,
		focusMode: false,
		focusInterval: 25
	});
	let dailyReminderTime = $state('09:00');
	let autoStartEnabled = $state(false);

	onMount(() => {
		const unsubTheme = theme.subscribe((value) => {
			currentTheme = value;
		});
		const unsubNotifications = notificationSettings.subscribe((value) => {
			settingsState = { ...value };
		});

		loadSettings();

		// Load daily reminder time from store
		void (async () => {
			const store = settingsStore;
			const saved = await store.get<string>('dailyReminderTime');
			if (saved) dailyReminderTime = saved;

			// Load autostart state
			try {
				const enabled = await invoke<boolean>('plugin:autostart|is_enabled');
				autoStartEnabled = enabled;
			} catch {
				// Autostart plugin may not be available on all platforms
			}
		})();

		return () => {
			unsubTheme();
			unsubNotifications();
		};
	});

	function setTheme(t: Theme) {
		theme.set(t);
		currentTheme = t;

		// Force immediate DOM update regardless of the store's state
		if (typeof document !== 'undefined') {
			try {
				const safeVal = (t || 'light').toLowerCase();
				document.documentElement.setAttribute('data-theme', safeVal);
				document.body.classList.remove(
					'light',
					'dark',
					'Light',
					'Dark',
					'baroque-blue',
					'forest',
					'celestial-night',
					'earthy',
					'charcoal'
				);
				document.body.classList.add(safeVal);
			} catch (e) {}
		}
	}

	function updateSetting(key: keyof SettingsState, value: SettingsState[keyof SettingsState]) {
		settingsState = { ...settingsState, [key]: value };
		notificationSettings.set(settingsState);
	}

	async function updateDailyReminderTime(time: string) {
		dailyReminderTime = time;
		// Save to both the Svelte store and the Rust backend
		const store = settingsStore;
		await store.set('dailyReminderTime', time);
		await NotificationManager.setDailyReminderTime(time);
	}

	async function toggleAutoStart() {
		try {
			if (autoStartEnabled) {
				await invoke('plugin:autostart|disable');
				autoStartEnabled = false;
			} else {
				await invoke('plugin:autostart|enable');
				autoStartEnabled = true;
			}
		} catch {
			// Handle gracefully
		}
	}
</script>

<Header page={pageTitle} />

<div class="mx-auto w-full max-w-4xl px-8 py-8">
	<!-- Appearance Section -->
	<div class="mb-8 rounded-[20px] border border-[var(--border)] bg-[var(--surface)] p-8 shadow-sm">
		<h2 class="mb-6 flex items-center gap-2 text-xl font-bold tracking-tight text-[var(--text)]">
			<span class="h-6 w-2 rounded-full bg-red-500"></span>
			Appearance
		</h2>
		<div class="relative">
			<select
				value={currentTheme}
				onchange={(e) => setTheme(e.currentTarget.value as Theme)}
				class="w-full cursor-pointer rounded-2xl border-2 border-[var(--border)] bg-[var(--surface-2)] px-6 py-4 font-semibold text-[var(--text)] shadow-sm transition-all duration-200 hover:border-red-400 focus:border-red-500 focus:ring-[3px] focus:ring-red-100 focus:outline-none"
			>
				<option value="light">Light Mode</option>
				<option value="dark">Dark Mode</option>
				<option value="baroque-blue">Baroque Blue</option>
				<option value="forest">Forest</option>
				<option value="celestial-night">Celestial Night</option>
				<option value="earthy">Earthy</option>
				<option value="charcoal">Charcoal</option>
			</select>
		</div>
	</div>

	<!-- Notifications Section -->
	<div class="mb-8 rounded-[20px] border border-[var(--border)] bg-[var(--surface)] p-8 shadow-sm">
		<h2 class="mb-6 flex items-center gap-2 text-xl font-bold tracking-tight text-[var(--text)]">
			<span class="h-6 w-2 rounded-full bg-red-500"></span>
			Notifications
		</h2>

		<div class="space-y-6">
			<!-- Main Toggle -->
			<div class="flex items-center justify-between rounded-2xl bg-[var(--surface-2)] p-4">
				<div>
					<p class="font-bold text-[var(--text)]">Enable System Notifications</p>
					<p class="text-sm text-[var(--muted)]">Get reminders for deadlines and focus nudges</p>
				</div>
				<button
					onclick={() => updateSetting('enabled', !settingsState.enabled)}
					aria-label="Toggle system notifications"
					title="Toggle system notifications"
					class="relative h-8 w-14 rounded-full transition-colors {settingsState.enabled
						? 'bg-red-500'
						: 'bg-gray-300'}"
				>
					<div
						class="absolute top-1 left-1 h-6 w-6 rounded-full bg-[var(--surface)] transition-transform {settingsState.enabled
							? 'translate-x-6'
							: ''}"
					></div>
				</button>
			</div>

			<div
				class="grid grid-cols-1 gap-6 md:grid-cols-2 {settingsState.enabled
					? ''
					: 'pointer-events-none opacity-50 transition-opacity'}"
			>
				<!-- Sound Toggle -->
				<div
					class="rounded-2xl border border-[var(--border-2)] p-4 transition-colors hover:border-red-200"
				>
					<div class="mb-2 flex items-center justify-between">
						<p class="font-bold text-[var(--text)]">Notification Sound</p>
						<input
							type="checkbox"
							checked={settingsState.sound}
							onchange={(e) => updateSetting('sound', e.currentTarget.checked)}
							class="h-5 w-5 accent-red-500"
						/>
					</div>
					<p class="text-xs text-[var(--muted)]">Play a subtle sound for every alert</p>
				</div>

				<!-- Focus Mode Toggle -->
				<div
					class="rounded-2xl border border-[var(--border-2)] p-4 transition-colors hover:border-red-200"
				>
					<div class="mb-2 flex items-center justify-between">
						<p class="font-bold text-[var(--text)]">Focus Mode</p>
						<input
							type="checkbox"
							checked={settingsState.focusMode}
							onchange={(e) => updateSetting('focusMode', e.currentTarget.checked)}
							class="h-5 w-5 accent-red-500"
						/>
					</div>
					<p class="text-xs text-[var(--muted)]">Suppresses all but urgent deadline alerts</p>
				</div>

				<!-- Daily Reminder Time -->
				<div
					class="rounded-2xl border border-[var(--border-2)] p-4 transition-colors hover:border-red-200"
				>
					<div class="mb-2 flex items-center justify-between">
						<p class="font-bold text-[var(--text)]">Daily Reminder Time</p>
						<input
							type="time"
							value={dailyReminderTime}
							onchange={(e) => updateDailyReminderTime(e.currentTarget.value)}
							class="cursor-pointer rounded-lg border border-[var(--border)] bg-[var(--surface)] px-3 py-1.5 text-sm font-bold text-[var(--text)] focus:border-red-500 focus:ring-[2px] focus:ring-red-100 focus:outline-none"
						/>
					</div>
					<p class="text-xs text-[var(--muted)]">
						Daily digest notification with your upcoming deadlines
					</p>
				</div>

				<!-- Start at Login -->
				<div
					class="rounded-2xl border border-[var(--border-2)] p-4 transition-colors hover:border-red-200"
				>
					<div class="mb-2 flex items-center justify-between">
						<p class="font-bold text-[var(--text)]">Start at Login</p>
						<input
							type="checkbox"
							checked={autoStartEnabled}
							onchange={() => toggleAutoStart()}
							class="h-5 w-5 accent-red-500"
						/>
					</div>
					<p class="text-xs text-[var(--muted)]">
						Launch OneThing when you log in so background reminders always work
					</p>
				</div>

				<!-- Interval Slider -->
				<div class="rounded-2xl border border-[var(--border-2)] p-4 md:col-span-2">
					<div class="mb-4 flex justify-between">
						<p class="font-bold text-[var(--text)]">Focus Nudge Interval</p>
						<span class="rounded-full bg-red-100 px-3 py-1 font-bold text-red-700"
							>{settingsState.focusInterval} mins</span
						>
					</div>
					<input
						type="range"
						min="5"
						max="60"
						step="5"
						value={settingsState.focusInterval}
						oninput={(e) => updateSetting('focusInterval', parseInt(e.currentTarget.value))}
						class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-[var(--surface-2)] accent-red-500"
					/>
					<div class="mt-2 flex justify-between text-[10px] font-medium text-[var(--muted-2)]">
						<span>5 MIN</span>
						<span>30 MIN</span>
						<span>60 MIN</span>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Background Behavior Section -->
	<div class="rounded-[20px] border border-[var(--border)] bg-[var(--surface)] p-8 shadow-sm">
		<h2 class="mb-6 flex items-center gap-2 text-xl font-bold tracking-tight text-[var(--text)]">
			<span class="h-6 w-2 rounded-full bg-red-500"></span>
			Background Behavior
		</h2>

		<div class="rounded-2xl bg-[var(--surface-2)] p-4">
			<div class="flex items-start gap-3">
				<div class="mt-0.5 flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-red-100">
					<svg class="h-4 w-4 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
					</svg>
				</div>
				<div>
					<p class="font-bold text-[var(--text)]">Closing minimizes to tray</p>
					<p class="mt-1 text-sm leading-relaxed text-[var(--muted)]">
						When you close the window, OneThing keeps running in the system tray so your notifications still fire on time. Right-click the tray icon to fully quit the app.
					</p>
				</div>
			</div>
		</div>
	</div>
</div>
