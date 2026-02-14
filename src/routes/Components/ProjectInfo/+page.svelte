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

    function removeProject(id: number) {
        if (!confirm("Delete this project?")) return;
        const index = projects.findIndex(u => u.id === id);

        if (index !== -1) {
            projects.splice(index, 1);
        }
    }

    function addTask(projectId: number, task: string) {
        const project = projects.find(p => p.id === projectId);
        if (!project || !task.trim()) return;

        project.tasks.push({
            id: Date.now(),
            task,
            status: "TODO"
        });
    }

    function toggleTask(projectId: number, taskId: number) {
        const project = projects.find(p => p.id === projectId);
        const task = project?.tasks.find(t => t.id === taskId);

        if (task) {
            task.status = task.status === "TODO" ? "DONE" : "TODO";
        }
    }

    function deleteTask(projectId: number, taskId: number) {
        const project = projects.find(p => p.id === projectId);
        if (!project) return;

        const index = project.tasks.findIndex(t => t.id === taskId);
        if (index !== -1) project.tasks.splice(index, 1);
    }


    function projectProgress(project: Project) {
        if (project.tasks.length === 0) return 0;

        const done = project.tasks.filter(
            t => t.status === "DONE"
        ).length;

        return Math.round((done / project.tasks.length) * 100);
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


    <!-- Project list -->

    {#each projects as project (project.id)}

    <div class="m-6 p-4 border rounded-lg">

        <!-- Project Info -->
        <div class="flex justify-between">
            <div>
                <h2 class="text-xl font-bold">{project.title}</h2>
                <p class="mb-3">Deadline: {project.deadline}</p>
            </div>

            <div>
                <button onclick={()=>removeProject(project.id)} class="p-2 px-4 bg-red-500 hover:bg-red-700 rounded-full">
                    Delete Project
                </button>
            </div>
            
        </div>

        <!-- Add task -->
        <input
            placeholder="New task"
            class="border p-2 mb-4"
            onkeydown={(e)=>{
                if(e.key==="Enter") {
                    addTask(
                        project.id,
                        (e.target as HTMLInputElement).value
                    );
                    (e.target as HTMLInputElement).value = "";
                }
            }}
        />

        <!-- Progress bar -->
        <div>
            <h3>{project.title}</h3>
            <progress
                max="100"
                value={projectProgress(project)}>
            </progress>

            <span>{projectProgress(project)}%</span>
        </div>
        

        <!-- Tasks Table -->
        <table class="table-auto w-full border">
            <thead>
                <tr>
                    <th class="border p-2">Task</th>
                    <th class="border p-2">Status</th>
                    <th class="border p-2">Toggle</th>
                    <th class="border p-2">Delete</th>
                </tr>
            </thead>

            <tbody>
                {#each project.tasks as task (task.id)}
                <tr>
                    <td class="border p-2">{task.task}</td>
                    <td class="border p-2">{task.status}</td>

                    <td class="border p-2">
                        <button onclick={() =>
                            toggleTask(project.id, task.id)} class="p-2 px-4 bg-cyan-500 hover:bg-cyan-700 rounded-full text-white">
                            Toggle
                        </button>
                    </td>

                    <td class="border p-2">
                        <button onclick={() =>
                            deleteTask(project.id, task.id)} class="text-red-500 underline">
                            Delete
                        </button>
                    </td>
                </tr>
                {/each}
            </tbody>
        </table>

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

