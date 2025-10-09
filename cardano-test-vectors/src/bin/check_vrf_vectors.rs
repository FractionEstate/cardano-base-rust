use std::{collections::BTreeMap, env, ffi::OsStr, fs, path::Path, process};

const PREFIX: &str = "vrf_ver13";

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(format!(
            "usage: {} <reference-dir> <local-dir>",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("check_vrf_vectors")
        ));
    }

    let reference_dir = Path::new(&args[1]);
    let local_dir = Path::new(&args[2]);

    let reference_vectors = load_vectors(reference_dir)?;
    let mut local_vectors = load_vectors(local_dir)?;

    if reference_vectors.is_empty() {
        return Err(format!(
            "no {PREFIX} files found in {}",
            reference_dir.display()
        ));
    }

    let mut mismatches = Vec::new();

    for (name, reference_bytes) in &reference_vectors {
        match local_vectors.remove(name) {
            Some(local_bytes) => {
                if local_bytes != *reference_bytes {
                    mismatches.push(format!(
                        "content mismatch for {name}: {} vs {}",
                        reference_dir.display(),
                        local_dir.display()
                    ));
                }
            },
            None => mismatches.push(format!("missing {name} in {}", local_dir.display())),
        }
    }

    for extra in local_vectors.keys() {
        mismatches.push(format!(
            "extra file {extra} present in {} but not in {}",
            local_dir.display(),
            reference_dir.display()
        ));
    }

    if !mismatches.is_empty() {
        let mut message = String::from("vector comparison failed:\n");
        for mismatch in mismatches {
            message.push_str("  - ");
            message.push_str(&mismatch);
            message.push('\n');
        }
        return Err(message);
    }

    println!(
        "All {count} {PREFIX} vectors match between {ref_dir} and {local_dir}",
        count = reference_vectors.len(),
        ref_dir = reference_dir.display(),
        local_dir = local_dir.display()
    );

    Ok(())
}

fn load_vectors(dir: &Path) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut vectors = BTreeMap::new();

    let read_dir =
        fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))?;

    for entry in read_dir {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(os_string) => {
                if starts_with_prefix(os_string.as_os_str()) {
                    return Err(format!("non-utf8 file name under {}", dir.display()));
                }
                continue;
            },
        };

        if !file_name.starts_with(PREFIX) {
            continue;
        }

        let bytes =
            fs::read(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        vectors.insert(file_name, bytes);
    }

    Ok(vectors)
}

fn starts_with_prefix(name: &OsStr) -> bool {
    name.to_str()
        .map(|s| s.starts_with(PREFIX))
        .unwrap_or(false)
}
