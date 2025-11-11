mod downloader;
mod osm_index;

use anyhow::Result;
use downloader::OSMDownloader;
use log::info;
use osm_index::OSMPBDownloads;
use osmpbf::{Element, ElementReader};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

// mad-osm-importer binary crate
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    pretty_env_logger::init();

    info!("Starting mad-osm-importer");

    // Create downloader and download the PBF file
    let downloader = OSMDownloader::new()?;
    let pbf_path = downloader.download(OSMPBDownloads::USPacific).await?;

    info!("Processing OSM data from {:?}", pbf_path);

    // Read and process the PBF file
    let reader = ElementReader::from_path(&pbf_path)?;

    // Limit for each element type
    const MAX_EXAMPLES: u64 = 2000;

    // Counters for each element type
    let mut ways_count = 0_u64;
    let mut nodes_count = 0_u64;
    let mut dense_nodes_count = 0_u64;
    let mut relations_count = 0_u64;

    // JSON files for each type
    let data_dir = downloader.data_dir();
    let ways_file_path = data_dir.join("ways.json");
    let nodes_file_path = data_dir.join("nodes.json");
    let dense_nodes_file_path = data_dir.join("dense_nodes.json");
    let relations_file_path = data_dir.join("relations.json");

    let mut ways_file = fs::File::create(&ways_file_path)?;
    let mut nodes_file = fs::File::create(&nodes_file_path)?;
    let mut dense_nodes_file = fs::File::create(&dense_nodes_file_path)?;
    let mut relations_file = fs::File::create(&relations_file_path)?;

    // Write opening brackets for JSON arrays
    ways_file.write_all(b"[\n")?;
    nodes_file.write_all(b"[\n")?;
    dense_nodes_file.write_all(b"[\n")?;
    relations_file.write_all(b"[\n")?;

    // Process each element
    reader.for_each(|element| match element {
        Element::Way(way) => {
            if ways_count < MAX_EXAMPLES {
                ways_count += 1;
                if ways_count > 1 {
                    let _ = ways_file.write_all(b",\n");
                }
                let json = serde_json::json!({
                    "id": way.id(),
                    "tags": way.tags().collect::<HashMap<_, _>>(),
                    "refs": way.refs().collect::<Vec<_>>(),
                });
                let _ = ways_file.write_all(json.to_string().as_bytes());
            }
        }
        Element::Node(node) => {
            if nodes_count < MAX_EXAMPLES {
                nodes_count += 1;
                if nodes_count > 1 {
                    let _ = nodes_file.write_all(b",\n");
                }
                let json = serde_json::json!({
                    "id": node.id(),
                    "lat": node.lat(),
                    "lon": node.lon(),
                    "tags": node.tags().collect::<HashMap<_, _>>(),
                });
                let _ = nodes_file.write_all(json.to_string().as_bytes());
            }
        }
        Element::DenseNode(dense_node) => {
            if dense_nodes_count < MAX_EXAMPLES {
                dense_nodes_count += 1;
                if dense_nodes_count > 1 {
                    let _ = dense_nodes_file.write_all(b",\n");
                }
                let json = serde_json::json!({
                    "id": dense_node.id(),
                    "lat": dense_node.lat(),
                    "lon": dense_node.lon(),
                    "tags": dense_node.tags().collect::<HashMap<_, _>>(),
                });
                let _ = dense_nodes_file.write_all(json.to_string().as_bytes());
            }
        }
        Element::Relation(relation) => {
            if relations_count < MAX_EXAMPLES {
                relations_count += 1;
                if relations_count > 1 {
                    let _ = relations_file.write_all(b",\n");
                }
                let members: Vec<_> = relation
                    .members()
                    .map(|m| {
                        serde_json::json!({
                            "member_id": m.member_id,
                            "role": m.role().ok(),
                            "member_type": format!("{:?}", m.member_type),
                        })
                    })
                    .collect();
                let json = serde_json::json!({
                    "id": relation.id(),
                    "tags": relation.tags().collect::<HashMap<_, _>>(),
                    "members": members,
                });
                let _ = relations_file.write_all(json.to_string().as_bytes());
            }
        }
    })?;

    // Write closing brackets
    ways_file.write_all(b"\n]\n")?;
    nodes_file.write_all(b"\n]\n")?;
    dense_nodes_file.write_all(b"\n]\n")?;
    relations_file.write_all(b"\n]\n")?;

    info!("Processing complete:");
    info!("  Ways: {} -> {:?}", ways_count, ways_file_path);
    info!("  Nodes: {} -> {:?}", nodes_count, nodes_file_path);
    info!(
        "  Dense Nodes: {} -> {:?}",
        dense_nodes_count, dense_nodes_file_path
    );
    info!(
        "  Relations: {} -> {:?}",
        relations_count, relations_file_path
    );

    Ok(())
}
