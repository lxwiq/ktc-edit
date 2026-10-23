//! Experimental, build-specific ARM64 day classification override.
use super::*;
const RVA: usize = 0x9a8554;
const ORIGINAL: [u8; 8] = [0xff,0x03,0x01,0xd1,0xf6,0x57,0x01,0xa9];
const ENABLED: [u8; 8] = [0x80,0x00,0x80,0x52,0xc0,0x03,0x5f,0xd6];
const CONTEXT: [u8; 16] = [0xf4,0x4f,0x02,0xa9,0xfd,0x7b,0x03,0xa9,0xfd,0xc3,0x00,0x91,0x3f,0x04,0x00,0x71];
// AddCycle: recovery/normal-day branches must also select bossWave (+0x20).
// Retaliation and portal-defense paths are outside these three call sites.
const WAVE_SITES: [(usize, u32); 3] = [
    (0x61220c, 0xf9401502),
    (0x61226c, 0xf9400d02),
    (0x6122b0, 0xf9400d02),
];
const BOSS_LOAD: u32 = 0xf9401102;

fn wave_sites(b: &[u8], method: usize) -> Result<Vec<usize>, String> {
    let base = method.checked_sub(RVA).ok_or("Invalid ARM64 base")?;
    WAVE_SITES.iter().map(|(rva, original)| {
        let at = base + rva;
        let raw: [u8;4] = b.get(at..at+4).ok_or("Truncated wave selector")?.try_into().unwrap();
        let word = u32::from_le_bytes(raw);
        if word != *original && word != BOSS_LOAD { return Err("Unsupported AddCycle wave selector".into()); }
        // Following instruction must still set the Director instance argument.
        if b.get(at+4..at+8) != Some(0xaa1403e0u32.to_le_bytes().as_slice()) {
            return Err("Unsupported AddCycle wave context".into());
        }
        Ok(at)
    }).collect()
}

#[derive(Serialize)]
pub struct Response { enabled: bool, backup: Option<String> }

fn location(b: &[u8]) -> Result<usize,String> {
    let be = |at| -> Result<u32,String> {
        Ok(u32::from_be_bytes(b.get(at..at+4).ok_or("Truncated Mach-O")?.try_into().unwrap()))
    };
    if be(0)? != 0xcafebabe { return Err("Unsupported Blood Moon Mach-O".into()); }
    let count = be(4)? as usize;
    if count > 16 { return Err("Invalid Mach-O slices".into()); }
    let mut found = None;
    for i in 0..count {
        let a = 8+i*20;
        if be(a)? != 0x100000c { continue; }
        if found.is_some() { return Err("Ambiguous ARM64 slice".into()); }
        let base = be(a+8)? as usize;
        let size = be(a+12)? as usize;
        if RVA+24 > size { return Err("Truncated ARM64 slice".into()); }
        let at = base.checked_add(RVA).ok_or("Invalid method offset")?;
        if b.get(at+8..at+24) != Some(CONTEXT.as_slice()) {
            return Err("Unsupported Blood Moon method context".into());
        }
        if b.get(at..at+8) != Some(ORIGINAL.as_slice()) && b.get(at..at+8) != Some(ENABLED.as_slice()) {
            return Err("Unsupported Blood Moon method instructions".into());
        }
        found = Some(at);
    }
    found.ok_or_else(|| "ARM64 slice not found".into())
}
fn state(b: &[u8]) -> Result<bool,String> {
    let at = location(b)?;
    let sites = wave_sites(b,at)?;
    Ok(b[at..at+8] == ENABLED && sites.iter().all(|p| b[*p..*p+4] == BOSS_LOAD.to_le_bytes()))
}
fn patch(b: &mut [u8], enabled: bool) -> Result<(),String> {
    let at = location(b)?;
    let sites = wave_sites(b,at)?;
    b[at..at+8].copy_from_slice(if enabled { &ENABLED } else { &ORIGINAL });
    for ((_, original), at) in WAVE_SITES.iter().zip(sites) {
        b[at..at+4].copy_from_slice(&if enabled { BOSS_LOAD } else { *original }.to_le_bytes());
    }
    Ok(())
}
#[tauri::command]
pub fn load_blood_moon(data_directory: String) -> Result<Response,String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) { return Err("Apple Silicon Macs only".into()); }
    let p = game_assembly_path(Path::new(&data_directory))?;
    Ok(Response { enabled: state(&fs::read(p).map_err(|e|e.to_string())?)?, backup: None })
}
#[tauri::command]
pub fn apply_blood_moon(data_directory: String, enabled: bool) -> Result<Response,String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) { return Err("Apple Silicon Macs only".into()); }
    if game_is_running() { return Err("Close Kingdom Two Crowns before applying changes".into()); }
    let p = game_assembly_path(Path::new(&data_directory))?;
    let mut b = fs::read(&p).map_err(|e|e.to_string())?;
    if state(&b)? == enabled { return Ok(Response { enabled, backup: None }); }
    patch(&mut b,enabled)?;
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
    let backup = p.with_file_name(format!("{GAME_ASSEMBLY}.bloodmoon.{stamp}.bak"));
    let temp = p.with_file_name(format!("{GAME_ASSEMBLY}.bloodmoon.{stamp}.tmp"));
    fs::copy(&p,&backup).map_err(|e|e.to_string())?;
    let result = (|| -> Result<(),String> {
        fs::write(&temp,&b).map_err(|e|e.to_string())?;
        fs::set_permissions(&temp,fs::metadata(&p).map_err(|e|e.to_string())?.permissions()).map_err(|e|e.to_string())?;
        for args in [vec!["--force","--sign","-"],vec!["--verify","--strict"]] {
            let r = std::process::Command::new("codesign").args(args).arg(&temp).output().map_err(|e|e.to_string())?;
            if !r.status.success() { return Err(format!("Code signing failed: {}",String::from_utf8_lossy(&r.stderr))); }
        }
        if state(&fs::read(&temp).map_err(|e|e.to_string())?)? != enabled { return Err("Blood Moon verification failed".into()); }
        fs::rename(&temp,&p).map_err(|e|e.to_string())?;
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result.map_err(|e|format!("{e}. Original unchanged. Backup: {}",backup.display()))?;
    Ok(Response { enabled, backup: Some(backup.to_string_lossy().into()) })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn installed_roundtrip_preserves_every_other_byte() {
        let Ok(p) = game_assembly_path(&default_game_data_directory()) else { return; };
        let original = fs::read(p).unwrap();
        let mut b = original.clone();
        let initial = state(&b).unwrap();
        let at = location(&b).unwrap();
        let sites = wave_sites(&b,at).unwrap();
        for enabled in [true,false] {
            patch(&mut b,enabled).unwrap();
            assert_eq!(state(&b).unwrap(),enabled);
            assert!(b.iter().zip(&original).enumerate().all(|(i,(x,y))|
                x==y || (at..at+8).contains(&i) || sites.iter().any(|p| (*p..*p+4).contains(&i))));
        }
        patch(&mut b,initial).unwrap();
        assert_eq!(b,original);
        b[at+8] ^= 1;
        assert!(patch(&mut b,true).is_err());
    }
}
