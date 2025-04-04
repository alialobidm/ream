use std::sync::Arc;

use alloy_primitives::B256;
use redb::{Database, Durability, TableDefinition};

use super::{SSZEncoding, Table};
use crate::errors::StoreError;

/// Table definition for the Slot Index table
///
/// Key: slot number
/// Value: block_root
pub const SLOT_INDEX_TABLE: TableDefinition<u64, SSZEncoding<B256>> =
    TableDefinition::new("slot_index");

pub struct SlotIndexTable {
    pub db: Arc<Database>,
}

impl Table for SlotIndexTable {
    type Key = u64;

    type Value = B256;

    fn get(&self, key: Self::Key) -> Result<Option<Self::Value>, StoreError> {
        let read_txn = self.db.begin_read()?;

        let table = read_txn.open_table(SLOT_INDEX_TABLE)?;
        let result = table.get(key)?;
        Ok(result.map(|res| res.value()))
    }

    fn insert(&self, key: Self::Key, value: Self::Value) -> Result<(), StoreError> {
        let mut write_txn = self.db.begin_write()?;
        write_txn.set_durability(Durability::Immediate);
        let mut table = write_txn.open_table(SLOT_INDEX_TABLE)?;
        table.insert(key, value)?;
        drop(table);
        write_txn.commit()?;
        Ok(())
    }
}
