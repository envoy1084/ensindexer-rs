use async_graphql::{Context, Result, SimpleObject};
use storage::{
    ExpiryExtendedEventRow, FusesSetEventRow, NameUnwrappedEventRow, NameWrappedEventRow,
    NewOwnerEventRow, NewResolverEventRow, NewTtlEventRow, TransferEventRow,
    WrappedTransferEventRow,
};

use super::lookup::{account_by_id, domain_by_id, resolver_by_id};
use crate::objects::{Account, Domain, Resolver};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "Transfer")]
pub struct TransferEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<TransferEventRow> for TransferEvent {
    fn from(value: TransferEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl TransferEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewOwner")]
pub struct NewOwnerEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub parent_domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<NewOwnerEventRow> for NewOwnerEvent {
    fn from(value: NewOwnerEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            parent_domain_id: value.parent_domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewOwnerEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn parent_domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.parent_domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewResolver")]
pub struct NewResolverEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<NewResolverEventRow> for NewResolverEvent {
    fn from(value: NewResolverEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            domain_id: value.domain_id,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewResolverEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NewTTL")]
pub struct NewTtlEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub ttl: String,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<NewTtlEventRow> for NewTtlEvent {
    fn from(value: NewTtlEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            ttl: value.ttl.to_string(),
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NewTtlEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}

macro_rules! impl_domain_owner_event {
    ($name:ident, $graphql_name:literal, $row:ident) => {
        #[derive(Debug, Clone, SimpleObject)]
        #[graphql(complex, name = $graphql_name)]
        pub struct $name {
            pub id: String,
            #[graphql(name = "blockNumber")]
            pub block_number: i32,
            #[graphql(name = "transactionID")]
            pub transaction_id: String,
            #[graphql(skip)]
            pub domain_id: String,
            #[graphql(skip)]
            pub owner_id: String,
        }

        impl From<$row> for $name {
            fn from(value: $row) -> Self {
                Self {
                    id: value.id,
                    block_number: value.block_number,
                    transaction_id: value.transaction_id,
                    domain_id: value.domain_id,
                    owner_id: value.owner_id,
                }
            }
        }

        #[async_graphql::ComplexObject]
        impl $name {
            pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
                domain_by_id(ctx, &self.domain_id).await
            }

            async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
                account_by_id(ctx, &self.owner_id).await
            }
        }
    };
}

impl_domain_owner_event!(
    WrappedTransferEvent,
    "WrappedTransfer",
    WrappedTransferEventRow
);
impl_domain_owner_event!(NameUnwrappedEvent, "NameUnwrapped", NameUnwrappedEventRow);

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameWrapped")]
pub struct NameWrappedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub name: Option<String>,
    pub fuses: i32,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub domain_id: String,
    #[graphql(skip)]
    pub owner_id: String,
}

impl From<NameWrappedEventRow> for NameWrappedEvent {
    fn from(value: NameWrappedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            name: value.name,
            fuses: value.fuses,
            expiry_date: value.expiry_date.to_string(),
            domain_id: value.domain_id,
            owner_id: value.owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameWrappedEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }

    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.owner_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "FusesSet")]
pub struct FusesSetEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub fuses: i32,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<FusesSetEventRow> for FusesSetEvent {
    fn from(value: FusesSetEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            fuses: value.fuses,
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl FusesSetEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "ExpiryExtended")]
pub struct ExpiryExtendedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub domain_id: String,
}

impl From<ExpiryExtendedEventRow> for ExpiryExtendedEvent {
    fn from(value: ExpiryExtendedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            domain_id: value.domain_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl ExpiryExtendedEvent {
    pub(crate) async fn domain(&self, ctx: &Context<'_>) -> Result<Option<Domain>> {
        domain_by_id(ctx, &self.domain_id).await
    }
}
