<script lang="ts">
    import Header from '../../../lib/layout/Header.svelte';
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    let pageTitle = $state('Project List');

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
        notes: string;
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
            tasks: [],
            notes: ""
        });

        newProject = "";
        newDeadline = "";
    }

    function removeProject(id: number) {
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

<Header page={pageTitle} />

<div class="px-8 py-8 max-w-5xl mx-auto w-full">
    <!-- Add Project Form -->
    <div class="bg-[var(--surface)] border border-[var(--border)] rounded-[14px] shadow-sm p-5 mb-8 flex gap-3">
        <div class="flex-1 relative">
            <input
                bind:value={newProject}
                placeholder="Project Title"
                class="block w-full px-4 py-3 bg-[var(--surface-2)] border border-[var(--border)] rounded-xl text-sm placeholder-[var(--muted-2)] focus:outline-none focus:ring-[3px] focus:ring-blue-100 focus:border-blue-500 focus:bg-[var(--surface)] transition-all shadow-sm text-[var(--text)]"
            />
        </div>
        <div class="w-48 relative">
            <input
                bind:value={newDeadline}
                placeholder="Deadline (DD-MM-YYYY)"
                class="block w-full px-4 py-3 bg-[var(--surface-2)] border border-[var(--border)] rounded-xl text-sm placeholder-[var(--muted-2)] focus:outline-none focus:ring-[3px] focus:ring-blue-100 focus:border-blue-500 focus:bg-[var(--surface)] transition-all shadow-sm text-[var(--text)]"
            />
        </div>
        <button
            onclick={() => addProjDetails(newProject,newDeadline)}
            class="px-5 py-3 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white font-medium rounded-xl transition-colors shadow-sm whitespace-nowrap">
            Add Project
        </button>
    </div>

    <!-- Project list -->
    <div class="flex flex-col gap-6">
        {#each projects as project (project.id)}
            <div 
                in:fly={{ y: 20, duration: 300 }}
                class="bg-[var(--surface)] border border-[var(--border)] rounded-[14px] shadow-sm overflow-hidden"
            >
                <!-- Project Header -->
                <div class="px-6 py-5 border-b border-[var(--border-2)] bg-[var(--surface-3)] flex justify-between items-start">
                    <div>
                        <h2 class="text-lg font-bold text-[var(--text)] tracking-tight mb-1.5">{project.title}</h2>
                        <div class="flex items-center gap-1.5 text-[13px] text-[var(--muted)] font-medium">
                            <svg class="w-4 h-4 text-[var(--muted-2)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                            Deadline: <span class="text-[var(--text)]">{project.deadline || 'No deadline'}</span>
                        </div>
                    </div>
                    
                    <button 
                        onclick={()=>removeProject(project.id)} 
                        class="p-2 text-[var(--muted-2)] hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors cursor-pointer"
                        title="Delete Project"
                        aria-label="Delete Project"
                    >
                        <svg class="w-[18px] h-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                    </button>
                </div>

                <div class="p-6">
                    <!-- Progress bar -->
                    <div class="mb-5">
                        <div class="flex justify-between items-end mb-2.5">
                            <span class="text-[12px] font-bold text-[var(--muted-2)] uppercase tracking-wider">Progress</span>
                            <span class="text-sm font-bold {projectProgress(project) === 100 ? 'text-emerald-500' : 'text-blue-600'}">{projectProgress(project)}%</span>
                        </div>
                        <div class="w-full h-2 bg-[var(--surface-2)] rounded-full overflow-hidden">
                            <div class="h-full {projectProgress(project) === 100 ? 'bg-emerald-500' : 'bg-blue-600'} rounded-full transition-all duration-500 ease-out" style="width: {projectProgress(project)}%"></div>
                        </div>
                    </div>

                    <!-- Tasks List -->
                    <div class="flex flex-col gap-2 mb-6">
                        <h3 class="text-[12px] font-bold text-[var(--muted-2)] uppercase tracking-wider mb-1">Tasks</h3>
                        {#each project.tasks as task (task.id)}
                            <div class="group flex items-center gap-3 p-3 bg-[var(--surface-3)] border border-[var(--border-2)] rounded-[10px] hover:border-[var(--border)] transition-colors">
                                <button 
                                    onclick={() => toggleTask(project.id, task.id)}
                                    class="flex-shrink-0 w-5 h-5 rounded-[6px] border-[1.5px] flex items-center justify-center transition-colors {task.status === 'DONE' ? 'bg-blue-600 border-blue-600' : 'border-[var(--border)] hover:border-blue-500'}"
                                    aria-label="Toggle task status"
                                >
                                    {#if task.status === 'DONE'}
                                        <svg class="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
                                    {/if}
                                </button>
                                <span class="flex-1 text-[13.5px] {task.status === 'DONE' ? 'text-[var(--muted-2)] line-through' : 'text-[var(--text)] font-medium'} select-none">
                                    {task.task}
                                </span>
                                <button 
                                    onclick={() => deleteTask(project.id, task.id)}
                                    class="flex-shrink-0 p-1.5 text-[var(--muted-2)] hover:text-red-500 hover:bg-red-50 rounded-[6px] opacity-0 group-hover:opacity-100 focus:opacity-100 transition-all"
                                    aria-label="Delete task"
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                                </button>
                            </div>
                        {/each}

                        <!-- Add task input -->
                        <div class="relative mt-1.5">
                            <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none">
                                <svg class="w-[18px] h-[18px] text-[var(--muted-2)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
                            </div>
                            <input
                                placeholder="Add a new task..."
                                class="w-full pl-10 pr-4 py-2.5 bg-[var(--surface)] border border-[var(--border)] rounded-[10px] text-[14px] placeholder-[var(--muted-2)] focus:outline-none focus:ring-[3px] focus:ring-blue-100 focus:border-blue-500 shadow-sm transition-all text-[var(--text)]"
                                onkeydown={(e)=>{
                                    if(e.key==="Enter") {
                                        addTask(project.id, (e.target as HTMLInputElement).value);
                                        (e.target as HTMLInputElement).value = "";
                                    }
                                }}
                            />
                        </div>
                    </div>

                    <!-- Notes Section -->
                    <div class="mt-4 pt-4 border-t border-[var(--border-2)]">
                        <h3 class="text-[12px] font-bold text-[var(--muted-2)] uppercase tracking-wider mb-2.5 flex items-center gap-1.5">
                            <svg class="w-3.5 h-3.5 text-[var(--muted-2)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
                            Notes
                        </h3>
                        <textarea
                            bind:value={project.notes}
                            placeholder="Add project notes, links, or details here..."
                            class="w-full bg-[var(--surface-3)] border border-[var(--border-2)] rounded-[10px] px-3.5 py-3 text-[13.5px] text-[var(--text)] placeholder-[var(--muted-2)] focus:outline-none focus:ring-[3px] focus:ring-blue-100 focus:border-blue-200 focus:bg-[var(--surface)] transition-all resize-none min-h-[100px]"
                        ></textarea>
                    </div>
                </div>
            </div>
        {/each}
        
        {#if projects.length === 0}
            <div class="text-center py-16 border-2 border-dashed border-[var(--border)] rounded-[14px] bg-[var(--surface-3)]">
                <div class="w-12 h-12 rounded-full bg-blue-100 flex items-center justify-center mx-auto mb-4">
                    <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                </div>
                <h3 class="text-[15px] font-semibold text-[var(--text)]">No Projects</h3>
                <p class="text-[13px] text-[var(--muted)] mt-1">Create a new project above to organize your tasks.</p>
            </div>
        {/if}
    </div>
</div>

