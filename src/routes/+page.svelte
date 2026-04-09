<script lang="ts">
	import { onMount } from 'svelte';
	import Header from '../lib/layout/Header.svelte';
	import { fly, fade } from 'svelte/transition';
	import { notificationSettings } from '$lib/stores/notifications';
	import { NotificationManager } from '$lib/notifications/notificationManager';
	import { focusScheduler } from '$lib/notifications/scheduler';
	import { focusTaskInput } from '$lib/stores/shortcuts';

	let pageTitle = $state('Daily TODO List');

	type Status = 'TODO' | 'DONE';

	type User = {
		id: number;
		task: string;
		deadline?: string; // Time string HH:mm
		status: Status;
	};
	let users = $state<User[]>([]);
	let newTask = $state('');
	let deadlineTime = $state('');

	// Keyboard navigation state
	let activeIndex = $state<number>(-1);
	let taskInputEl = $state<HTMLInputElement | null>(null);

	// Listen for the global "focus new task input" signal
	const unsubFocus = focusTaskInput.subscribe((should) => {
		if (should) {
			taskInputEl?.focus();
			focusTaskInput.set(false);
		}
	});

	function handleListKey(e: KeyboardEvent) {
		const target = e.target as HTMLElement;
		const isTyping =
			target.tagName === 'INPUT' ||
			target.tagName === 'TEXTAREA' ||
			target.isContentEditable;

		// Escape: blur the currently focused input
		if (e.key === 'Escape' && isTyping) {
			(target as HTMLInputElement).blur();
			return;
		}

		if (isTyping || users.length === 0) return;

		if (e.key === 'ArrowDown' || e.key === 'j') {
			e.preventDefault();
			activeIndex = Math.min(activeIndex + 1, users.length - 1);
			return;
		}
		if (e.key === 'ArrowUp' || e.key === 'k') {
			e.preventDefault();
			activeIndex = Math.max(activeIndex - 1, 0);
			return;
		}
		if (e.key === ' ' && activeIndex >= 0) {
			e.preventDefault();
			toggle(users[activeIndex].id);
			return;
		}
		if ((e.key === 'Delete' || e.key === 'Backspace') && activeIndex >= 0) {
			e.preventDefault();
			removeTask(users[activeIndex].id);
			activeIndex = Math.min(activeIndex, users.length - 2);
			return;
		}
	}

	onMount(() => {
		const stored = localStorage.getItem('tasks');
		if (stored) {
			users.push(...JSON.parse(stored));
		}

		// Start scheduler if enabled
		const unsubscribe = notificationSettings.subscribe((settings) => {
			if (settings.enabled) {
				focusScheduler.updateSettings(settings);
				focusScheduler.start();
			} else {
				focusScheduler.stop();
			}
		});

		return () => {
			unsubscribe();
			unsubFocus();
		};
	});

	$effect(() => {
		localStorage.setItem('tasks', JSON.stringify(users));
	});

	async function addTask(task: string) {
		if (!task.trim()) return;

		const id = Date.now();
		const newUser: User = {
			id,
			task,
			status: 'TODO'
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
					'Task Deadline Approaching!',
					`Your task "${task}" is due soon.`,
					deadline
				);
			}
		}

		users.push(newUser);
		newTask = '';
		deadlineTime = '';
	}

	function toggle(id: number) {
		const user = users.find((u) => u.id === id);
		if (user) {
			user.status = user.status === 'TODO' ? 'DONE' : 'TODO';
		}
	}

	function removeTask(id: number) {
		const index = users.findIndex((u) => u.id === id);
		if (index !== -1) {
			users.splice(index, 1);
		}
	}

	let completionPercentage = $derived(
		users.length === 0
			? 0
			: Math.round((users.filter((u) => u.status === 'DONE').length / users.length) * 100)
	);
</script>

<svelte:window onkeydown={handleListKey} />

<Header page={pageTitle} />

<div class="mx-auto w-full max-w-4xl px-8 py-8">
	<!-- Progress & Header Area -->
	<div class="mb-10 flex items-center justify-between">
		<div>
			<h2 class="mb-1.5 text-[22px] font-bold tracking-tight text-[var(--text)]">My Tasks</h2>
			<p class="text-[13px] font-medium text-[var(--muted)]">
				{users.length}
				{users.length === 1 ? 'task' : 'tasks'} total • {completionPercentage}% completed
			</p>
		</div>

		<!-- Progress Bar -->
		<div class="h-2.5 w-32 flex-shrink-0 overflow-hidden rounded-full bg-[var(--surface-2)]">
			<div
				class="h-full rounded-full bg-red-600 transition-all duration-500 ease-out"
				style="width: {completionPercentage}%"
			></div>
		</div>
	</div>

	<!-- Input Area -->
	<div
		class="mb-8 flex items-center gap-2 rounded-[24px] border border-[var(--border)] bg-[var(--surface)] p-2 shadow-sm transition-all group-focus-within:border-red-400 group-focus-within:ring-[3px] group-focus-within:ring-red-100"
	>
		<div class="pointer-events-none flex flex-shrink-0 items-center pl-4">
			<svg
				class="h-5 w-5 text-[var(--muted-2)]"
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 20 20"
				fill="currentColor"
			>
				<path
					fill-rule="evenodd"
					d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z"
					clip-rule="evenodd"
				/>
			</svg>
		</div>
		<input
			bind:value={newTask}
			bind:this={taskInputEl}
			onkeydown={(e) => {
				if (e.key === 'Enter') addTask(newTask);
			}}
			placeholder="Add a new task... (Ctrl+N or /)"
			class="flex-1 bg-transparent py-3 text-sm placeholder-gray-400 focus:outline-none"
		/>

		<div
			class="flex items-center gap-1.5 rounded-[14px] bg-[var(--surface-2)] px-2 py-1 transition-colors hover:bg-[var(--surface-3)]"
		>
			<svg class="h-4 w-4 text-[var(--muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
				></path></svg
			>
			<input
				type="time"
				bind:value={deadlineTime}
				title="Set Deadline"
				class="cursor-pointer border-none bg-transparent p-0 text-xs font-bold text-[var(--text)] focus:outline-none"
			/>
		</div>

		<button
			onclick={() => addTask(newTask)}
			class="mr-1.5 rounded-[14px] bg-red-600 p-2 text-white shadow-sm transition-colors hover:bg-red-700"
			aria-label="Add task"
		>
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
				><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"
				></path></svg
			>
		</button>
	</div>

	<!-- Task List -->
	<div class="flex flex-col gap-2.5">
		{#if users.length === 0}
			<div
				class="mt-4 rounded-2xl border-2 border-dashed border-[var(--border)] bg-[var(--surface-3)] px-4 py-14 text-center"
			>
				<div
					class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-100"
				>
					<svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						></path></svg
					>
				</div>
				<h3 class="text-sm font-semibold text-[var(--text)]">All caught up!</h3>
				<p class="mt-1.5 text-[13px] text-[var(--muted)]">Add a new task above to get started.</p>
			</div>
		{:else}
			{#each users as user, i (user.id)}
				<div
					in:fly={{ y: 15, duration: 250 }}
					out:fade={{ duration: 150 }}
					role="button"
					tabindex="0"
					onclick={() => (activeIndex = i)}
					onkeydown={(e) => { if (e.key === 'Enter') { activeIndex = i; toggle(user.id); } }}
					class="group flex items-center gap-3.5 rounded-[14px] border p-3.5 shadow-sm transition-all duration-200 hover:shadow-md
						{activeIndex === i
							? 'border-red-400 bg-[var(--surface)] ring-2 ring-red-200'
							: 'border-[var(--border)] bg-[var(--surface)]'}"
				>
					<!-- Checkbox Toggle -->
					<button
						onclick={() => toggle(user.id)}
						class="flex h-5 w-5 flex-shrink-0 items-center justify-center rounded-[6px] border-[1.5px] transition-colors {user.status ===
						'DONE'
							? 'border-red-600 bg-red-600'
							: 'border-[var(--border)] hover:border-red-500'}"
					>
						{#if user.status === 'DONE'}
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

					<!-- Task Text -->
					<div class="flex min-w-0 flex-1 flex-col justify-center">
						<span
							class="text-[14px] {user.status === 'DONE'
								? 'text-[var(--muted-2)] line-through'
								: 'font-medium text-[var(--text)]'} truncate px-1 transition-all select-none"
						>
							{user.task}
						</span>
						{#if user.deadline}
							<span
								class="mt-0.5 flex items-center gap-1 px-1 text-[10px] font-bold text-[var(--muted-2)]"
							>
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
									></path></svg
								>
								{user.deadline}
							</span>
						{/if}
					</div>

					<!-- Delete Button -->
					<button
						onclick={() => removeTask(user.id)}
						class="flex-shrink-0 cursor-pointer rounded-lg p-1.5 text-[var(--muted-2)] opacity-0 transition-all group-hover:opacity-100 hover:bg-red-50 hover:text-red-600 focus:opacity-100"
						aria-label="Delete task"
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
			{/each}
		{/if}
	</div>
</div>
