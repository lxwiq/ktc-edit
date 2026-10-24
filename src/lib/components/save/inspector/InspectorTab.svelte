<script lang="ts">
  import * as Empty from "$lib/components/ui/empty";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import { IslandIcon } from "$lib/components/icons";
  import IslandMap from "./IslandMap.svelte";
  import CastleCard from "./CastleCard.svelte";
  import DeitiesCard from "./DeitiesCard.svelte";
  import WallsCard from "./WallsCard.svelte";
  import TowersCard from "./TowersCard.svelte";
  import TreesCard from "./TreesCard.svelte";
  import CatapultCard from "../CatapultCard.svelte";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);
</script>

<PageIntro title={t.inspector.title} description={t.inspector.description} />

{#if editor.islandOverview}
  {@const overview = editor.islandOverview}
  <IslandMap {overview} />
  <CastleCard {overview} />
  <DeitiesCard deities={overview.deities} />
  <WallsCard walls={overview.walls} />
  <CatapultCard />
  <TowersCard towers={overview.towers} />
  <TreesCard trees={overview.trees} />
{:else}
  <Empty.Root class="border">
    <Empty.Header>
      <Empty.Media variant="icon">
        <IslandIcon />
      </Empty.Media>
      <Empty.Title>{t.status.noFileLoaded}</Empty.Title>
    </Empty.Header>
  </Empty.Root>
{/if}
