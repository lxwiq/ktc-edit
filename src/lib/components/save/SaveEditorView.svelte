<script lang="ts">
  import UpdateBanner from "$lib/components/UpdateBanner.svelte";
  import SaveHeader from "./SaveHeader.svelte";
  import InspectorTab from "./inspector/InspectorTab.svelte";
  import ResourcesTab from "./ResourcesTab.svelte";
  import NavigationTab from "./NavigationTab.svelte";
  import CombatTab from "./CombatTab.svelte";
  import ConstructionTab from "./ConstructionTab.svelte";
  import RecruitmentTab from "./RecruitmentTab.svelte";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;

  $effect(() => {
    editor.clampSelection();
  });

  function handleKeydown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "s") {
      event.preventDefault();
      if (editor.canSave) editor.save();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<SaveHeader />
<div class="mx-auto flex w-full max-w-6xl flex-1 flex-col gap-6 p-4 md:p-6">
  <UpdateBanner />
  {#if editor.activeTab === "inspector"}
    <InspectorTab />
  {:else if editor.activeTab === "resources"}
    <ResourcesTab />
  {:else if editor.activeTab === "navigation"}
    <NavigationTab />
  {:else if editor.activeTab === "combat"}
    <CombatTab />
  {:else if editor.activeTab === "construction"}
    <ConstructionTab />
  {:else if editor.activeTab === "recruitment"}
    <RecruitmentTab />
  {/if}
</div>
