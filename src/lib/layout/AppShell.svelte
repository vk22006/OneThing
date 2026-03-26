<script lang="ts">
	import Sidebar from './Sidebar.svelte';

	let { children } = $props();
</script>

<div class="flex h-screen w-screen overflow-hidden bg-[var(--app-bg)] text-[var(--text)] font-sans antialiased">
	<Sidebar />

	<main class="flex-1 flex flex-col h-full overflow-hidden bg-[var(--surface)] shadow-[-10px_0_15px_-3px_rgba(0,0,0,0.10)] rounded-tl-2xl border-l border-[var(--border)] z-10 transition-all duration-300">
		<div class="flex-1 overflow-y-auto w-full">
			<div class="max-w-[1200px] mx-auto w-full">
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