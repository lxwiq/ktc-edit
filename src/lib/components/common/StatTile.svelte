<script lang="ts">
  import type { Component, Snippet } from "svelte";

  let {
    label,
    value,
    hint,
    icon: Icon,
    children,
  }: {
    label: string;
    value: string | number;
    hint?: string;
    icon?: Component<{ class?: string }>;
    children?: Snippet;
  } = $props();
</script>

<div class="bg-muted/40 flex flex-col gap-1 rounded-lg border p-4">
  <div class="text-muted-foreground flex items-center gap-2 text-xs font-medium tracking-wide uppercase">
    {#if Icon}<Icon class="size-4" />{/if}
    <span class="truncate">{label}</span>
  </div>
  <div class="font-heading text-2xl font-semibold tabular-nums">{value}</div>
  {#if hint}
    <div class="text-muted-foreground truncate text-xs">{hint}</div>
  {/if}
  {@render children?.()}
</div>
