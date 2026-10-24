<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import { Separator } from "$lib/components/ui/separator";

  let { title, actions }: { title: Snippet; actions?: Snippet } = $props();
</script>

<header class="bg-background/90 sticky top-0 z-10 flex h-14 shrink-0 items-center gap-2 border-b px-4 backdrop-blur">
  <Sidebar.Trigger class="-ml-1" />
  <Separator orientation="vertical" class="mr-2 data-[orientation=vertical]:h-4" />
  <div class="min-w-0 flex-1">
    {@render title()}
  </div>
  {#if actions}
    <div class="flex shrink-0 items-center gap-2">
      {@render actions()}
    </div>
  {/if}
</header>
