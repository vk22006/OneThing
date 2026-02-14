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

    let projects = $state<Project[]>([]);

    onMount(() => {
        const stored = localStorage.getItem("project_info");

        if (stored) {
            projects = JSON.parse(stored);
        }
    });

    let totalProjects = $derived(projects.length);
    let totalTasks = $derived(
        projects.reduce(
            (sum, p) => sum + p.tasks.length, 0
        ));
    let completedTasks = $derived(
        projects.reduce(
            (sum, p) =>
                sum + p.tasks.filter(
                    t => t.status === "DONE"
                ).length,
            0
        ));
    let overallProgress =$derived(
        totalTasks === 0
            ? 0
            : Math.round((completedTasks / totalTasks) * 100));


    


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
    
    <div class="grid grid-cols-2 gap-4 p-6">

    <div class="p-4 border rounded">
        <h3>Total Projects</h3>
        <p class="text-2xl">{totalProjects}</p>
    </div>

    <div class="p-4 border rounded">
        <h3>Total Tasks</h3>
        <p class="text-2xl">{totalTasks}</p>
    </div>

    <div class="p-4 border rounded">
        <h3>Completed Tasks</h3>
        <p class="text-2xl">{completedTasks}</p>
    </div>

    <div class="p-4 border rounded">
        <h3>Overall Progress</h3>
        <p class="text-2xl">{overallProgress}%</p>
    </div>

    </div>

    <div class="p-6">
        <h3 class="mb-2">Overall Completion</h3>

        <progress
            max="100"
            value={overallProgress}
            class="w-full h-4">
        </progress>

        <p>{overallProgress}% completed</p>
    </div>

    {#each projects as project}

    {@const done =
        project.tasks.filter(
            t => t.status === "DONE"
        ).length}

    {@const percent =
        project.tasks.length === 0
            ? 0
            : Math.round((done / project.tasks.length)*100)}

    <div class="p-4 border rounded mb-3">

    <h4>{project.title}</h4>

    <progress max="100" value={percent} class="w-full"></progress>

    <p>{percent}% complete</p>

    </div>

    {/each}




    </main>

</div>


<!-- CSS Styling -->
<style>
    table {
        margin: 0 auto;
    }
</style>

