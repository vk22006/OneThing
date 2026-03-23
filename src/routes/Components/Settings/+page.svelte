<script lang="ts">
    import Header from '../../../lib/layout/Header.svelte';
    import { theme, type Theme } from "$lib/stores/theme";

    let pageTitle = $state('Settings');

    function setTheme(t: Theme) {
        theme.set(t);
    }
</script>

<Header page={pageTitle} />

<div class="px-8 py-8 max-w-4xl mx-auto w-full">
    <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-6 mb-8">
        <h2 class="text-lg font-semibold text-gray-900 mb-4 tracking-tight">Appearance</h2>
        <div class="flex gap-4">
            <button
                onclick={() => setTheme("light")}
                class="flex-1 py-3 px-4 rounded-xl border-2 transition-all font-medium { $theme === 'light' ? 'border-blue-500 bg-blue-50/50 text-blue-700' : 'border-gray-200 text-gray-600 hover:border-gray-300 hover:bg-gray-50' }">
                Light Mode
            </button>
            <button
                onclick={() => setTheme("dark")}
                class="flex-1 py-3 px-4 rounded-xl border-2 transition-all font-medium { $theme === 'dark' ? 'border-blue-500 bg-blue-50/50 text-blue-700' : 'border-gray-200 text-gray-600 hover:border-gray-300 hover:bg-gray-50' }">
                Dark Mode
            </button>
        </div>
    </div>
</div>
