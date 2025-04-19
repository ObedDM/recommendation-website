use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();
        db.execute_unprepared(
            r#"ALTER TABLE "user" DROP CONSTRAINT user_pkey"#
        ).await?;
        
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string())
                    ).to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(User::Table)
                .drop_column(Alias::new("id"))
                .to_owned()
        )
        .await?;

    manager
        .alter_table(
            Table::alter()
                .table(User::Table)
                .modify_column(
                    ColumnDef::new(User::username)
                        .string_len(18)
                        .primary_key()
                )
                .to_owned()
        )
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
