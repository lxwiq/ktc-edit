<script lang="ts">
  import type { Component, Snippet } from "svelte";
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import { Badge } from "$lib/components/ui/badge";

  let {
    title,
    icon: Icon,
    description,
    badge,
    note,
    children,
    footer,
  }: {
    title: string;
    icon?: Component<{ class?: string }>;
    description?: string;
    badge?: string;
    /** Explanatory text shown under the fields. */
    note?: string;
    children?: Snippet;
    footer?: Snippet;
  } = $props();
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2">
      {#if Icon}<Icon class="size-5" />{/if}
      {title}
    </Card.Title>
    {#if description}
      <Card.Description>{description}</Card.Description>
    {/if}
    {#if badge}
      <Card.Action>
        <Badge variant="secondary">{badge}</Badge>
      </Card.Action>
    {/if}
  </Card.Header>
  <Card.Content class="flex flex-col gap-4">
    {#if children}
      <Field.Group class="gap-4">
        {@render children()}
      </Field.Group>
    {/if}
    {#if note}
      <p class="text-muted-foreground text-xs leading-relaxed">{note}</p>
    {/if}
  </Card.Content>
  {#if footer}
    <Card.Footer class="flex-col items-stretch gap-3">
      {@render footer()}
    </Card.Footer>
  {/if}
</Card.Root>
