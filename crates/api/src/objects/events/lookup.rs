use async_graphql::{Context, Result};
use storage::{DomainRow, RegistrationRow, ResolverRow, Storage};

use crate::objects::{Account, Domain, Registration, Resolver};

pub(super) async fn account_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Account>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage.accounts().find_by_id(id).await?.map(Into::into))
}

pub(super) async fn domain_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Domain>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage.domains().find_by_id(id).await?.map(domain_from_row))
}

pub(super) async fn registration_by_id(
    ctx: &Context<'_>,
    id: &str,
) -> Result<Option<Registration>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage
        .registrations()
        .find_by_id(id)
        .await?
        .map(registration_from_row))
}

pub(super) async fn resolver_by_id(ctx: &Context<'_>, id: &str) -> Result<Option<Resolver>> {
    let storage = ctx.data::<Storage>()?;
    Ok(storage
        .resolvers()
        .find_by_id(id)
        .await?
        .map(resolver_from_row))
}

pub(super) fn domain_from_row(row: DomainRow) -> Domain {
    row.into()
}

pub(super) fn registration_from_row(row: RegistrationRow) -> Registration {
    row.into()
}

pub(super) fn resolver_from_row(row: ResolverRow) -> Resolver {
    row.into()
}
