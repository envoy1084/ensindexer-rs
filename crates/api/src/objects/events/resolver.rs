use async_graphql::{Context, Result, SimpleObject};
use storage::{
    AbiChangedEventRow, AddrChangedEventRow, AuthorisationChangedEventRow,
    ContenthashChangedEventRow, InterfaceChangedEventRow, MulticoinAddrChangedEventRow,
    NameChangedEventRow, PubkeyChangedEventRow, TextChangedEventRow, VersionChangedEventRow,
};

use super::lookup::{account_by_id, resolver_by_id};
use crate::objects::{Account, Resolver};

macro_rules! impl_resolver_event {
    ($name:ident, $graphql_name:literal, $row:ident, { $($field:ident : $ty:ty => $expr:expr),* $(,)? }) => {
        #[derive(Debug, Clone, SimpleObject)]
        #[graphql(complex, name = $graphql_name)]
        pub struct $name {
            pub id: String,
            #[graphql(name = "blockNumber")]
            pub block_number: i32,
            #[graphql(name = "transactionID")]
            pub transaction_id: String,
            $(pub $field: $ty,)*
            #[graphql(skip)]
            pub resolver_id: String,
        }

        impl From<$row> for $name {
            fn from(value: $row) -> Self {
                Self {
                    id: value.id.clone(),
                    block_number: value.block_number,
                    transaction_id: value.transaction_id.clone(),
                    $($field: $expr(&value),)*
                    resolver_id: value.resolver_id,
                }
            }
        }

        #[async_graphql::ComplexObject]
        impl $name {
            pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
                resolver_by_id(ctx, &self.resolver_id).await
            }
        }
    };
}

impl_resolver_event!(NameChangedEvent, "NameChanged", NameChangedEventRow, { name: String => |v: &NameChangedEventRow| v.name.clone() });
impl_resolver_event!(PubkeyChangedEvent, "PubkeyChanged", PubkeyChangedEventRow, { x: String => |v: &PubkeyChangedEventRow| v.x.clone(), y: String => |v: &PubkeyChangedEventRow| v.y.clone() });
impl_resolver_event!(TextChangedEvent, "TextChanged", TextChangedEventRow, { key: String => |v: &TextChangedEventRow| v.key.clone(), value: Option<String> => |v: &TextChangedEventRow| v.value.clone() });
impl_resolver_event!(ContenthashChangedEvent, "ContenthashChanged", ContenthashChangedEventRow, { hash: String => |v: &ContenthashChangedEventRow| v.hash.clone() });
impl_resolver_event!(VersionChangedEvent, "VersionChanged", VersionChangedEventRow, { version: String => |v: &VersionChangedEventRow| v.version.to_string() });

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "AddrChanged")]
pub struct AddrChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(skip)]
    pub resolver_id: String,
    #[graphql(skip)]
    pub addr_id: String,
}

impl From<AddrChangedEventRow> for AddrChangedEvent {
    fn from(value: AddrChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            resolver_id: value.resolver_id,
            addr_id: value.addr_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AddrChangedEvent {
    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }

    async fn addr(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        account_by_id(ctx, &self.addr_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "MulticoinAddrChanged")]
pub struct MulticoinAddrChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "coinType")]
    pub coin_type: String,
    pub addr: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<MulticoinAddrChangedEventRow> for MulticoinAddrChangedEvent {
    fn from(value: MulticoinAddrChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            coin_type: value.coin_type.to_string(),
            addr: value.addr,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl MulticoinAddrChangedEvent {
    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "ABIChanged")]
pub struct AbiChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "contentType")]
    pub content_type: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<AbiChangedEventRow> for AbiChangedEvent {
    fn from(value: AbiChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            content_type: value.content_type.to_string(),
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AbiChangedEvent {
    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "InterfaceChanged")]
pub struct InterfaceChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    #[graphql(name = "interfaceID")]
    pub interface_id: String,
    pub implementer: String,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<InterfaceChangedEventRow> for InterfaceChangedEvent {
    fn from(value: InterfaceChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            interface_id: value.interface_id,
            implementer: value.implementer,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl InterfaceChangedEvent {
    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex, name = "AuthorisationChanged")]
pub struct AuthorisationChangedEvent {
    pub id: String,
    #[graphql(name = "blockNumber")]
    pub block_number: i32,
    #[graphql(name = "transactionID")]
    pub transaction_id: String,
    pub owner: String,
    pub target: String,
    #[graphql(name = "isAuthorized")]
    pub is_authorized: bool,
    #[graphql(skip)]
    pub resolver_id: String,
}

impl From<AuthorisationChangedEventRow> for AuthorisationChangedEvent {
    fn from(value: AuthorisationChangedEventRow) -> Self {
        Self {
            id: value.id,
            block_number: value.block_number,
            transaction_id: value.transaction_id,
            owner: value.owner,
            target: value.target,
            is_authorized: value.is_authorized,
            resolver_id: value.resolver_id,
        }
    }
}

#[async_graphql::ComplexObject]
impl AuthorisationChangedEvent {
    pub(crate) async fn resolver(&self, ctx: &Context<'_>) -> Result<Option<Resolver>> {
        resolver_by_id(ctx, &self.resolver_id).await
    }
}
