<script>
  export let title;
</script>


<nav class="sidebar">
  <h2 class="text-xl font-bold mb-6">{title}</h2>

  <slot />
</nav>

<style>
  .sidebar {
    width: 220px;
    background: #0b1a2b;
    color: white;
    padding: 1rem;
    flex-shrink: 0;
  }
/* 
  a {
    display: block;
    margin: 1rem 0;
    color: white;
    text-decoration: none;
  } */
</style>
