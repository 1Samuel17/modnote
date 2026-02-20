use crate::entities::notebooks::{self};
use crate::entities::prelude::Notebooks;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, QueryFilter,
};

// Create
pub async fn create_notebook(
    db: &DbConn,
    name: &String,
    desc: &String,
) -> Result<notebooks::Model, DbErr> {
    let new_notebook = notebooks::ActiveModel {
        id: NotSet,
        notebook_name: Set(name.to_owned()),
        description: Set(desc.to_owned()),
    };

    new_notebook.insert(db).await
}

// Read

// Update

// Delete
pub async fn delete_notebook(db: &DbConn, name: &String) -> Result<String, DbErr> {
    let book: Option<notebooks::Model> = Notebooks::find()
        .filter(notebooks::Column::NotebookName.eq(name.to_owned()))
        .one(db)
        .await?;
    let book: notebooks::Model = book.unwrap();
    book.delete(db).await?;
    Ok("Success".to_string())
}
