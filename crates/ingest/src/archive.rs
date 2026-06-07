mod coverage;
mod manifest;
mod model;
mod resolvers;

use std::path::{Path, PathBuf};

pub use coverage::{ArchiveGap, ArchiveStatus, inspect_archive};
pub use manifest::ArchiveManifestRange;
pub(crate) use model::ArchivedRange;
pub use resolvers::{ResolverCacheStatus, rebuild_resolver_cache};
pub(crate) use resolvers::{add_resolver_from_log, load_resolver_cache, write_resolver_cache};

use self::manifest::{load_manifest, upsert_manifest_range, verify_manifest_range_checksum};

pub(crate) fn write_range(dir: &Path, range: &ArchivedRange) -> anyhow::Result<PathBuf> {
    let ranges_dir = dir.join("ranges");
    std::fs::create_dir_all(&ranges_dir)?;

    let path = range_path(dir, range.from_block, range.to_block);
    let tmp_path = path.with_extension("bin.tmp");
    let bytes = encode_range_binary(range)?;
    let checksum = manifest::sha256_hex(&bytes);
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, &path)?;
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
    let bytes = std::fs::read(dir.join(&entry.file))?;
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

    let mut converted = 0;
    for entry in manifest
        .ranges
        .iter()
        .filter(|entry| entry.file.ends_with(".json"))
    {
        let range = read_range_entry(dir, expected_chain_id, entry)?;
        write_range(dir, &range)?;
        converted += 1;
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
    Ok(bincode::serde::encode_to_vec(
        range,
        bincode::config::standard(),
    )?)
}

pub(crate) fn decode_range_entry(
    entry: &ArchiveManifestRange,
    bytes: &[u8],
) -> anyhow::Result<ArchivedRange> {
    if entry.file.ends_with(".bin") {
        let (range, consumed): (ArchivedRange, usize) =
            bincode::serde::decode_from_slice(bytes, bincode::config::standard())?;
        anyhow::ensure!(
            consumed == bytes.len(),
            "binary archive range {} had trailing bytes",
            entry.file
        );
        Ok(range)
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
