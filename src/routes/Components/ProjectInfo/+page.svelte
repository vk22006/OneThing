<script lang="ts">
	import Header from '../../../lib/layout/Header.svelte';
	import { fly, fade } from 'svelte/transition';
	import { onMount } from 'svelte';

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
	let newProject = $state('');
	let newDeadline = $state('');

	onMount(() => {
		const stored = localStorage.getItem('project_info');
		if (stored) {
			projects.push(...JSON.parse(stored));
		}
	});

	$effect(() => {
		localStorage.setItem('project_info', JSON.stringify(projects));
	});

	function addProjDetails(title: string, deadline: string) {
		if (!title.trim()) return;

		projects.push({
			id: Date.now(),
			title,
			deadline,
			tasks: [],
			notes: ''
		});

		newProject = '';
		newDeadline = '';
	}

	function removeProject(id: number) {
		const index = projects.findIndex((u) => u.id === id);
		if (index !== -1) {
			projects.splice(index, 1);
		}
	}

	function addTask(projectId: number, task: string) {
		const project = projects.find((p) => p.id === projectId);
		if (!project || !task.trim()) return;

		project.tasks.push({
			id: Date.now(),
			task,
			status: 'TODO'
		});
	}

	function toggleTask(projectId: number, taskId: number) {
		const project = projects.find((p) => p.id === projectId);
		const task = project?.tasks.find((t) => t.id === taskId);

		if (task) {
			task.status = task.status === 'TODO' ? 'DONE' : 'TODO';
		}
	}

	function deleteTask(projectId: number, taskId: number) {
		const project = projects.find((p) => p.id === projectId);
		if (!project) return;

		const index = project.tasks.findIndex((t) => t.id === taskId);
		if (index !== -1) project.tasks.splice(index, 1);
	}

	function projectProgress(project: Project) {
		if (project.tasks.length === 0) return 0;

		const done = project.tasks.filter((t) => t.status === 'DONE').length;

		return Math.round((done / project.tasks.length) * 100);
	}

	function daysRemaining(deadlineStr: string) {
		if (!deadlineStr) return null;
		
		// Expected format: DD-MM-YYYY
		const parts = deadlineStr.split('-');
		if (parts.length !== 3) return null;
		
		const day = parseInt(parts[0], 10);
		const month = parseInt(parts[1], 10) - 1; // Month is 0-indexed
		const year = parseInt(parts[2], 10);
		
		if (isNaN(day) || isNaN(month) || isNaN(year)) return null;

		const deadlineDate = new Date(year, month, day);
		const today = new Date();
		today.setHours(0, 0, 0, 0);
		deadlineDate.setHours(0, 0, 0, 0);
		
		const diffMs = deadlineDate.getTime() - today.getTime();
		const diffDays = Math.ceil(diffMs / (1000 * 60 * 60 * 24));
		return diffDays;
	}
</script>

<Header page={pageTitle} />

<div class="mx-auto w-full max-w-5xl px-8 py-8">
	<!-- Add Project Form -->
	<div
		class="mb-8 flex gap-3 rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm"
	>
		<div class="relative flex-1">
			<input
				bind:value={newProject}
				placeholder="Project Title"
				class="block w-full rounded-xl border border-[var(--border)] bg-[var(--surface-2)] px-4 py-3 text-sm text-[var(--text)] placeholder-[var(--muted-2)] shadow-sm transition-all focus:border-red-500 focus:bg-[var(--surface)] focus:ring-[3px] focus:ring-red-100 focus:outline-none"
				onkeydown={(e) => {
					if (e.key === 'Enter') addProjDetails(newProject, newDeadline);
				}}
			/>
		</div>
		<div class="relative w-48">
			<input
				bind:value={newDeadline}
				placeholder="Deadline (DD-MM-YYYY)"
				class="block w-full rounded-xl border border-[var(--border)] bg-[var(--surface-2)] px-4 py-3 text-sm text-[var(--text)] placeholder-[var(--muted-2)] shadow-sm transition-all focus:border-red-500 focus:bg-[var(--surface)] focus:ring-[3px] focus:ring-red-100 focus:outline-none"
				onkeydown={(e) => {
					if (e.key === 'Enter') addProjDetails(newProject, newDeadline);
				}}
			/>
		</div>
		<button
			onclick={() => addProjDetails(newProject, newDeadline)}
			class="rounded-xl bg-red-600 px-5 py-3 font-medium whitespace-nowrap text-white shadow-sm transition-colors hover:bg-red-700 active:bg-red-800"
		>
			Add Project
		</button>
	</div>

	<!-- Project list -->
	<div class="flex flex-col gap-6">
		{#each projects as project (project.id)}
			<div
				in:fly={{ y: 20, duration: 300 }}
				class="overflow-hidden rounded-[14px] border border-[var(--border)] bg-[var(--surface)] shadow-sm"
			>
				<!-- Project Header -->
				<div
					class="flex items-start justify-between border-b border-[var(--border-2)] bg-[var(--surface-3)] px-6 py-5"
				>
					<div>
						<h2 class="mb-1.5 text-lg font-bold tracking-tight text-[var(--text)]">
							{project.title}
						</h2>
						<div class="flex items-center gap-1.5 text-[13px] font-medium text-[var(--muted)]">
							<svg
								class="h-4 w-4 text-[var(--muted-2)]"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
								></path></svg
							>
							Deadline: <span class="text-[var(--text)]">{project.deadline || 'No deadline'}</span>
						</div>
						{#if project.deadline}
							{@const remaining = daysRemaining(project.deadline)}
							{#if remaining !== null}
								<div class="mt-1 flex items-center gap-1.5 text-[13px] font-medium text-[var(--muted)]">
									<svg class="h-4 w-4 text-[var(--muted-2)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
									Days Remaining: <span class="font-bold {remaining < 0 ? 'text-red-500' : remaining <= 3 ? 'text-amber-500' : 'text-[var(--text)]'}">{remaining < 0 ? 'Overdue' : remaining}</span>
								</div>
							{/if}
						{/if}
					</div>

					<button
						onclick={() => removeProject(project.id)}
						class="cursor-pointer rounded-lg p-2 text-[var(--muted-2)] transition-colors hover:bg-red-50 hover:text-red-500"
						title="Delete Project"
						aria-label="Delete Project"
					>
						<svg class="h-[18px] w-[18px]" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
							></path></svg
						>
					</button>
				</div>

				<div class="p-6">
					<!-- Progress bar -->
					<div class="mb-5">
						<div class="mb-2.5 flex items-end justify-between">
							<span class="text-[12px] font-bold tracking-wider text-[var(--muted-2)] uppercase"
								>Progress</span
							>
							<span
								class="text-sm font-bold {projectProgress(project) === 100
									? 'text-emerald-500'
									: 'text-red-600'}">{projectProgress(project)}%</span
							>
						</div>
						<div class="h-2 w-full overflow-hidden rounded-full bg-[var(--surface-2)]">
							<div
								class="h-full {projectProgress(project) === 100
									? 'bg-emerald-500'
									: 'bg-red-600'} rounded-full transition-all duration-500 ease-out"
								style="width: {projectProgress(project)}%"
							></div>
						</div>
					</div>

					<!-- Tasks List -->
					<div class="mb-6 flex flex-col gap-2">
						<h3 class="mb-1 text-[12px] font-bold tracking-wider text-[var(--muted-2)] uppercase">
							Tasks
						</h3>
						{#each project.tasks as task (task.id)}
							<div
								class="group flex items-center gap-3 rounded-[10px] border border-[var(--border-2)] bg-[var(--surface-3)] p-3 transition-colors hover:border-[var(--border)]"
							>
								<button
									onclick={() => toggleTask(project.id, task.id)}
									class="flex h-5 w-5 flex-shrink-0 items-center justify-center rounded-[6px] border-[1.5px] transition-colors {task.status ===
									'DONE'
										? 'border-red-600 bg-red-600'
										: 'border-[var(--border)] hover:border-red-500'}"
									aria-label="Toggle task status"
								>
									{#if task.status === 'DONE'}
										<svg
											class="h-3.5 w-3.5 text-white"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
											><path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="3"
												d="M5 13l4 4L19 7"
											></path></svg
										>
									{/if}
								</button>
								<span
									class="flex-1 text-[13.5px] {task.status === 'DONE'
										? 'text-[var(--muted-2)] line-through'
										: 'font-medium text-[var(--text)]'} select-none"
								>
									{task.task}
								</span>
								<button
									onclick={() => deleteTask(project.id, task.id)}
									class="flex-shrink-0 rounded-[6px] p-1.5 text-[var(--muted-2)] opacity-0 transition-all group-hover:opacity-100 hover:bg-red-50 hover:text-red-500 focus:opacity-100"
									aria-label="Delete task"
								>
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
										><path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M6 18L18 6M6 6l12 12"
										></path></svg
									>
								</button>
							</div>
						{/each}

						<!-- Add task input -->
						<div class="relative mt-1.5">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5">
								<svg
									class="h-[18px] w-[18px] text-[var(--muted-2)]"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 4v16m8-8H4"
									></path></svg
								>
							</div>
							<input
								placeholder="Add a new task..."
								class="w-full rounded-[10px] border border-[var(--border)] bg-[var(--surface)] py-2.5 pr-4 pl-10 text-[14px] text-[var(--text)] placeholder-[var(--muted-2)] shadow-sm transition-all focus:border-red-500 focus:ring-[3px] focus:ring-red-100 focus:outline-none"
								onkeydown={(e) => {
									if (e.key === 'Enter') {
										addTask(project.id, (e.target as HTMLInputElement).value);
										(e.target as HTMLInputElement).value = '';
									}
								}}
							/>
						</div>
					</div>

					<!-- Notes Section -->
					<div class="mt-4 border-t border-[var(--border-2)] pt-4">
						<h3
							class="mb-2.5 flex items-center gap-1.5 text-[12px] font-bold tracking-wider text-[var(--muted-2)] uppercase"
						>
							<svg
								class="h-3.5 w-3.5 text-[var(--muted-2)]"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
								></path></svg
							>
							Notes
						</h3>
						<textarea
							bind:value={project.notes}
							placeholder="Add project notes, links, or details here..."
							class="min-h-[100px] w-full resize-none rounded-[10px] border border-[var(--border-2)] bg-[var(--surface-3)] px-3.5 py-3 text-[13.5px] text-[var(--text)] placeholder-[var(--muted-2)] transition-all focus:border-red-200 focus:bg-[var(--surface)] focus:ring-[3px] focus:ring-red-100 focus:outline-none"
						></textarea>
					</div>
				</div>
			</div>
		{/each}

		{#if projects.length === 0}
			<div
				class="rounded-[14px] border-2 border-dashed border-[var(--border)] bg-[var(--surface-3)] py-16 text-center"
			>
				<div
					class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-100"
				>
					<svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
						></path></svg
					>
				</div>
				<h3 class="text-[15px] font-semibold text-[var(--text)]">No Projects</h3>
				<p class="mt-1 text-[13px] text-[var(--muted)]">
					Create a new project above to organize your tasks.
				</p>
			</div>
		{/if}
	</div>
</div>
