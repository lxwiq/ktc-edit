use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use log::{debug, error, info};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod camp;
mod coin_bag;

const SAVE_FILENAME: &str = "global-v35";
const RESOURCES_ASSETS: &str = "resources.assets";
const SHARED_ASSETS: &str = "sharedassets0.assets";
const GLOBAL_MANAGERS_ASSETS: &str = "globalgamemanagers.assets";
const GAME_ASSEMBLY: &str = "GameAssembly.dylib";
// Instructions following the array-length MOVZ in BuffUnitsSteedAbility..cctor
// for the verified Apple Silicon build. Refuse to patch if this context changes.
const WARHORSE_COLLIDER_CONTEXT: [u8; 24] = [
    0xbc, 0x77, 0xf1, 0x97, 0x68, 0x02, 0x40, 0xf9, 0x08, 0x5d, 0x40, 0xf9, 0x1f, 0x29, 0x00, 0xb9,
    0x1f, 0xfd, 0x01, 0xa9, 0x00, 0xfd, 0x00, 0xa9,
];
const UPDATED_COLLIDER_CONTEXT: [u8; 24] = [
    0xd2, 0x73, 0xf1, 0x97, 0x68, 0x02, 0x40, 0xf9, 0x08, 0x5d, 0x40, 0xf9, 0x1f, 0x29, 0x00, 0xb9,
    0x1f, 0xfd, 0x01, 0xa9, 0x00, 0xfd, 0x00, 0xa9,
];
const EXPECTED_RESOURCES_SIZE: u64 = 59_975_696;
const EXPECTED_SHARED_SIZE: u64 = 59_876_160;
const UPDATED_RESOURCES_SIZE: u64 = 59_380_416;
const UPDATED_SHARED_SIZE: u64 = 59_877_328;

fn supported_asset_sizes(resources: u64, shared: u64) -> bool {
    matches!(
        (resources, shared),
        (EXPECTED_RESOURCES_SIZE, EXPECTED_SHARED_SIZE)
            | (UPDATED_RESOURCES_SIZE, UPDATED_SHARED_SIZE)
    )
}

#[derive(Clone, Copy)]
struct FloatTarget {
    file: &'static str,
    offset: usize,
}

#[derive(Clone, Copy)]
struct ComponentFloatTarget {
    file: &'static str,
    game_object: &'static str,
    relative_offset: usize,
}

const GRIFFIN_NAMES: [&str; 5] = [
    "Griffin P1",
    "Griffin Greece",
    "Griffin P2",
    "Griffin Skull P2",
    "Griffin Skull P1",
];
const GRIFFIN_RUN: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 196);
const GRIFFIN_FOREST: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 200);
const GRIFFIN_RUN_STAMINA: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 208);
const GRIFFIN_SKILL_COST: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 32);
const GRIFFIN_PUSH_WIDTH: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 40);
const GRIFFIN_PUSH_HEIGHT: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 44);
const LIZARD_NAMES: [&str; 1] = ["Lizard"];
const LIZARD_RUN: [ComponentFloatTarget; 1] =
    component_targets(RESOURCES_ASSETS, LIZARD_NAMES, 196);
const LIZARD_RUN_STAMINA: [ComponentFloatTarget; 1] =
    component_targets(RESOURCES_ASSETS, LIZARD_NAMES, 208);
const LIZARD_SKILL_COST: [ComponentFloatTarget; 1] =
    component_targets(RESOURCES_ASSETS, LIZARD_NAMES, 32);
const HORSE_RUN: [ComponentFloatTarget; 4] = [
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P1 Greece",
        relative_offset: 188,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2 Greece",
        relative_offset: 188,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2",
        relative_offset: 196,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "Horse Regular",
        relative_offset: 196,
    },
];
const HORSE_RUN_STAMINA: [ComponentFloatTarget; 4] = [
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P1 Greece",
        relative_offset: 200,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2 Greece",
        relative_offset: 200,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2",
        relative_offset: 208,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "Horse Regular",
        relative_offset: 208,
    },
];
const WARHORSE_NAMES: [&str; 2] = ["Warhorse P1", "Warhorse P2"];
const WARHORSE_RUN: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_NAMES, 196);
const WARHORSE_RUN_STAMINA: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_NAMES, 208);
const WARHORSE_PLAGUE_NAMES: [&str; 2] = ["Warhorse Plague P1", "Warhorse Plague P2"];
const WARHORSE_PLAGUE_RUN: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_PLAGUE_NAMES, 196);
const WARHORSE_PLAGUE_RUN_STAMINA: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_PLAGUE_NAMES, 208);
const WARHORSE_SKILL_NAMES: [&str; 4] = [
    "Warhorse P1",
    "Warhorse P2",
    "Warhorse Plague P1",
    "Warhorse Plague P2",
];
const WARHORSE_SKILL_COST: [ComponentFloatTarget; 4] =
    component_targets(RESOURCES_ASSETS, WARHORSE_SKILL_NAMES, 32);
const WARHORSE_COOLDOWN: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_NAMES, 52);
const WARHORSE_PLAGUE_COOLDOWN: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_PLAGUE_NAMES, 52);
const WARHORSE_RANGE: [ComponentFloatTarget; 4] =
    component_targets(RESOURCES_ASSETS, WARHORSE_SKILL_NAMES, 104);
const ARCHER_NAMES: [&str; 3] = ["Archer_norselands", "Archer_Soldier_norselands", "Archer"];
const ARCHER_WALK_SPEED: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 140);
const ARCHER_RUN_SPEED: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 144);
const ARCHER_SHOOT_PREP: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 72);
const ARCHER_SHOOT_COOLDOWN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 76);
const ARCHER_KNIGHT_COOLDOWN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 88);
const ARCHER_INTERVAL_MIN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 104);
const ARCHER_INTERVAL_MAX: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 108);
const ARCHER_FORMATION_INTERVAL_MIN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 112);
const ARCHER_FORMATION_INTERVAL_MAX: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 116);
const BUILDER_NAMES: [&str; 2] = ["Worker_norselands", "Worker"];
const BUILDER_WALK_SPEED: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 40);
const BUILDER_RUN_SPEED: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 44);
const BUILDER_WORK_TIME: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 48);
const WALL5_STANDARD_NAMES: [&str; 6] = [
    "Wall5",
    "Wall5_horn",
    "Wall5_greece",
    "Wall5_horn_greece",
    "Wall5_Bamboo",
    "Wall5_Horn_Bamboo",
];
const WALL5_NORSELANDS_NAMES: [&str; 2] = ["Wall5_norselands", "Wall5_horn_norselands"];
const WALL5_STANDARD_HP: [ComponentFloatTarget; 6] =
    component_targets(RESOURCES_ASSETS, WALL5_STANDARD_NAMES, 44);
const WALL5_NORSELANDS_HP: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WALL5_NORSELANDS_NAMES, 44);
const KNIGHT_WALLET_NAMES: [&str; 3] = ["Knight", "Knight_norselands", "Knight_greece"];
const KNIGHT_COIN_DROP_PROBABILITY: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, KNIGHT_WALLET_NAMES, 56);
const KNIGHT_WALLET_CAPACITY: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, KNIGHT_WALLET_NAMES, 268);
const PLAYER_WALLET_CAPACITY: [ComponentFloatTarget; 1] =
    component_targets(RESOURCES_ASSETS, ["Player"], 268);
const KNIGHT_WALLET_PAY_TAXES_ABOVE: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, KNIGHT_WALLET_NAMES, 40);
const BAG_SCALE: [ComponentFloatTarget; 12] = [
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 48,
    },
];

const fn component_targets<const N: usize>(
    file: &'static str,
    game_objects: [&'static str; N],
    relative_offset: usize,
) -> [ComponentFloatTarget; N] {
    let mut result = [ComponentFloatTarget {
        file,
        game_object: "",
        relative_offset,
    }; N];
    let mut index = 0;
    while index < N {
        result[index] = ComponentFloatTarget {
            file,
            game_object: game_objects[index],
            relative_offset,
        };
        index += 1;
    }
    result
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetSettings {
    griffin_run_speed: f32,
    griffin_forest_multiplier: f32,
    griffin_run_stamina_rate: f32,
    griffin_skill_stamina_cost: f32,
    griffin_push_width: f32,
    griffin_push_height: f32,
    lizard_run_speed: f32,
    lizard_run_stamina_rate: f32,
    lizard_skill_stamina_cost: f32,
    horse_run_speed: f32,
    horse_run_stamina_rate: f32,
    warhorse_run_speed: f32,
    warhorse_run_stamina_rate: f32,
    warhorse_plague_run_speed: f32,
    warhorse_plague_run_stamina_rate: f32,
    warhorse_skill_stamina_cost: f32,
    warhorse_cooldown: f32,
    warhorse_plague_cooldown: f32,
    warhorse_buff_duration: f32,
    warhorse_buff_range: f32,
    archer_walk_speed: f32,
    archer_run_speed: f32,
    archer_shoot_prep_time: f32,
    archer_shoot_cooldown_time: f32,
    archer_shoot_cooldown_with_knight_time: f32,
    archer_interval_min: f32,
    archer_interval_max: f32,
    archer_formation_interval_min: f32,
    archer_formation_interval_max: f32,
    builder_walk_speed: f32,
    builder_run_speed: f32,
    builder_work_time: f32,
    wall5_standard_hit_points: i32,
    wall5_norselands_hit_points: i32,
    knight_wallet_capacity: i32,
    player_wallet_capacity: i32,
    knight_wallet_pay_taxes_above: i32,
    knight_coin_drop_probability: f32,
    bag_scale: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssetResponse {
    data_directory: String,
    settings: AssetSettings,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssetApplyResponse {
    resources_backup: String,
    shared_assets_backup: String,
    settings: AssetSettings,
    save_cleared_walls: Option<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ColliderLimitResponse {
    limit: u16,
    backup: Option<String>,
}

fn game_assembly_path(data_directory: &Path) -> Result<PathBuf, String> {
    validate_asset_directory(data_directory)?;
    let contents = data_directory
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "The Data folder is not inside a game bundle".to_string())?;
    let path = contents.join("Frameworks").join(GAME_ASSEMBLY);
    if !path.is_file() {
        return Err(format!("Could not find {}", path.display()));
    }
    Ok(path)
}

fn collider_limit_instruction_offset(bytes: &[u8]) -> Result<usize, String> {
    let mut matches = bytes
        .windows(WARHORSE_COLLIDER_CONTEXT.len())
        .enumerate()
        .filter(|(_, window)| {
            *window == WARHORSE_COLLIDER_CONTEXT || *window == UPDATED_COLLIDER_CONTEXT
        })
        .map(|(index, _)| index);
    let context_offset = matches
        .next()
        .ok_or_else(|| "Unsupported GameAssembly.dylib: Warhorse code was not found".to_string())?;
    if matches.next().is_some() || context_offset < 4 {
        return Err("Unsupported GameAssembly.dylib: Warhorse code is ambiguous".into());
    }
    Ok(context_offset - 4)
}

fn read_collider_limit(bytes: &[u8]) -> Result<(usize, u16), String> {
    let offset = collider_limit_instruction_offset(bytes)?;
    let instruction = u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .map_err(|_| "Incomplete Warhorse instruction".to_string())?,
    );
    // ARM64 MOVZ W1, #imm16, without a shift.
    if instruction & !0x001f_ffe0 != 0x5280_0001 {
        return Err("Unsupported GameAssembly.dylib: unexpected Warhorse instruction".into());
    }
    Ok((offset, ((instruction >> 5) & 0xffff) as u16))
}

#[tauri::command]
fn load_warhorse_collider_limit(data_directory: String) -> Result<ColliderLimitResponse, String> {
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    return Err(
        "Warhorse collider limit editing currently supports Apple Silicon Macs only".into(),
    );

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let path = game_assembly_path(Path::new(&data_directory))?;
        let bytes = fs::read(path).map_err(|err| err.to_string())?;
        let (_, limit) = read_collider_limit(&bytes)?;
        Ok(ColliderLimitResponse {
            limit,
            backup: None,
        })
    }
}

#[tauri::command]
fn apply_warhorse_collider_limit(
    data_directory: String,
    limit: u16,
) -> Result<ColliderLimitResponse, String> {
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    return Err(
        "Warhorse collider limit editing currently supports Apple Silicon Macs only".into(),
    );

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        use std::process::Command;
        if limit == 0 {
            return Err("Collider limit must be positive (ARM64 immediate maximum: 65535)".into());
        }
        if game_is_running() {
            return Err("Close Kingdom Two Crowns before applying changes".into());
        }
        let path = game_assembly_path(Path::new(&data_directory))?;
        let mut bytes = fs::read(&path).map_err(|err| err.to_string())?;
        let (offset, current) = read_collider_limit(&bytes)?;
        if current == limit {
            return Ok(ColliderLimitResponse {
                limit,
                backup: None,
            });
        }
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| err.to_string())?
            .as_nanos();
        let backup = path.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{timestamp}.bak"));
        fs::copy(&path, &backup)
            .map_err(|err| format!("Could not back up GameAssembly.dylib: {err}"))?;

        let instruction = 0x5280_0001_u32 | (u32::from(limit) << 5);
        bytes[offset..offset + 4].copy_from_slice(&instruction.to_le_bytes());
        let temporary = path.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{timestamp}.tmp"));
        let result = (|| -> Result<(), String> {
            fs::write(&temporary, &bytes).map_err(|err| err.to_string())?;
            let permissions = fs::metadata(&path)
                .map_err(|err| err.to_string())?
                .permissions();
            fs::set_permissions(&temporary, permissions).map_err(|err| err.to_string())?;
            fs::rename(&temporary, &path).map_err(|err| err.to_string())?;
            let sign = Command::new("codesign")
                .args(["--force", "--sign", "-"])
                .arg(&path)
                .output()
                .map_err(|err| format!("Could not run codesign: {err}"))?;
            if !sign.status.success() {
                return Err(format!(
                    "Could not sign GameAssembly.dylib: {}",
                    String::from_utf8_lossy(&sign.stderr)
                ));
            }
            let verify = Command::new("codesign")
                .args(["--verify", "--verbose=2"])
                .arg(&path)
                .output()
                .map_err(|err| format!("Could not verify code signature: {err}"))?;
            if !verify.status.success() {
                return Err(format!(
                    "GameAssembly.dylib signature verification failed: {}",
                    String::from_utf8_lossy(&verify.stderr)
                ));
            }
            let written = fs::read(&path).map_err(|err| err.to_string())?;
            if read_collider_limit(&written)?.1 != limit {
                return Err("Warhorse collider limit verification failed".into());
            }
            Ok(())
        })();
        let _ = fs::remove_file(&temporary);
        if let Err(error) = result {
            let restored = fs::copy(&backup, &path);
            return Err(match restored {
                Ok(_) => format!(
                    "{error}. Original GameAssembly.dylib was restored from {}",
                    backup.display()
                ),
                Err(restore_error) => format!(
                    "{error}. Restore failed: {restore_error}. Backup: {}",
                    backup.display()
                ),
            });
        }
        Ok(ColliderLimitResponse {
            limit,
            backup: Some(backup.to_string_lossy().to_string()),
        })
    }
}

#[tauri::command]
fn restore_warhorse_collider_limit(
    data_directory: String,
    backup: String,
) -> Result<ColliderLimitResponse, String> {
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    return Err(
        "Warhorse collider limit editing currently supports Apple Silicon Macs only".into(),
    );

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        use std::process::Command;
        if game_is_running() {
            return Err("Close Kingdom Two Crowns before restoring changes".into());
        }
        let path = game_assembly_path(Path::new(&data_directory))?;
        let backup_path = PathBuf::from(backup);
        let parent = path.parent().ok_or("Invalid GameAssembly.dylib path")?;
        let backup_name = backup_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if backup_path.parent() != Some(parent)
            || !backup_name.starts_with("GameAssembly.dylib.ktcedit.")
            || !backup_name.ends_with(".bak")
        {
            return Err("Invalid GameAssembly.dylib backup path".into());
        }
        let bytes = fs::read(&backup_path).map_err(|err| err.to_string())?;
        let (_, limit) = read_collider_limit(&bytes)?;
        let signature = Command::new("codesign")
            .args(["--verify", "--verbose=2"])
            .arg(&backup_path)
            .output()
            .map_err(|err| err.to_string())?;
        if !signature.status.success() {
            return Err("The selected GameAssembly.dylib backup has an invalid signature".into());
        }
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| err.to_string())?
            .as_nanos();
        let safety = path.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{timestamp}.bak"));
        fs::copy(&path, &safety).map_err(|err| err.to_string())?;
        fs::copy(&backup_path, &path).map_err(|err| err.to_string())?;
        let restored = fs::read(&path).map_err(|err| err.to_string())?;
        if read_collider_limit(&restored)?.1 != limit {
            let _ = fs::copy(&safety, &path);
            return Err(
                "Restored Warhorse collider limit did not verify; previous file was restored"
                    .into(),
            );
        }
        Ok(ColliderLimitResponse {
            limit,
            backup: Some(safety.to_string_lossy().to_string()),
        })
    }
}

#[derive(Serialize)]
struct LoadResponse {
    path: String,
    data: Value,
}

#[tauri::command]
fn load_save_file(path: String) -> Result<LoadResponse, String> {
    let path_buf = PathBuf::from(&path);
    validate_path(&path_buf)?;

    info!("Opening save file at {}", path_buf.display());

    let file = std::fs::File::open(&path_buf).map_err(|err| {
        error!("Unable to open {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    let mut decoder = GzDecoder::new(file);
    let mut json = String::new();
    decoder.read_to_string(&mut json).map_err(|err| {
        error!("Failed to decompress {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    debug!(
        "Decompression complete ({} JSON bytes)",
        json.as_bytes().len()
    );
    let data: Value = serde_json::from_str(&json).map_err(|err| {
        error!("JSON invalide dans {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    if let Some(obj) = data.as_object() {
        info!("File loaded with {} top-level keys", obj.len());
    }

    Ok(LoadResponse { path, data })
}

#[tauri::command]
fn select_save_file() -> Result<LoadResponse, String> {
    let Some(selected) = FileDialog::new().set_title("Choose global-v35").pick_file() else {
        return Err("Selection cancelled".into());
    };

    let path = selected.to_string_lossy().to_string();
    info!("Selected file: {}", path);
    load_save_file(path)
}

#[tauri::command]
fn save_save_file(path: String, data: Value) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    validate_path(&path_buf)?;

    info!("Save requested for {}", path_buf.display());

    let backup_path = create_backup(&path_buf)?;

    let json = serde_json::to_string_pretty(&data).map_err(|err| {
        error!(
            "Unable to serialize data for {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json.as_bytes()).map_err(|err| {
        error!("Failed to recompress {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    let compressed = encoder.finish().map_err(|err| {
        error!(
            "Failed to finish compression for {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;

    log_resource_summaries(&data);

    info!(
        "Save: {} JSON bytes, {} gzip bytes",
        json.as_bytes().len(),
        compressed.len()
    );

    fs::write(&path_buf, compressed).map_err(|err| {
        error!("Unable to write {}: {}", path_buf.display(), err);
        err.to_string()
    })?;

    info!(
        "Save applied to {}, previous save: {}",
        path_buf.display(),
        backup_path.display()
    );

    Ok(backup_path.to_string_lossy().to_string())
}

fn log_resource_summaries(data: &Value) {
    const ARCHER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Archer",
        "Prefabs/Characters/norselands/Archer_norselands",
    ];
    const WORKER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Worker",
        "Prefabs/Characters/norselands/Worker_norselands",
    ];
    const FARMER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Farmer",
        "Prefabs/Characters/norselands/Farmer_norselands",
    ];
    const PIKEMAN_PREFABS: [&str; 3] = [
        "Prefabs/Characters/Pikeman",
        "Prefabs/Characters/norselands/Knight_norselands",
        "Prefabs/Characters/norselands/Pikeman_norselands",
    ];

    if let Some(campaigns) = data.get("campaigns").and_then(|value| value.as_array()) {
        for (campaign_index, campaign) in campaigns.iter().enumerate() {
            if let Some(carry) = campaign.get("carryForward").and_then(|v| v.as_object()) {
                let coins_p1 = carry
                    .get("currencyP1")
                    .and_then(|v| v.get("Coins"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let coins_p2 = carry
                    .get("currencyP2")
                    .and_then(|v| v.get("Coins"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let gems_p1 = carry
                    .get("currencyP1")
                    .and_then(|v| v.get("Gems"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let gems_p2 = carry
                    .get("currencyP2")
                    .and_then(|v| v.get("Gems"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);

                info!(
                    "Campaign {}: coins P1 = {}, coins P2 = {}, gems P1 = {}, gems P2 = {}",
                    campaign_index, coins_p1, coins_p2, gems_p1, gems_p2
                );
            }

            if let Some(islands) = campaign.get("_islands").and_then(|v| v.as_array()) {
                for (island_index, island) in islands.iter().enumerate() {
                    if let Some(objects) = island.get("objects").and_then(|v| v.as_array()) {
                        let archers = count_prefab(objects, &ARCHER_PREFABS);
                        let workers = count_prefab(objects, &WORKER_PREFABS);
                        let farmers = count_prefab(objects, &FARMER_PREFABS);
                        let pikemen = count_prefab(objects, &PIKEMAN_PREFABS);

                        info!(
                            "Campaign {} island {}: archers={}, workers={}, farmers={}, pikemen={}",
                            campaign_index, island_index, archers, workers, farmers, pikemen
                        );
                    }
                }
            }
        }
    }
}

fn count_prefab(objects: &[Value], prefabs: &[&str]) -> usize {
    objects
        .iter()
        .filter_map(|entry| entry.as_object())
        .filter(|entry| {
            entry
                .get("prefabPath")
                .and_then(|v| v.as_str())
                .map(|path| prefabs.iter().any(|candidate| path.contains(candidate)))
                .unwrap_or(false)
        })
        .count()
}

fn validate_path(path: &Path) -> Result<(), String> {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return Err("Invalid file name".into());
    };

    debug!("Validating file: {}", name);
    if name != SAVE_FILENAME {
        return Err(format!("The selected file ({name}) is not {SAVE_FILENAME}"));
    }

    Ok(())
}

fn create_backup(original: &Path) -> Result<PathBuf, String> {
    let parent = original
        .parent()
        .ok_or_else(|| "Unable to determine the file directory".to_string())?;
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(SAVE_FILENAME);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();

    let backup_name = format!("{}_{}.bak", stem, timestamp);
    let backup_path = parent.join(backup_name);

    fs::copy(original, &backup_path).map_err(|err| {
        error!(
            "Unable to create backup {} -> {}: {}",
            original.display(),
            backup_path.display(),
            err
        );
        err.to_string()
    })?;

    info!("Backup copy created: {}", backup_path.display());

    // Automatically retain only the 5 most recent backups for this file
    if let Ok(entries) = fs::read_dir(parent) {
        let prefix = format!("{}_", stem);
        let mut backups: Vec<(PathBuf, SystemTime)> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                let fname = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                fname.starts_with(&prefix) && fname.ends_with(".bak")
            })
            .filter_map(|p| {
                let time = fs::metadata(&p)
                    .and_then(|m| m.modified())
                    .unwrap_or(UNIX_EPOCH);
                Some((p, time))
            })
            .collect();

        if backups.len() > 5 {
            backups.sort_by_key(|b| b.1); // Sort oldest to newest
            let to_delete = backups.len() - 5;
            for (old_backup, _) in backups.into_iter().take(to_delete) {
                if let Err(e) = fs::remove_file(&old_backup) {
                    debug!(
                        "Failed to clean up old backup {}: {}",
                        old_backup.display(),
                        e
                    );
                } else {
                    info!("Cleaned up old backup: {}", old_backup.display());
                }
            }
        }
    }

    Ok(backup_path)
}

fn default_game_data_directory() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        return home.join("Library/Application Support/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns.app/Contents/Resources/Data");
    }
    #[cfg(target_os = "windows")]
    {
        let program_files = std::env::var_os("PROGRAMFILES(X86)")
            .or_else(|| std::env::var_os("PROGRAMFILES"))
            .map(PathBuf::from)
            .unwrap_or_default();
        return program_files
            .join("Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns_Data");
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        home.join(".local/share/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns_Data")
    }
}

fn default_save_file_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME").map(PathBuf::from)?;
        let path = home
            .join("Library/Application Support/nl.noio.kingdom-two-crowns/Release")
            .join(SAVE_FILENAME);
        if path.exists() {
            return Some(path);
        }
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var_os("APPDATA")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)?;
        let path = appdata
            .join("LocalLow/Raw Fury/Kingdom Two Crowns/Release")
            .join(SAVE_FILENAME);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn reset_walls_damageable_in_save_file(save_path: &Path) -> Result<usize, String> {
    let file = fs::File::open(save_path).map_err(|err| err.to_string())?;
    let mut decoder = GzDecoder::new(file);
    let mut json = String::new();
    decoder
        .read_to_string(&mut json)
        .map_err(|err| err.to_string())?;
    let mut data: Value = serde_json::from_str(&json).map_err(|err| err.to_string())?;

    fn clear_walls_in_container(container: &mut [Value]) -> usize {
        let mut count = 0;
        for entry in container {
            if let Some(islands) = entry.get_mut("_islands").and_then(|v| v.as_array_mut()) {
                for island in islands {
                    if let Some(objects) = island.get_mut("objects").and_then(|v| v.as_array_mut())
                    {
                        for obj in objects {
                            let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let prefab =
                                obj.get("prefabPath").and_then(|v| v.as_str()).unwrap_or("");
                            let is_wall = (name.contains("Wall") || prefab.contains("Wall"))
                                && !name.contains("Wreck");
                            if is_wall {
                                if let Some(components) =
                                    obj.get_mut("componentData2").and_then(|v| v.as_array_mut())
                                {
                                    let before_len = components.len();
                                    components.retain(|c| {
                                        c.get("name").and_then(|v| v.as_str()) != Some("Damageable")
                                    });
                                    if components.len() < before_len {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        count
    }

    let mut cleared_count = 0;
    if let Some(campaigns) = data.get_mut("campaigns").and_then(|v| v.as_array_mut()) {
        cleared_count += clear_walls_in_container(campaigns);
    }
    if let Some(challenges) = data.get_mut("challenges").and_then(|v| v.as_array_mut()) {
        cleared_count += clear_walls_in_container(challenges);
    }

    if cleared_count > 0 {
        create_backup(save_path)?;
        let modified_json = serde_json::to_string(&data).map_err(|err| err.to_string())?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(modified_json.as_bytes())
            .map_err(|err| err.to_string())?;
        let compressed = encoder.finish().map_err(|err| err.to_string())?;
        atomic_replace(save_path, &compressed)?;
        info!(
            "Reset Damageable on {} walls in {}",
            cleared_count,
            save_path.display()
        );
    }

    Ok(cleared_count)
}

fn validate_asset_directory(directory: &Path) -> Result<(), String> {
    let resources = directory.join(RESOURCES_ASSETS);
    let shared = directory.join(SHARED_ASSETS);
    let resources_size = fs::metadata(&resources)
        .map_err(|_| {
            format!(
                "Could not find {} in {}",
                RESOURCES_ASSETS,
                directory.display()
            )
        })?
        .len();
    let shared_size = fs::metadata(&shared)
        .map_err(|_| {
            format!(
                "Could not find {} in {}",
                SHARED_ASSETS,
                directory.display()
            )
        })?
        .len();

    if !supported_asset_sizes(resources_size, shared_size) {
        return Err(format!(
            "These assets do not match the supported KTC profile (resources: {resources_size}, shared: {shared_size}). No changes were written."
        ));
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct UnityObject {
    path_id: i64,
    byte_start: usize,
    byte_size: usize,
    class_id: i32,
}

fn read_be_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| "Unity asset header is truncated".to_string())?;
    Ok(u32::from_be_bytes(value.try_into().unwrap()))
}

fn read_be_u64(bytes: &[u8], offset: usize) -> Result<u64, String> {
    let value = bytes
        .get(offset..offset + 8)
        .ok_or_else(|| "Unity asset header is truncated".to_string())?;
    Ok(u64::from_be_bytes(value.try_into().unwrap()))
}

fn read_le_i16(bytes: &[u8], offset: &mut usize) -> Result<i16, String> {
    let value = bytes
        .get(*offset..*offset + 2)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 2;
    Ok(i16::from_le_bytes(value.try_into().unwrap()))
}

fn read_le_i32(bytes: &[u8], offset: &mut usize) -> Result<i32, String> {
    let value = bytes
        .get(*offset..*offset + 4)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 4;
    Ok(i32::from_le_bytes(value.try_into().unwrap()))
}

fn read_le_i64(bytes: &[u8], offset: &mut usize) -> Result<i64, String> {
    let value = bytes
        .get(*offset..*offset + 8)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 8;
    Ok(i64::from_le_bytes(value.try_into().unwrap()))
}

fn skip_c_string(bytes: &[u8], offset: &mut usize) -> Result<(), String> {
    let remainder = bytes
        .get(*offset..)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    let length = remainder
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| "Unity asset string is not terminated".to_string())?;
    *offset += length + 1;
    Ok(())
}

fn align_four(offset: &mut usize) {
    *offset = (*offset + 3) & !3;
}

fn parse_unity_objects(bytes: &[u8]) -> Result<Vec<UnityObject>, String> {
    if read_be_u32(bytes, 8)? != 22 {
        return Err("Only Unity serialized-file version 22 is supported".into());
    }
    if bytes.get(16).copied() != Some(0) {
        return Err("Big-endian Unity assets are not supported".into());
    }
    let data_offset = usize::try_from(read_be_u64(bytes, 32)?)
        .map_err(|_| "Unity asset data offset is too large".to_string())?;
    let mut cursor = 48;
    skip_c_string(bytes, &mut cursor)?;
    cursor += 4; // target platform
    let type_tree_enabled = *bytes
        .get(cursor)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    cursor += 1;
    if type_tree_enabled != 0 {
        return Err("Embedded Unity type trees are not supported by this reader".into());
    }

    let type_count = read_le_i32(bytes, &mut cursor)?;
    if !(0..=10_000).contains(&type_count) {
        return Err("Unity asset contains an invalid type count".into());
    }
    let mut class_ids = Vec::with_capacity(type_count as usize);
    for _ in 0..type_count {
        let class_id = read_le_i32(bytes, &mut cursor)?;
        cursor += 1; // is stripped type
        let _script_type_index = read_le_i16(bytes, &mut cursor)?;
        if class_id == 114 {
            cursor += 16; // script id
        }
        cursor += 16; // old type hash
        if cursor > bytes.len() {
            return Err("Unity serialized type is truncated".into());
        }
        class_ids.push(class_id);
    }

    let object_count = read_le_i32(bytes, &mut cursor)?;
    if !(0..=2_000_000).contains(&object_count) {
        return Err("Unity asset contains an invalid object count".into());
    }
    let mut objects = Vec::with_capacity(object_count as usize);
    for _ in 0..object_count {
        align_four(&mut cursor);
        let path_id = read_le_i64(bytes, &mut cursor)?;
        let relative_start = read_le_i64(bytes, &mut cursor)?;
        let byte_size = read_le_i32(bytes, &mut cursor)?;
        let type_id = read_le_i32(bytes, &mut cursor)?;
        if relative_start < 0 || byte_size < 0 || type_id < 0 {
            return Err("Unity asset contains an invalid object entry".into());
        }
        let class_id = *class_ids
            .get(type_id as usize)
            .ok_or_else(|| "Unity object references an unknown type".to_string())?;
        let byte_start = data_offset
            .checked_add(relative_start as usize)
            .ok_or_else(|| "Unity object position overflowed".to_string())?;
        let byte_size = byte_size as usize;
        if byte_start
            .checked_add(byte_size)
            .filter(|end| *end <= bytes.len())
            .is_none()
        {
            return Err("Unity object points outside the asset file".into());
        }
        objects.push(UnityObject {
            path_id,
            byte_start,
            byte_size,
            class_id,
        });
    }
    Ok(objects)
}

fn object_bytes<'a>(bytes: &'a [u8], object: &UnityObject) -> Result<&'a [u8], String> {
    bytes
        .get(object.byte_start..object.byte_start + object.byte_size)
        .ok_or_else(|| "Unity object points outside the asset file".to_string())
}

fn read_aligned_string(bytes: &[u8], cursor: &mut usize) -> Result<String, String> {
    let length = read_le_i32(bytes, cursor)?;
    if length < 0 {
        return Err("Unity string has a negative length".into());
    }
    let value = bytes
        .get(*cursor..*cursor + length as usize)
        .ok_or_else(|| "Unity string is truncated".to_string())?;
    *cursor += length as usize;
    align_four(cursor);
    String::from_utf8(value.to_vec()).map_err(|_| "Unity string is not UTF-8".to_string())
}

fn mono_script_class_name(bytes: &[u8], object: &UnityObject) -> Result<String, String> {
    let raw = object_bytes(bytes, object)?;
    let mut cursor = 0;
    let _name = read_aligned_string(raw, &mut cursor)?;
    cursor += 4 + 16; // execution order and properties hash
    read_aligned_string(raw, &mut cursor)
}

fn game_object_data(bytes: &[u8], object: &UnityObject) -> Result<(String, Vec<i64>), String> {
    let raw = object_bytes(bytes, object)?;
    let mut cursor = 0;
    let count = read_le_i32(raw, &mut cursor)?;
    if !(0..=10_000).contains(&count) {
        return Err("GameObject contains an invalid component count".into());
    }
    let mut components = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let file_id = read_le_i32(raw, &mut cursor)?;
        let path_id = read_le_i64(raw, &mut cursor)?;
        if file_id == 0 {
            components.push(path_id);
        }
    }
    cursor += 4; // layer
    let name = read_aligned_string(raw, &mut cursor)?;
    Ok((name, components))
}

fn find_mono_script_path(global_managers: &[u8], class_name: &str) -> Result<i64, String> {
    let objects = parse_unity_objects(global_managers)?;
    for object in objects.iter().filter(|object| object.class_id == 115) {
        if mono_script_class_name(global_managers, object)? == class_name {
            return Ok(object.path_id);
        }
    }
    Err(format!("Could not find MonoScript {class_name}"))
}

fn resolve_component_offset(
    bytes: &[u8],
    expected_script_path: i64,
    game_object_name: &str,
    relative_offset: usize,
) -> Result<usize, String> {
    let objects = parse_unity_objects(bytes)?;
    let game_object = objects
        .iter()
        .filter(|object| object.class_id == 1)
        .find_map(|object| {
            game_object_data(bytes, object)
                .ok()
                .filter(|(name, _)| name == game_object_name)
                .map(|data| (object, data.1))
        })
        .ok_or_else(|| format!("Could not find GameObject {game_object_name}"))?;

    for component_path in game_object.1 {
        let Some(component) = objects
            .iter()
            .find(|object| object.path_id == component_path && object.class_id == 114)
        else {
            continue;
        };
        let raw = object_bytes(bytes, component)?;
        if raw.len() < 28 {
            continue;
        }
        let owner_path = i64::from_le_bytes(raw[4..12].try_into().unwrap());
        let script_file = i32::from_le_bytes(raw[16..20].try_into().unwrap());
        let script_path = i64::from_le_bytes(raw[20..28].try_into().unwrap());
        if owner_path == game_object.0.path_id
            && script_file == 1
            && script_path == expected_script_path
        {
            if relative_offset
                .checked_add(4)
                .filter(|end| *end <= component.byte_size)
                .is_none()
            {
                return Err(format!(
                    "Component field +{relative_offset} is outside {game_object_name}'s {}-byte blob",
                    component.byte_size
                ));
            }
            return Ok(component.byte_start + relative_offset);
        }
    }
    Err(format!(
        "Could not find the requested component on {game_object_name}"
    ))
}

fn resolve_builtin_component_offset(
    bytes: &[u8],
    component_class_id: i32,
    game_object_name: &str,
    relative_offset: usize,
) -> Result<usize, String> {
    let objects = parse_unity_objects(bytes)?;
    let game_object = objects
        .iter()
        .filter(|object| object.class_id == 1)
        .find_map(|object| {
            game_object_data(bytes, object)
                .ok()
                .filter(|(name, _)| name == game_object_name)
                .map(|data| (object, data.1))
        })
        .ok_or_else(|| format!("Could not find GameObject {game_object_name}"))?;

    for component_path in game_object.1 {
        let Some(component) = objects.iter().find(|object| {
            object.path_id == component_path && object.class_id == component_class_id
        }) else {
            continue;
        };
        let raw = object_bytes(bytes, component)?;
        if raw.len() < 12 {
            continue;
        }
        let owner_path = i64::from_le_bytes(raw[4..12].try_into().unwrap());
        if owner_path == game_object.0.path_id {
            if relative_offset
                .checked_add(4)
                .filter(|end| *end <= component.byte_size)
                .is_none()
            {
                return Err(format!(
                    "Component field +{relative_offset} is outside {game_object_name}'s {}-byte blob",
                    component.byte_size
                ));
            }
            return Ok(component.byte_start + relative_offset);
        }
    }
    Err(format!(
        "Could not find component class {component_class_id} on {game_object_name}"
    ))
}

fn warhorse_duration_target(resources: &[u8], global: &[u8]) -> Result<[FloatTarget; 1], String> {
    let script = find_mono_script_path(global, "BuffData")?;
    let mut found = Vec::new();
    for object in parse_unity_objects(resources)?
        .iter()
        .filter(|o| o.class_id == 114)
    {
        let raw = object_bytes(resources, object)?;
        if raw.len() < 32 {
            continue;
        }
        if i32::from_le_bytes(raw[16..20].try_into().unwrap()) != 1
            || i64::from_le_bytes(raw[20..28].try_into().unwrap()) != script
        {
            continue;
        }
        let mut cursor = 28;
        if read_aligned_string(raw, &mut cursor)? == "Buff_invulnerability_warhorse" {
            if object.byte_size != 264 {
                return Err("Unsupported Warhorse BuffData layout".into());
            }
            found.push(FloatTarget {
                file: RESOURCES_ASSETS,
                offset: object.byte_start + 72,
            });
        }
    }
    if found.len() != 1 {
        return Err("Expected exactly one Warhorse BuffData asset".into());
    }
    Ok([found[0]])
}

fn read_float(bytes: &[u8], offset: usize) -> Result<f32, String> {
    let slice = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("Địa chỉ {offset} nằm ngoài file"))?;
    Ok(f32::from_le_bytes(
        slice
            .try_into()
            .map_err(|_| "Could not read a float value".to_string())?,
    ))
}

fn write_float(bytes: &mut [u8], offset: usize, value: f32) -> Result<(), String> {
    if !value.is_finite() {
        return Err("Value must be a finite number".into());
    }
    let destination = bytes
        .get_mut(offset..offset + 4)
        .ok_or_else(|| format!("Địa chỉ {offset} nằm ngoài file"))?;
    destination.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn read_int(bytes: &[u8], offset: usize) -> Result<i32, String> {
    let slice = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("Address {offset} is outside the asset file"))?;
    Ok(i32::from_le_bytes(slice.try_into().unwrap()))
}

fn write_int(bytes: &mut [u8], offset: usize, value: i32) -> Result<(), String> {
    let destination = bytes
        .get_mut(offset..offset + 4)
        .ok_or_else(|| format!("Address {offset} is outside the asset file"))?;
    destination.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn consistent_value(
    resources: &[u8],
    shared: &[u8],
    targets: &[FloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "Profile không có địa chỉ".to_string())?;
    let value = read_float(
        if first.file == RESOURCES_ASSETS {
            resources
        } else {
            shared
        },
        first.offset,
    )?;
    for target in &targets[1..] {
        let candidate = read_float(
            if target.file == RESOURCES_ASSETS {
                resources
            } else {
                shared
            },
            target.offset,
        )?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    target: &ComponentFloatTarget,
) -> Result<f32, String> {
    let bytes = if target.file == RESOURCES_ASSETS {
        resources
    } else {
        shared
    };
    let offset = resolve_component_offset(
        bytes,
        script_path,
        target.game_object,
        target.relative_offset,
    )?;
    read_float(bytes, offset)
}

fn consistent_component_int_value(
    resources: &[u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
) -> Result<i32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "No component targets configured".to_string())?;
    let offset = resolve_component_offset(
        resources,
        script_path,
        first.game_object,
        first.relative_offset,
    )?;
    let value = read_int(resources, offset)?;
    for target in &targets[1..] {
        let offset = resolve_component_offset(
            resources,
            script_path,
            target.game_object,
            target.relative_offset,
        )?;
        if read_int(resources, offset)? != value {
            return Err(format!(
                "Component variants have different values; check {} before applying changes",
                target.game_object
            ));
        }
    }
    Ok(value)
}

fn consistent_component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "No Steed targets configured".to_string())?;
    let value = component_value(resources, shared, script_path, first)?;
    for target in &targets[1..] {
        let candidate = component_value(resources, shared, script_path, target)?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Steed variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn builtin_component_value(
    resources: &[u8],
    shared: &[u8],
    component_class_id: i32,
    target: &ComponentFloatTarget,
) -> Result<f32, String> {
    let bytes = if target.file == RESOURCES_ASSETS {
        resources
    } else {
        shared
    };
    let offset = resolve_builtin_component_offset(
        bytes,
        component_class_id,
        target.game_object,
        target.relative_offset,
    )?;
    read_float(bytes, offset)
}

fn consistent_builtin_component_value(
    resources: &[u8],
    shared: &[u8],
    component_class_id: i32,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "No component targets configured".to_string())?;
    let value = builtin_component_value(resources, shared, component_class_id, first)?;
    for target in &targets[1..] {
        let candidate = builtin_component_value(resources, shared, component_class_id, target)?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Component variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn primary_component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let target = targets
        .first()
        .ok_or_else(|| "No Steed targets configured".to_string())?;
    component_value(resources, shared, script_path, target)
}

fn read_asset_settings(directory: &Path) -> Result<AssetSettings, String> {
    validate_asset_directory(directory)?;
    let resources = fs::read(directory.join(RESOURCES_ASSETS)).map_err(|err| err.to_string())?;
    let shared = fs::read(directory.join(SHARED_ASSETS)).map_err(|err| err.to_string())?;
    let global_managers = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS))
        .map_err(|err| format!("Could not read {GLOBAL_MANAGERS_ASSETS}: {err}"))?;
    let steed_script_path = find_mono_script_path(&global_managers, "Steed")?;
    let griffin_skill_script_path =
        find_mono_script_path(&global_managers, "PushAttackSteedAbility")?;
    let pushable_pusher_script_path = find_mono_script_path(&global_managers, "PushablePusher")?;
    let lizard_skill_script_path = find_mono_script_path(&global_managers, "SpitSteedAbility")?;
    let warhorse_skill_script_path =
        find_mono_script_path(&global_managers, "BuffUnitsSteedAbility")?;
    let archer_script_path = find_mono_script_path(&global_managers, "Archer")?;
    let worker_script_path = find_mono_script_path(&global_managers, "Worker")?;
    let damageable_script_path = find_mono_script_path(&global_managers, "Damageable")?;
    let wallet_script_path = find_mono_script_path(&global_managers, "Wallet")?;
    Ok(AssetSettings {
        // Clean builds have different Greece defaults. Apply synchronizes all variants.
        griffin_run_speed: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_RUN,
        )?,
        griffin_forest_multiplier: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_FOREST,
        )?,
        griffin_run_stamina_rate: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_RUN_STAMINA,
        )?,
        griffin_skill_stamina_cost: consistent_component_value(
            &resources,
            &shared,
            griffin_skill_script_path,
            &GRIFFIN_SKILL_COST,
        )?,
        griffin_push_width: consistent_component_value(
            &resources,
            &shared,
            pushable_pusher_script_path,
            &GRIFFIN_PUSH_WIDTH,
        )?,
        griffin_push_height: consistent_component_value(
            &resources,
            &shared,
            pushable_pusher_script_path,
            &GRIFFIN_PUSH_HEIGHT,
        )?,
        lizard_run_speed: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &LIZARD_RUN,
        )?,
        lizard_run_stamina_rate: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &LIZARD_RUN_STAMINA,
        )?,
        lizard_skill_stamina_cost: consistent_component_value(
            &resources,
            &shared,
            lizard_skill_script_path,
            &LIZARD_SKILL_COST,
        )?,
        // Older ktc-edit builds only changed the three resources.assets variants.
        // Read the primary variant so users can open those files and use Apply to
        // synchronize the previously omitted sharedassets0.assets prefab.
        horse_run_speed: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &HORSE_RUN,
        )?,
        horse_run_stamina_rate: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &HORSE_RUN_STAMINA,
        )?,
        warhorse_run_speed: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_RUN,
        )?,
        warhorse_run_stamina_rate: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_RUN_STAMINA,
        )?,
        warhorse_plague_run_speed: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_PLAGUE_RUN,
        )?,
        warhorse_plague_run_stamina_rate: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_PLAGUE_RUN_STAMINA,
        )?,
        warhorse_skill_stamina_cost: consistent_component_value(
            &resources,
            &shared,
            warhorse_skill_script_path,
            &WARHORSE_SKILL_COST,
        )?,
        warhorse_cooldown: consistent_component_value(
            &resources,
            &shared,
            warhorse_skill_script_path,
            &WARHORSE_COOLDOWN,
        )?,
        warhorse_plague_cooldown: consistent_component_value(
            &resources,
            &shared,
            warhorse_skill_script_path,
            &WARHORSE_PLAGUE_COOLDOWN,
        )?,
        warhorse_buff_duration: consistent_value(
            &resources,
            &shared,
            &warhorse_duration_target(&resources, &global_managers)?,
        )?,
        warhorse_buff_range: consistent_component_value(
            &resources,
            &shared,
            warhorse_skill_script_path,
            &WARHORSE_RANGE,
        )?,
        archer_walk_speed: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_WALK_SPEED,
        )?,
        archer_run_speed: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_RUN_SPEED,
        )?,
        archer_shoot_prep_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_SHOOT_PREP,
        )?,
        archer_shoot_cooldown_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_SHOOT_COOLDOWN,
        )?,
        archer_shoot_cooldown_with_knight_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_KNIGHT_COOLDOWN,
        )?,
        archer_interval_min: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_INTERVAL_MIN,
        )?,
        archer_interval_max: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_INTERVAL_MAX,
        )?,
        archer_formation_interval_min: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_FORMATION_INTERVAL_MIN,
        )?,
        archer_formation_interval_max: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_FORMATION_INTERVAL_MAX,
        )?,
        builder_walk_speed: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_WALK_SPEED,
        )?,
        builder_run_speed: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_RUN_SPEED,
        )?,
        builder_work_time: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_WORK_TIME,
        )?,
        wall5_standard_hit_points: consistent_component_int_value(
            &resources,
            damageable_script_path,
            &WALL5_STANDARD_HP,
        )?,
        wall5_norselands_hit_points: consistent_component_int_value(
            &resources,
            damageable_script_path,
            &WALL5_NORSELANDS_HP,
        )?,
        knight_wallet_capacity: consistent_component_int_value(
            &resources,
            wallet_script_path,
            &KNIGHT_WALLET_CAPACITY,
        )?,
        player_wallet_capacity: consistent_component_int_value(
            &resources,
            wallet_script_path,
            &PLAYER_WALLET_CAPACITY,
        )?,
        knight_wallet_pay_taxes_above: consistent_component_int_value(
            &resources,
            wallet_script_path,
            &KNIGHT_WALLET_PAY_TAXES_ABOVE,
        )?,
        knight_coin_drop_probability: consistent_component_value(
            &resources,
            &shared,
            find_mono_script_path(&global_managers, "Character")?,
            &KNIGHT_COIN_DROP_PROBABILITY,
        )?,
        bag_scale: consistent_builtin_component_value(&resources, &shared, 4, &BAG_SCALE)?,
    })
}

#[tauri::command]
fn load_game_assets(data_directory: Option<String>) -> Result<AssetResponse, String> {
    let directory = data_directory
        .map(PathBuf::from)
        .unwrap_or_else(default_game_data_directory);
    let settings = read_asset_settings(&directory)?;
    Ok(AssetResponse {
        data_directory: directory.to_string_lossy().to_string(),
        settings,
    })
}

#[tauri::command]
fn select_game_data_directory() -> Result<AssetResponse, String> {
    let Some(directory) = FileDialog::new()
        .set_title("Select the Kingdom Two Crowns Data folder")
        .pick_folder()
    else {
        return Err("Folder selection was cancelled".into());
    };
    load_game_assets(Some(directory.to_string_lossy().to_string()))
}

fn write_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    targets: &[FloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        write_float(
            if target.file == RESOURCES_ASSETS {
                resources
            } else {
                shared
            },
            target.offset,
            value,
        )?;
    }
    Ok(())
}

fn write_component_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        let bytes = if target.file == RESOURCES_ASSETS {
            &mut *resources
        } else {
            &mut *shared
        };
        let offset = resolve_component_offset(
            bytes,
            script_path,
            target.game_object,
            target.relative_offset,
        )?;
        write_float(bytes, offset, value)?;
    }
    Ok(())
}

fn write_component_int_targets(
    resources: &mut [u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
    value: i32,
) -> Result<(), String> {
    for target in targets {
        let offset = resolve_component_offset(
            resources,
            script_path,
            target.game_object,
            target.relative_offset,
        )?;
        write_int(resources, offset, value)?;
    }
    Ok(())
}

fn write_builtin_component_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    component_class_id: i32,
    targets: &[ComponentFloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        let bytes = if target.file == RESOURCES_ASSETS {
            &mut *resources
        } else {
            &mut *shared
        };
        let offset = resolve_builtin_component_offset(
            bytes,
            component_class_id,
            target.game_object,
            target.relative_offset,
        )?;
        write_float(bytes, offset, value)?;
    }
    Ok(())
}

fn valid_nonnegative_asset_number(value: f32) -> bool {
    value.is_finite() && value >= 0.0
}

fn validate_settings(settings: &AssetSettings) -> Result<(), String> {
    if !settings.knight_coin_drop_probability.is_finite()
        || !(0.0..=1.0).contains(&settings.knight_coin_drop_probability)
    {
        return Err("Knight coin drop probability must be between 0 and 1".into());
    }
    if !(1..=1_000_000).contains(&settings.wall5_standard_hit_points)
        || !(1..=1_000_000).contains(&settings.wall5_norselands_hit_points)
    {
        return Err("Wall5 HP must be a whole number between 1 and 1,000,000".into());
    }
    if !(1..=1_000_000).contains(&settings.knight_wallet_capacity)
        || !(0..=1_000_000).contains(&settings.knight_wallet_pay_taxes_above)
    {
        return Err("Knight Wallet values must be whole numbers between 0 and 1,000,000; capacity must be at least 1".into());
    }
    if !(1..=1_000_000).contains(&settings.player_wallet_capacity) {
        return Err("Player Wallet capacity must be a whole number between 1 and 1,000,000".into());
    }
    let values = [
        settings.griffin_run_speed,
        settings.griffin_forest_multiplier,
        settings.griffin_skill_stamina_cost,
        settings.griffin_push_width,
        settings.griffin_push_height,
        settings.lizard_run_speed,
        settings.lizard_skill_stamina_cost,
        settings.horse_run_speed,
        settings.warhorse_run_speed,
        settings.warhorse_plague_run_speed,
        settings.warhorse_skill_stamina_cost,
        settings.warhorse_cooldown,
        settings.warhorse_plague_cooldown,
        settings.warhorse_buff_range,
        settings.archer_walk_speed,
        settings.archer_run_speed,
        settings.archer_shoot_prep_time,
        settings.archer_shoot_cooldown_time,
        settings.archer_shoot_cooldown_with_knight_time,
        settings.archer_interval_min,
        settings.archer_interval_max,
        settings.archer_formation_interval_min,
        settings.archer_formation_interval_max,
        settings.builder_walk_speed,
        settings.builder_run_speed,
        settings.builder_work_time,
        settings.bag_scale,
    ];
    if values
        .iter()
        .any(|value| !valid_nonnegative_asset_number(*value))
    {
        return Err("Speeds, costs, sizes, and cooldowns must be finite and non-negative".into());
    }
    let stamina_rates = [
        settings.griffin_run_stamina_rate,
        settings.lizard_run_stamina_rate,
        settings.horse_run_stamina_rate,
        settings.warhorse_run_stamina_rate,
        settings.warhorse_plague_run_stamina_rate,
    ];
    if stamina_rates.iter().any(|value| !value.is_finite()) {
        return Err("Run stamina rates must be finite; negative values are allowed".into());
    }
    if !settings.warhorse_buff_duration.is_finite()
        || !(0.0..=1_000_000.0).contains(&settings.warhorse_buff_duration)
    {
        return Err("Warhorse buff duration must be between 0 and 1000000 seconds".into());
    }
    if settings.archer_interval_min > settings.archer_interval_max {
        return Err("Archer normal interval minimum cannot exceed its maximum".into());
    }
    if settings.archer_formation_interval_min > settings.archer_formation_interval_max {
        return Err("Archer formation interval minimum cannot exceed its maximum".into());
    }
    Ok(())
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp = path.with_extension("ktcedit.tmp");
    fs::write(&temp, bytes).map_err(|err| format!("Could not write the temporary file: {err}"))?;
    fs::rename(&temp, path).map_err(|err| format!("Could not replace {}: {err}", path.display()))
}

#[tauri::command]
fn apply_game_assets(
    data_directory: String,
    settings: AssetSettings,
    reset_save_walls: Option<bool>,
) -> Result<AssetApplyResponse, String> {
    validate_settings(&settings)?;
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before applying changes".into());
    }
    let directory = PathBuf::from(data_directory);
    let _current = read_asset_settings(&directory)?;
    let resources_path = directory.join(RESOURCES_ASSETS);
    let shared_path = directory.join(SHARED_ASSETS);
    let mut resources = fs::read(&resources_path).map_err(|err| err.to_string())?;
    let mut shared = fs::read(&shared_path).map_err(|err| err.to_string())?;
    let global_managers = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS))
        .map_err(|err| format!("Could not read {GLOBAL_MANAGERS_ASSETS}: {err}"))?;
    let steed_script_path = find_mono_script_path(&global_managers, "Steed")?;
    let griffin_skill_script_path =
        find_mono_script_path(&global_managers, "PushAttackSteedAbility")?;
    let pushable_pusher_script_path = find_mono_script_path(&global_managers, "PushablePusher")?;
    let lizard_skill_script_path = find_mono_script_path(&global_managers, "SpitSteedAbility")?;
    let warhorse_skill_script_path =
        find_mono_script_path(&global_managers, "BuffUnitsSteedAbility")?;
    let archer_script_path = find_mono_script_path(&global_managers, "Archer")?;
    let worker_script_path = find_mono_script_path(&global_managers, "Worker")?;
    let damageable_script_path = find_mono_script_path(&global_managers, "Damageable")?;
    let wallet_script_path = find_mono_script_path(&global_managers, "Wallet")?;

    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_RUN,
        settings.griffin_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_FOREST,
        settings.griffin_forest_multiplier,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_RUN_STAMINA,
        settings.griffin_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        griffin_skill_script_path,
        &GRIFFIN_SKILL_COST,
        settings.griffin_skill_stamina_cost,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        pushable_pusher_script_path,
        &GRIFFIN_PUSH_WIDTH,
        settings.griffin_push_width,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        pushable_pusher_script_path,
        &GRIFFIN_PUSH_HEIGHT,
        settings.griffin_push_height,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &LIZARD_RUN,
        settings.lizard_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &LIZARD_RUN_STAMINA,
        settings.lizard_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        lizard_skill_script_path,
        &LIZARD_SKILL_COST,
        settings.lizard_skill_stamina_cost,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &HORSE_RUN,
        settings.horse_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &HORSE_RUN_STAMINA,
        settings.horse_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_RUN,
        settings.warhorse_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_RUN_STAMINA,
        settings.warhorse_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_PLAGUE_RUN,
        settings.warhorse_plague_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_PLAGUE_RUN_STAMINA,
        settings.warhorse_plague_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        warhorse_skill_script_path,
        &WARHORSE_SKILL_COST,
        settings.warhorse_skill_stamina_cost,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        warhorse_skill_script_path,
        &WARHORSE_COOLDOWN,
        settings.warhorse_cooldown,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        warhorse_skill_script_path,
        &WARHORSE_PLAGUE_COOLDOWN,
        settings.warhorse_plague_cooldown,
    )?;
    let duration_target = warhorse_duration_target(&resources, &global_managers)?;
    write_targets(
        &mut resources,
        &mut shared,
        &duration_target,
        settings.warhorse_buff_duration,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        warhorse_skill_script_path,
        &WARHORSE_RANGE,
        settings.warhorse_buff_range,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_WALK_SPEED,
        settings.archer_walk_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_RUN_SPEED,
        settings.archer_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_SHOOT_PREP,
        settings.archer_shoot_prep_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_SHOOT_COOLDOWN,
        settings.archer_shoot_cooldown_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_KNIGHT_COOLDOWN,
        settings.archer_shoot_cooldown_with_knight_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_INTERVAL_MIN,
        settings.archer_interval_min,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_INTERVAL_MAX,
        settings.archer_interval_max,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_FORMATION_INTERVAL_MIN,
        settings.archer_formation_interval_min,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_FORMATION_INTERVAL_MAX,
        settings.archer_formation_interval_max,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_WALK_SPEED,
        settings.builder_walk_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_RUN_SPEED,
        settings.builder_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_WORK_TIME,
        settings.builder_work_time,
    )?;
    write_component_int_targets(
        &mut resources,
        damageable_script_path,
        &WALL5_STANDARD_HP,
        settings.wall5_standard_hit_points,
    )?;
    write_component_int_targets(
        &mut resources,
        damageable_script_path,
        &WALL5_NORSELANDS_HP,
        settings.wall5_norselands_hit_points,
    )?;
    write_component_int_targets(
        &mut resources,
        wallet_script_path,
        &KNIGHT_WALLET_CAPACITY,
        settings.knight_wallet_capacity,
    )?;
    write_component_int_targets(
        &mut resources,
        wallet_script_path,
        &PLAYER_WALLET_CAPACITY,
        settings.player_wallet_capacity,
    )?;
    write_component_int_targets(
        &mut resources,
        wallet_script_path,
        &KNIGHT_WALLET_PAY_TAXES_ABOVE,
        settings.knight_wallet_pay_taxes_above,
    )?;
    write_builtin_component_targets(
        &mut resources,
        &mut shared,
        4,
        &BAG_SCALE,
        settings.bag_scale,
    )?;

    write_component_targets(
        &mut resources,
        &mut shared,
        find_mono_script_path(&global_managers, "Character")?,
        &KNIGHT_COIN_DROP_PROBABILITY,
        settings.knight_coin_drop_probability,
    )?;

    let resources_backup = create_backup(&resources_path)?;
    let shared_backup = create_backup(&shared_path)?;
    atomic_replace(&resources_path, &resources)?;
    if let Err(error) = atomic_replace(&shared_path, &shared) {
        let _ = fs::copy(&resources_backup, &resources_path);
        return Err(error);
    }

    let written_resources = fs::read(&resources_path).map_err(|err| err.to_string())?;
    let written_shared = fs::read(&shared_path).map_err(|err| err.to_string())?;
    let verified_horse_run = consistent_component_value(
        &written_resources,
        &written_shared,
        steed_script_path,
        &HORSE_RUN,
    )?;
    let verified_horse_stamina = consistent_component_value(
        &written_resources,
        &written_shared,
        steed_script_path,
        &HORSE_RUN_STAMINA,
    )?;
    if (verified_horse_run - settings.horse_run_speed).abs() > 0.0001
        || (verified_horse_stamina - settings.horse_run_stamina_rate).abs() > 0.0001
    {
        return Err("Regular Horse values were not written consistently".into());
    }

    let verified = read_asset_settings(&directory)?;

    let mut cleared_walls = None;
    if reset_save_walls.unwrap_or(false) {
        if let Some(save_file) = default_save_file_path() {
            match reset_walls_damageable_in_save_file(&save_file) {
                Ok(count) => cleared_walls = Some(count),
                Err(err) => error!(
                    "Failed to reset walls in save {}: {}",
                    save_file.display(),
                    err
                ),
            }
        }
    }

    Ok(AssetApplyResponse {
        resources_backup: resources_backup.to_string_lossy().to_string(),
        shared_assets_backup: shared_backup.to_string_lossy().to_string(),
        settings: verified,
        save_cleared_walls: cleared_walls,
    })
}

fn game_is_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        return std::process::Command::new("tasklist")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .to_lowercase()
                    .contains("kingdomtwocrowns")
            })
            .unwrap_or(false);
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("pgrep")
            .args(["-if", "KingdomTwoCrowns"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

#[tauri::command]
fn restore_game_assets(
    data_directory: String,
    resources_backup: String,
    shared_assets_backup: String,
) -> Result<AssetResponse, String> {
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before restoring a backup".into());
    }
    let directory = PathBuf::from(data_directory);
    let resources_backup = PathBuf::from(resources_backup);
    let shared_backup = PathBuf::from(shared_assets_backup);
    let resources_bytes = fs::read(&resources_backup)
        .map_err(|err| format!("Could not read the resources backup: {err}"))?;
    let shared_bytes = fs::read(&shared_backup)
        .map_err(|err| format!("Could not read the sharedassets backup: {err}"))?;
    if !supported_asset_sizes(resources_bytes.len() as u64, shared_bytes.len() as u64) {
        return Err("The backup file sizes are invalid".into());
    }
    validate_asset_directory(&directory)?;
    if fs::metadata(directory.join(RESOURCES_ASSETS))
        .map_err(|e| e.to_string())?
        .len()
        != resources_bytes.len() as u64
        || fs::metadata(directory.join(SHARED_ASSETS))
            .map_err(|e| e.to_string())?
            .len()
            != shared_bytes.len() as u64
    {
        return Err(
            "Backup belongs to a different game build; refusing cross-build restore".into(),
        );
    }
    let resources_path = directory.join(RESOURCES_ASSETS);
    let shared_path = directory.join(SHARED_ASSETS);
    let _safety_resources = create_backup(&resources_path)?;
    let _safety_shared = create_backup(&shared_path)?;
    atomic_replace(&resources_path, &resources_bytes)?;
    atomic_replace(&shared_path, &shared_bytes)?;
    load_game_assets(Some(directory.to_string_lossy().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{read::GzDecoder, write::GzEncoder, Compression};
    use serde_json::json;

    #[test]
    fn warhorse_collider_limit_decodes_and_encodes_arm64_movz() {
        let mut bytes = vec![0; 8];
        bytes.extend_from_slice(&0x5280_0281_u32.to_le_bytes()); // mov w1, #20
        bytes.extend_from_slice(&WARHORSE_COLLIDER_CONTEXT);
        let (offset, value) = read_collider_limit(&bytes).expect("decode original limit");
        assert_eq!(value, 20);
        bytes[offset..offset + 4].copy_from_slice(&(0x5280_0001_u32 | (100 << 5)).to_le_bytes());
        assert_eq!(
            read_collider_limit(&bytes).expect("decode new limit").1,
            100
        );
    }

    #[test]
    fn warhorse_collider_limit_rejects_unrecognized_code() {
        assert!(read_collider_limit(&[0; 64]).is_err());
    }

    #[test]
    fn accepts_only_verified_asset_pairs() {
        assert!(supported_asset_sizes(
            EXPECTED_RESOURCES_SIZE,
            EXPECTED_SHARED_SIZE
        ));
        assert!(supported_asset_sizes(
            UPDATED_RESOURCES_SIZE,
            UPDATED_SHARED_SIZE
        ));
        assert!(!supported_asset_sizes(
            UPDATED_RESOURCES_SIZE,
            EXPECTED_SHARED_SIZE
        ));
        assert!(!supported_asset_sizes(
            EXPECTED_RESOURCES_SIZE,
            UPDATED_SHARED_SIZE
        ));
        assert!(!supported_asset_sizes(0, 0));
    }

    #[test]
    fn negative_run_stamina_rates_are_allowed() {
        let dir = default_game_data_directory();
        if !dir.join(RESOURCES_ASSETS).exists() {
            return;
        }
        let mut settings = read_asset_settings(&dir).unwrap();
        settings.griffin_run_stamina_rate = -0.05;
        settings.lizard_run_stamina_rate = -0.05;
        settings.horse_run_stamina_rate = -0.05;
        settings.warhorse_run_stamina_rate = -0.05;
        settings.warhorse_plague_run_stamina_rate = -0.05;
        assert!(validate_settings(&settings).is_ok());
        settings.horse_run_stamina_rate = f32::NAN;
        assert!(validate_settings(&settings).is_err());
    }

    #[test]
    fn asset_values_above_2000_are_allowed_but_invalid_floats_are_not() {
        assert!(valid_nonnegative_asset_number(5000.0));
        assert!(valid_nonnegative_asset_number(f32::MAX));
        assert!(!valid_nonnegative_asset_number(f32::INFINITY));
        assert!(!valid_nonnegative_asset_number(f32::NAN));
        assert!(!valid_nonnegative_asset_number(-1.0));
    }

    #[test]
    fn updated_warhorse_fields_roundtrip_in_memory() {
        let dir = default_game_data_directory();
        let Ok(mut resources) = fs::read(dir.join(RESOURCES_ASSETS)) else {
            return;
        };
        let mut shared = fs::read(dir.join(SHARED_ASSETS)).unwrap();
        let global = fs::read(dir.join(GLOBAL_MANAGERS_ASSETS)).unwrap();
        let script = find_mono_script_path(&global, "BuffUnitsSteedAbility").unwrap();
        let original = resources.clone();
        let mut allowed = Vec::new();
        for targets in [
            &WARHORSE_COOLDOWN[..],
            &WARHORSE_PLAGUE_COOLDOWN[..],
            &WARHORSE_RANGE[..],
        ] {
            for t in targets {
                allowed.push(
                    resolve_component_offset(&resources, script, t.game_object, t.relative_offset)
                        .unwrap(),
                );
            }
            write_component_targets(&mut resources, &mut shared, script, targets, 123.0).unwrap();
            assert_eq!(
                consistent_component_value(&resources, &shared, script, targets).unwrap(),
                123.0
            );
        }
        let duration = warhorse_duration_target(&resources, &global).unwrap();
        allowed.push(duration[0].offset);
        write_targets(&mut resources, &mut shared, &duration, 321.0).unwrap();
        assert_eq!(
            consistent_value(&resources, &shared, &duration).unwrap(),
            321.0
        );
        assert!(resources
            .iter()
            .zip(&original)
            .enumerate()
            .all(|(i, (a, b))| a == b || allowed.iter().any(|o| (*o..*o + 4).contains(&i))));
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn reads_installed_warhorse_collider_limit_when_available() {
        let data_directory = default_game_data_directory();
        if !data_directory.join(RESOURCES_ASSETS).exists() {
            return;
        }
        let response = load_warhorse_collider_limit(data_directory.to_string_lossy().to_string())
            .expect("read installed GameAssembly.dylib");
        assert!(response.limit > 0);
    }
    use std::{
        fs,
        io::{Read, Write},
        path::{Path, PathBuf},
    };
    use tempfile::tempdir;

    fn write_test_save(dir: &Path, payload: &serde_json::Value) -> PathBuf {
        let path = dir.join(SAVE_FILENAME);
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let json_bytes = serde_json::to_vec(payload).expect("serialize payload");
        encoder
            .write_all(&json_bytes)
            .expect("write compressed payload");
        let compressed = encoder.finish().expect("finish compression");
        fs::write(&path, compressed).expect("write save file");
        path
    }

    fn read_save(path: &Path) -> serde_json::Value {
        let file = fs::File::open(path).expect("open save file");
        let mut decoder = GzDecoder::new(file);
        let mut json = String::new();
        decoder
            .read_to_string(&mut json)
            .expect("decompress save file");
        serde_json::from_str(&json).expect("parse json")
    }

    #[test]
    fn validate_path_accepts_exact_name() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join(SAVE_FILENAME);
        fs::write(&path, b"test").expect("write file");
        assert!(validate_path(&path).is_ok());
    }

    #[test]
    fn validate_path_rejects_other_names() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("global-v35.bak");
        fs::write(&path, b"test").expect("write file");
        assert!(validate_path(&path).is_err());
    }

    #[test]
    fn load_save_file_returns_json_payload() {
        let dir = tempdir().expect("temp dir");
        let expected = json!({
            "serializedSaveDataVersion": 35,
            "campaigns": [
                {
                    "id": 1,
                    "progress": 42
                }
            ]
        });
        let path = write_test_save(dir.path(), &expected);
        let path_string = path.to_string_lossy().to_string();

        let response = load_save_file(path_string.clone()).expect("load save file");

        assert_eq!(response.path, path_string);
        assert_eq!(response.data, expected);
    }

    #[test]
    fn save_save_file_creates_backup_and_updates_contents() {
        let dir = tempdir().expect("temp dir");
        let original = json!({ "value": 10, "flags": [true, false] });
        let path = write_test_save(dir.path(), &original);
        let path_string = path.to_string_lossy().to_string();

        let updated = json!({ "value": 99, "flags": [false], "note": "updated" });
        let backup_path =
            save_save_file(path_string.clone(), updated.clone()).expect("save save file");
        let backup = PathBuf::from(&backup_path);

        assert!(backup.exists(), "backup file should exist");

        let backup_contents = read_save(&backup);
        assert_eq!(backup_contents, original);

        let rewritten = read_save(&path);
        assert_eq!(rewritten, updated);
    }

    #[test]
    fn regular_horse_targets_are_relative_and_include_default_prefab() {
        assert_eq!(HORSE_RUN.len(), 4);
        assert_eq!(HORSE_RUN_STAMINA.len(), 4);
        assert!(HORSE_RUN.iter().any(|target| target.file == SHARED_ASSETS
            && target.game_object == "Horse Regular"
            && target.relative_offset == 196));
        assert!(HORSE_RUN_STAMINA
            .iter()
            .any(|target| target.file == SHARED_ASSETS
                && target.game_object == "Horse Regular"
                && target.relative_offset == 208));
    }

    #[test]
    fn resolves_component_fields_from_installed_assets_when_available() {
        let directory = default_game_data_directory();
        let Ok(resources) = fs::read(directory.join(RESOURCES_ASSETS)) else {
            return;
        };
        let Ok(shared) = fs::read(directory.join(SHARED_ASSETS)) else {
            return;
        };
        let Ok(global_managers) = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS)) else {
            return;
        };
        let script = find_mono_script_path(&global_managers, "Steed").expect("Steed script");
        for target in GRIFFIN_RUN
            .iter()
            .chain(LIZARD_RUN.iter())
            .chain(HORSE_RUN.iter())
            .chain(WARHORSE_RUN.iter())
            .chain(WARHORSE_PLAGUE_RUN.iter())
            .chain(WARHORSE_PLAGUE_RUN_STAMINA.iter())
        {
            let bytes = if target.file == RESOURCES_ASSETS {
                &resources
            } else {
                &shared
            };
            let offset =
                resolve_component_offset(bytes, script, target.game_object, target.relative_offset)
                    .expect("dynamic Steed field");
            assert!(read_float(bytes, offset).expect("Steed float").is_finite());
        }
        let regular_warhorse_run =
            consistent_component_value(&resources, &shared, script, &WARHORSE_RUN)
                .expect("regular Warhorse run speed");
        let mut changed_plague_resources = resources.clone();
        let mut changed_plague_shared = shared.clone();
        for targets in [&WARHORSE_PLAGUE_RUN[..], &WARHORSE_PLAGUE_RUN_STAMINA[..]] {
            let original = consistent_component_value(&resources, &shared, script, targets)
                .expect("Plague Warhorse value");
            write_component_targets(
                &mut changed_plague_resources,
                &mut changed_plague_shared,
                script,
                targets,
                original + 1.0,
            )
            .expect("write Plague Warhorse in memory");
            assert_eq!(
                consistent_component_value(
                    &changed_plague_resources,
                    &changed_plague_shared,
                    script,
                    targets,
                )
                .expect("read changed Plague Warhorse value"),
                original + 1.0
            );
        }
        assert_eq!(
            consistent_component_value(
                &changed_plague_resources,
                &changed_plague_shared,
                script,
                &WARHORSE_RUN,
            )
            .expect("unchanged regular Warhorse run speed"),
            regular_warhorse_run
        );
        for (class_name, targets) in [
            ("PushAttackSteedAbility", GRIFFIN_SKILL_COST.as_slice()),
            ("PushablePusher", GRIFFIN_PUSH_WIDTH.as_slice()),
            ("PushablePusher", GRIFFIN_PUSH_HEIGHT.as_slice()),
            ("SpitSteedAbility", LIZARD_SKILL_COST.as_slice()),
            ("BuffUnitsSteedAbility", WARHORSE_SKILL_COST.as_slice()),
            ("Archer", ARCHER_SHOOT_PREP.as_slice()),
            ("Archer", ARCHER_WALK_SPEED.as_slice()),
            ("Archer", ARCHER_RUN_SPEED.as_slice()),
            ("Worker", BUILDER_WALK_SPEED.as_slice()),
        ] {
            let script =
                find_mono_script_path(&global_managers, class_name).expect("ability script");
            for target in targets {
                let bytes = if target.file == RESOURCES_ASSETS {
                    &resources
                } else {
                    &shared
                };
                let offset = resolve_component_offset(
                    bytes,
                    script,
                    target.game_object,
                    target.relative_offset,
                )
                .expect("dynamic ability field");
                assert!(read_float(bytes, offset)
                    .expect("component float")
                    .is_finite());
            }
        }
        for target in &BAG_SCALE {
            let bytes = if target.file == RESOURCES_ASSETS {
                &resources
            } else {
                &shared
            };
            let offset = resolve_builtin_component_offset(
                bytes,
                4,
                target.game_object,
                target.relative_offset,
            )
            .expect("dynamic Transform field");
            assert!(read_float(bytes, offset)
                .expect("Transform float")
                .is_finite());
        }
        let archer = find_mono_script_path(&global_managers, "Archer").expect("Archer script");
        let original_prep =
            consistent_component_value(&resources, &shared, archer, &ARCHER_SHOOT_PREP)
                .expect("Archer shoot preparation");
        let mut changed_archer_resources = resources.clone();
        let mut changed_archer_shared = shared.clone();
        for targets in [&ARCHER_WALK_SPEED[..], &ARCHER_RUN_SPEED[..]] {
            let original = consistent_component_value(&resources, &shared, archer, targets)
                .expect("Archer movement speed");
            write_component_targets(
                &mut changed_archer_resources,
                &mut changed_archer_shared,
                archer,
                targets,
                original + 1.0,
            )
            .expect("write Archer movement speed in memory");
            assert_eq!(
                consistent_component_value(
                    &changed_archer_resources,
                    &changed_archer_shared,
                    archer,
                    targets,
                )
                .expect("read changed Archer movement speed"),
                original + 1.0
            );
        }
        assert_eq!(
            consistent_component_value(
                &changed_archer_resources,
                &changed_archer_shared,
                archer,
                &ARCHER_SHOOT_PREP,
            )
            .expect("unchanged Archer shoot preparation"),
            original_prep
        );
        let damageable =
            find_mono_script_path(&global_managers, "Damageable").expect("Damageable script");
        let mut changed_resources = resources.clone();
        for targets in [&WALL5_STANDARD_HP[..], &WALL5_NORSELANDS_HP[..]] {
            let original = consistent_component_int_value(&resources, damageable, targets)
                .expect("Wall5 initial HP");
            write_component_int_targets(&mut changed_resources, damageable, targets, original + 1)
                .expect("write Wall5 initial HP in memory");
            assert_eq!(
                consistent_component_int_value(&changed_resources, damageable, targets)
                    .expect("read changed Wall5 initial HP"),
                original + 1
            );
        }
        let wallet = find_mono_script_path(&global_managers, "Wallet").expect("Wallet script");
        let character = find_mono_script_path(&global_managers, "Character").expect("Character script");
        let squire_drop_offset = resolve_component_offset(&resources, character, "Squire", 56)
            .expect("Squire coin drop probability");
        let mut changed_drop_resources = resources.clone();
        let mut changed_drop_shared = shared.clone();
        write_component_targets(
            &mut changed_drop_resources, &mut changed_drop_shared, character,
            &KNIGHT_COIN_DROP_PROBABILITY, 0.0,
        ).expect("write Knight coin drop probability in memory");
        assert_eq!(consistent_component_value(
            &changed_drop_resources, &changed_drop_shared, character,
            &KNIGHT_COIN_DROP_PROBABILITY,
        ).unwrap(), 0.0);
        assert_eq!(&changed_drop_resources[squire_drop_offset..squire_drop_offset + 4],
            &resources[squire_drop_offset..squire_drop_offset + 4]);
        assert_eq!(changed_drop_shared, shared);
        let squire_wallet_offset =
            resolve_component_offset(&resources, wallet, "Squire", 268).expect("Squire Wallet");
        let squire_capacity = read_int(&resources, squire_wallet_offset).expect("Squire capacity");
        for targets in [
            &PLAYER_WALLET_CAPACITY[..],
            &KNIGHT_WALLET_CAPACITY[..],
            &KNIGHT_WALLET_PAY_TAXES_ABOVE[..],
        ] {
            let original = consistent_component_int_value(&resources, wallet, targets)
                .expect("Knight Wallet value");
            write_component_int_targets(&mut changed_resources, wallet, targets, original + 1)
                .expect("write Knight Wallet in memory");
            assert_eq!(
                consistent_component_int_value(&changed_resources, wallet, targets)
                    .expect("read changed Knight Wallet"),
                original + 1
            );
        }
        assert_eq!(
            read_int(&changed_resources, squire_wallet_offset).expect("unchanged Squire capacity"),
            squire_capacity
        );
        read_asset_settings(&directory).expect("installed asset settings should load");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_save_file,
            save_save_file,
            select_save_file,
            load_game_assets,
            select_game_data_directory,
            apply_game_assets,
            restore_game_assets,
            load_warhorse_collider_limit,
            coin_bag::load_coin_visual_limit,
            coin_bag::apply_coin_visual_limit,
            camp::load_camp_preservation,
            camp::apply_camp_preservation,
            apply_warhorse_collider_limit,
            restore_warhorse_collider_limit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
