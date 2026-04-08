<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { onComplete } = $props<{ onComplete?: () => void }>();

	let progress = $state(0);
	let visible = $state(true);
	let fadeOut = $state(false);

	onMount(() => {
		let start = performance.now();
		const duration = 2500;

		function animate(time: number) {
			let fraction = (time - start) / duration;
			if (fraction > 1) fraction = 1;

			// Ease out quad
			progress = fraction * (2 - fraction) * 100;

			if (fraction < 1) {
				requestAnimationFrame(animate);
			} else {
				setTimeout(() => {
					fadeOut = true;
					setTimeout(async () => {
						visible = false;

						// Native Window Resize Logic
						try {
							await invoke('restore_main_window');
						} catch (err) {
							console.warn('Window resize failed:', err);
						}

						if (onComplete) onComplete();
					}, 800); // fade out duration
				}, 200); // short pause at 100%
			}
		}

		setTimeout(() => {
			requestAnimationFrame(animate);
		}, 100);
	});
</script>

{#if visible}
	<!-- Full screen blocking overlay -->
	<!-- z-[99999] ensures it's above everything -->
	<div
		class="fixed inset-0 z-[99999] flex flex-col items-center justify-end bg-black bg-cover bg-center bg-no-repeat transition-opacity duration-[800ms] ease-in-out select-none {fadeOut
			? 'opacity-0'
			: 'opacity-100'}"
		style="background-image: url('/assets/app_img/splash_screen_.png');"
	>
		<!-- Stacked elements matching exact reference layout -->
		<div class="mb-[20vh] flex w-full flex-col items-center">
			<!-- Progress Bar (300px wide, ~14px tall) -->
			<div class="h-[14px] w-[300px] overflow-hidden rounded-full bg-[#1a1a1a]">
				<!-- Progress Bar Fill (Orange to Red gradient) -->
				<div
					class="h-full rounded-full bg-gradient-to-r from-[#f97316] to-[#ef4444]"
					style="width: {Math.max(3, progress)}%; transition: width 0.05s linear;"
				></div>
			</div>

			<!-- Title text -->
			<div class="mt-6 text-[32px] leading-none font-medium tracking-normal text-white">
				OneThing
			</div>

			<!-- Version text -->
			<div class="mt-[10px] text-[14px] font-normal tracking-normal text-[#9ca3af]">
				Version 1.1.4
			</div>
		</div>
	</div>
{/if}
