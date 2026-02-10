<script>
  export let title;
</script>

<div class="h-screen w-60 bg-gray-900 text-white p-4">
  <h2 class="text-xl font-bold mb-6">{title}</h2>
  <slot />
</div>
