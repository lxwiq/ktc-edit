<script lang="ts" module>
  export type SideFilterValue = "all" | "left" | "right";
</script>

<script lang="ts">
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  let {
    value = $bindable("all"),
    counts,
  }: {
    value?: SideFilterValue;
    counts: Record<SideFilterValue, number>;
  } = $props();

  const t = $derived(saveEditor.t);
</script>

<ToggleGroup.Root
  type="single"
  variant="outline"
  size="sm"
  aria-label={t.recruitment.side}
  bind:value={() => value, (next) => { if (next) value = next as SideFilterValue; }}
>
  <ToggleGroup.Item value="all">{t.inspector.all} ({counts.all})</ToggleGroup.Item>
  <ToggleGroup.Item value="left">
    <ArrowLeftIcon data-icon="inline-start" />
    {t.recruitment.sideLeft} ({counts.left})
  </ToggleGroup.Item>
  <ToggleGroup.Item value="right">
    {t.recruitment.sideRight} ({counts.right})
    <ArrowRightIcon data-icon="inline-end" />
  </ToggleGroup.Item>
</ToggleGroup.Root>
