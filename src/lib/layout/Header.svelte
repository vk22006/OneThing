<script lang="ts">
	let { page } = $props();
</script>

<header
	class="sticky top-0 z-20 flex items-center justify-between border-b border-[var(--border)] bg-[var(--header-bg)] px-8 py-5 backdrop-blur-md"
>
	<h1 class="text-[17px] font-semibold tracking-tight text-[var(--text)]">{page}</h1>
</header>
