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
pub struct Response { enabled: bool, backup: Option<String>, save_backup: Option<String>, converted_events: usize, added_events: usize, editor_data: Option<Value> }

fn convert_events(doc: &mut Value) -> Result<(usize, usize), String> {
    let mut converted = 0;
    let mut added = 0;
    let Some(campaigns) = doc.get_mut("campaigns").and_then(Value::as_array_mut) else { return Ok((0,0)); };
    for campaign in campaigns {
        let Some(islands) = campaign.get_mut("_islands").and_then(Value::as_array_mut) else { continue; };
        for island in islands {
            let Some(objects) = island.get_mut("objects").and_then(Value::as_array_mut) else { continue; };
            for object in objects {
                let Some(components) = object.get_mut("componentData2").and_then(Value::as_array_mut) else { continue; };
                for component in components {
                    if component.get("type").and_then(Value::as_str) != Some("DirectorData") { continue; }
                    let raw = component.get("data").ok_or("Missing DirectorData payload")?;
                    let string_payload = raw.is_string();
                    let mut director: Value = if let Some(s) = raw.as_str() { serde_json::from_str(s).map_err(|e|format!("Invalid DirectorData: {e}"))? } else { raw.clone() };
                    let day = director.get("daysInReign").and_then(Value::as_i64).ok_or("Missing Director day")?;
                    let time = director.get("currentTime").and_then(Value::as_f64).ok_or("Missing Director time")?;
                    let events = director.get_mut("events").and_then(Value::as_array_mut).ok_or("Missing Director events")?;
                    let mut changed = false;
                    for event in events.iter_mut() {
                        let event_day = event.get("startDay").and_then(Value::as_i64).ok_or("Missing event day")?;
                        let pending = event_day > day || (event_day == day && event.get("startTime").and_then(Value::as_f64).unwrap_or(0.0) >= time);
                        if !pending || !matches!(event.get("type").and_then(Value::as_i64), Some(2 | 4)) { continue; }
                        let boss = match event.get("waveName").and_then(Value::as_str).unwrap_or("") {
                            "Regular" => "Boss", "Dead Lands Regular" => "Dead Lands Boss",
                            "Norselands Regular" => "Norselands Boss", "Plague_Regular" => "Plague_BloodMoon",
                            "Trade Routes Regular" => "Trade Routes Boss", "SkullIsland" => "SkullIsland Boss",
                            "Anniversary_Regular_Wave" => "Anniversary_Boss_Wave", "DireIslandRegular" => "DireIslandBloodMoon",
                            _ => continue,
                        };
                        event["waveName"] = Value::String(boss.into());
                        converted += 1;
                        changed = true;
                    }
                    // Existing Boss schedules also need a missing side filled in.
                    // Group type 2 and 4 together: one side may already be scheduled
                    // at a concrete time while its counterpart is still type 4.
                    let mut groups = std::collections::BTreeMap::<_, (Value, std::collections::BTreeSet<i64>)>::new();
                    for event in events.iter() {
                        let event_day = event["startDay"].as_i64().unwrap_or(-1);
                        let pending = event_day > day || (event_day == day && event["startTime"].as_f64().unwrap_or(0.0) >= time);
                        let name = event["waveName"].as_str().unwrap_or("");
                        if !pending || !matches!(event["type"].as_i64(),Some(2 | 4)) || !matches!(name,
                            "Boss" | "Dead Lands Boss" | "Norselands Boss" | "Plague_BloodMoon" |
                            "Trade Routes Boss" | "SkullIsland Boss" | "Anniversary_Boss_Wave" | "DireIslandBloodMoon") { continue; }
                        let side = event["side"].as_i64().ok_or("Missing Boss wave side")?;
                        if side != -1 && side != 1 { return Err("Unsupported Boss wave side; save unchanged".into()); }
                        groups.entry((event_day,name.to_string())).or_insert_with(||(event.clone(),Default::default())).1.insert(side);
                    }
                    for (_, (template, sides)) in groups {
                        for side in [-1,1] {
                            if sides.contains(&side) { continue; }
                            let mut counterpart = template.clone();
                            counterpart["side"] = side.into();
                            events.push(counterpart);
                            added += 1;
                            changed = true;
                        }
                    }
                    if changed { component["data"] = if string_payload { Value::String(serde_json::to_string(&director).map_err(|e|e.to_string())?) } else { director }; }
                }
            }
        }
    }
    Ok((converted,added))
}

fn decode_save(bytes: &[u8]) -> Result<Value,String> {
    let mut text = String::new();
    GzDecoder::new(bytes).read_to_string(&mut text).map_err(|e|e.to_string())?;
    serde_json::from_str(&text).map_err(|e|e.to_string())
}

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
    Ok(Response { enabled: state(&fs::read(p).map_err(|e|e.to_string())?)?, backup: None, save_backup: None, converted_events: 0, added_events: 0, editor_data: None })
}
#[tauri::command]
pub fn apply_blood_moon(data_directory: String, enabled: bool, save_path: Option<String>, editor_data: Option<Value>) -> Result<Response,String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) { return Err("Apple Silicon Macs only".into()); }
    if game_is_running() { return Err("Close Kingdom Two Crowns before applying changes".into()); }
    let p = game_assembly_path(Path::new(&data_directory))?;
    let mut b = fs::read(&p).map_err(|e|e.to_string())?;
    let native_original = b.clone();
    let native_changed = state(&b)? != enabled;
    // Prepare and validate the save before modifying either file.
    let mut save = None;
    let mut converted_events = 0;
    let mut added_events = 0;
    let mut editor_data = editor_data;
    if enabled {
        let path = save_path.map(PathBuf::from).or_else(default_save_file_path).ok_or("Choose a global-v35 save before enabling Blood Moon")?;
        validate_path(&path)?;
        let original = fs::read(&path).map_err(|e|e.to_string())?;
        let mut doc = decode_save(&original)?;
        (converted_events, added_events) = convert_events(&mut doc)?;
        if let Some(editor) = editor_data.as_mut() { convert_events(editor)?; }
        if converted_events > 0 || added_events > 0 {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&serde_json::to_vec(&doc).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let compressed = encoder.finish().map_err(|e|e.to_string())?;
            if decode_save(&compressed)? != doc { return Err("Save preparation verification failed".into()); }
            save = Some((path, original, compressed, doc));
        }
    }
    patch(&mut b,enabled)?;
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
    let backup = p.with_file_name(format!("{GAME_ASSEMBLY}.bloodmoon.{stamp}.bak"));
    let temp = p.with_file_name(format!("{GAME_ASSEMBLY}.bloodmoon.{stamp}.tmp"));
    if native_changed { fs::copy(&p,&backup).map_err(|e|e.to_string())?; }
    let mut save_backup = None;
    let result = (|| -> Result<(),String> {
        fs::write(&temp,&b).map_err(|e|e.to_string())?;
        fs::set_permissions(&temp,fs::metadata(&p).map_err(|e|e.to_string())?.permissions()).map_err(|e|e.to_string())?;
        for args in [vec!["--force","--sign","-"],vec!["--verify","--strict"]] {
            let r = std::process::Command::new("codesign").args(args).arg(&temp).output().map_err(|e|e.to_string())?;
            if !r.status.success() { return Err(format!("Code signing failed: {}",String::from_utf8_lossy(&r.stderr))); }
        }
        if state(&fs::read(&temp).map_err(|e|e.to_string())?)? != enabled { return Err("Blood Moon verification failed".into()); }
        if game_is_running() { return Err("Game started during preparation; close it and retry".into()); }
        if fs::read(&p).map_err(|e|e.to_string())? != native_original { return Err("GameAssembly changed concurrently; retry".into()); }
        if let Some((path, original, compressed, doc)) = &save {
            if fs::read(path).map_err(|e|e.to_string())? != *original { return Err("Save changed concurrently; retry".into()); }
            save_backup = Some(create_backup(path)?.to_string_lossy().into_owned());
            atomic_replace(path, compressed)?;
            let verification = fs::read(path).map_err(|e|e.to_string()).and_then(|v|decode_save(&v));
            if !verification.as_ref().is_ok_and(|v| v == doc) {
                return Err("Save verification failed".into());
            }
        }
        if native_changed { fs::rename(&temp,&p).map_err(|e|e.to_string())?; }
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    if let Err(e) = result {
        let rollback = if save_backup.is_some() { save.as_ref().map(|(path,original,_,_)|atomic_replace(path,original)) } else { None };
        return Err(format!("{e}. Save rollback: {rollback:?}. Save backup: {save_backup:?}"));
    }
    Ok(Response { enabled, backup: native_changed.then(||backup.to_string_lossy().into()), save_backup, converted_events, added_events, editor_data })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_boss_gets_missing_side_without_duplicates() {
        let director = serde_json::json!({"daysInReign":10,"currentTime":12,"events":[
            {"type":2,"startDay":10,"startTime":22,"waveName":"Boss","side":1},
            {"type":4,"startDay":11,"startTime":0,"waveName":"Boss","side":-1},
            {"type":2,"startDay":12,"startTime":22,"waveName":"Boss","side":1},
            {"type":4,"startDay":12,"startTime":0,"waveName":"Boss","side":-1},
            {"type":2,"startDay":9,"startTime":22,"waveName":"Boss","side":1}
        ]});
        let mut doc=serde_json::json!({"campaigns":[{"_islands":[{"objects":[{"componentData2":[{"type":"DirectorData","data":director}]}]}]}]});
        assert_eq!(convert_events(&mut doc).unwrap(),(0,2));
        let events=doc["campaigns"][0]["_islands"][0]["objects"][0]["componentData2"][0]["data"]["events"].as_array().unwrap();
        assert_eq!(events.len(),7);
        assert_eq!(events[5]["type"],2);
        assert_eq!(events[5]["side"],-1);
        assert_eq!(events[6]["type"],4);
        assert_eq!(events[6]["side"],1);
        let snapshot=doc.clone();
        assert_eq!(convert_events(&mut doc).unwrap(),(0,0));
        assert_eq!(doc,snapshot);
    }
    #[test]
    fn pending_campaign_waves_only_and_idempotent() {
        let events = serde_json::json!([
            {"type":2,"startDay":10,"startTime":5,"waveName":"Regular","side":-1},
            {"type":2,"startDay":10,"startTime":22,"waveName":"Regular","side":-1},
            {"type":2,"startDay":10,"startTime":23,"waveName":"Regular","side":1},
            {"type":4,"startDay":11,"startTime":0,"waveName":"Regular","side":1},
            {"type":4,"startDay":11,"startTime":0,"waveName":"Regular","side":-1},
            {"type":2,"startDay":11,"startTime":3,"waveName":"Retaliation","side":-1},
            {"type":1,"startDay":11,"startTime":3,"waveName":"Regular","side":-1}
        ]);
        let director = serde_json::json!({"daysInReign":10,"currentTime":12,"events":events});
        let mut doc = serde_json::json!({"campaigns":[{"_islands":[{"objects":[{"componentData2":[{"type":"DirectorData","data":director.to_string()}]}]}]}],"challenges":{"untouched":true}});
        assert_eq!(convert_events(&mut doc).unwrap(),(4,0));
        let d: Value = serde_json::from_str(doc["campaigns"][0]["_islands"][0]["objects"][0]["componentData2"][0]["data"].as_str().unwrap()).unwrap();
        assert_eq!(d["events"].as_array().unwrap().len(),7);
        assert_eq!(d["events"][0]["waveName"],"Regular");
        assert_eq!(d["events"][1]["waveName"],"Boss");
        assert_eq!(d["events"][5]["waveName"],"Retaliation");
        assert_eq!(doc["challenges"]["untouched"],true);
        let snapshot=doc.clone();
        assert_eq!(convert_events(&mut doc).unwrap(),(0,0));
        assert_eq!(doc,snapshot);
    }
    #[test]
    fn installed_roundtrip_preserves_every_other_byte() {
        let Ok(p) = game_assembly_path(&default_game_data_directory()) else { return; };
        let original = fs::read(p).unwrap();
        let mut b = original.clone();
        state(&b).unwrap();
        let at = location(&b).unwrap();
        let sites = wave_sites(&b,at).unwrap();
        for enabled in [true,false] {
            patch(&mut b,enabled).unwrap();
            assert_eq!(state(&b).unwrap(),enabled);
            assert!(b.iter().zip(&original).enumerate().all(|(i,(x,y))|
                x==y || (at..at+8).contains(&i) || sites.iter().any(|p| (*p..*p+4).contains(&i))));
        }
        // A previously enabled v1 has only the day override, not all selectors.
        // Restore the exact input bytes rather than assuming a canonical state.
        b[at..at+8].copy_from_slice(&original[at..at+8]);
        for p in sites { b[p..p+4].copy_from_slice(&original[p..p+4]); }
        assert!(b == original, "Roundtrip changed bytes outside patch sites");
        b[at+8] ^= 1;
        assert!(patch(&mut b,true).is_err());
    }
}
