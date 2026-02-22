use crate::entities::notebooks::{self};
use crate::entities::prelude::Notebooks;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, QueryFilter,
};
use std::io;

// CRUD: Create
pub async fn create_notebook(
    db: &DbConn,
    name: String,
    desc: String,
) -> Result<notebooks::Model, DbErr> {
    let new_notebook =
        notebooks::ActiveModel { id: NotSet, notebook_name: Set(name), description: Set(desc) };

    new_notebook.insert(db).await
}

// CRUD: Read (Get)
pub async fn get_all_notebooks(db: &DbConn) -> Result<Vec<notebooks::Model>, DbErr> {
    Notebooks::find().all(db).await
}

pub async fn get_notebook_by_name(db: &DbConn, name: String) -> Result<notebooks::Model, DbErr> {
    Notebooks::find()
        .filter(notebooks::Column::NotebookName.eq(name.to_owned()))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!("Notebook with id {} not found", name)))
}

// CRUD: Update
pub async fn update_notebook_by_name(db: &DbConn, name: Option<String>) -> Result<(), DbErr> {
    // Find the notebook by name
    let book: Option<notebooks::Model> = Notebooks::find()
        .filter(notebooks::Column::NotebookName.eq(name.to_owned().unwrap_or_default()))
        .one(db)
        .await
        .unwrap();
    // Create an active model from the found notebook
    let mut book: notebooks::ActiveModel = book.unwrap().into();
    // Get new name from user input and update the active model
    println!("Enter new name for notebook:");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).expect("Failed to read line");
    let new_name = new_name.trim().to_string();
    book.notebook_name = Set(new_name);
    //` Get new description from user input and update the active model
    let mut new_desc = String::new();
    println!("Enter new description for notebook:");
    io::stdin().read_line(&mut new_desc).expect("Failed to read line");
    let new_desc = new_desc.trim().to_string();
    book.description = Set(new_desc);
    // Save the updated notebook back to the database
    book.update(db).await.unwrap();

    Ok(())
}

// CRUD: Delete
pub async fn delete_notebook_by_name(db: &DbConn, name: &Option<String>) -> Result<String, DbErr> {
    let book: Option<notebooks::Model> = Notebooks::find()
        .filter(notebooks::Column::NotebookName.eq(name.to_owned().unwrap_or_default()))
        .one(db)
        .await?;
    let book: notebooks::Model = book.unwrap();
    book.delete(db).await?;
    Ok("Successfully deleted notebook".to_string())
}

pub async fn delete_all_notebooks(db: &DbConn) -> Result<String, DbErr> {
    let deleted = Notebooks::delete_many().filter(notebooks::Column::Id.gt(0)).exec(db).await?;
    Ok(format!("Successfully deleted {} notebooks", deleted.rows_affected))
}
