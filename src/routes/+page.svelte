<script lang="ts">
    import { onMount } from "svelte";
    import Header from "../lib/layout/Header.svelte";
    import { fly, fade } from "svelte/transition";
    import { notificationSettings } from "$lib/stores/notifications";
    import { NotificationManager } from "$lib/notifications/notificationManager";
    import { focusScheduler } from "$lib/notifications/scheduler";

    let pageTitle = $state('Daily TODO List');

    type Status = 'TODO' | 'DONE';

    type User = {
        id: number;
        task: string;
        deadline?: string; // Time string HH:mm
        status: Status;
    };
    let users = $state<User[]>([]);
    let newTask = $state("");
    let deadlineTime = $state("");

    onMount(() => {
        const stored = localStorage.getItem("tasks");
        if (stored) {
            users.push(...JSON.parse(stored));
        }

        // Start scheduler if enabled
        const unsubscribe = notificationSettings.subscribe(settings => {
            if (settings.enabled) {
                focusScheduler.updateSettings(settings);
                focusScheduler.start();
            } else {
                focusScheduler.stop();
            }
        });

        return () => unsubscribe();
    });

    $effect(() => {
        localStorage.setItem("tasks", JSON.stringify(users));
    });

    async function addTask(task: string) {
        if (!task.trim()) return;

        const id = Date.now();
        const newUser: User = {
            id,
            task,
            status: "TODO"
        };

        if (deadlineTime) {
            newUser.deadline = deadlineTime;
            
            // Calculate absolute time for notification
            // Simple logic: Assume deadline is today
            const now = new Date();
            const [hours, mins] = deadlineTime.split(':').map(Number);
            const deadline = new Date(now.getFullYear(), now.getMonth(), now.getDate(), hours, mins);
            
            // If the deadline is in the future, schedule a notification
            if (deadline > now) {
                await NotificationManager.schedule(
                    id.toString(),
                    "Task Deadline Approaching!",
                    `Your task "${task}" is due soon.`,
                    deadline
                );
            }
        }

        users.push(newUser);
        newTask = "";
        deadlineTime = "";
    }

    function toggle(id: number) {
        const user = users.find(u => u.id === id);
        if (user) {
            user.status = user.status === "TODO" ? "DONE" : "TODO";
        }
    }

    function removeTask(id: number) {
        const index = users.findIndex(u => u.id === id);
        if (index !== -1) {
            users.splice(index, 1);
        }
    }

    let completionPercentage = $derived(
        users.length === 0 ? 0 : Math.round((users.filter(u => u.status === 'DONE').length / users.length) * 100)
    );
</script>

<Header page={pageTitle} />

<div class="px-8 py-8 max-w-4xl mx-auto w-full">
    <!-- Progress & Header Area -->
    <div class="mb-10 flex items-center justify-between">
        <div>
            <h2 class="text-[22px] font-bold text-gray-900 tracking-tight mb-1.5">My Tasks</h2>
            <p class="text-[13px] text-gray-500 font-medium">
                {users.length} {users.length === 1 ? 'task' : 'tasks'} total • {completionPercentage}% completed
            </p>
        </div>
        
        <!-- Progress Bar -->
        <div class="w-32 h-2.5 bg-gray-100 rounded-full overflow-hidden flex-shrink-0">
            <div 
                class="h-full bg-blue-600 rounded-full transition-all duration-500 ease-out" 
                style="width: {completionPercentage}%">
            </div>
        </div>
    </div>

    <!-- Input Area -->
    <div class="bg-white border border-gray-200 rounded-[24px] shadow-sm p-2 mb-8 flex items-center gap-2 group-focus-within:border-blue-400 group-focus-within:ring-[3px] group-focus-within:ring-blue-100 transition-all">
        <div class="flex-shrink-0 pl-4 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z" clip-rule="evenodd" />
            </svg>
        </div>
        <input
            bind:value={newTask}
            onkeydown={(e) => {
                if(e.key === "Enter") addTask(newTask);
            }}
            placeholder="Add a new task..."
            class="flex-1 py-3 text-sm placeholder-gray-400 focus:outline-none bg-transparent"
        />
        
        <div class="flex items-center gap-1.5 px-2 py-1 bg-gray-100 rounded-[14px] hover:bg-gray-200 transition-colors">
            <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            <input 
                type="time" 
                bind:value={deadlineTime}
                title="Set Deadline"
                class="bg-transparent text-xs font-bold text-gray-700 focus:outline-none border-none p-0 cursor-pointer"
            />
        </div>

        <button 
            onclick={() => addTask(newTask)}
            class="mr-1.5 p-2 bg-blue-600 text-white rounded-[14px] hover:bg-blue-700 transition-colors shadow-sm"
            aria-label="Add task"
        >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"></path></svg>
        </button>
    </div>

    <!-- Task List -->
    <div class="flex flex-col gap-2.5">
        {#if users.length === 0}
            <div class="text-center py-14 px-4 border-2 border-dashed border-gray-200 rounded-2xl bg-gray-50/50 mt-4">
                <div class="w-12 h-12 rounded-full bg-blue-100 flex items-center justify-center mx-auto mb-4">
                    <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                </div>
                <h3 class="text-sm font-semibold text-gray-900">All caught up!</h3>
                <p class="text-[13px] text-gray-500 mt-1.5">Add a new task above to get started.</p>
            </div>
        {:else}
            {#each users as user (user.id)}
                <div 
                    in:fly={{ y: 15, duration: 250 }} 
                    out:fade={{ duration: 150 }}
                    class="group flex items-center gap-3.5 p-3.5 bg-white border border-gray-200 rounded-[14px] shadow-sm hover:shadow-md hover:border-gray-300 transition-all duration-200"
                >
                    <!-- Checkbox Toggle -->
                    <button 
                        onclick={() => toggle(user.id)}
                        class="flex-shrink-0 w-5 h-5 rounded-[6px] border-[1.5px] flex items-center justify-center transition-colors {user.status === 'DONE' ? 'bg-blue-600 border-blue-600' : 'border-gray-300 hover:border-blue-500'}"
                    >
                        {#if user.status === 'DONE'}
                            <svg class="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
                        {/if}
                    </button>

                    <!-- Task Text -->
                    <div class="flex-1 flex flex-col justify-center min-w-0">
                        <span class="text-[14px] {user.status === 'DONE' ? 'text-gray-400 line-through' : 'text-gray-800 font-medium'} transition-all px-1 select-none truncate">
                            {user.task}
                        </span>
                        {#if user.deadline}
                            <span class="text-[10px] text-gray-400 font-bold px-1 mt-0.5 flex items-center gap-1">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                {user.deadline}
                            </span>
                        {/if}
                    </div>

                    <!-- Delete Button -->
                    <button 
                        onclick={() => removeTask(user.id)}
                        class="flex-shrink-0 p-1.5 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-lg opacity-0 group-hover:opacity-100 focus:opacity-100 transition-all cursor-pointer"
                        aria-label="Delete task"
                    >
                        <svg class="w-[18px] h-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>
