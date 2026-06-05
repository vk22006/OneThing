<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { theme, initThemeStore } from '$lib/stores/theme';
	import { onMount } from 'svelte';
	import { loadSettings } from '$lib/stores/notifications';
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

		// Load notification settings (used by other components)
		void loadSettings();

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
