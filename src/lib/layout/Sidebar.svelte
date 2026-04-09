<script lang="ts">
	let title = 'OneThing';

	const navItems = [
		{ name: 'Tasks', href: '/', shortcut: '⌥1' },
		{ name: 'Project Info', href: '/Components/ProjectInfo', shortcut: '⌥2' },
		{ name: 'Progress', href: '/Components/Progress', shortcut: '⌥3' },
		{ name: 'Settings', href: '/Components/Settings', shortcut: '⌥4' }
	];

	const globalShortcuts = [
		{ keys: 'Ctrl+N  or  /', label: 'New task' },
		{ keys: 'Ctrl+,', label: 'Settings' },
	];

	const listShortcuts = [
		{ keys: '↑ ↓  or  j k', label: 'Navigate' },
		{ keys: 'Space', label: 'Toggle done' },
		{ keys: 'Del', label: 'Delete task' },
		{ keys: 'Esc', label: 'Blur input' },
	];
</script>

<nav
	class="flex h-full w-64 shrink-0 flex-col border-r border-[var(--border)] bg-[var(--sidebar-bg)] px-4 py-6 selection:bg-transparent"
>
	<div class="mb-8 flex items-center px-2">
		<img
			src="/assets/app_img/logo_full.png"
			alt="OneThing Logo"
			class="h-14 w-auto object-contain"
		/>
	</div>

	<div class="flex flex-col gap-1">
		<div
			class="mt-4 mb-2 px-2 text-[11px] font-semibold tracking-wider text-[var(--muted)] uppercase"
		>
			Workspace
		</div>
		{#each navItems as item (item.name)}
			<a
				href={item.href}
				class="flex items-center justify-between rounded-md px-3 py-2 text-[13px] font-medium text-[var(--muted)] transition-colors duration-150 ease-in-out hover:bg-[var(--nav-hover-bg)] hover:text-[var(--text)] active:scale-[0.98]"
			>
				<span>{item.name}</span>
				<kbd class="rounded bg-[var(--surface-2)] px-1.5 py-0.5 text-[10px] font-mono text-[var(--muted-2)]">{item.shortcut}</kbd>
			</a>
		{/each}
	</div>

	<!-- Keyboard shortcut legend -->
	<div class="mt-auto pt-4 border-t border-[var(--border)]">
		<div class="mb-2 px-2 text-[11px] font-semibold tracking-wider text-[var(--muted)] uppercase">Shortcuts</div>
		<div class="flex flex-col gap-1 px-2">
			{#each [...globalShortcuts, ...listShortcuts] as s}
				<div class="flex items-center justify-between">
					<span class="text-[10px] text-[var(--muted)]">{s.label}</span>
					<kbd class="rounded bg-[var(--surface-2)] px-1.5 py-0.5 text-[10px] font-mono text-[var(--muted-2)] whitespace-nowrap">{s.keys}</kbd>
				</div>
			{/each}
		</div>
	</div>
</nav>
