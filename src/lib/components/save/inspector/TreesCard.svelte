<script lang="ts" module>
  type TreeFilter = "all" | "standing" | "marked" | "danger";

  // Kept outside the component so the range and filter survive tab switches.
  const form = $state<{ status: TreeFilter; min: number; max: number }>({ status: "all", min: -50, max: 50 });
</script>

<script lang="ts">
  import * as Alert from "$lib/components/ui/alert";
  import * as Card from "$lib/components/ui/card";
  import * as Item from "$lib/components/ui/item";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Toggle } from "$lib/components/ui/toggle";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import { AxeIcon, CampIcon, TreeIcon } from "$lib/components/icons";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
  import { setTreeMark, type TreeInfo } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  let { trees }: { trees: TreeInfo[] } = $props();

  const editor = saveEditor;
  const t = $derived(editor.t);

  const visibleTrees = $derived(trees.filter((tr) => {
    if (form.status === 'standing' && tr.marked) return false;
    if (form.status === 'marked' && !tr.marked) return false;
    if (form.status === 'danger' && !tr.isNearBeggarCamp) return false;
    return true;
  }));

  function handleToggleTreeMark(treeId: string, currentMarked: boolean) {
    editor.edit(
      (doc) => setTreeMark(doc, { ...editor.islandTarget, treeId, marked: !currentMarked }),
      t.status.treeMarkUpdated
    );
  }

  function handleBatchTreeMark(marked: boolean) {
    const minVal = Math.min(form.min, form.max);
    const maxVal = Math.max(form.min, form.max);
    editor.edit(
      (doc) => setTreeMark(doc, { ...editor.islandTarget, xMin: minVal, xMax: maxVal, marked }),
      t.status.treeMarkUpdated
    );
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><TreeIcon class="size-5" />{t.inspector.trees}</Card.Title>
    <Card.Description>
      {trees.length} · {trees.filter((tr) => tr.marked).length} {t.inspector.markedToCut}
    </Card.Description>
  </Card.Header>
  <Card.Content class="flex flex-col gap-4">
    <div class="flex flex-wrap items-end gap-3">
      <NumberField label={t.inspector.rangeFrom} class="w-32" bind:value={form.min} />
      <NumberField label={t.inspector.rangeTo} class="w-32" bind:value={form.max} />
      <Button variant="outline" onclick={() => handleBatchTreeMark(true)}>
        <AxeIcon data-icon="inline-start" />
        {t.inspector.markRange}
      </Button>
      <Button variant="ghost" onclick={() => handleBatchTreeMark(false)}>
        <TreeIcon data-icon="inline-start" />
        {t.inspector.unmarkRange}
      </Button>
    </div>

    <Alert.Root>
      <CampIcon />
      <Alert.Description>{t.inspector.nearBeggarCampWarn}</Alert.Description>
    </Alert.Root>

    <ToggleGroup.Root
      type="single"
      variant="outline"
      size="sm"
      class="flex-wrap"
      aria-label={t.inspector.trees}
      bind:value={() => form.status, (next) => { if (next) form.status = next as TreeFilter; }}
    >
      <ToggleGroup.Item value="all">{t.inspector.all} ({trees.length})</ToggleGroup.Item>
      <ToggleGroup.Item value="standing">
        <TreeIcon data-icon="inline-start" />
        {t.inspector.standing} ({trees.filter((tr) => !tr.marked).length})
      </ToggleGroup.Item>
      <ToggleGroup.Item value="marked">
        <AxeIcon data-icon="inline-start" />
        {t.inspector.markedToCut} ({trees.filter((tr) => tr.marked).length})
      </ToggleGroup.Item>
      <ToggleGroup.Item value="danger">
        <TriangleAlertIcon data-icon="inline-start" />
        {t.inspector.nearCamps} ({trees.filter((tr) => tr.isNearBeggarCamp).length})
      </ToggleGroup.Item>
    </ToggleGroup.Root>

    <Item.Group class="grid grid-cols-[repeat(auto-fill,minmax(13rem,1fr))] gap-2">
      {#each visibleTrees as tree (tree.id)}
        <Item.Root variant="outline" size="sm">
          <Item.Media variant="icon">
            {#if tree.marked}<AxeIcon />{:else}<TreeIcon />{/if}
          </Item.Media>
          <Item.Content>
            <Item.Title class="font-mono">X = {tree.x}</Item.Title>
            <Item.Description>
              {#if tree.isNearBeggarCamp}
                <Badge variant="destructive">
                  <TriangleAlertIcon data-icon="inline-start" />
                  {t.inspector.campDistance.replace("{distance}", String(tree.nearCampDist))}
                </Badge>
              {:else}
                {tree.distanceToCastle}m
              {/if}
            </Item.Description>
          </Item.Content>
          <Item.Actions>
            <Toggle
              variant="outline"
              size="sm"
              aria-label={tree.marked ? t.inspector.unmarkTree : t.inspector.markTree}
              title={tree.marked ? t.inspector.unmarkTree : t.inspector.markTree}
              bind:pressed={() => tree.marked, () => handleToggleTreeMark(tree.id, tree.marked)}
            >
              <AxeIcon />
            </Toggle>
          </Item.Actions>
        </Item.Root>
      {/each}
    </Item.Group>
  </Card.Content>
</Card.Root>
