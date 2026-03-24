pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260321_142900_create_table::Migration),
            Box::new(m20260321_142905_create_user::Migration),
            Box::new(m20260321_142910_create_refresh_token::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        "ledger_migrations".into_iden()
    }
}

mod m20260321_142900_create_table;
mod m20260321_142905_create_user;
mod m20260321_142910_create_refresh_token;
