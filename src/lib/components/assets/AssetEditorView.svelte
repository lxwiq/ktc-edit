<script lang="ts" module>
  // Kept outside the component so the open section survives leaving the editor.
  const view = $state({ tab: "mounts" });
</script>

<script lang="ts">
  import * as Alert from "$lib/components/ui/alert";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import * as Empty from "$lib/components/ui/empty";
  import * as Field from "$lib/components/ui/field";
  import * as Tabs from "$lib/components/ui/tabs";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { Separator } from "$lib/components/ui/separator";
  import { Spinner } from "$lib/components/ui/spinner";
  import PageHeader from "$lib/components/app/PageHeader.svelte";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import StatusAlert from "$lib/components/common/StatusAlert.svelte";
  import UpdateBanner from "$lib/components/UpdateBanner.svelte";
  import AssetCard from "./AssetCard.svelte";
  import AssetField from "./AssetField.svelte";
  import {
    BloodMoonIcon,
    BowIcon,
    CampIcon,
    CoinBagIcon,
    CoinIcon,
    GriffinIcon,
    HammerIcon,
    HorseIcon,
    IronWallIcon,
    KnightHelmIcon,
    LizardIcon,
    WarhorseIcon,
  } from "$lib/components/icons";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import CheckIcon from "@lucide/svelte/icons/check";
  import CpuIcon from "@lucide/svelte/icons/cpu";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import HourglassIcon from "@lucide/svelte/icons/hourglass";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import UndoIcon from "@lucide/svelte/icons/undo-2";
  import UserIcon from "@lucide/svelte/icons/user";
  import WalletIcon from "@lucide/svelte/icons/wallet";
  import { assetEditor, type AssetBooleanKey } from "$lib/state/asset-editor.svelte";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const assets = assetEditor;
  const t = $derived(saveEditor.t);

  const invulnerability: { key: AssetBooleanKey; label: string }[] = [
    { key: "archerInvulnerable", label: "Archer — invulnerable + always attackable" },
    { key: "builderInvulnerable", label: "Builder — invulnerable + always attackable" },
    { key: "farmerInvulnerable", label: "Farmer — invulnerable + always attackable" },
  ];
</script>

<PageHeader>
  {#snippet title()}
    <Breadcrumb.Root>
      <Breadcrumb.List>
        <Breadcrumb.Item>
          <Breadcrumb.Page>{t.nav.gameAssets}</Breadcrumb.Page>
        </Breadcrumb.Item>
      </Breadcrumb.List>
    </Breadcrumb.Root>
  {/snippet}

  {#snippet actions()}
    {#if assets.busy}
      <Badge variant="outline" class="hidden md:inline-flex">
        <Spinner data-icon="inline-start" />
        Processing…
      </Badge>
    {/if}
    <Button variant="ghost" onclick={() => assets.close()}>
      <ArrowLeftIcon data-icon="inline-start" />
      Back
    </Button>
    <Button variant="outline" aria-label="Select Game Folder" onclick={() => assets.open(true)} disabled={assets.busy}>
      <FolderOpenIcon data-icon="inline-start" />
      <span class="hidden lg:inline">Select Game Folder</span>
    </Button>
    <Button variant="outline" aria-label="Reload" onclick={() => assets.open(false)} disabled={assets.busy}>
      <RotateCwIcon data-icon="inline-start" />
      <span class="hidden lg:inline">Reload</span>
    </Button>
    <Button variant="outline" aria-label="Restore" onclick={() => assets.restore()} disabled={assets.busy || assets.backups.length !== 2}>
      <UndoIcon data-icon="inline-start" />
      <span class="hidden lg:inline">Restore</span>
    </Button>
    <Button onclick={() => assets.apply()} disabled={assets.busy || !assets.settings}>
      <CheckIcon data-icon="inline-start" />
      Apply
    </Button>
  {/snippet}
</PageHeader>

<div class="mx-auto flex w-full max-w-6xl flex-1 flex-col gap-6 p-4 md:p-6">
  <UpdateBanner />

  <PageIntro title="Game Assets" description="Edit mounts, units, and bag items directly in Kingdom Two Crowns.">
    {#if assets.directory}
      <Badge variant="outline" class="max-w-full font-mono">
        <span class="truncate">Data: {assets.directory}</span>
      </Badge>
    {/if}
  </PageIntro>

  <StatusAlert status={assets.status} error={assets.error} />

  {#if assets.settings}
    {@const settings = assets.settings}
    <Tabs.Root bind:value={view.tab} class="gap-6">
      <Tabs.List class="w-full justify-start overflow-x-auto">
        <Tabs.Trigger value="mounts"><HorseIcon />Mounts</Tabs.Trigger>
        <Tabs.Trigger value="units"><KnightHelmIcon />Units</Tabs.Trigger>
        <Tabs.Trigger value="kingdom"><IronWallIcon />Wallets & walls</Tabs.Trigger>
        <Tabs.Trigger value="patches"><CpuIcon />Apple Silicon patches</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="mounts">
        <div class="grid items-start gap-6 md:grid-cols-2 xl:grid-cols-3">
          <AssetCard title="Griffin" icon={GriffinIcon}>
            <AssetField key="griffinRunSpeed" label="Run speed" step="0.1" />
            <AssetField key="griffinForestMultiplier" label="Forest multiplier" step="0.1" />
            <AssetField key="griffinRunStaminaRate" label="Run stamina rate" step="0.01" />
            <AssetField key="griffinSkillStaminaCost" label="Skill stamina cost" step="0.01" />
            <AssetField key="griffinPushWidth" label="Push range width" min="0" step="0.1" />
            <AssetField key="griffinPushHeight" label="Push range height" min="0" step="0.1" />
          </AssetCard>

          <AssetCard title="Regular Horse" icon={HorseIcon}>
            <AssetField key="horseRunSpeed" label="Run speed" step="0.1" />
            <AssetField key="horseRunStaminaRate" label="Run stamina rate" step="0.01" />
          </AssetCard>

          <AssetCard title="Lizard" icon={LizardIcon}>
            <AssetField key="lizardRunSpeed" label="Run speed" step="0.1" />
            <AssetField key="lizardRunStaminaRate" label="Run stamina rate" step="0.01" />
            <AssetField key="lizardSkillStaminaCost" label="Skill stamina cost" min="0" step="0.01" />
          </AssetCard>

          <AssetCard
            title="Warhorse"
            icon={WarhorseIcon}
            note="Skill stamina cost and buff range also apply to Warhorse Plague. The collider scan limit is under Apple Silicon patches."
          >
            <AssetField key="warhorseRunSpeed" label="Run speed" step="0.1" />
            <AssetField key="warhorseRunStaminaRate" label="Run stamina rate" step="0.01" />
            <AssetField key="warhorseSkillStaminaCost" label="Skill stamina cost" step="0.01" />
            <AssetField key="warhorseCooldown" label="Skill cooldown (seconds)" step="0.1" />
            <AssetField key="warhorseBuffDuration" label="Buff duration (seconds)" min="0" max="1000000" step="0.1" />
            <AssetField key="warhorseBuffRange" label="Buff range" step="0.1" />
          </AssetCard>

          <AssetCard title="Warhorse Plague" icon={WarhorseIcon} note="Applied to Warhorse Plague P1 and P2 only.">
            <AssetField key="warhorsePlagueRunSpeed" label="Run speed" min="0" step="0.1" />
            <AssetField key="warhorsePlagueRunStaminaRate" label="Run stamina rate" step="0.01" />
            <AssetField key="warhorsePlagueCooldown" label="Skill cooldown (seconds)" step="0.1" />
          </AssetCard>
        </div>
      </Tabs.Content>

      <Tabs.Content value="units">
        <div class="grid items-start gap-6 md:grid-cols-2 xl:grid-cols-3">
          <AssetCard title="Archer" icon={BowIcon} note="Applied to regular, Norse Lands, and Knight squad archers.">
            <AssetField key="archerWalkSpeed" label="Walk speed" min="0" step="0.05" />
            <AssetField key="archerRunSpeed" label="Run speed" min="0" step="0.05" />
            <AssetField key="archerShootPrepTime" label="Shoot preparation (seconds)" min="0" step="0.05" />
            <AssetField key="archerShootCooldownTime" label="Normal cooldown (seconds)" min="0" step="0.05" />
            <AssetField key="archerShootCooldownWithKnightTime" label="Knight squad cooldown (seconds)" min="0" step="0.05" />
            <AssetField key="archerIntervalMin" label="Normal interval minimum (seconds)" min="0" step="0.05" />
            <AssetField key="archerIntervalMax" label="Normal interval maximum (seconds)" min="0" step="0.05" />
            <AssetField key="archerFormationIntervalMin" label="Formation interval minimum (seconds)" min="0" step="0.05" />
            <AssetField key="archerFormationIntervalMax" label="Formation interval maximum (seconds)" min="0" step="0.05" />
          </AssetCard>

          <AssetCard
            title="Builder"
            icon={HammerIcon}
            note="Applied to regular and Norse Lands builders. Lower work time means faster building and chopping."
          >
            <AssetField key="builderWalkSpeed" label="Walk speed" min="0" step="0.05" />
            <AssetField key="builderRunSpeed" label="Run speed" min="0" step="0.05" />
            <AssetField key="builderWorkTime" label="Work time (seconds)" min="0" step="0.05" />
          </AssetCard>

          <AssetCard
            title="Peasant"
            icon={UserIcon}
            note="Edits walkSpeed and runSpeed on the Peasant prefab. Other unit prefabs are unchanged. Disabled fields are not recognized in this game build and are left untouched."
          >
            <AssetField key="peasantWalkSpeed" label="Walk speed" min="0" step="0.05" />
            <AssetField key="peasantRunSpeed" label="Run speed" min="0" step="0.05" />
          </AssetCard>

          <AssetCard title="Knight" icon={KnightHelmIcon} note="Applied to regular, Norse Lands, and Greece Knights. Squires are unchanged.">
            <AssetField key="knightWalkSpeed" label="Walk speed" min="0" step="0.05" />
            <AssetField key="knightRunSpeed" label="Run speed" min="0" step="0.05" />
            <AssetField key="knightRetreatSpeed" label="Retreat speed" min="0" step="0.05" />
            <Field.Separator>Wallet</Field.Separator>
            <AssetField key="knightCoinDropProbability" label="Coin drop probability (0–1)" min="0" max="1" step="0.05" />
            <Field.Description>
              Character.coinDropProbability: 0 means no coin loss through this damage path; 0.4 means 40% per coin-drop check. This is not full invulnerability.
            </Field.Description>
            <AssetField key="knightWalletCapacity" label="Coin capacity" min="1" max="1000000" step="1" />
            <AssetField key="knightWalletPayTaxesAbove" label="Pay taxes above" min="0" max="1000000" step="1" />
          </AssetCard>

          <div class="md:col-span-2">
            <AssetCard title="Unit damage protection" icon={ShieldIcon}>
              <Field.Set>
                <Field.Legend class="sr-only">Unit damage protection</Field.Legend>
                <Field.Group class="gap-3">
                  {#each invulnerability as entry (entry.key)}
                    <Field.Field orientation="horizontal">
                      <Checkbox
                        id={`asset-${entry.key}`}
                        bind:checked={
                          () => settings[entry.key] === true,
                          (checked) => assets.setBoolean(entry.key, checked)
                        }
                        bind:indeterminate={
                          () => settings[entry.key] === null,
                          () => {}
                        }
                      />
                      <Field.Label for={`asset-${entry.key}`} class="font-normal">{entry.label}</Field.Label>
                    </Field.Field>
                  {/each}
                </Field.Group>
              </Field.Set>
              <Field.Description>
                Apply sets both Damageable.isInvulnerableInitially and alwaysAttackable: checked enables both, unchecked disables both. Applies to regular and Norse Lands prefabs, including Knight squad archers. Mixed checkboxes preserve differing flags or variants until changed. Does not change HP or Pikemen. Always attackable keeps units eligible as targets; scripts can still change runtime invulnerability. Gameplay against all enemy types is unverified.
              </Field.Description>
              <Field.Separator>Existing units in save</Field.Separator>
              <Field.Description>
                After applying assets, reset saved Damageable overrides so existing units can use prefab defaults on reload. This also resets their saved HP. Targets all Archers, Builders and Farmers across Campaigns and Challenges; walls, enemies, Knights and Pikemen are untouched.
              </Field.Description>
              <p class="text-sm">
                <span class="text-muted-foreground">Target:</span>
                <span class="font-mono break-all">{saveEditor.filePath ?? 'global-v35 in the default game save folder (detected automatically)'}</span>
              </p>
              <div>
                <Button variant="outline" onclick={() => assets.resetUnitDamageable()} disabled={assets.busy || saveEditor.isLoading}>
                  Reset &amp; save Damageable — Archer, Builder, Farmer
                </Button>
              </div>
              <Field.Description>
                Close the game first. Writes directly with an automatic backup and verification; no need to open global-v35 or click Save. If a save is open, uses that file and preserves unrelated unsaved editor changes without writing them. Asset Apply/Restore does not undo this save edit.
              </Field.Description>
              <StatusAlert status={assets.unitResetStatus} error={assets.unitResetError} />
            </AssetCard>
          </div>
        </div>
      </Tabs.Content>

      <Tabs.Content value="kingdom">
        <div class="grid items-start gap-6 md:grid-cols-2 xl:grid-cols-3">
          <AssetCard
            title="Player Wallet"
            icon={WalletIcon}
            note="Edits Wallet.TotalCapacity on the Player prefab in resources.assets. Current save and runtime state may affect when the change takes effect."
          >
            <AssetField key="playerWalletCapacity" label="Total capacity" min="1" max="1000000" step="1" />
          </AssetCard>

          <AssetCard title="Iron Wall (Wall5)" icon={IronWallIcon} note="Applies to normal and horn variants. Wall Statue multipliers are unchanged.">
            <AssetField key="wall5StandardHitPoints" label="Base HP — regular, Greece, Bamboo" min="1" max="1000000" step="1" />
            <AssetField key="wall5NorselandsHitPoints" label="Base HP — Norse Lands" min="1" max="1000000" step="1" />
            <Field.Field orientation="horizontal">
              <Checkbox id="asset-reset-save-walls" bind:checked={assets.resetSaveWalls} />
              <Field.Label for="asset-reset-save-walls" class="font-normal">
                Reset all existing walls in save (removes Damageable overrides so all walls use this new HP)
              </Field.Label>
            </Field.Field>
          </AssetCard>

          <AssetCard title="Bag" icon={CoinBagIcon} note="This value is applied to coins, gems, and biome variants.">
            <AssetField key="bagScale" label="Coin and gem scale" step="0.05" />
          </AssetCard>
        </div>
      </Tabs.Content>

      <Tabs.Content value="patches">
        <div class="grid items-start gap-6 md:grid-cols-2">
          <AssetCard
            title="Warhorse collider limit"
            icon={WarhorseIcon}
            badge="Apple Silicon"
            note={assets.warhorseColliderLimit !== null
              ? "Separate from asset Apply. Patches and re-signs GameAssembly.dylib on Apple Silicon Macs; close the game first. This limits colliders scanned, not the exact number of soldiers buffed."
              : "Collider scan limit is unavailable for this game build."}
          >
            {#if assets.warhorseColliderLimit !== null}
              <NumberField
                label="Collider scan limit"
                min="1"
                max="65535"
                step="1"
                bind:value={assets.warhorseColliderLimit}
              />
            {/if}
            {#snippet footer()}
              {#if assets.warhorseColliderLimit !== null}
                <div class="flex flex-wrap gap-2">
                  <Button variant="outline" onclick={() => assets.applyWarhorseColliderLimit()} disabled={assets.busy}>Apply collider limit</Button>
                  <Button variant="ghost" onclick={() => assets.restoreWarhorseColliderLimit()} disabled={assets.busy || !assets.warhorseColliderBackup}>Restore collider limit</Button>
                </div>
              {/if}
              <StatusAlert status={assets.warhorseColliderStatus} error={assets.warhorseColliderError} />
            {/snippet}
          </AssetCard>

          <AssetCard
            title="Coin bag display"
            icon={CoinIcon}
            badge="Apple Silicon"
            description={`Current: ${assets.coinVisualCurrent === null ? 'Original display (no visual cap)' : `${assets.coinVisualCurrent} coin objects maximum`}`}
            note="Separate from asset Apply. Patches and re-signs the selected game's GameAssembly.dylib on supported Apple Silicon builds. Close the game first. Wallet balance and gems are unchanged. Ground coins are not capped. Backups are created automatically."
          >
            <NumberField label="Maximum visible coins" min="1" max="65535" step="1" emptyValue={null} bind:value={assets.coinVisualLimit} />
            {#snippet footer()}
              <div class="flex flex-wrap gap-2">
                <Button variant="outline" onclick={() => assets.applyCoinVisualLimit()} disabled={assets.busy || !assets.coinVisualAvailable}>Apply coin display limit</Button>
                <Button variant="ghost" onclick={() => assets.applyCoinVisualLimit(true)} disabled={assets.busy || !assets.coinVisualAvailable || assets.coinVisualCurrent === null}>Restore original coin display</Button>
              </div>
              <StatusAlert status={assets.coinVisualStatus} error={assets.coinVisualError} />
            {/snippet}
          </AssetCard>

          <AssetCard
            title="Clock speed"
            icon={HourglassIcon}
            badge="Apple Silicon"
            description={`Current: ${assets.clockAvailable && assets.clockCurrent !== null ? `×${assets.clockCurrent} in ResumeTime` : 'Unavailable for this game build or platform'}`}
            note="Apple Silicon only; close the game first. Patches ResumeTime and writes all Campaign/Challenge island clockSpeedModifier values in the open save (or default global-v35), with backups and verification. Does not change unit movement or animation speed. PauseTime still uses zero. Other code such as AdjustTime can still change the multiplier; this is not a permanent override of every clock writer. Reset writes ×1, not the previous save snapshot. Already-running games require restart. Runtime behavior unverified."
          >
            <NumberField
              label="Day/night clock multiplier"
              min="0"
              step="0.5"
              emptyValue={null}
              disabled={assets.busy || !assets.clockAvailable}
              bind:value={assets.clockMultiplier}
            />
            {#snippet footer()}
              <div class="flex flex-wrap gap-2">
                <Button variant="outline" onclick={() => assets.applyClockSpeed(assets.clockMultiplier)} disabled={assets.busy || !assets.clockAvailable}>Apply clock speed to dylib + save</Button>
                <Button variant="ghost" onclick={() => assets.applyClockSpeed(1)} disabled={assets.busy || !assets.clockAvailable || assets.clockCurrent === 1}>Reset clock speed to ×1</Button>
              </div>
              <StatusAlert status={assets.clockStatus} error={assets.clockError} />
            {/snippet}
          </AssetCard>

          <AssetCard
            title="Vagrant camps"
            icon={CampIcon}
            badge="Apple Silicon"
            description={`Current: ${!assets.campAvailable ? 'Unavailable for this game build' : assets.campNeedsUpgrade ? 'Old patch — upgrade required to detach camps from the forest' : assets.campEnabled ? 'Preserved and detached after deforestation' : 'Original camp removal'}`}
            note="Separate from asset Apply. Patches and re-signs GameAssembly.dylib on supported Apple Silicon builds; close the game first. Forest boundaries still shrink normally. Restore keeps other patches. Backups are automatic. Multiplayer is unverified."
          >
            {#snippet footer()}
              <div class="flex flex-wrap gap-2">
                <Button variant="outline" onclick={() => assets.applyCampPreservation(true)} disabled={assets.busy || !assets.campAvailable || (assets.campEnabled && !assets.campNeedsUpgrade)}>
                  {assets.campNeedsUpgrade ? 'Upgrade camp preservation' : 'Preserve vagrant camps'}
                </Button>
                <Button variant="ghost" onclick={() => assets.applyCampPreservation(false)} disabled={assets.busy || !assets.campAvailable || !assets.campEnabled}>Restore camp removal</Button>
              </div>
              <StatusAlert status={assets.campStatus} error={assets.campError} />
            {/snippet}
          </AssetCard>

          <div class="md:col-span-2">
            <AssetCard
              title="Blood Moon"
              icon={BloodMoonIcon}
              badge="Experimental"
              description={`Current: ${!assets.bloodMoonAvailable ? 'Unavailable for this game build' : assets.bloodMoonEnabled ? 'Every day classified as bossDay' : 'Original day schedule'}`}
              note="Close the game first. Patches the selected game's dylib to schedule ten Boss events per side every day (20 total), including new campaigns and future schedules. Failed Boss spawns no longer clear Red Moon while enemies are attacking or retaliation is queued; normal danger/safety cleanup remains intact. Waves may overlap and increase CPU/memory load. Also converts pending saved Regular waves and fills pending Boss schedules to ten per side/day in the open global-v35 (or default save); existing excess events are not removed. Reapplying adds nothing once each side has ten. Writes with backups and verification. Retaliation and saved weather/challenge events are not edited. Already-spawned waves are unchanged. Restore removes the 76-byte repeat cave, 28-byte clear guard, both hooks and day/wave/side patches, not converted save events. Runtime and multiplayer behavior remain unverified."
            >
              {#snippet footer()}
                <div class="flex flex-wrap gap-2">
                  <Button variant="outline" onclick={() => assets.applyBloodMoon(true)} disabled={assets.busy || !assets.bloodMoonAvailable}>Enable 20 Boss events/day (10 per side)</Button>
                  <Button variant="ghost" onclick={() => assets.applyBloodMoon(false)} disabled={assets.busy || !assets.bloodMoonAvailable || !assets.bloodMoonEnabled}>Restore original day schedule</Button>
                </div>
                <StatusAlert status={assets.bloodMoonStatus} error={assets.bloodMoonError} />
              {/snippet}
            </AssetCard>
          </div>
        </div>
      </Tabs.Content>
    </Tabs.Root>

    <Separator />

    <Alert.Root>
      <ShieldCheckIcon />
      <Alert.Title>Safety</Alert.Title>
      <Alert.Description class="flex flex-col gap-2">
        <p>The app writes only when both files match the verified game profile. Two backups are created before every change.</p>
        {#if assets.backups.length}
          <ul class="flex flex-col gap-1">
            {#each assets.backups as path (path)}
              <li><code class="font-mono text-xs break-all">{path}</code></li>
            {/each}
          </ul>
        {/if}
      </Alert.Description>
    </Alert.Root>
  {:else if !assets.busy}
    <Empty.Root class="border">
      <Empty.Header>
        <Empty.Media variant="icon">
          <FolderOpenIcon />
        </Empty.Media>
        <Empty.Title>Game not found</Empty.Title>
        <Empty.Description>
          Kingdom Two Crowns was not found in your Steam libraries. Select the game folder (the one that contains <code>KingdomTwoCrowns.app</code> or <code>KingdomTwoCrowns_Data</code>).
        </Empty.Description>
      </Empty.Header>
      <Empty.Content>
        <Button onclick={() => assets.open(true)}>
          <FolderOpenIcon data-icon="inline-start" />
          Select Game Folder
        </Button>
      </Empty.Content>
    </Empty.Root>
  {:else}
    <Empty.Root class="border">
      <Empty.Header>
        <Empty.Media variant="icon">
          <Spinner />
        </Empty.Media>
        <Empty.Title>Processing…</Empty.Title>
      </Empty.Header>
    </Empty.Root>
  {/if}
</div>
