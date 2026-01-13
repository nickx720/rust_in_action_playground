use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result};
use flate2::bufread::GzDecoder;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use tar::Archive;

use crate::write_file;

#[derive(Deserialize, Debug)]
struct DockerToken {
    access_token: String,
    token: String,
    expires_in: u32,
    issued_at: String,
}

fn safe_join(base: &Path, entry_path: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::from(base);
    for comp in entry_path.components() {
        match comp {
            Component::Prefix(_) | Component::RootDir | Component::ParentDir => return None,
            Component::CurDir => {}
            Component::Normal(p) => out.push(p),
        }
    }
    Some(out)
}
pub fn get_docker_manifest() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let auth_token = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/busybox:pull";
    let auth_response: DockerToken = client
        .get(auth_token)
        .header(CONTENT_TYPE, "application/json")
        .send()?
        .json()?;
    let resource = "https://registry-1.docker.io/v2/library/busybox/manifests/latest";
    let resp: serde_json::Value = client
        .get(resource)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", auth_response.access_token),
        )
        .header(
            ACCEPT,
            "application/vnd.docker.distribution.manifest.v2+json",
        )
        .send()?
        .json()?;
    let _ = write_file("respose.json", &resp.to_string()).unwrap();
    let arm_64_digest = resp.get("manifests").unwrap().as_array().unwrap()[7]
        .get("digest")
        .unwrap()
        .as_str()
        .unwrap();
    let resource = format!(
        "https://registry-1.docker.io/v2/library/busybox/manifests/{}",
        arm_64_digest.to_string().trim()
    );
    let resp: serde_json::Value = client
        .get(resource)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", auth_response.access_token),
        )
        .header(
            ACCEPT,
            "application/vnd.docker.distribution.manifest.v2+json",
        )
        .send()?
        .json()?;
    let output = Path::new("/mnt/hgfs/rust-docker/dist/output");
    let layers: &Vec<serde_json::Value> = resp.get("layers").unwrap().as_array().unwrap();
    let config = resp
        .get("config")
        .unwrap()
        .as_object()
        .unwrap()
        .get("digest")
        .unwrap()
        .as_str()
        .unwrap();
    let url = format!(
        "https://registry-1.docker.io/v2/library/busybox/blobs/{}",
        config
    );
    let mut resp = client
        .get(url)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", auth_response.access_token),
        )
        .header(USER_AGENT, "rust-reqwest-blocking/0.1")
        .send()?;
    // storing config body into a file
    let file = File::create("/mnt/hgfs/rust-docker/dist/config.json")?;
    let writer = BufWriter::new(file);
    let json: serde_json::Value = resp.json()?;
    serde_json::to_writer_pretty(writer, &json)?;
    for layer in layers {
        let layer = layer
            .get("digest")
            .expect("digest not found")
            .as_str()
            .expect("digest to string conv failed");
        let url = format!(
            "https://registry-1.docker.io/v2/library/busybox/blobs/{}",
            layer
        );
        // Get the layer
        let mut resp = client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", auth_response.access_token),
            )
            .header(USER_AGENT, "rust-reqwest-blocking/0.1")
            .send()?;
        let gz = GzDecoder::new(BufReader::new(resp));
        let mut archive = Archive::new(gz);
        for entries in archive.entries().context("Error reading") {
            for entry in entries {
                let mut entry = entry.context("reading tar entry")?;
                let entry_type = entry.header().entry_type();
                // Resolve and sanitize path inside dest_dir
                let raw_path = entry.path().context("getting entry path")?;
                let outpath = match safe_join(output, raw_path.as_ref()) {
                    Some(p) => p,
                    None => continue, // skip suspicious paths
                };
                // Create parent directories
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).with_context(|| format!("creating {:?}", parent))?;
                }
                // Handle only regular files and directories for safety
                if entry_type.is_dir() {
                    fs::create_dir_all(&outpath)?;
                } else if entry_type.is_file() {
                    entry
                        .unpack(&outpath)
                        .with_context(|| format!("writing {:?}", outpath))?;
                } else {
                    // why are we skipping bin files
                    // Skip symlinks/hardlinks/devs for safety; handle explicitly if needed
                    // doesn't not generate symlinks when running without root access
                    continue;
                }
            }
        }
        // Get the config
    }
    Ok(())
}
