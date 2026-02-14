<script lang="ts">
    import Header from './Header.svelte';
    import Sidebar from './sidebar.svelte';
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    let page = $state('TODO List');

    type Status = 'TODO' | 'DONE';

    type User = {
        id: number;
        task: string;
        status: Status;
    };
    let users = $state<User[]>([]);
    let newTask = $state("");

    onMount(() => {
        const stored = localStorage.getItem("tasks");

        if (stored) {
        users.push(...JSON.parse(stored));
        }
    });

    $effect(() => {
        localStorage.setItem("tasks", JSON.stringify(users));
    });

    function addTask(task: string) {
        if (!task.trim()) return;

        users.push({
            id: Date.now(),
            task,
            status: "TODO"
        });

        newTask = "";
    }

    function toggle(id: number) {
        const user = users.find(u => u.id === id);
        if (user) {
            user.status = user.status === "TODO" ? "DONE" : "TODO";
        }
    }

    function removeTask(id: number) {
        if (!confirm("Delete this task?")) return;
        const index = users.findIndex(u => u.id === id);

        if (index !== -1) {
            users.splice(index, 1);
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
    <!-- Input for new task addition -->
    <div class="flex justify-center items-center gap-3 mt-4 p-5">

    <input
        bind:value={newTask}
        onkeydown={(e) => {
            if(e.key=="Enter") addTask(newTask);
        }}
        placeholder="New Task"
        class="w-full max-w-md p-2 border border-gray-400 rounded-md focus:outline-none focus:ring-2 focus:ring-cyan-400"
    />

    <button
        onclick={() => addTask(newTask)}
        class="p-2 px-4 bg-cyan-500 hover:bg-cyan-700 rounded-lg text-white">
        Add Task
    </button>

    </div>


    <!-- TODO list -->
    <table class="m-4 p-4 table-auto w-xl">
    <thead>
    <tr>
        <th class="p-2 border border-gray-300">S.no</th>
        <th class="p-2 border border-gray-300">Task</th>
        <th class="p-2 border border-gray-300">Status</th>
        <th class="p-2 border border-gray-300">Toggle</th>
        <th class="p-2 border border-gray-300">Delete</th>
    </tr>
    </thead>
    <tbody>
        {#each users as user,index (user.id)}
        <tr in:fly={{ y: 20, duration: 250 }} out:fade>
            <td class="p-2 border border-gray-300">{index +1}</td>
            <td class="p-2 border border-gray-300">{user.task}</td>
            <td class="p-2 border border-gray-300">{user.status}</td>
            <td class="p-2 border border-gray-300 justify-center">
                <button onclick={() => toggle(user.id)}
                class="p-2 bg-cyan-500 hover:bg-cyan-700 rounded-full justify-center"> Toggle status </button>
            </td>
            <td class="p-2 border border-gray-300">
                <button onclick={() => removeTask(user.id)} 
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
