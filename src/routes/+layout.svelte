<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { theme } from '$lib/stores/theme';

	let { children } = $props();

	$effect(() => {
		document.body.className = $theme;
	});

</script>

<svelte:head><link rel="icon" href={favicon} />
</svelte:head>
<svelte:body />


{@render children()}


