<script lang="ts">
  import type { Snippet } from "svelte";

  let { title, description, children }: { title: string; description?: string; children?: Snippet } = $props();
</script>

<div class="flex flex-wrap items-end justify-between gap-4">
  <div class="flex min-w-0 flex-col gap-1">
    <h1 class="font-heading text-2xl font-semibold tracking-wide text-balance">{title}</h1>
    {#if description}
      <p class="text-muted-foreground text-sm">{description}</p>
    {/if}
  </div>
  {@render children?.()}
</div>
