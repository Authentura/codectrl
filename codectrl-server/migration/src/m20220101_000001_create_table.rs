use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Connection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connection::Uuid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connection::SentLogs).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Connection::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Connection {
    Table,
    Uuid,
    SentLogs,
}
