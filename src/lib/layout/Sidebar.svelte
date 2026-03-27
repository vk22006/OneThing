<script lang="ts">
	let title = 'OneThing';

	const navItems = [
		{ name: 'Tasks', href: '/' },
		{ name: 'Project Info', href: '/Components/ProjectInfo' },
		{ name: 'Progress', href: '/Components/Progress' },
		{ name: 'Settings', href: '/Components/Settings' }
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
				class="flex items-center rounded-md px-3 py-2 text-[13px] font-medium text-[var(--muted)] transition-colors duration-150 ease-in-out hover:bg-[var(--nav-hover-bg)] hover:text-[var(--text)] active:scale-[0.98]"
			>
				{item.name}
			</a>
		{/each}
	</div>
</nav>
