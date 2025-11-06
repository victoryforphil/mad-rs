use anyhow::{Context, Result};
use log::{debug, info};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::osm_index::OSMPBDownloads;

pub struct OSMDownloader {
    data_dir: PathBuf,
}

impl OSMDownloader {
    /// Create a new OSMDownloader with the data directory at CRATE_PATH/data
    pub fn new() -> Result<Self> {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let data_dir = PathBuf::from(crate_path).join("data");

        // Create data directory if it doesn't exist
        fs::create_dir_all(&data_dir)
            .context(format!("Failed to create data directory: {:?}", data_dir))?;

        debug!("Initialized OSMDownloader with data directory: {:?}", data_dir);
        Ok(Self { data_dir })
    }

    /// Download a PBF file from the given OSMPBDownloads enum variant
    /// Checks if file exists and skips download if size is similar (within 1%)
    pub async fn download(&self, source: OSMPBDownloads) -> Result<PathBuf> {
        let url = source.url();
        let filename = source.filename();
        let file_path = self.data_dir.join(filename);

        // Check if file already exists
        if file_path.exists() {
            let local_size = fs::metadata(&file_path)
                .context("Failed to get local file metadata")?
                .len();
            
            info!("File exists locally: {:?} ({:.2} MB)", file_path, local_size as f64 / 1_000_000.0);

            // Check remote size with HEAD request
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .context("Failed to build HTTP client")?;

            let response = client
                .head(url)
                .send()
                .await
                .context(format!("Failed to check remote file size from {}", url))?;

            if let Some(remote_size) = response.content_length() {
                info!("Remote file size: {:.2} MB", remote_size as f64 / 1_000_000.0);
                
                // Check if sizes are within 1% of each other
                let size_diff = ((remote_size as i64 - local_size as i64).abs() as f64) / remote_size as f64;
                if size_diff < 0.01 {
                    info!("File size matches (within 1%), skipping download");
                    return Ok(file_path);
                } else {
                    info!("File size differs by {:.2}%, re-downloading", size_diff * 100.0);
                }
            } else {
                info!("Could not determine remote file size, re-downloading");
            }
        }

        info!("Downloading {} to {:?}...", url, file_path);

        // Create HTTP client
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3600)) // 1 hour timeout for large files
            .build()
            .context("Failed to build HTTP client")?;

        // Download the file
        let response = client
            .get(url)
            .send()
            .await
            .context(format!("Failed to download from {}", url))?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        // Get content length for progress reporting
        let total_size = response.content_length();
        if let Some(size) = total_size {
            info!("File size: {:.2} MB", size as f64 / 1_000_000.0);
        }

        // Write to file
        let mut file =
            fs::File::create(&file_path).context("Failed to create output file")?;

        let bytes = response
            .bytes()
            .await
            .context("Failed to read response body")?;

        file.write_all(&bytes)
            .context("Failed to write to file")?;

        let downloaded = bytes.len() as u64;

        if let Some(total) = total_size {
            info!(
                "Download complete: {:.2} MB / {:.2} MB",
                downloaded as f64 / 1_000_000.0,
                total as f64 / 1_000_000.0
            );
        } else {
            info!(
                "Download complete: {:.2} MB",
                downloaded as f64 / 1_000_000.0
            );
        }

        Ok(file_path)
    }

    /// Get the path to the data directory
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}