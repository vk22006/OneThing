<script lang="ts">
    import Header from '../../Header.svelte';
    import Sidebar from '../../sidebar.svelte';
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    let page = $state('Project List');

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

    // type User = {
    //     id: number;
    //     project: string;
    //     deadline: string;
    // };
    let projects = $state<Project[]>([]);
    let newProject = $state("");
    let newDeadline = $state("");

    onMount(() => {
        const stored = localStorage.getItem("project_info");

        if (stored) {
        projects.push(...JSON.parse(stored));
        }
    });

    $effect(() => {
        localStorage.setItem("project_info", JSON.stringify(projects));
    });

    function addProjDetails(title: string, deadline: string) {
        if (!title.trim()) return;

        projects.push({
            id: Date.now(),
            title,
            deadline,
            tasks: []
        });

        newProject = "";
        newDeadline = "";
    }

    function removeTask(id: number) {
        if (!confirm("Delete this task?")) return;
        const index = projects.findIndex(u => u.id === id);

        if (index !== -1) {
            projects.splice(index, 1);
        }
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
    <div class="flex justify-center items-center gap-3 mt-4 p-5">

    <input
        bind:value={newProject}
        placeholder="Project Details"
        class="w-full max-w-md p-2 border border-gray-400 rounded-md focus:outline-none focus:ring-2 focus:ring-cyan-400"
    />
    <input
        bind:value={newDeadline}
        placeholder="Deadline (in DD-MM-YYYY)"
        class="w-full max-w-md p-2 border border-gray-400 rounded-md focus:outline-none focus:ring-2 focus:ring-cyan-400"
    />

    <button
        onclick={() => addProjDetails(newProject,newDeadline)}
        class="p-2 px-4 bg-cyan-500 hover:bg-cyan-700 rounded-lg text-white">
        Add Project info
    </button>


    </div>


    <!-- TODO list -->
    <table class="m-4 p-4 table-auto w-xl">
    <thead>
    <tr>
        <th class="p-2 border border-gray-300">S.no</th>
        <th class="p-2 border border-gray-300">Project</th>
        <th class="p-2 border border-gray-300">Deadline</th>
        <th class="p-2 border border-gray-300">Delete</th>
    </tr>
    </thead>
    <tbody>
        {#each projects as project_,index (project_.id)}
        <tr in:fly={{ y: 20, duration: 250 }} out:fade>
            <td class="p-2 border border-gray-300">{index +1}</td>
            <td class="p-2 border border-gray-300">{project_.title}</td>
            <td class="p-2 border border-gray-300">{project_.deadline}</td>
            <td class="p-2 border border-gray-300">
                <button onclick={() => removeTask(project_.id)} 
                class="p-2 bg-red-500 hover:bg-red-700 text-white rounded-full"> Delete </button>
            </td>
        </tr>
        {/each}
    </tbody>
    </table>

    </main>

</div>


<!-- CSS Styling -->
<style>
    table {
        margin: 0 auto;
    }
</style>

