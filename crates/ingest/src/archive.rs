mod coverage;
mod manifest;
mod model;
mod resolvers;

use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Context;
pub use coverage::{ArchiveGap, ArchiveStatus, inspect_archive};
pub use manifest::ArchiveManifestRange;
pub(crate) use model::ArchivedRange;
pub use resolvers::{ResolverCacheStatus, rebuild_resolver_cache};
pub(crate) use resolvers::{add_resolver_from_log, load_resolver_cache, write_resolver_cache};

use self::manifest::{load_manifest, upsert_manifest_range, verify_manifest_range_checksum};

pub(crate) fn write_range(dir: &Path, range: &ArchivedRange) -> anyhow::Result<PathBuf> {
    let ranges_dir = dir.join("ranges");
    std::fs::create_dir_all(&ranges_dir).with_context(|| {
        format!(
            "failed to create archive ranges dir {}",
            ranges_dir.display()
        )
    })?;

    let path = range_path(dir, range.from_block, range.to_block);
    let tmp_path = path.with_extension("bin.tmp");
    let bytes = encode_range_binary(range)?;
    let checksum = manifest::sha256_hex(&bytes);
    std::fs::write(&tmp_path, bytes)
        .with_context(|| format!("failed to write archive temp file {}", tmp_path.display()))?;
    std::fs::rename(&tmp_path, &path).with_context(|| {
        format!(
            "failed to move archive temp file {} to {}",
            tmp_path.display(),
            path.display()
        )
    })?;
    upsert_manifest_range(dir, range, &path, &checksum)?;
    Ok(path)
}

pub(crate) fn range_entries(
    dir: &Path,
    expected_chain_id: u64,
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<Vec<ArchiveManifestRange>> {
    anyhow::ensure!(
        from_block <= to_block,
        "from block must be less than or equal to to block"
    );

    let manifest = load_manifest(dir)?;
    anyhow::ensure!(
        manifest.chain_id == 0 || manifest.chain_id == expected_chain_id,
        "archive manifest chain_id {} does not match configured chain_id {}",
        manifest.chain_id,
        expected_chain_id
    );
    let mut ranges = manifest
        .ranges
        .into_iter()
        .filter(|range| range.to_block >= from_block && range.from_block <= to_block)
        .collect::<Vec<_>>();
    ranges.sort_by_key(|range| range.from_block);
    validate_contiguous_entries(&ranges, from_block, to_block)?;
    Ok(ranges)
}

pub(crate) fn read_range_entry(
    dir: &Path,
    expected_chain_id: u64,
    entry: &ArchiveManifestRange,
) -> anyhow::Result<ArchivedRange> {
    verify_manifest_range_checksum(dir, expected_chain_id, entry)?;
    let path = dir.join(&entry.file);
    let bytes = std::fs::read(&path)
        .with_context(|| format!("failed to read archive range {}", path.display()))?;
    let range = decode_range_entry(entry, &bytes)?;
    range.validate(expected_chain_id)?;
    Ok(range)
}

pub fn convert_json_archive_to_binary(dir: &Path, expected_chain_id: u64) -> anyhow::Result<usize> {
    let manifest = load_manifest(dir)?;
    anyhow::ensure!(
        manifest.chain_id == 0 || manifest.chain_id == expected_chain_id,
        "archive manifest chain_id {} does not match configured chain_id {}",
        manifest.chain_id,
        expected_chain_id
    );

    let binary_ranges = manifest
        .ranges
        .iter()
        .filter(|entry| entry.file.ends_with(".bin"))
        .map(|entry| (entry.from_block, entry.to_block))
        .collect::<std::collections::BTreeSet<_>>();
    let entries = manifest
        .ranges
        .iter()
        .filter(|entry| entry.file.ends_with(".json"))
        .filter(|entry| !binary_ranges.contains(&(entry.from_block, entry.to_block)))
        .cloned()
        .collect::<Vec<_>>();

    let total = entries.len();
    let mut converted = 0;
    for (index, entry) in entries.into_iter().enumerate() {
        let started = Instant::now();
        tracing::info!(
            archive_file = %entry.file,
            from_block = entry.from_block,
            to_block = entry.to_block,
            bytes = entry.bytes,
            logs = entry.logs,
            range_index = index + 1,
            total_ranges = total,
            "converting archive range to binary"
        );
        println!(
            "converting archive range {}/{} {} {}..{} logs={} bytes={}",
            index + 1,
            total,
            entry.file,
            entry.from_block,
            entry.to_block,
            entry.logs,
            entry.bytes
        );
        let _ = std::io::stdout().flush();
        let result = (|| {
            let range = read_range_entry(dir, expected_chain_id, &entry)
                .with_context(|| format!("failed to read archive range {}", entry.file))?;
            write_range(dir, &range)
                .with_context(|| format!("failed to convert archive range {}", entry.file))?;
            anyhow::Ok(())
        })();
        match result {
            Ok(()) => {
                converted += 1;
                tracing::info!(
                    archive_file = %entry.file,
                    from_block = entry.from_block,
                    to_block = entry.to_block,
                    range_index = index + 1,
                    total_ranges = total,
                    elapsed_ms = started.elapsed().as_millis(),
                    "converted archive range to binary"
                );
                println!(
                    "converted archive range {}/{} {} elapsed_ms={}",
                    index + 1,
                    total,
                    entry.file,
                    started.elapsed().as_millis()
                );
                let _ = std::io::stdout().flush();
            }
            Err(error) => {
                tracing::error!(
                    archive_file = %entry.file,
                    from_block = entry.from_block,
                    to_block = entry.to_block,
                    range_index = index + 1,
                    total_ranges = total,
                    elapsed_ms = started.elapsed().as_millis(),
                    %error,
                    "failed to convert archive range to binary"
                );
                eprintln!(
                    "failed archive range {}/{} {} elapsed_ms={} error={:#}",
                    index + 1,
                    total,
                    entry.file,
                    started.elapsed().as_millis(),
                    error
                );
                return Err(error);
            }
        }
    }
    Ok(converted)
}

pub(crate) fn available_bounds(dir: &Path, expected_chain_id: u64) -> anyhow::Result<(u64, u64)> {
    let manifest = load_manifest(dir)?;
    anyhow::ensure!(
        manifest.chain_id == 0 || manifest.chain_id == expected_chain_id,
        "archive manifest chain_id {} does not match configured chain_id {}",
        manifest.chain_id,
        expected_chain_id
    );
    let from = manifest.ranges.iter().map(|range| range.from_block).min();
    let to = manifest.ranges.iter().map(|range| range.to_block).max();

    match (from, to) {
        (Some(from), Some(to)) => Ok((from, to)),
        _ => anyhow::bail!("raw archive contains no range files"),
    }
}

pub(crate) fn range_path(dir: &Path, from_block: u64, to_block: u64) -> PathBuf {
    dir.join("ranges")
        .join(format!("{from_block:020}-{to_block:020}.bin"))
}

fn validate_contiguous_entries(
    ranges: &[ArchiveManifestRange],
    from_block: u64,
    to_block: u64,
) -> anyhow::Result<()> {
    let mut expected = from_block;
    for range in ranges {
        anyhow::ensure!(
            range.from_block == expected,
            "raw archive gap: expected range starting at {}, found {}..{}",
            expected,
            range.from_block,
            range.to_block
        );

        expected = range
            .to_block
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("raw archive range cannot end at u64::MAX"))?;
    }

    anyhow::ensure!(
        expected == to_block.saturating_add(1),
        "raw archive gap: missing range ending at {}",
        to_block
    );
    Ok(())
}

pub(crate) fn verify_manifest_range(
    dir: &Path,
    expected_chain_id: u64,
    entry: &ArchiveManifestRange,
) -> anyhow::Result<()> {
    verify_manifest_range_checksum(dir, expected_chain_id, entry)
}

pub(crate) fn encode_range_binary(range: &ArchivedRange) -> anyhow::Result<Vec<u8>> {
    Ok(rmp_serde::to_vec_named(range)?)
}

pub(crate) fn decode_range_entry(
    entry: &ArchiveManifestRange,
    bytes: &[u8],
) -> anyhow::Result<ArchivedRange> {
    if entry.file.ends_with(".bin") {
        Ok(rmp_serde::from_slice(bytes)?)
    } else {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_archive_range_roundtrips() {
        let range = ArchivedRange::new(1, 10, 20, Vec::new(), Default::default(), vec![]);
        let bytes = encode_range_binary(&range).unwrap();
        let entry = ArchiveManifestRange {
            from_block: 10,
            to_block: 20,
            file: "ranges/00000000000000000010-00000000000000000020.bin".to_owned(),
            sha256: manifest::sha256_hex(&bytes),
            bytes: bytes.len() as u64,
            logs: 0,
        };

        let decoded = decode_range_entry(&entry, &bytes).unwrap();

        assert_eq!(decoded.chain_id, range.chain_id);
        assert_eq!(decoded.from_block, range.from_block);
        assert_eq!(decoded.to_block, range.to_block);
        assert_eq!(decoded.logs.len(), 0);
    }
}
