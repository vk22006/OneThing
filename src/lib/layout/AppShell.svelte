<script lang="ts">
	import Sidebar from './Sidebar.svelte';

	let { children } = $props();
</script>

<div
	class="flex h-screen w-screen overflow-hidden bg-[var(--app-bg)] font-sans text-[var(--text)] antialiased"
>
	<Sidebar />

	<main
		class="z-10 flex h-full flex-1 flex-col overflow-hidden rounded-tl-2xl border-l border-[var(--border)] bg-[var(--surface)] shadow-[-10px_0_15px_-3px_rgba(0,0,0,0.10)] transition-all duration-300"
	>
		<div class="w-full flex-1 overflow-y-auto">
			<div class="mx-auto w-full max-w-[1200px]">
				{@render children()}
			</div>
		</div>
	</main>
</div>

<style>
	:global(body) {
		font-family: 'Inter', sans-serif;
	}
</style>
