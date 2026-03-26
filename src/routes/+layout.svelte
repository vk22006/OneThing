<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { theme } from '$lib/stores/theme';
	import { onMount } from 'svelte';
	import { LazyStore } from '@tauri-apps/plugin-store';
	import { notificationSettings, loadSettings } from '$lib/stores/notifications';
	import { NotificationManager } from '$lib/notifications/notificationManager';
	import AppShell from '$lib/layout/AppShell.svelte';

	let { children } = $props();
	let currentTheme = $state<'light' | 'dark'>('light');

	$effect(() => {
		// Apply theme on client only; keep it mutually exclusive.
		if (typeof document === 'undefined') return;
		// 1) Drive CSS variables via <html data-theme="..."> (reliable across WebViews)
		document.documentElement.dataset.theme = currentTheme;

		// 2) Keep body classes too (backwards compatibility for any selectors)
		document.body.classList.remove('light', 'dark');
		document.body.classList.add(currentTheme);
	});

	function localDateKey(d: Date): string {
		// Use local time to avoid "yesterday/tomorrow" issues around midnight.
		const yyyy = d.getFullYear();
		const mm = String(d.getMonth() + 1).padStart(2, '0');
		const dd = String(d.getDate()).padStart(2, '0');
		return `${yyyy}-${mm}-${dd}`;
	}

	onMount(() => {
		const unsubTheme = theme.subscribe((value) => {
			currentTheme = value;
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
			const store = new LazyStore('settings.json');
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
					: 'Review your deadlines for today. Open your tasks to check what\'s coming up.'
			);

			await store.set('lastDeadlineReminderDate', today);
			await store.save();
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

<AppShell>
	{@render children()}
</AppShell>



