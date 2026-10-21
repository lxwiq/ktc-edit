use super::*;
const HOOK: usize = 0xb12460;
const CAVE: usize = 0x44c6c90;
const ORIGINAL: u32 = 0xd360feba;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    limit: Option<u16>,
    backup: Option<String>,
}

fn word(b: &[u8], p: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        b.get(p..p + 4)
            .ok_or("Truncated dylib")?
            .try_into()
            .unwrap(),
    ))
}
fn branch(a: usize, z: usize) -> u32 {
    0x14000000 | (((z as i64 - a as i64) / 4) as u32 & 0x3ffffff)
}
#[cfg(test)]
fn code(limit: u16) -> Vec<u8> {
    code_at(limit, HOOK, CAVE)
}
fn code_at(limit: u16, hook: usize, cave: usize) -> Vec<u8> {
    [
        ORIGINAL,
        0x35000095,
        0x52800009 | (u32::from(limit) << 5),
        0x6b09035f,
        0x1a89d35a,
        branch(cave + 20, hook + 4),
    ]
    .into_iter()
    .flat_map(u32::to_le_bytes)
    .collect()
}
#[allow(non_snake_case)]
pub(super) fn layout(b: &[u8], off: usize) -> Result<(usize, usize), String> {
    for (hook, cave) in [(HOOK, CAVE), (0xb1512c, 0x44d76b0)] {
        if word(b, off + hook - 4)? == 0xaa0003e8
            && word(b, off + hook + 4)? == 0xf9403a60
            && word(b, off + hook + 8)? == 0x6b1a011f
        {
            return Ok((hook, cave));
        }
    }
    Err("Unsupported ShowCurrency code".into())
}
pub(super) fn locate(b: &[u8]) -> Result<usize, String> {
    if b.get(..4) != Some(&[0xca, 0xfe, 0xba, 0xbe]) {
        return Err("Unsupported dylib container".into());
    }
    let be = |p: usize| -> Result<usize, String> {
        Ok(u32::from_be_bytes(
            b.get(p..p + 4)
                .ok_or("Truncated Mach-O")?
                .try_into()
                .unwrap(),
        ) as usize)
    };
    let mut found = None;
    for i in 0..be(4)?.min(32) {
        let p = 8 + i * 20;
        if be(p)? == 0x100000c {
            if found.is_some() {
                return Err("Ambiguous ARM64 slice".into());
            }
            let off = be(p + 8)?;
            let size = be(p + 12)?;
            if size < CAVE + 24 || off.checked_add(size).is_none_or(|end| end > b.len()) {
                return Err("Unsupported ARM64 layout".into());
            }
            found = Some(off);
        }
    }
    let off = found.ok_or("No ARM64 slice")?;
    #[allow(non_snake_case)]
    let (hook, cave) = layout(b, off)?;
    // Exact surrounding instructions for the gameplay-tested build, fail closed.
    if word(b, off + hook - 4)? != 0xaa0003e8
        || word(b, off + hook + 4)? != 0xf9403a60
        || word(b, off + hook + 8)? != 0x6b1a011f
    {
        return Err("Unsupported ShowCurrency code".into());
    }
    // Verify code cave is unused executable segment padding, not a section.
    let mut p = off + 32;
    let mut executable = false;
    for _ in 0..word(b, off + 16)? {
        let cmd = word(b, p)?;
        let size = word(b, p + 4)? as usize;
        if size < 8 {
            return Err("Invalid Mach-O command".into());
        }
        if cmd == 25 && b.get(p + 8..p + 14) == Some(b"__TEXT") {
            let q = |at| -> Result<u64, String> {
                Ok(u64::from_le_bytes(
                    b.get(at..at + 8)
                        .ok_or("Truncated segment")?
                        .try_into()
                        .unwrap(),
                ))
            };
            if q(p + 24)? != 0
                || q(p + 40)? != 0
                || q(p + 48)? < (cave + 24) as u64
                || word(b, p + 60)? & 4 == 0
            {
                return Err("Invalid executable padding".into());
            }
            for i in 0..word(b, p + 64)? as usize {
                let s = p + 72 + i * 80;
                if q(s + 32)?
                    .checked_add(q(s + 40)?)
                    .is_none_or(|end| end > cave as u64)
                {
                    return Err("Code cave overlaps a section".into());
                }
            }
            executable = true;
        }
        p = p.checked_add(size).ok_or("Invalid commands")?;
    }
    if !executable {
        return Err("No executable code cave".into());
    }
    Ok(off)
}
fn read(b: &[u8]) -> Result<Option<u16>, String> {
    let o = locate(b)?;
    #[allow(non_snake_case)]
    let (hook_address, cave) = layout(b, o)?;
    let hook = word(b, o + hook_address)?;
    if hook == ORIGINAL && b[o + cave..o + cave + 24].iter().all(|x| *x == 0) {
        return Ok(None);
    }
    if hook != branch(hook_address, cave) {
        return Err("Unknown coin bag patch".into());
    }
    let mov = word(b, o + cave + 8)?;
    let limit = ((mov >> 5) & 0xffff) as u16;
    if limit == 0 || b[o + cave..o + cave + 24] != code_at(limit, hook_address, cave) {
        return Err("Invalid coin bag trampoline".into());
    }
    Ok(Some(limit))
}
fn patch(b: &mut [u8], limit: Option<u16>) -> Result<(), String> {
    read(b)?;
    if limit == Some(0) {
        return Err("Visual limit must be positive (ARM64 immediate maximum: 65535)".into());
    }
    let o = locate(b)?;
    #[allow(non_snake_case)]
    let (hook, cave) = layout(b, o)?;
    b[o + hook..o + hook + 4].copy_from_slice(
        &limit
            .map(|_| branch(hook, cave))
            .unwrap_or(ORIGINAL)
            .to_le_bytes(),
    );
    b[o + cave..o + cave + 24].copy_from_slice(
        &limit
            .map(|n| code_at(n, hook, cave))
            .unwrap_or_else(|| vec![0; 24]),
    );
    Ok(())
}
#[tauri::command]
pub fn load_coin_visual_limit(data_directory: String) -> Result<Response, String> {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    let p = game_assembly_path(Path::new(&data_directory))?;
    Ok(Response {
        limit: read(&fs::read(p).map_err(|e| e.to_string())?)?,
        backup: None,
    })
}
#[tauri::command]
pub fn apply_coin_visual_limit(
    data_directory: String,
    limit: Option<u16>,
) -> Result<Response, String> {
    use std::process::Command;
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before applying changes".into());
    }
    let p = game_assembly_path(Path::new(&data_directory))?;
    let mut b = fs::read(&p).map_err(|e| e.to_string())?;
    if read(&b)? == limit {
        return Ok(Response {
            limit,
            backup: None,
        });
    }
    patch(&mut b, limit)?;
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
        let sign = Command::new("codesign")
            .args(["--force", "--sign", "-"])
            .arg(&temp)
            .output()
            .map_err(|e| e.to_string())?;
        if !sign.status.success() {
            return Err(String::from_utf8_lossy(&sign.stderr).into());
        }
        let verify = Command::new("codesign")
            .args(["--verify", "--strict"])
            .arg(&temp)
            .output()
            .map_err(|e| e.to_string())?;
        if !verify.status.success() {
            return Err("Signature verification failed".into());
        }
        if read(&fs::read(&temp).map_err(|e| e.to_string())?)? != limit {
            return Err("Patch verification failed".into());
        }
        fs::rename(&temp, &p).map_err(|e| e.to_string())?;
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result?;
    Ok(Response {
        limit,
        backup: Some(backup.to_string_lossy().into()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tested_trampoline_encoding() {
        assert_eq!(
            code(30),
            [
                0xd360feba_u32,
                0x35000095,
                0x528003c9,
                0x6b09035f,
                0x1a89d35a,
                branch(CAVE + 20, HOOK + 4)
            ]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>()
        );
    }
    #[test]
    fn installed_read_patch_roundtrip() {
        let p=Path::new("/Users/quangvictornguyen/Library/Application Support/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns.app/Contents/Frameworks/GameAssembly.dylib");
        if !p.exists() {
            return;
        }
        let mut b = fs::read(p).unwrap();
        let original = b.clone();
        #[allow(non_snake_case)]
        let (hook, _) = layout(&original, locate(&original).unwrap()).unwrap();
        let old = read(&b).unwrap();
        for n in [1, 30, 2001, 5000, u16::MAX] {
            patch(&mut b, Some(n)).unwrap();
            assert_eq!(read(&b).unwrap(), Some(n));
        }
        patch(&mut b, old).unwrap();
        assert_eq!(b, original);
        assert!(patch(&mut b, Some(0)).is_err());
        b[locate(&original).unwrap() + hook + 4] ^= 1;
        assert!(read(&b).is_err());
    }
}
