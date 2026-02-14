<script lang="ts">
    import Header from '../../Header.svelte';
    import Sidebar from '../../sidebar.svelte';
    import { theme, type Theme } from "$lib/stores/theme";

    let page = $state('Settings');

    function setTheme(t: Theme) {
        theme.set(t);
    }
</script>

<div class="flex">

  <Sidebar title="OneThing">
    <button class="block hover:bg-gray-700 p-2 rounded"><a href="/">Tasks</a></button>
    <button class="block hover:bg-gray-700 p-2 rounded"><a href="/Components/ProjectInfo">Project Info</a></button>
    <button class="block hover:bg-gray-700 p-2 rounded"><a href="/Components/Progress">Progress</a></button>
    <button class="block hover:bg-gray-700 p-2 rounded"><a href="/Components/Settings">Settings</a></button>
  </Sidebar>

  <main class="flex-1 p-6">
    
    <Header {page} />
    
    <h2 class="text-xl mb-4">Theme Settings</h2>

    <div class="flex gap-4">

    <button
        onclick={() => setTheme("light")}
        class="p-3 border rounded"
        class:selected={$theme === "light"}>
        Light
    </button>

    <button
        onclick={() => setTheme("dark")}
        class="p-3 border rounded"
        class:selected={$theme === "dark"}>
        Dark
    </button>

    </div>


    </main>

</div>


<!-- CSS Styling -->
<style>
    .selected {
    border: 2px solid cyan;
    }

</style>
