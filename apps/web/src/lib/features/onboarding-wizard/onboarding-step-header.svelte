<script lang="ts">
let { title, description }: { title: string; description: string } = $props();
</script>

<header class="space-y-2 border-b border-border pb-5">
  <h1 class="text-xl font-semibold tracking-tight text-foreground sm:text-2xl">{title}</h1>
  <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">{description}</p>
</header>
