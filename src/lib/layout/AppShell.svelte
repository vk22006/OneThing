<script lang="ts">
	import sidebar from './Sidebar.svelte';

	let { children } = $props();
</script>

<div class="app-shell">
	<sidebar />

	<main class="workspace">
		{@render children()}
	</main>
</div>

<style>
	.app-shell {
		display: flex;
		height: 100vh;
		overflow: hidden;
	}

	.workspace {
		flex: 1;
		overflow-y: auto;
		padding: 2rem;
		background: #f7f8fa;
	}
</style>