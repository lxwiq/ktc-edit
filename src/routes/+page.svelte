<script lang="ts">
  import AppShell from "$lib/components/app/AppShell.svelte";
  import WelcomeScreen from "$lib/components/welcome/WelcomeScreen.svelte";
  import SaveEditorView from "$lib/components/save/SaveEditorView.svelte";
  import AssetEditorView from "$lib/components/assets/AssetEditorView.svelte";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import { assetEditor } from "$lib/state/asset-editor.svelte";
</script>

{#if assetEditor.active || saveEditor.data}
  <AppShell>
    {#if assetEditor.active}
      <AssetEditorView />
    {:else}
      <SaveEditorView />
    {/if}
  </AppShell>
{:else}
  <WelcomeScreen />
{/if}
