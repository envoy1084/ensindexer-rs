use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use alloy_primitives::Address;
use contracts::{EnsEvent, decode_fixed_source_log};
use serde::{Deserialize, Serialize};

use crate::{
    archive::{manifest::load_manifest, model::ArchivedRange},
    sources::LogSource,
};

const RESOLVER_CACHE_VERSION: u32 = 1;
const RESOLVER_CACHE_FILE: &str = "resolvers.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResolverCache {
    version: u32,
    chain_id: u64,
    updated_to_block: u64,
    addresses: Vec<Address>,
}

#[derive(Debug, Clone)]
pub struct ResolverCacheStatus {
    pub chain_id: u64,
    pub updated_to_block: u64,
    pub addresses: usize,
    pub path: PathBuf,
}

pub(crate) fn load_resolver_cache(
    dir: &Path,
    expected_chain_id: u64,
) -> anyhow::Result<Option<(BTreeSet<Address>, u64)>> {
    let path = resolver_cache_path(dir);
    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path)?;
    let cache: ResolverCache = serde_json::from_slice(&bytes)?;
    cache.validate(expected_chain_id)?;
    Ok(Some((
        cache.addresses.into_iter().collect(),
        cache.updated_to_block,
    )))
}

pub(crate) fn write_resolver_cache(
    dir: &Path,
    chain_id: u64,
    updated_to_block: u64,
    addresses: &BTreeSet<Address>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(dir)?;
    let cache = ResolverCache {
        version: RESOLVER_CACHE_VERSION,
        chain_id,
        updated_to_block,
        addresses: addresses.iter().copied().collect(),
    };
    let path = resolver_cache_path(dir);
    let tmp_path = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(&cache)?;
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, &path)?;
    Ok(())
}

pub fn rebuild_resolver_cache(
    dir: &Path,
    expected_chain_id: u64,
    from_block: Option<u64>,
    to_block: Option<u64>,
) -> anyhow::Result<ResolverCacheStatus> {
    let mut addresses = BTreeSet::new();
    let mut updated_to_block = None;
    let mut ranges = load_manifest(dir)?
        .ranges
        .into_iter()
        .filter(|range| {
            from_block.is_none_or(|from| range.to_block >= from)
                && to_block.is_none_or(|to| range.from_block <= to)
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|range| range.from_block);
    for range in ranges {
        let bytes = std::fs::read(dir.join(&range.file))?;
        let archived: ArchivedRange = serde_json::from_slice(&bytes)?;
        archived.validate(expected_chain_id)?;
        for (source, log) in archived.into_parts().0 {
            add_resolver_from_log(&mut addresses, source, &log)?;
        }
        updated_to_block = Some(range.to_block);
    }

    let updated_to_block =
        updated_to_block.ok_or_else(|| anyhow::anyhow!("archive contains no matching ranges"))?;
    write_resolver_cache(dir, expected_chain_id, updated_to_block, &addresses)?;
    Ok(ResolverCacheStatus {
        chain_id: expected_chain_id,
        updated_to_block,
        addresses: addresses.len(),
        path: resolver_cache_path(dir),
    })
}

pub(crate) fn add_resolver_from_log(
    addresses: &mut BTreeSet<Address>,
    source: LogSource,
    log: &alloy::rpc::types::Log,
) -> anyhow::Result<()> {
    if !matches!(source, LogSource::Fixed(_)) {
        return Ok(());
    }

    let Ok(EnsEvent::RegistryNewResolver { resolver, .. }) =
        decode_fixed_source_log(source.fixed_source()?, log)
    else {
        return Ok(());
    };

    if resolver != Address::ZERO {
        addresses.insert(resolver);
    }
    Ok(())
}

fn resolver_cache_path(dir: &Path) -> PathBuf {
    dir.join(RESOLVER_CACHE_FILE)
}

impl ResolverCache {
    fn validate(&self, expected_chain_id: u64) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.version == RESOLVER_CACHE_VERSION,
            "unsupported resolver cache version {}",
            self.version
        );
        anyhow::ensure!(
            self.chain_id == expected_chain_id,
            "resolver cache chain_id {} does not match configured chain_id {}",
            self.chain_id,
            expected_chain_id
        );
        Ok(())
    }
}
