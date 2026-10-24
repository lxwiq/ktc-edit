<script lang="ts">
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Kbd } from "$lib/components/ui/kbd";
  import { Spinner } from "$lib/components/ui/spinner";
  import PageHeader from "$lib/components/app/PageHeader.svelte";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import SaveIcon from "@lucide/svelte/icons/save";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);
  const isMac = navigator.userAgent.toLowerCase().includes("mac");

  const location = $derived(
    editor.targetMode === "campaign"
      ? [
          editor.campaigns[editor.selectedCampaign]?.label ?? t.context.campaign,
          `${t.context.island} ${editor.selectedIsland + 1}`,
        ]
      : [editor.challenges[editor.selectedChallenge]?.label ?? t.context.challenge]
  );

  let confirmOpen = $state(false);
  let pendingAction: (() => void) | null = null;

  /** Runs an action that replaces the open document, asking first when edits would be lost. */
  function guard(action: () => void) {
    if (!editor.dirty) {
      action();
      return;
    }
    pendingAction = action;
    confirmOpen = true;
  }

  function confirmDiscard() {
    confirmOpen = false;
    pendingAction?.();
    pendingAction = null;
  }
</script>

<PageHeader>
  {#snippet title()}
    <Breadcrumb.Root>
      <Breadcrumb.List class="flex-nowrap">
        {#each location as part (part)}
          <Breadcrumb.Item class="hidden min-w-0 md:inline-flex">
            <span class="truncate">{part}</span>
          </Breadcrumb.Item>
          <Breadcrumb.Separator class="hidden md:block" />
        {/each}
        <Breadcrumb.Item class="min-w-0">
          <Breadcrumb.Page class="truncate">{t.tabs[editor.activeTab]}</Breadcrumb.Page>
        </Breadcrumb.Item>
      </Breadcrumb.List>
    </Breadcrumb.Root>
  {/snippet}

  {#snippet actions()}
    {#if editor.dirty}
      <Badge variant="outline" class="hidden xl:inline-flex">
        <span class="bg-warning size-1.5 rounded-full"></span>
        {t.header.unsaved}
      </Badge>
    {/if}
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon"
            aria-label={t.header.reload}
            disabled={editor.isLoading}
            onclick={() => guard(() => editor.reloadFile())}
          >
            <RotateCwIcon />
          </Button>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content>{t.header.reload}</Tooltip.Content>
    </Tooltip.Root>
    <Button variant="outline" disabled={editor.isLoading} aria-label={t.header.changeFile} onclick={() => guard(() => editor.selectFile())}>
      <FolderOpenIcon data-icon="inline-start" />
      <span class="hidden lg:inline">{editor.isLoading ? t.welcome.loading : t.header.changeFile}</span>
    </Button>
    <Button disabled={!editor.canSave} onclick={() => editor.save()}>
      {#if editor.isLoading}
        <Spinner data-icon="inline-start" />
      {:else}
        <SaveIcon data-icon="inline-start" />
      {/if}
      {t.header.save}
      <Kbd class="hidden xl:inline-flex">{isMac ? "⌘" : "Ctrl"} S</Kbd>
    </Button>
  {/snippet}
</PageHeader>

<AlertDialog.Root bind:open={confirmOpen}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{t.header.discardTitle}</AlertDialog.Title>
      <AlertDialog.Description>{t.header.discardDescription}</AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>{t.header.cancel}</AlertDialog.Cancel>
      <AlertDialog.Action onclick={confirmDiscard}>{t.header.discardConfirm}</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
