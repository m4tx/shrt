pub use sea_orm_migration::prelude::*;

mod m20240302_000001_create_table_link;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240302_000001_create_table_link::Migration)]
    }
}
