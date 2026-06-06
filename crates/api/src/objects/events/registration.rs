use async_graphql::{Context, Result, SimpleObject};
use storage::{NameRegisteredEventRow, NameRenewedEventRow, NameTransferredEventRow};

use super::lookup::{account_by_id, registration_by_id};
use crate::objects::{Account, Registration};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameRegistered")]
pub struct NameRegisteredEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub registration_id: String,
    #[graphql(skip)]
    pub registrant_id: String,
}

impl From<NameRegisteredEventRow> for NameRegisteredEvent {
    fn from(value: NameRegisteredEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            registration_id: value.registration_id,
            registrant_id: value.registrant_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameRegisteredEvent {
    pub(crate) async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }

    async fn registrant(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.registrant_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameRenewed")]
pub struct NameRenewedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "expiryDate")]
    pub expiry_date: String,
    #[graphql(skip)]
    pub registration_id: String,
}

impl From<NameRenewedEventRow> for NameRenewedEvent {
    fn from(value: NameRenewedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            expiry_date: value.expiry_date.to_string(),
            registration_id: value.registration_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameRenewedEvent {
    pub(crate) async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "NameTransferred")]
pub struct NameTransferredEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub registration_id: String,
    #[graphql(skip)]
    pub new_owner_id: String,
}

impl From<NameTransferredEventRow> for NameTransferredEvent {
    fn from(value: NameTransferredEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            registration_id: value.registration_id,
            new_owner_id: value.new_owner_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl NameTransferredEvent {
    pub(crate) async fn registration(&self, ctx: &Context<'_>) -> Result<Option<Registration>> {
        registration_by_id(ctx, &self.registration_id).await
    }

    async fn new_owner(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.new_owner_id).await
    }
}
