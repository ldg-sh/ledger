pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250819_160016_create_table::Migration)]
    }

    fn migration_table_name() -> DynIden {
        "ledger_migrations".into_iden()
    }
}
mod m20250819_160016_create_table;
