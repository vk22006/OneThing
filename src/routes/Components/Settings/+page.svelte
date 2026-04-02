<script lang="ts">
	import Header from '../../../lib/layout/Header.svelte';
	import { theme, type Theme } from '$lib/stores/theme';
	import { notificationSettings } from '$lib/stores/notifications';
	import { onMount } from 'svelte';
	import { loadSettings } from '$lib/stores/notifications';

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

	onMount(() => {
		const unsubTheme = theme.subscribe((value) => {
			currentTheme = value;
		});
		const unsubNotifications = notificationSettings.subscribe((value) => {
			settingsState = { ...value };
		});

		loadSettings();

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
	<div class="rounded-[20px] border border-[var(--border)] bg-[var(--surface)] p-8 shadow-sm">
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
</div>
