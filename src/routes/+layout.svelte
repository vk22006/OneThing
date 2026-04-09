<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { theme, initThemeStore } from '$lib/stores/theme';
	import { onMount } from 'svelte';
	import { settingsStore } from '$lib/stores/settings';
	import { notificationSettings, loadSettings } from '$lib/stores/notifications';
	import { NotificationManager } from '$lib/notifications/notificationManager';
	import AppShell from '$lib/layout/AppShell.svelte';
	import SplashScreen from '$lib/components/SplashScreen.svelte';
	import { goto } from '$app/navigation';
	import { focusTaskInput } from '$lib/stores/shortcuts';

	let { children } = $props();

	let appLoaded = $state(false);

	const NAV_ROUTES = [
		'/',
		'/Components/ProjectInfo',
		'/Components/Progress',
		'/Components/Settings'
	];

	function handleGlobalKey(e: KeyboardEvent) {
		const target = e.target as HTMLElement;
		const isTyping =
			target.tagName === 'INPUT' ||
			target.tagName === 'TEXTAREA' ||
			target.isContentEditable;

		const mod = e.ctrlKey || e.metaKey;

		// Alt + 1-4 → navigate to the matching route
		if (e.altKey && !e.ctrlKey && !e.metaKey) {
			const idx = parseInt(e.key) - 1;
			if (idx >= 0 && idx < NAV_ROUTES.length) {
				e.preventDefault();
				goto(NAV_ROUTES[idx]);
				return;
			}
		}

		// Ctrl/Cmd + , → Settings
		if (mod && e.key === ',') {
			e.preventDefault();
			goto('/Components/Settings');
			return;
		}

		// Ctrl/Cmd + N → focus new-task input (go to Tasks first if needed)
		if (mod && e.key === 'n') {
			e.preventDefault();
			goto('/').then(() => focusTaskInput.set(true));
			return;
		}

		// '/' when not typing → focus new-task input (same as Ctrl+N)
		if (!isTyping && e.key === '/') {
			e.preventDefault();
			goto('/').then(() => focusTaskInput.set(true));
		}
	}

	function localDateKey(d: Date): string {
		// Use local time to avoid "yesterday/tomorrow" issues around midnight.
		const yyyy = d.getFullYear();
		const mm = String(d.getMonth() + 1).padStart(2, '0');
		const dd = String(d.getDate()).padStart(2, '0');
		return `${yyyy}-${mm}-${dd}`;
	}

	onMount(() => {
		// Initialize persistent theme store
		initThemeStore();

		// Force DOM sync from within the mounted component
		const unsubTheme = theme.subscribe((value) => {
			if (typeof document === 'undefined') return;
			try {
				const safeVal = (value || 'light').toLowerCase();
				document.documentElement.setAttribute('data-theme', safeVal);
				document.body.classList.remove('light', 'dark', 'Light', 'Dark', 'baroque-blue', 'forest', 'celestial-night', 'earthy', 'charcoal');
				document.body.classList.add(safeVal);
			} catch (e) {}
		});

		void (async () => {
			// Load notification settings from the persisted store before deciding whether to send.
			await loadSettings();

			const settings = await new Promise<{
				enabled: boolean;
				sound: boolean;
				focusMode: boolean;
				focusInterval: number;
			}>((resolve) => {
				const unsub = notificationSettings.subscribe((v) => {
					resolve(v as any);
					unsub();
				});
			});

			if (!settings.enabled) return;

			// Send at most once per calendar day (while the app is opened).
			const today = localDateKey(new Date());
			const store = settingsStore;
			const lastSent = await store.get<string>('lastDeadlineReminderDate');

			if (lastSent === today) return;

			// Build a simple "deadline for today" message from localStorage.
			const stored = localStorage.getItem('tasks');
			let tasks: Array<{ status: string; deadline?: string }> = [];
			if (stored) {
				try {
					tasks = JSON.parse(stored) ?? [];
				} catch {
					// Ignore malformed localStorage.
				}
			}

			const now = new Date();
			const upcomingCount = tasks.filter((t) => {
				if (t.status !== 'TODO' || !t.deadline) return false;
				const [hours, mins] = t.deadline.split(':').map(Number);
				const due = new Date(now.getFullYear(), now.getMonth(), now.getDate(), hours, mins);
				return due.getTime() > now.getTime();
			}).length;

			// Request permission if needed. Only mark "sent today" if we can actually send.
			const granted = await NotificationManager.checkPermission();
			if (!granted) return;

			await NotificationManager.notify(
				'Daily Deadline Reminder',
				upcomingCount > 0
					? `You have ${upcomingCount} deadline${upcomingCount === 1 ? '' : 's'} coming up today. Open your tasks to review.`
					: "Review your deadlines for today. Open your tasks to check what's coming up."
			);

			await store.set('lastDeadlineReminderDate', today);
		})();

		return () => {
			unsubTheme();
		};
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<svelte:body />
<svelte:window onkeydown={handleGlobalKey} />

<SplashScreen onComplete={() => (appLoaded = true)} />

<div style="display: {appLoaded ? 'contents' : 'none'}">
	<AppShell>
		{@render children()}
	</AppShell>
</div>
