//! Archive helpers. Demo target for ProofPatch: intentionally vulnerable. Never deploy this code.
use std::fs;
use std::path::Path;
use std::process::Command;

/// Checksum line for an archive, computed by the system tool.
pub fn checksum(file: &str) -> String {
    let out = Command::new("sh").arg("-c").arg(format!("cksum {}", file)).output().expect("run cksum");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Contents of a file inside the archive folder.
pub fn read_entry(root: &str, name: &str) -> std::io::Result<String> {
    let base = Path::new(root);
    fs::read_to_string(base.join(name))
}

/// Human-readable size.
pub fn size_label(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_sizes() {
        assert_eq!(size_label(512), "512 B");
        assert_eq!(size_label(2048), "2.0 KB");
    }

    #[test]
    fn reads_entry() {
        let dir = std::env::temp_dir().join("archive-rs-test");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.txt"), "ok").unwrap();
        assert_eq!(read_entry(dir.to_str().unwrap(), "a.txt").unwrap(), "ok");
    }

    #[test]
    fn checksums_a_file() {
        let dir = std::env::temp_dir().join("archive-rs-test2");
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("b.txt");
        fs::write(&file, "data").unwrap();
        assert!(checksum(file.to_str().unwrap()).ends_with("b.txt"));
    }
}
