use async_graphql::Result;
use storage::{EventReferenceRow, Storage};

use super::domain::{
    ExpiryExtendedEvent, FusesSetEvent, NameUnwrappedEvent, NameWrappedEvent, NewOwnerEvent,
    NewResolverEvent, NewTtlEvent, TransferEvent, WrappedTransferEvent,
};
use super::interfaces::{DomainEvent, RegistrationEvent, ResolverEvent};
use super::registration::{NameRegisteredEvent, NameRenewedEvent, NameTransferredEvent};
use super::resolver::{
    AbiChangedEvent, AddrChangedEvent, AuthorisationChangedEvent, ContenthashChangedEvent,
    InterfaceChangedEvent, MulticoinAddrChangedEvent, NameChangedEvent, PubkeyChangedEvent,
    TextChangedEvent, VersionChangedEvent,
};

pub async fn hydrate_domain_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<DomainEvent>> {
    let event = match reference.kind.as_str() {
        "Transfer" => storage
            .events()
            .find_transfer_by_id(&reference.id)
            .await?
            .map(TransferEvent::from)
            .map(DomainEvent::from),
        "NewOwner" => storage
            .events()
            .find_new_owner_by_id(&reference.id)
            .await?
            .map(NewOwnerEvent::from)
            .map(DomainEvent::from),
        "NewResolver" => storage
            .events()
            .find_new_resolver_by_id(&reference.id)
            .await?
            .map(NewResolverEvent::from)
            .map(DomainEvent::from),
        "NewTTL" => storage
            .events()
            .find_new_ttl_by_id(&reference.id)
            .await?
            .map(NewTtlEvent::from)
            .map(DomainEvent::from),
        "WrappedTransfer" => storage
            .events()
            .find_wrapped_transfer_by_id(&reference.id)
            .await?
            .map(WrappedTransferEvent::from)
            .map(DomainEvent::from),
        "NameWrapped" => storage
            .events()
            .find_name_wrapped_by_id(&reference.id)
            .await?
            .map(NameWrappedEvent::from)
            .map(DomainEvent::from),
        "NameUnwrapped" => storage
            .events()
            .find_name_unwrapped_by_id(&reference.id)
            .await?
            .map(NameUnwrappedEvent::from)
            .map(DomainEvent::from),
        "FusesSet" => storage
            .events()
            .find_fuses_set_by_id(&reference.id)
            .await?
            .map(FusesSetEvent::from)
            .map(DomainEvent::from),
        "ExpiryExtended" => storage
            .events()
            .find_expiry_extended_by_id(&reference.id)
            .await?
            .map(ExpiryExtendedEvent::from)
            .map(DomainEvent::from),
        _ => None,
    };
    Ok(event)
}

pub async fn hydrate_registration_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<RegistrationEvent>> {
    let event = match reference.kind.as_str() {
        "NameRegistered" => storage
            .events()
            .find_name_registered_by_id(&reference.id)
            .await?
            .map(NameRegisteredEvent::from)
            .map(RegistrationEvent::from),
        "NameRenewed" => storage
            .events()
            .find_name_renewed_by_id(&reference.id)
            .await?
            .map(NameRenewedEvent::from)
            .map(RegistrationEvent::from),
        "NameTransferred" => storage
            .events()
            .find_name_transferred_by_id(&reference.id)
            .await?
            .map(NameTransferredEvent::from)
            .map(RegistrationEvent::from),
        _ => None,
    };
    Ok(event)
}

pub async fn hydrate_resolver_event(
    storage: &Storage,
    reference: EventReferenceRow,
) -> Result<Option<ResolverEvent>> {
    let event = match reference.kind.as_str() {
        "AddrChanged" => storage
            .events()
            .find_addr_changed_by_id(&reference.id)
            .await?
            .map(AddrChangedEvent::from)
            .map(ResolverEvent::from),
        "MulticoinAddrChanged" => storage
            .events()
            .find_multicoin_addr_changed_by_id(&reference.id)
            .await?
            .map(MulticoinAddrChangedEvent::from)
            .map(ResolverEvent::from),
        "NameChanged" => storage
            .events()
            .find_name_changed_by_id(&reference.id)
            .await?
            .map(NameChangedEvent::from)
            .map(ResolverEvent::from),
        "AbiChanged" => storage
            .events()
            .find_abi_changed_by_id(&reference.id)
            .await?
            .map(AbiChangedEvent::from)
            .map(ResolverEvent::from),
        "PubkeyChanged" => storage
            .events()
            .find_pubkey_changed_by_id(&reference.id)
            .await?
            .map(PubkeyChangedEvent::from)
            .map(ResolverEvent::from),
        "TextChanged" => storage
            .events()
            .find_text_changed_by_id(&reference.id)
            .await?
            .map(TextChangedEvent::from)
            .map(ResolverEvent::from),
        "ContenthashChanged" => storage
            .events()
            .find_contenthash_changed_by_id(&reference.id)
            .await?
            .map(ContenthashChangedEvent::from)
            .map(ResolverEvent::from),
        "InterfaceChanged" => storage
            .events()
            .find_interface_changed_by_id(&reference.id)
            .await?
            .map(InterfaceChangedEvent::from)
            .map(ResolverEvent::from),
        "AuthorisationChanged" => storage
            .events()
            .find_authorisation_changed_by_id(&reference.id)
            .await?
            .map(AuthorisationChangedEvent::from)
            .map(ResolverEvent::from),
        "VersionChanged" => storage
            .events()
            .find_version_changed_by_id(&reference.id)
            .await?
            .map(VersionChangedEvent::from)
            .map(ResolverEvent::from),
        _ => None,
    };
    Ok(event)
}
