use crate::{error::*, filters::*, models::*};

use super::{EventsRepo, columns::*};

impl EventsRepo<'_> {
    pub async fn find_addr_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AddrChangedEventRow>> {
        self.find_event("addr_changed_events", addr_changed_event_columns(), id)
            .await
    }

    pub async fn list_addr_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AddrChangedEventRow>> {
        self.list_events(
            "addr_changed_events",
            addr_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_multicoin_addr_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<MulticoinAddrChangedEventRow>> {
        self.find_event(
            "multicoin_addr_changed_events",
            multicoin_addr_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_multicoin_addr_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<MulticoinAddrChangedEventRow>> {
        self.list_events(
            "multicoin_addr_changed_events",
            multicoin_addr_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameChangedEventRow>> {
        self.find_event("name_changed_events", name_changed_event_columns(), id)
            .await
    }

    pub async fn list_name_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameChangedEventRow>> {
        self.list_events(
            "name_changed_events",
            name_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_abi_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AbiChangedEventRow>> {
        self.find_event("abi_changed_events", abi_changed_event_columns(), id)
            .await
    }

    pub async fn list_abi_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AbiChangedEventRow>> {
        self.list_events(
            "abi_changed_events",
            abi_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_pubkey_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<PubkeyChangedEventRow>> {
        self.find_event("pubkey_changed_events", pubkey_changed_event_columns(), id)
            .await
    }

    pub async fn list_pubkey_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<PubkeyChangedEventRow>> {
        self.list_events(
            "pubkey_changed_events",
            pubkey_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_text_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<TextChangedEventRow>> {
        self.find_event("text_changed_events", text_changed_event_columns(), id)
            .await
    }

    pub async fn list_text_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<TextChangedEventRow>> {
        self.list_events(
            "text_changed_events",
            text_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_contenthash_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<ContenthashChangedEventRow>> {
        self.find_event(
            "contenthash_changed_events",
            contenthash_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_contenthash_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ContenthashChangedEventRow>> {
        self.list_events(
            "contenthash_changed_events",
            contenthash_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_interface_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<InterfaceChangedEventRow>> {
        self.find_event(
            "interface_changed_events",
            interface_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_interface_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<InterfaceChangedEventRow>> {
        self.list_events(
            "interface_changed_events",
            interface_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_authorisation_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<AuthorisationChangedEventRow>> {
        self.find_event(
            "authorisation_changed_events",
            authorisation_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_authorisation_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<AuthorisationChangedEventRow>> {
        self.list_events(
            "authorisation_changed_events",
            authorisation_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_version_changed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<VersionChangedEventRow>> {
        self.find_event(
            "version_changed_events",
            version_changed_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_version_changed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<VersionChangedEventRow>> {
        self.list_events(
            "version_changed_events",
            version_changed_event_columns(),
            "resolver_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }
}
