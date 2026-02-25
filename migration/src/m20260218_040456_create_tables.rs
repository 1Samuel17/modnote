use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create notebooks table
        manager
            .create_table(
                Table::create()
                    .table("Notebooks")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("notebook_name"))
                    .col(string("description"))
                    .to_owned(),
            )
            .await?;

        // create notes table
        manager
            .create_table(
                Table::create()
                    .table("Notes")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("title"))
                    .col(string("content"))
                    .to_owned(),
            )
            .await?;

        // create tags table
        manager
            .create_table(
                Table::create()
                    .table("Tags")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("tag_name"))
                    .to_owned(),
            )
            .await?;

        // create the relationship tables: Notebook -> Notes (notebook has many notes)
        manager
            .create_table(
                Table::create()
                    .table("NotebookNotes")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("notebook_id"))
                    .col(integer("note_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notebooknotes-notebook_id")
                            .from(Alias::new("NotebookNotes"), Alias::new("notebook_id"))
                            .to(Alias::new("Notebooks"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notebooknotes-note_id")
                            .from(Alias::new("NotebookNotes"), Alias::new("note_id"))
                            .to(Alias::new("Notes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create the relationship tables: Note -> Tags (note has many tags)
        manager
            .create_table(
                Table::create()
                    .table("NoteTags")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(integer("note_id"))
                    .col(integer("tag_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notetags-notebook_id")
                            .from(Alias::new("NoteTags"), Alias::new("note_id"))
                            .to(Alias::new("Notes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-notetags-tag_id")
                            .from(Alias::new("NoteTags"), Alias::new("tag_id"))
                            .to(Alias::new("Tags"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("Notebooks").to_owned()).await?;
        manager.drop_table(Table::drop().table("Notes").to_owned()).await?;
        manager.drop_table(Table::drop().table("Tags").to_owned()).await?;
        manager.drop_table(Table::drop().table("NotebookNotes").to_owned()).await?;
        manager.drop_table(Table::drop().table("NoteTags").to_owned()).await?;
        Ok(())
    }
}
