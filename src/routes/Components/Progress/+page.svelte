<script lang="ts">
    import Header from '../../../lib/layout/Header.svelte';
    import { onMount } from "svelte";

    let pageTitle = $state('Progress Dashboard');

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
    let totalTasks = $derived(projects.reduce((sum, p) => sum + p.tasks.length, 0));
    let completedTasks = $derived(projects.reduce((sum, p) => sum + p.tasks.filter(t => t.status === "DONE").length, 0));
    let overallProgress = $derived(totalTasks === 0 ? 0 : Math.round((completedTasks / totalTasks) * 100));
</script>

<Header page={pageTitle} />

<div class="px-8 py-8 max-w-5xl mx-auto w-full">
    <!-- Overview Cards -->
    <div class="grid grid-cols-4 gap-4 mb-10">
        <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-5 hover:shadow-md transition-shadow">
            <p class="text-[13px] font-bold text-gray-400 uppercase tracking-wide mb-1.5">Projects</p>
            <p class="text-3xl font-bold text-gray-900">{totalProjects}</p>
        </div>
        <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-5 hover:shadow-md transition-shadow">
            <p class="text-[13px] font-bold text-gray-400 uppercase tracking-wide mb-1.5">Tasks</p>
            <p class="text-3xl font-bold text-gray-900">{totalTasks}</p>
        </div>
        <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-5 hover:shadow-md transition-shadow">
            <p class="text-[13px] font-bold text-emerald-500 uppercase tracking-wide mb-1.5">Completed</p>
            <p class="text-3xl font-bold text-emerald-600">{completedTasks}</p>
        </div>
        <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-5 hover:shadow-md transition-shadow">
            <p class="text-[13px] font-bold text-blue-500 uppercase tracking-wide mb-1.5">Progress</p>
            <p class="text-3xl font-bold text-blue-600">{overallProgress}%</p>
        </div>
    </div>

    <!-- Overall Completion bar -->
    <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-6 mb-10">
        <div class="flex justify-between items-end mb-3.5">
            <h3 class="text-[15px] font-bold text-gray-900 tracking-tight">Overall Completion</h3>
            <span class="text-[14px] font-bold {overallProgress === 100 ? 'text-emerald-500' : 'text-blue-600'}">{overallProgress}%</span>
        </div>
        <div class="w-full h-3 bg-gray-100/80 rounded-full overflow-hidden">
            <div class="h-full {overallProgress === 100 ? 'bg-emerald-500' : 'bg-blue-600'} rounded-full transition-all duration-700 ease-out" style="width: {overallProgress}%"></div>
        </div>
    </div>

    <h2 class="text-lg font-bold text-gray-900 tracking-tight mb-5">Project Breakdown</h2>

    <!-- Project-wise progress -->
    <div class="grid grid-cols-2 gap-4">
        {#each projects as project}
            {@const done = project.tasks.filter(t => t.status === "DONE").length}
            {@const percent = project.tasks.length === 0 ? 0 : Math.round((done / project.tasks.length)*100)}
            
            <div class="bg-white border border-gray-200 rounded-[14px] shadow-sm p-5 hover:shadow-md hover:border-gray-300 transition-all cursor-default">
                <div class="flex justify-between mb-3.5">
                    <h4 class="font-semibold text-gray-900 translate-y-[-1px]">{project.title}</h4>
                    <span class="text-[13px] font-bold {percent === 100 ? 'text-emerald-500' : 'text-gray-500'}">{percent}%</span>
                </div>
                <div class="w-full h-2 bg-gray-100 rounded-full overflow-hidden mb-2.5">
                    <div class="h-full {percent === 100 ? 'bg-emerald-500' : 'bg-blue-500'} rounded-full transition-all duration-500 ease-out" style="width: {percent}%"></div>
                </div>
                <p class="text-[12px] font-medium text-gray-400">{done} of {project.tasks.length} tasks completed</p>
            </div>
        {/each}
        {#if projects.length === 0}
            <div class="col-span-2 text-center py-12 border-2 border-dashed border-gray-200 rounded-2xl bg-gray-50/50">
                <p class="text-[14px] font-medium text-gray-500">No projects added yet.</p>
            </div>
        {/if}
    </div>
</div>
