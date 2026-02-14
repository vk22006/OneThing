<script lang="ts">
    import Header from '../../Header.svelte';
    import Sidebar from '../../sidebar.svelte';
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    let page = $state('Progress Dashboard');

    type Status = 'TODO' | 'DONE';

    type Task = {
        id: number;
        task: string;
        status: Status;
    };

    type Project = {
        id: number;
        title: string;
        deadline: string;
        tasks: Task[];
    };

    let projects: Project[] = [];

    onMount(() => {
        const stored = localStorage.getItem("project_info");

        if (stored) {
            projects = JSON.parse(stored);
        }
    });


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
    

    </main>

</div>


<!-- CSS Styling -->
<style>
    table {
        margin: 0 auto;
    }
</style>

