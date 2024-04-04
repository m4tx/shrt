use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Link::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Link::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Link::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Link::Url).string().not_null())
                    .col(ColumnDef::new(Link::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Link::Visits).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-link_created_at")
                    .table(Link::Table)
                    .col(Link::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-link_created_at").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Link::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Link {
    Table,
    Id,
    Slug,
    Url,
    CreatedAt,
    Visits,
}
