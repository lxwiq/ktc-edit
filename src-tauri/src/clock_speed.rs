use super::*;
const RVA: usize = 0x611a8c;

fn site(bytes: &[u8]) -> Result<(usize,f32),String> {
    let be=|p| -> Result<u32,String> { Ok(u32::from_be_bytes(bytes.get(p..p+4).ok_or("Truncated Mach-O")?.try_into().unwrap())) };
    if be(0)?!=0xcafebabe || be(4)?>16 { return Err("Unsupported clock-speed Mach-O".into()); }
    let mut result=None;
    for i in 0..be(4)? as usize {
        let p=8+i*20;
        if be(p)?!=0x100000c { continue; }
        if result.is_some() { return Err("Ambiguous ARM64 slice".into()); }
        let base=be(p+8)? as usize; let size=be(p+12)? as usize;
        if RVA+12>size { return Err("Truncated ResumeTime".into()); }
        let at=base+RVA;
        let word=u32::from_le_bytes(bytes.get(at..at+4).ok_or("Truncated ResumeTime")?.try_into().unwrap());
        if word & 0xffe0001f != 0x52a00008 || bytes.get(at+4..at+12)!=Some([0x08,0x54,0x01,0xb9,0xc0,0x03,0x5f,0xd6].as_slice()) {
            return Err("Unsupported ResumeTime instructions; nothing written".into());
        }
        let value=f32::from_bits(((word>>5)&0xffff)<<16);
        if !value.is_finite() || value<=0.0 { return Err("Unsupported clock-speed instruction value".into()); }
        result=Some((at,value));
    }
    result.ok_or("ARM64 slice not found".into())
}

fn patch(bytes: &mut [u8], multiplier: f32) -> Result<(),String> {
    // ResumeTime has one MOVZ instruction available. Never overwrite the store/RET.
    if !multiplier.is_finite() || multiplier<=0.0 || multiplier.to_bits() & 0xffff != 0 {
        return Err("Enter a positive exactly supported multiplier, for example 1, 2, 3, 5 or 10".into());
    }
    let (at,_)=site(bytes)?;
    let word=0x52a00008 | ((multiplier.to_bits()>>16)<<5);
    bytes[at..at+4].copy_from_slice(&word.to_le_bytes());
    Ok(())
}

fn update_save(value: &mut Value, multiplier: f32) -> Result<usize,String> {
    let mut count=0;
    if let Some(object)=value.as_object_mut() {
        if object.get("type").and_then(Value::as_str)==Some("DirectorData") {
            let raw=object.get("data").ok_or("Missing DirectorData payload")?;
            let string=raw.is_string();
            let mut data: Value=if let Some(s)=raw.as_str() {serde_json::from_str(s).map_err(|e|format!("Invalid DirectorData: {e}"))?} else {raw.clone()};
            if !data.is_object() { return Err("Invalid DirectorData object".into()); }
            if data.get("clockSpeedModifier").and_then(Value::as_f64)!=Some(multiplier as f64) {
                data["clockSpeedModifier"]=serde_json::json!(multiplier);
                object.insert("data".into(),if string {Value::String(serde_json::to_string(&data).map_err(|e|e.to_string())?)} else {data});
                count+=1;
            }
        } else {
            for v in object.values_mut() {count+=update_save(v,multiplier)?;}
        }
    } else if let Some(array)=value.as_array_mut() {for v in array {count+=update_save(v,multiplier)?;}}
    Ok(count)
}
fn decode(bytes: &[u8]) -> Result<Value,String> {
    let mut s=String::new(); GzDecoder::new(bytes).read_to_string(&mut s).map_err(|e|e.to_string())?;
    serde_json::from_str(&s).map_err(|e|e.to_string())
}
#[derive(Serialize)]
pub struct Response { multiplier:f32, native_backup:Option<String>, save_backup:Option<String>, updated_directors:usize, editor_data:Option<Value> }

#[tauri::command]
pub fn load_clock_speed(data_directory:String) -> Result<f32,String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) {return Err("Apple Silicon Macs only".into());}
    let bytes=fs::read(game_assembly_path(Path::new(&data_directory))?).map_err(|e|e.to_string())?;
    Ok(site(&bytes)?.1)
}

#[tauri::command]
pub fn apply_clock_speed(data_directory:String, multiplier:f32, save_path:Option<String>, editor_data:Option<Value>) -> Result<Response,String> {
    if !cfg!(all(target_os="macos",target_arch="aarch64")) {return Err("Apple Silicon Macs only".into());}
    if game_is_running() {return Err("Close Kingdom Two Crowns first".into());}
    let native=game_assembly_path(Path::new(&data_directory))?;
    let original=fs::read(&native).map_err(|e|e.to_string())?;
    let mut patched=original.clone();patch(&mut patched,multiplier)?;
    let path=save_path.map(PathBuf::from).or_else(default_save_file_path).ok_or("Choose a global-v35 save first")?;
    validate_path(&path)?;
    let save_original=fs::read(&path).map_err(|e|e.to_string())?;
    let mut doc=decode(&save_original)?;
    let mut updated_directors=0;
    for key in ["campaigns","challenges"] {if let Some(v)=doc.get_mut(key) {updated_directors+=update_save(v,multiplier)?;}}
    let mut editor_data=editor_data;
    if let Some(editor)=editor_data.as_mut() {for key in ["campaigns","challenges"] {if let Some(v)=editor.get_mut(key) {update_save(v,multiplier)?;}}}
    let mut encoder=GzEncoder::new(Vec::new(),Compression::default());
    encoder.write_all(&serde_json::to_vec(&doc).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    let encoded=encoder.finish().map_err(|e|e.to_string())?;
    if decode(&encoded)?!=doc {return Err("Save preparation verification failed".into());}
    let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
    let temp=native.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.tmp"));
    let mut native_backup=None; let mut save_backup=None;
    let result=(|| -> Result<(),String> {
        if patched!=original {
            fs::write(&temp,&patched).map_err(|e|e.to_string())?;
            fs::set_permissions(&temp,fs::metadata(&native).map_err(|e|e.to_string())?.permissions()).map_err(|e|e.to_string())?;
            for args in [vec!["--force","--sign","-"],vec!["--verify","--strict"]] {
                let out=std::process::Command::new("codesign").args(args).arg(&temp).output().map_err(|e|e.to_string())?;
                if !out.status.success() {return Err(format!("Code signing failed: {}",String::from_utf8_lossy(&out.stderr)));}
            }
            if site(&fs::read(&temp).map_err(|e|e.to_string())?)?.1!=multiplier {return Err("Native verification failed".into());}
        }
        if game_is_running() || fs::read(&native).map_err(|e|e.to_string())?!=original || fs::read(&path).map_err(|e|e.to_string())?!=save_original {return Err("Game started or files changed; retry".into());}
        if patched!=original {let backup=native.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.bak"));fs::copy(&native,&backup).map_err(|e|e.to_string())?;native_backup=Some(backup.to_string_lossy().into_owned());}
        if updated_directors>0 {
            save_backup=Some(create_backup(&path)?.to_string_lossy().into_owned());
            atomic_replace(&path,&encoded)?;
            if decode(&fs::read(&path).map_err(|e|e.to_string())?)?!=doc {return Err("Save verification failed".into());}
        }
        if patched!=original {fs::rename(&temp,&native).map_err(|e|e.to_string())?;}
        Ok(())
    })();
    let _=fs::remove_file(&temp);
    if let Err(e)=result {
        let rollback=if save_backup.is_some() {Some(atomic_replace(&path,&save_original))} else {None};
        return Err(format!("{e}. Save rollback: {rollback:?}; backup: {save_backup:?}"));
    }
    Ok(Response{multiplier,native_backup,save_backup,updated_directors,editor_data})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn save_changes_only_clock_and_is_idempotent() {
        let mut doc=serde_json::json!({"type":"DirectorData","data":"{\"clockSpeedModifier\":1,\"currentTime\":12,\"events\":[1]}"});
        assert_eq!(update_save(&mut doc,3.0).unwrap(),1);
        let d:Value=serde_json::from_str(doc["data"].as_str().unwrap()).unwrap();
        assert_eq!(d,serde_json::json!({"clockSpeedModifier":3.0,"currentTime":12,"events":[1]}));
        assert_eq!(update_save(&mut doc,3.0).unwrap(),0);
    }
    #[test]
    fn native_patch_changes_only_mov_and_restores() {
        let Ok(path)=game_assembly_path(&default_game_data_directory()) else {return;};
        let original=fs::read(path).unwrap();let mut bytes=original.clone();let (at,old)=site(&bytes).unwrap();
        for speed in [3.0,1.0,10.0] {
            patch(&mut bytes,speed).unwrap();assert_eq!(site(&bytes).unwrap().1,speed);
            assert!(bytes.iter().zip(&original).enumerate().all(|(i,(a,b))|a==b || (at..at+4).contains(&i)));
        }
        patch(&mut bytes,old).unwrap();assert!(bytes==original);
        for invalid in [0.0,-1.0,f32::NAN,1.001] {assert!(patch(&mut bytes,invalid).is_err());assert!(bytes==original);}
        bytes[at+4]^=1;let before=bytes.clone();assert!(patch(&mut bytes,3.0).is_err());assert!(bytes==before);
    }
}
