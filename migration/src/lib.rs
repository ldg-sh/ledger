pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250819_160016_create_table::Migration),
            Box::new(m20251009_223003_access::Migration),
        ]
    }
}
mod m20250819_160016_create_table;
mod m20251009_223003_access;
