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
              console.warn("Window resize failed:", err);
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
    class="fixed inset-0 z-[99999] flex flex-col items-center justify-end bg-black bg-cover bg-center bg-no-repeat transition-opacity duration-[800ms] ease-in-out select-none {fadeOut ? 'opacity-0' : 'opacity-100'}"
    style="background-image: url('/assets/app_img/splash_screen.png');"
  >
    <!-- Stacked elements matching exact reference layout -->
    <div class="flex flex-col items-center mb-[20vh] w-full">
      
      <!-- Progress Bar (300px wide, ~14px tall) -->
      <div class="w-[300px] h-[14px] bg-[#1a1a1a] rounded-full overflow-hidden">
        <!-- Progress Bar Fill (Orange to Red gradient) -->
        <div 
          class="h-full rounded-full bg-gradient-to-r from-[#f97316] to-[#ef4444]"
          style="width: {Math.max(3, progress)}%; transition: width 0.05s linear;"
        ></div>
      </div>
      
      <!-- Title text -->
      <div class="text-white text-[32px] font-medium mt-6 leading-none tracking-normal">
        OneThing
      </div>
      
      <!-- Version text -->
      <div class="text-[#9ca3af] text-[14px] mt-[10px] font-normal tracking-normal">
        Version 1.1.2
      </div>
      
    </div>
  </div>
{/if}
