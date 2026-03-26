<script lang="ts">
    let { page } = $props();
</script>

<header class="flex items-center justify-between px-8 py-5 border-b border-[var(--border)] bg-[var(--header-bg)] backdrop-blur-md sticky top-0 z-20">
    <h1 class="text-[17px] font-semibold text-[var(--text)] tracking-tight">{page}</h1>
</header>