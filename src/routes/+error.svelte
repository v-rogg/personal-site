<script>
  import { page } from '$app/stores';
</script>

<section class="container">
  <i class="bug fa-regular fa-bug"></i>
  <h1>
    {$page.status} {$page.error.name}
  </h1>
  <p>
    {$page.error.message}
  </p>
</section>

<style>
  .bug {
    font-size: 10rem;
    margin-bottom: 32px;
  }

  .container {
    margin-top: 10rem;
  }
</style>
