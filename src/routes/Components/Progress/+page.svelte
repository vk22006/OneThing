<script lang="ts">
	import Header from '../../../lib/layout/Header.svelte';
	import { onMount } from 'svelte';

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
		const stored = localStorage.getItem('project_info');
		if (stored) {
			projects = JSON.parse(stored);
		}
	});

	let totalProjects = $derived(projects.length);
	let totalTasks = $derived(projects.reduce((sum, p) => sum + p.tasks.length, 0));
	let completedTasks = $derived(
		projects.reduce((sum, p) => sum + p.tasks.filter((t) => t.status === 'DONE').length, 0)
	);
	let overallProgress = $derived(
		totalTasks === 0 ? 0 : Math.round((completedTasks / totalTasks) * 100)
	);
</script>

<Header page={pageTitle} />

<div class="mx-auto w-full max-w-5xl px-8 py-8">
	<!-- Overview Cards -->
	<div class="mb-10 grid grid-cols-4 gap-4">
		<div
			class="rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm transition-shadow hover:shadow-md"
		>
			<p class="mb-1.5 text-[13px] font-bold tracking-wide text-[var(--muted-2)] uppercase">
				Projects
			</p>
			<p class="text-3xl font-bold text-[var(--text)]">{totalProjects}</p>
		</div>
		<div
			class="rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm transition-shadow hover:shadow-md"
		>
			<p class="mb-1.5 text-[13px] font-bold tracking-wide text-[var(--muted-2)] uppercase">
				Tasks
			</p>
			<p class="text-3xl font-bold text-[var(--text)]">{totalTasks}</p>
		</div>
		<div
			class="rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm transition-shadow hover:shadow-md"
		>
			<p class="mb-1.5 text-[13px] font-bold tracking-wide text-emerald-500 uppercase">Completed</p>
			<p class="text-3xl font-bold text-emerald-600">{completedTasks}</p>
		</div>
		<div
			class="rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm transition-shadow hover:shadow-md"
		>
			<p class="mb-1.5 text-[13px] font-bold tracking-wide text-red-500 uppercase">Progress</p>
			<p class="text-3xl font-bold text-red-600">{overallProgress}%</p>
		</div>
	</div>

	<!-- Overall Completion bar -->
	<div class="mb-10 rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-6 shadow-sm">
		<div class="mb-3.5 flex items-end justify-between">
			<h3 class="text-[15px] font-bold tracking-tight text-[var(--text)]">Overall Completion</h3>
			<span
				class="text-[14px] font-bold {overallProgress === 100
					? 'text-emerald-500'
					: 'text-red-600'}">{overallProgress}%</span
			>
		</div>
		<div class="h-3 w-full overflow-hidden rounded-full bg-[var(--surface-2)]">
			<div
				class="h-full {overallProgress === 100
					? 'bg-emerald-500'
					: 'bg-red-600'} rounded-full transition-all duration-700 ease-out"
				style="width: {overallProgress}%"
			></div>
		</div>
	</div>

	<h2 class="mb-5 text-lg font-bold tracking-tight text-[var(--text)]">Project Breakdown</h2>

	<!-- Project-wise progress -->
	<div class="grid grid-cols-2 gap-4">
		{#each projects as project}
			{@const done = project.tasks.filter((t) => t.status === 'DONE').length}
			{@const percent =
				project.tasks.length === 0 ? 0 : Math.round((done / project.tasks.length) * 100)}

			<div
				class="cursor-default rounded-[14px] border border-[var(--border)] bg-[var(--surface)] p-5 shadow-sm transition-all hover:shadow-md"
			>
				<div class="mb-3.5 flex justify-between">
					<h4 class="translate-y-[-1px] font-semibold text-[var(--text)]">{project.title}</h4>
					<span
						class="text-[13px] font-bold {percent === 100
							? 'text-emerald-500'
							: 'text-[var(--muted)]'}">{percent}%</span
					>
				</div>
				<div class="mb-2.5 h-2 w-full overflow-hidden rounded-full bg-[var(--surface-2)]">
					<div
						class="h-full {percent === 100
							? 'bg-emerald-500'
							: 'bg-red-500'} rounded-full transition-all duration-500 ease-out"
						style="width: {percent}%"
					></div>
				</div>
				<p class="text-[12px] font-medium text-[var(--muted-2)]">
					{done} of {project.tasks.length} tasks completed
				</p>
			</div>
		{/each}
		{#if projects.length === 0}
			<div
				class="col-span-2 rounded-2xl border-2 border-dashed border-[var(--border)] bg-[var(--surface-3)] py-12 text-center"
			>
				<p class="text-[14px] font-medium text-[var(--muted)]">No projects added yet.</p>
			</div>
		{/if}
	</div>
</div>
