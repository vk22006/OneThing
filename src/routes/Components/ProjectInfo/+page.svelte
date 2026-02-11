<script lang="ts">
    import Header from '../../Header.svelte';
    import Sidebar from '../../sidebar.svelte';
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    let page = $state('Project List');

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

