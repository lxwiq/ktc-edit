//! Build-specific ARM64 guard, identical to the separately tested camp patch.
use super::*;
const HOOK: usize = 0x660dbc;
const CAVE: usize = 0x44c6d00;
const CONTINUE: usize = 0x660dc0;
const FADE: usize = 0x661a88;
const GUARD_LEN: usize = 244;

#[derive(Serialize)]
pub struct Response {
    enabled: bool,
    #[serde(rename = "needsUpgrade")]
    needs_upgrade: bool,
    backup: Option<String>,
}

fn branch(a: usize, z: usize, link: bool) -> u32 {
    (if link { 0x94000000 } else { 0x14000000 }) | (((z as i64 - a as i64) / 4) as u32 & 0x3ffffff)
}
fn legacy_code() -> Vec<u8> {
    [
        0xf9402008_u32,
        0xb40002e8,
        0x1000000a,
        0xd28da10b,
        0xf2a0898b,
        0xcb0b014a,
        0xd2825a0b,
        0xf2a00e0b,
        0x8b0b014a,
        0xf9400909,
        0xeb0a013f,
        0x540001c0,
        0xf9403d08,
        0xb4000168,
        0xb940190c,
        0x91008108,
        0x3400010c,
        0xf8408509,
        0xb4000089,
        0xf9400929,
        0xeb0a013f,
        0x54000080,
        0x5100058c,
        0x17fffff9,
        branch(CAVE + 0x60, FADE, true),
        branch(CAVE + 0x64, CONTINUE, false),
    ]
    .into_iter()
    .flat_map(u32::to_le_bytes)
    .collect()
}
fn code() -> Vec<u8> {
    let mut guard = legacy_code();
    guard[44..48].copy_from_slice(&0x540001e0_u32.to_le_bytes());
    guard[84..88].copy_from_slice(&0x540000a0_u32.to_le_bytes());
    // Remove current circular-list node exactly as LinkedList.Remove does, then
    // clear the item's links. Restore UpdateRegion's frame and tail-restart it:
    // no stale enumerator, recursive stack growth, or camp destruction.
    for word in [
        0xf94032a8_u32,
        0xb40002c8,
        0xf9401669,
        0xf9400d0a,
        0xf940110b,
        0xeb08015f,
        0x54000100,
        0xf900114b,
        0xf9000d6a,
        0xf940092c,
        0xeb08019f,
        0x54000081,
        0xf900092a,
        0x14000002,
        0xf900093f,
        0xa9017d1f,
        0xf900111f,
        0xb940192a,
        0x5100054a,
        0xb900192a,
        0xb9401d2a,
        0x1100054a,
        0xb9001d2a,
        0xf90032bf,
        0xf9001ebf,
        0xaa1303e0,
        0xa94c7bfd,
        0xa94b4ff4,
        0xa94a57f6,
        0xa9495ff8,
        0xa94867fa,
        0xa9476ffc,
        0x6d4623e9,
        0x910343ff,
        branch(CAVE + 240, 0x660a08, false),
    ] {
        guard.extend_from_slice(&word.to_le_bytes());
    }
    guard
}
fn word(b: &[u8], p: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        b.get(p..p + 4)
            .ok_or("Truncated camp code")?
            .try_into()
            .unwrap(),
    ))
}
fn layout(b: &[u8], off: usize) -> Result<(usize, usize, usize, usize), String> {
    if coin_bag::layout(b, off)?.0 == 0xb1512c {
        Ok((0x662030, 0x44d7700, 0x662034, 0x662cfc))
    } else {
        Ok((HOOK, CAVE, CONTINUE, FADE))
    }
}
fn guard_for(b: &[u8], off: usize) -> Result<Vec<u8>, String> {
    let (hook, cave, cont, fade) = layout(b, off)?;
    let mut guard = code();
    if hook != HOOK {
        let set = |g: &mut Vec<u8>, p: usize, w: u32| g[p..p + 4].copy_from_slice(&w.to_le_bytes());
        let mov = |v: usize, base: u32| base | (((v as u32) & 0xffff) << 5);
        set(&mut guard, 12, mov(cave + 8, 0xd280000b));
        set(&mut guard, 16, mov((cave + 8) >> 16, 0xf2a0000b));
        set(&mut guard, 24, mov(0x70254c, 0xd280000b));
        set(&mut guard, 28, mov(0x70254c >> 16, 0xf2a0000b));
        set(&mut guard, 96, branch(cave + 96, fade, true));
        set(&mut guard, 100, branch(cave + 100, cont, false));
        set(&mut guard, 240, branch(cave + 240, 0x661c7c, false));
    }
    Ok(guard)
}
fn locate(b: &[u8]) -> Result<usize, String> {
    let off = coin_bag::locate(b)?;
    let (hook, cave, _, _) = layout(b, off)?;
    if hook != HOOK {
        for (a, w) in [
            (0x662020, 0x394162a8),
            (0x662024, 0x3707fb68),
            (0x662028, 0x2f00e400),
            (0x66202c, 0xaa1503e0),
            (0x662034, 0x17ffffd7),
            (0x7022d4, 0xf9400102),
            (0x7022d8, 0xaa1303e1),
            (0x7022e0, 0x94880d10),
            (0x7022f0, 0x948c76cd),
            (0x2905738, 0xf9400448),
            (0x290573c, 0xf9000808),
            (0x661c7c, 0xd10343ff),
            (0x248634c, 0xf9400e88),
            (0x2486388, 0xa9017e9f),
            (0x24863a0, 0xfd000e60),
        ] {
            if word(b, off + a)? != w {
                return Err("Unsupported updated camp code".into());
            }
        }
        if b.get(off + 0x4916ed8..off + 0x4916ee0) != Some(&0x60000977_u64.to_le_bytes()) {
            return Err("Unsupported updated camp callback".into());
        }
    } else {
        // The exact loop, callback construction and delegate layout inspected for this build.
        for (a, w) in [
            (0x660dac, 0x394162a8),
            (0x660db0, 0x3707fb68),
            (0x660db4, 0x2f00e400),
            (0x660db8, 0xaa1503e0),
            (CONTINUE, 0x17ffffd7),
            (0x701058, 0xf9400102),
            (0x70105c, 0xaa1303e1),
            (0x701064, 0x9487e085),
            (0x701074, 0x948c4a42),
            (0x28f9290, 0xf9400448),
            (0x28f9294, 0xf9000808),
            (0x660a08, 0xd10343ff),
            (0x2473060, 0xf9400e88),
            (0x247309c, 0xa9017e9f),
            (0x24730b4, 0xfd000e60),
        ] {
            if word(b, off + a)? != w {
                return Err("Unsupported Beggar Camp native code".into());
            }
        }
        if b.get(off + 0x4906b90..off + 0x4906b98) != Some(&0x60000977_u64.to_le_bytes()) {
            return Err("Unsupported camp callback metadata reference".into());
        }
    }
    // coin_bag::locate verifies all sections end before its earlier cave.
    // Check our later guard also remains inside the executable __TEXT file range.
    let mut p = off + 32;
    let mut fits = false;
    for _ in 0..word(b, off + 16)? {
        let cmd = word(b, p)?;
        let size = word(b, p + 4)? as usize;
        if size < 8 {
            return Err("Invalid Mach-O command".into());
        }
        if cmd == 25 && b.get(p + 8..p + 14) == Some(b"__TEXT") {
            let fs = u64::from_le_bytes(
                b.get(p + 48..p + 56)
                    .ok_or("Truncated segment")?
                    .try_into()
                    .unwrap(),
            );
            fits = fs >= (cave + GUARD_LEN) as u64 && word(b, p + 60)? & 4 != 0;
        }
        p = p.checked_add(size).ok_or("Invalid Mach-O layout")?;
    }
    if !fits {
        return Err("Camp guard does not fit executable padding".into());
    }
    Ok(off)
}
// 0 = original, 1 = old preservation-only patch, 2 = detached-camp patch.
fn mode(b: &[u8]) -> Result<u8, String> {
    let off = locate(b)?;
    #[allow(non_snake_case)]
    let (hook, cave, _, fade) = layout(b, off)?;
    let guard = b
        .get(off + cave..off + cave + GUARD_LEN)
        .ok_or("Truncated guard")?;
    if word(b, off + hook)? == branch(hook, fade, true) && guard.iter().all(|x| *x == 0) {
        return Ok(0);
    }
    if word(b, off + hook)? == branch(hook, cave, false) && guard == guard_for(b, off)? {
        return Ok(2);
    }
    if hook == HOOK
        && word(b, off + hook)? == branch(hook, cave, false)
        && guard[..104] == legacy_code()
        && guard[104..].iter().all(|x| *x == 0)
    {
        return Ok(1);
    }
    Err("Unrecognized camp patch or occupied code cave".into())
}
fn read(b: &[u8]) -> Result<bool, String> {
    Ok(mode(b)? != 0)
}
fn patch(b: &mut [u8], enabled: bool) -> Result<(), String> {
    read(b)?;
    let off = locate(b)?;
    let guard = guard_for(b, off)?;
    #[allow(non_snake_case)]
    let (hook_address, cave, _, fade) = layout(b, off)?;
    let hook = if enabled {
        branch(hook_address, cave, false)
    } else {
        branch(hook_address, fade, true)
    };
    b[off + hook_address..off + hook_address + 4].copy_from_slice(&hook.to_le_bytes());
    b[off + cave..off + cave + GUARD_LEN].copy_from_slice(&if enabled {
        guard
    } else {
        vec![0; GUARD_LEN]
    });
    Ok(())
}
fn check_metadata(directory: &Path) -> Result<(), String> {
    let b = fs::read(directory.join("il2cpp_data/Metadata/global-metadata.dat"))
        .map_err(|e| e.to_string())?;
    let strings = word(&b, 24)? as usize;
    let methods = word(&b, 48)? as usize;
    let name = strings
        .checked_add(word(&b, methods + 1211 * 36)? as usize)
        .ok_or("Invalid metadata")?;
    if b.get(name..name + 17) != Some(b"SpawnCitizenHouse") {
        return Err("Unsupported camp callback metadata".into());
    }
    Ok(())
}
#[tauri::command]
pub fn load_camp_preservation(data_directory: String) -> Result<Response, String> {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    let dir = Path::new(&data_directory);
    let p = game_assembly_path(dir)?;
    check_metadata(dir)?;
    let state = mode(&fs::read(p).map_err(|e| e.to_string())?)?;
    Ok(Response {
        enabled: state != 0,
        needs_upgrade: state == 1,
        backup: None,
    })
}
#[tauri::command]
pub fn apply_camp_preservation(data_directory: String, enabled: bool) -> Result<Response, String> {
    use std::process::Command;
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before applying changes".into());
    }
    let dir = Path::new(&data_directory);
    let p = game_assembly_path(dir)?;
    check_metadata(dir)?;
    let mut b = fs::read(&p).map_err(|e| e.to_string())?;
    if mode(&b)? == if enabled { 2 } else { 0 } {
        return Ok(Response {
            enabled,
            needs_upgrade: false,
            backup: None,
        });
    }
    patch(&mut b, enabled)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let backup = p.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.bak"));
    let temp = p.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.tmp"));
    fs::copy(&p, &backup).map_err(|e| e.to_string())?;
    let result = (|| -> Result<(), String> {
        fs::write(&temp, &b).map_err(|e| e.to_string())?;
        fs::set_permissions(
            &temp,
            fs::metadata(&p).map_err(|e| e.to_string())?.permissions(),
        )
        .map_err(|e| e.to_string())?;
        for args in [vec!["--force", "--sign", "-"], vec!["--verify", "--strict"]] {
            let r = Command::new("codesign")
                .args(args)
                .arg(&temp)
                .output()
                .map_err(|e| e.to_string())?;
            if !r.status.success() {
                return Err(format!(
                    "Code signing failed: {}",
                    String::from_utf8_lossy(&r.stderr)
                ));
            }
        }
        if read(&fs::read(&temp).map_err(|e| e.to_string())?)? != enabled {
            return Err("Camp patch verification failed".into());
        }
        fs::rename(&temp, &p).map_err(|e| e.to_string())?;
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result.map_err(|e| format!("{e}. Original file unchanged. Backup: {}", backup.display()))?;
    Ok(Response {
        enabled,
        needs_upgrade: false,
        backup: Some(backup.to_string_lossy().into()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_and_preserve_coin_patch() {
        let p=Path::new("/Users/quangvictornguyen/Documents/Codex/2026-09-19/l/camp-detach-test/KingdomTwoCrowns-CampTest.app/Contents/Frameworks/GameAssembly.dylib");
        if !p.exists() {
            return;
        }
        let mut b = fs::read(p).unwrap();
        patch(&mut b, false).unwrap();
        let off = locate(&b).unwrap();
        b[off + HOOK..off + HOOK + 4].copy_from_slice(&branch(HOOK, CAVE, false).to_le_bytes());
        b[off + CAVE..off + CAVE + 104].copy_from_slice(&legacy_code());
        assert_eq!(mode(&b).unwrap(), 1);
        patch(&mut b, true).unwrap();
        assert_eq!(mode(&b).unwrap(), 2);
        patch(&mut b, false).unwrap();
        assert_eq!(mode(&b).unwrap(), 0);
        let before = b.clone();
        let state = read(&b).unwrap();
        let off = locate(&b).unwrap();
        patch(&mut b, !state).unwrap();
        assert_eq!(read(&b).unwrap(), !state);
        assert_eq!(
            &b[off + 0x44c6c90..off + 0x44c6ca8],
            &before[off + 0x44c6c90..off + 0x44c6ca8]
        );
        assert!(b.iter().zip(&before).enumerate().all(|(i, (a, z))| a == z
            || (off + HOOK..off + HOOK + 4).contains(&i)
            || (off + CAVE..off + CAVE + GUARD_LEN).contains(&i)));
        patch(&mut b, state).unwrap();
        assert_eq!(b, before);
        b[off + CONTINUE] ^= 1;
        assert!(read(&b).is_err());
    }
    #[test]
    fn updated_build_roundtrip_changes_only_reserved_bytes() {
        let p = Path::new("/Users/quangvictornguyen/Library/Application Support/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns.app/Contents/Frameworks/GameAssembly.dylib");
        if !p.exists() {
            return;
        }
        let mut original = fs::read(p).unwrap();
        // Start from an unpatched in-memory baseline even if the user enabled it.
        patch(&mut original, false).unwrap();
        let off = locate(&original).unwrap();
        let (hook, cave, _, _) = layout(&original, off).unwrap();
        if hook == HOOK {
            return;
        }
        let mut b = original.clone();
        patch(&mut b, true).unwrap();
        assert_eq!(mode(&b).unwrap(), 2);
        assert!(b.iter().zip(&original).enumerate().all(|(i, (a, z))| a == z
            || (off + hook..off + hook + 4).contains(&i)
            || (off + cave..off + cave + GUARD_LEN).contains(&i)));
        patch(&mut b, false).unwrap();
        assert!(
            b == original,
            "Restoring must recover the in-memory baseline"
        );
        b[off + 0x66202c] ^= 1;
        assert!(patch(&mut b, true).is_err());
    }
    #[test]
    fn matches_gameplay_tested_artifact() {
        let p = Path::new(
            "/Users/quangvictornguyen/Documents/Codex/2026-09-19/l/camp-detach-test/guard.bin",
        );
        if p.exists() {
            assert_eq!(code(), fs::read(p).unwrap());
        }
    }
}
