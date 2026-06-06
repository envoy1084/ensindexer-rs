use async_graphql::Enum;
use storage::EventOrderField;

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
#[graphql(name = "DomainEvent_orderBy")]
pub enum EventOrderBy {
    #[default]
    #[graphql(name = "id")]
    Id,
    #[graphql(name = "blockNumber")]
    BlockNumber,
    #[graphql(name = "transactionID")]
    TransactionId,
    #[graphql(name = "domain")]
    Domain,
    #[graphql(name = "domain__id")]
    DomainId,
    #[graphql(name = "domain__name")]
    DomainName,
    #[graphql(name = "domain__labelName")]
    DomainLabelName,
    #[graphql(name = "domain__labelhash")]
    DomainLabelhash,
    #[graphql(name = "domain__subdomainCount")]
    DomainSubdomainCount,
    #[graphql(name = "domain__ttl")]
    DomainTtl,
    #[graphql(name = "domain__isMigrated")]
    DomainIsMigrated,
    #[graphql(name = "domain__createdAt")]
    DomainCreatedAt,
    #[graphql(name = "domain__expiryDate")]
    DomainExpiryDate,
}

impl From<EventOrderBy> for EventOrderField {
    fn from(value: EventOrderBy) -> Self {
        match value {
            EventOrderBy::Id => Self::Id,
            EventOrderBy::BlockNumber => Self::BlockNumber,
            EventOrderBy::TransactionId => Self::TransactionId,
            EventOrderBy::Domain => Self::Domain,
            EventOrderBy::DomainId => Self::DomainId,
            EventOrderBy::DomainName => Self::DomainName,
            EventOrderBy::DomainLabelName => Self::DomainLabelName,
            EventOrderBy::DomainLabelhash => Self::DomainLabelhash,
            EventOrderBy::DomainSubdomainCount => Self::DomainSubdomainCount,
            EventOrderBy::DomainTtl => Self::DomainTtl,
            EventOrderBy::DomainIsMigrated => Self::DomainIsMigrated,
            EventOrderBy::DomainCreatedAt => Self::DomainCreatedAt,
            EventOrderBy::DomainExpiryDate => Self::DomainExpiryDate,
        }
    }
}

macro_rules! event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        #[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Default)]
        #[graphql(name = $graphql_name)]
        pub enum $name {
            #[default]
            #[graphql(name = "id")]
            Id,
            #[graphql(name = "blockNumber")]
            BlockNumber,
            #[graphql(name = "transactionID")]
            TransactionId,
            $(
                #[graphql(name = $graphql_name_variant)]
                $variant,
            )*
        }

        impl From<$name> for EventOrderField {
            fn from(value: $name) -> Self {
                match value {
                    $name::Id => Self::Id,
                    $name::BlockNumber => Self::BlockNumber,
                    $name::TransactionId => Self::TransactionId,
                    $($name::$variant => Self::$field,)*
                }
            }
        }
    };
}

macro_rules! domain_event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        event_order_wrapper!($name, $graphql_name, [
            Domain => ("domain", Domain),
            DomainId => ("domain__id", DomainId),
            DomainName => ("domain__name", DomainName),
            DomainLabelName => ("domain__labelName", DomainLabelName),
            DomainLabelhash => ("domain__labelhash", DomainLabelhash),
            DomainSubdomainCount => ("domain__subdomainCount", DomainSubdomainCount),
            DomainTtl => ("domain__ttl", DomainTtl),
            DomainIsMigrated => ("domain__isMigrated", DomainIsMigrated),
            DomainCreatedAt => ("domain__createdAt", DomainCreatedAt),
            DomainExpiryDate => ("domain__expiryDate", DomainExpiryDate),
            $($variant => ($graphql_name_variant, $field),)*
        ]);
    };
}

macro_rules! parent_domain_event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        domain_event_order_wrapper!($name, $graphql_name, [
            ParentDomain => ("parentDomain", ParentDomain),
            ParentDomainId => ("parentDomain__id", ParentDomainId),
            ParentDomainName => ("parentDomain__name", ParentDomainName),
            ParentDomainLabelName => ("parentDomain__labelName", ParentDomainLabelName),
            ParentDomainLabelhash => ("parentDomain__labelhash", ParentDomainLabelhash),
            ParentDomainSubdomainCount => ("parentDomain__subdomainCount", ParentDomainSubdomainCount),
            ParentDomainTtl => ("parentDomain__ttl", ParentDomainTtl),
            ParentDomainIsMigrated => ("parentDomain__isMigrated", ParentDomainIsMigrated),
            ParentDomainCreatedAt => ("parentDomain__createdAt", ParentDomainCreatedAt),
            ParentDomainExpiryDate => ("parentDomain__expiryDate", ParentDomainExpiryDate),
            $($variant => ($graphql_name_variant, $field),)*
        ]);
    };
}

macro_rules! registration_event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        event_order_wrapper!($name, $graphql_name, [
            Registration => ("registration", Registration),
            RegistrationId => ("registration__id", RegistrationId),
            RegistrationRegistrationDate => ("registration__registrationDate", RegistrationRegistrationDate),
            RegistrationExpiryDate => ("registration__expiryDate", RegistrationExpiryDate),
            RegistrationCost => ("registration__cost", RegistrationCost),
            RegistrationLabelName => ("registration__labelName", RegistrationLabelName),
            $($variant => ($graphql_name_variant, $field),)*
        ]);
    };
}

macro_rules! resolver_event_order_wrapper {
    ($name:ident, $graphql_name:literal, [$($variant:ident => ($graphql_name_variant:literal, $field:ident)),* $(,)?]) => {
        event_order_wrapper!($name, $graphql_name, [
            Resolver => ("resolver", Resolver),
            ResolverId => ("resolver__id", ResolverId),
            ResolverAddress => ("resolver__address", ResolverAddress),
            ResolverContentHash => ("resolver__contentHash", ResolverContentHash),
            $($variant => ($graphql_name_variant, $field),)*
        ]);
    };
}

domain_event_order_wrapper!(TransferOrderBy, "Transfer_orderBy", [
    Owner => ("owner", Owner),
    OwnerId => ("owner__id", OwnerId),
]);
parent_domain_event_order_wrapper!(NewOwnerOrderBy, "NewOwner_orderBy", [
    Owner => ("owner", Owner),
    OwnerId => ("owner__id", OwnerId),
]);
domain_event_order_wrapper!(NewResolverOrderBy, "NewResolver_orderBy", [
    Resolver => ("resolver", Resolver),
    ResolverId => ("resolver__id", ResolverId),
    ResolverAddress => ("resolver__address", ResolverAddress),
    ResolverContentHash => ("resolver__contentHash", ResolverContentHash),
]);
domain_event_order_wrapper!(NewTtlOrderBy, "NewTTL_orderBy", [
    Ttl => ("ttl", Ttl),
]);
domain_event_order_wrapper!(WrappedTransferOrderBy, "WrappedTransfer_orderBy", [
    Owner => ("owner", Owner),
    OwnerId => ("owner__id", OwnerId),
]);
domain_event_order_wrapper!(NameWrappedOrderBy, "NameWrapped_orderBy", [
    Name => ("name", Name),
    Fuses => ("fuses", Fuses),
    Owner => ("owner", Owner),
    OwnerId => ("owner__id", OwnerId),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
domain_event_order_wrapper!(NameUnwrappedOrderBy, "NameUnwrapped_orderBy", [
    Owner => ("owner", Owner),
    OwnerId => ("owner__id", OwnerId),
]);
domain_event_order_wrapper!(FusesSetOrderBy, "FusesSet_orderBy", [
    Fuses => ("fuses", Fuses),
]);
domain_event_order_wrapper!(ExpiryExtendedOrderBy, "ExpiryExtended_orderBy", [
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
registration_event_order_wrapper!(NameRegisteredOrderBy, "NameRegistered_orderBy", [
    Registrant => ("registrant", Registrant),
    RegistrantId => ("registrant__id", RegistrantId),
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
registration_event_order_wrapper!(NameRenewedOrderBy, "NameRenewed_orderBy", [
    ExpiryDate => ("expiryDate", ExpiryDate),
]);
registration_event_order_wrapper!(NameTransferredOrderBy, "NameTransferred_orderBy", [
    NewOwner => ("newOwner", NewOwner),
    NewOwnerId => ("newOwner__id", NewOwnerId),
]);
resolver_event_order_wrapper!(AddrChangedOrderBy, "AddrChanged_orderBy", [
    Addr => ("addr", Addr),
    AddrId => ("addr__id", AddrId),
]);
resolver_event_order_wrapper!(MulticoinAddrChangedOrderBy, "MulticoinAddrChanged_orderBy", [
    CoinType => ("coinType", CoinType),
    Addr => ("addr", Addr),
]);
resolver_event_order_wrapper!(NameChangedOrderBy, "NameChanged_orderBy", [
    Name => ("name", Name),
]);
resolver_event_order_wrapper!(AbiChangedOrderBy, "AbiChanged_orderBy", [
    ContentType => ("contentType", ContentType),
]);
resolver_event_order_wrapper!(PubkeyChangedOrderBy, "PubkeyChanged_orderBy", [
    X => ("x", X),
    Y => ("y", Y),
]);
resolver_event_order_wrapper!(TextChangedOrderBy, "TextChanged_orderBy", [
    Key => ("key", Key),
    Value => ("value", Value),
]);
resolver_event_order_wrapper!(ContenthashChangedOrderBy, "ContenthashChanged_orderBy", [
    Hash => ("hash", Hash),
]);
resolver_event_order_wrapper!(InterfaceChangedOrderBy, "InterfaceChanged_orderBy", [
    InterfaceId => ("interfaceID", InterfaceId),
    Implementer => ("implementer", Implementer),
]);
resolver_event_order_wrapper!(AuthorisationChangedOrderBy, "AuthorisationChanged_orderBy", [
    Owner => ("owner", Owner),
    Target => ("target", Target),
    IsAuthorized => ("isAuthorized", IsAuthorized),
]);
resolver_event_order_wrapper!(VersionChangedOrderBy, "VersionChanged_orderBy", [
    Version => ("version", Version),
]);
registration_event_order_wrapper!(RegistrationEventOrderBy, "RegistrationEvent_orderBy", []);
resolver_event_order_wrapper!(ResolverEventOrderBy, "ResolverEvent_orderBy", []);
