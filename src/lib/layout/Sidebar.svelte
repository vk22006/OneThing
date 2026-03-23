<script lang="ts">
  let title = "OneThing";
  
  const navItems = [
    { name: "Tasks", href: "/" },
    { name: "Project Info", href: "/Components/ProjectInfo" },
    { name: "Progress", href: "/Components/Progress" },
    { name: "Settings", href: "/Components/Settings" }
  ];
</script>

<nav class="w-64 h-full bg-[#F9F9FB] border-r border-gray-200 flex flex-col py-6 px-4 shrink-0 selection:bg-transparent">
  <div class="mb-8 px-2 flex items-center gap-3">
    <div class="w-7 h-7 rounded-md bg-blue-600 shadow-sm flex items-center justify-center">
      <span class="text-white font-bold text-sm leading-none">O</span>
    </div>
    <h2 class="text-[15px] font-semibold text-gray-900 tracking-tight">{title}</h2>
  </div>

  <div class="flex flex-col gap-1">
    <div class="text-[11px] font-semibold text-gray-400 uppercase tracking-wider mb-2 px-2 mt-4">Workspace</div>
    {#each navItems as item (item.name)}
      <a 
        href={item.href} 
        class="flex items-center px-3 py-2 text-[13px] font-medium rounded-md transition-colors duration-150 ease-in-out text-gray-600 hover:bg-black/5 hover:text-gray-900 active:scale-[0.98]"
      >
        {item.name}
      </a>
    {/each}
  </div>
</nav>
