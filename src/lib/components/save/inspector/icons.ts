import type { Component } from "svelte";
import type { SpecialPointInfo, TowerType, WallInfo } from "$lib/saveEditActions";
import {
  AnchorIcon,
  ArcherTowerIcon,
  BakeryIcon,
  BallistaIcon,
  BoatIcon,
  BowIcon,
  CampIcon,
  HammerIcon,
  HornIcon,
  IronWallIcon,
  KnightHelmIcon,
  MoundIcon,
  PickaxeIcon,
  PortalIcon,
  ScytheIcon,
  StatueIcon,
  WallIcon,
} from "$lib/components/icons";
import MapPinIcon from "@lucide/svelte/icons/map-pin";
import SwordsIcon from "@lucide/svelte/icons/swords";

export function wallIcon(wall: Pick<WallInfo, "level" | "hasHorn">): Component {
  if (wall.level === 5 && wall.hasHorn) return HornIcon;
  if (wall.level === 5) return IronWallIcon;
  if (wall.level > 0) return WallIcon;
  return MoundIcon;
}

export function towerIcon(type: TowerType): Component {
  switch (type) {
    case "tower0":
      return MoundIcon;
    case "ballista":
      return BallistaIcon;
    case "baker":
      return BakeryIcon;
    case "knight":
      return KnightHelmIcon;
    default:
      return ArcherTowerIcon;
  }
}

export function specialPointIcon(type: SpecialPointInfo["type"]): Component {
  switch (type) {
    case "beggar_camp":
      return CampIcon;
    case "portal":
      return PortalIcon;
    case "statue":
      return StatueIcon;
    case "mine":
      return PickaxeIcon;
    case "wreck":
      return AnchorIcon;
    case "boat":
      return BoatIcon;
    default:
      return MapPinIcon;
  }
}

/** Deities follow the order used by the game: archer, farmer, builder, knight. */
export function deityIcon(index: number): Component {
  if (index === 0) return BowIcon;
  if (index === 1) return ScytheIcon;
  if (index === 2) return HammerIcon;
  return SwordsIcon;
}
