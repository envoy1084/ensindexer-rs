use std::{cmp, collections::BTreeSet, str::FromStr};

use alloy::{providers::ProviderBuilder, rpc::types::Log};
use alloy_primitives::Address;
use contracts::{EnsEvent, decode_fixed_source_log};
use storage::BlockInsert;

use super::IngestService;
use crate::{
    decode::decode_log,
    rpc::{
        fetch_block_meta_by_number, fetch_block_metadata, fetch_resolver_logs, fetch_source_logs,
    },
    sources::{LogSource, fixed_sources},
};

impl IngestService {
    pub async fn backfill_range(&self, from_block: u64, to_block: u64) -> anyhow::Result<()> {
        anyhow::ensure!(
            from_block <= to_block,
            "from block must be less than or equal to to block"
        );

        tracing::info!(
            chain_id = self.config.chain_id,
            from_block,
            to_block,
            batch_blocks = self.config.backfill_batch_blocks,
            "starting fixed-source backfill"
        );

        let provider = ProviderBuilder::new()
            .connect(self.config.eth_rpc_url.as_str())
            .await?;
        let sources = fixed_sources()?;
        let mut range_start = from_block;

        while range_start <= to_block {
            let range_end = cmp::min(
                to_block,
                range_start.saturating_add(self.config.backfill_batch_blocks.saturating_sub(1)),
            );

            let mut raw_logs = Vec::new();
            for source in &sources {
                let logs = fetch_source_logs(&provider, source, range_start, range_end).await?;
                tracing::debug!(
                    ?source,
                    from_block = range_start,
                    to_block = range_end,
                    logs = logs.len(),
                    "fetched logs"
                );

                raw_logs.extend(
                    logs.into_iter()
                        .map(|log| (LogSource::Fixed(source.source), log)),
                );
            }

            let resolver_addresses = self
                .resolver_addresses_for_batch(&raw_logs, range_start)
                .await?;
            let resolver_logs =
                fetch_resolver_logs(&provider, &resolver_addresses, range_start, range_end).await?;
            tracing::debug!(
                from_block = range_start,
                to_block = range_end,
                addresses = resolver_addresses.len(),
                logs = resolver_logs.len(),
                "fetched resolver logs"
            );
            raw_logs.extend(
                resolver_logs
                    .into_iter()
                    .map(|log| (LogSource::Resolver, log)),
            );

            let mut block_meta = fetch_block_metadata(&provider, &raw_logs).await?;
            let active_sources = sources
                .iter()
                .filter(|source| range_end >= source.start_block)
                .collect::<Vec<_>>();
            if !active_sources.is_empty() && !block_meta.contains_key(&range_end) {
                block_meta.insert(
                    range_end,
                    fetch_block_meta_by_number(&provider, range_end).await?,
                );
            }

            for meta in block_meta.values() {
                self.storage
                    .blocks()
                    .upsert(BlockInsert {
                        number: meta.number_i64()?,
                        hash: types::hex_b256(meta.hash),
                        parent_hash: Some(types::hex_b256(meta.parent_hash)),
                        timestamp: meta.timestamp_i64()?,
                    })
                    .await?;
            }

            let mut decoded = Vec::new();
            for (source, log) in raw_logs {
                match decode_log(source, log, &block_meta) {
                    Ok(indexed) => decoded.push(indexed),
                    Err(error) => tracing::warn!(?source, %error, "skipping undecodable log"),
                }
            }

            decoded.sort_by_key(|event| {
                (
                    event.ctx.block_number,
                    event.ctx.transaction_index,
                    event.ctx.log_index,
                )
            });

            for event in decoded {
                projection::apply_event(&self.storage, event).await?;
            }

            for source in active_sources {
                if let Some(meta) = block_meta.get(&range_end) {
                    self.storage
                        .checkpoints()
                        .upsert(
                            source.checkpoint_name(),
                            meta.number_i64()?,
                            &types::hex_b256(meta.hash),
                        )
                        .await?;
                }
            }

            tracing::info!(
                from_block = range_start,
                to_block = range_end,
                "applied fixed-source backfill range"
            );

            if range_end == u64::MAX {
                break;
            }
            range_start = range_end + 1;
        }

        Ok(())
    }

    async fn resolver_addresses_for_batch(
        &self,
        raw_logs: &[(LogSource, Log)],
        range_start: u64,
    ) -> anyhow::Result<Vec<Address>> {
        let mut addresses = BTreeSet::new();

        if range_start
            > fixed_sources()?
                .first()
                .map_or(0, |source| source.start_block)
        {
            for address in self.storage.resolvers().list_distinct_addresses().await? {
                addresses.insert(Address::from_str(&address)?);
            }
        }

        for (source, log) in raw_logs {
            if !matches!(source, LogSource::Fixed(_)) {
                continue;
            }

            let Ok(EnsEvent::RegistryNewResolver { resolver, .. }) =
                decode_fixed_source_log(source.fixed_source()?, log)
            else {
                continue;
            };

            if resolver != Address::ZERO {
                addresses.insert(resolver);
            }
        }

        Ok(addresses.into_iter().collect())
    }
}
