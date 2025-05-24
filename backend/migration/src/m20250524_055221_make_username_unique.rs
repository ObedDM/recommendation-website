use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(
                        ColumnDef::new(User::username)
                        .string_len(18)
                        .not_null()
                        .unique_key()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(
                        ColumnDef::new(User::username)
                        .string_len(18)
                        .not_null()
                    )
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    username,
    email,
    password,
    registration_date,
    country
}
