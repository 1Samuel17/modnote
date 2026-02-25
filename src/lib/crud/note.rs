use crate::entities::notes::{self};
use crate::entities::prelude::Notes;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, QueryFilter,
};
use std::io;

// CRUD: Create
pub async fn create_note(
    db: &DbConn,
    note_name: String,
    content: String,
) -> Result<notes::Model, DbErr> {
    let new_note =
        notes::ActiveModel { id: NotSet, note_name: Set(note_name), content: Set(content) };

    // create relationship between note and notebook


    new_note.insert(db).await
}

// CRUD: Read (Get)
pub async fn get_all_notes(db: &DbConn) -> Result<Vec<notes::Model>, DbErr> {
    Notes::find().all(db).await
}

pub async fn get_note_by_title(db: &DbConn, title: String) -> Result<notes::Model, DbErr> {
    Notes::find()
        .filter(notes::Column::NoteName.eq(title.to_owned()))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!("Note with title {} not found", title)))
}

// CRUD: Update
pub async fn update_note_by_title(db: &DbConn, title: Option<String>) -> Result<(), DbErr> {
    // Find the note by title
    let note: Option<notes::Model> = Notes::find()
        .filter(notes::Column::NoteName.eq(title.to_owned().unwrap_or_default()))
        .one(db)
        .await
        .unwrap();
    // Create an active model from the found note
    let mut note: notes::ActiveModel = note.unwrap().into();

    // Get new title from user input and update the active model
    println!("Enter new title for note:");
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).expect("Failed to read line");
    let new_title = new_title.trim().to_string();
    note.note_name = Set(new_title);

    // Get new content from user input and update the active model
    let mut new_content = String::new();
    println!("Enter new content for note:");
    io::stdin().read_line(&mut new_content).expect("Failed to read line");
    let new_content = new_content.trim().to_string();
    note.content = Set(new_content);

    // Save the updated note back to the database
    note.update(db).await.unwrap();

    Ok(())
}

// CRUD: Delete
pub async fn delete_note_by_title(db: &DbConn, title: &Option<String>) -> Result<String, DbErr> {
    let note: Option<notes::Model> = Notes::find()
        .filter(notes::Column::NoteName.eq(title.to_owned().unwrap_or_default()))
        .one(db)
        .await?;
    let note: notes::Model = note.unwrap();
    note.delete(db).await?;
    Ok("Successfully deleted note".to_string())
}

pub async fn delete_all_notes(db: &DbConn) -> Result<String, DbErr> {
    let deleted = Notes::delete_many().filter(notes::Column::Id.gt(0)).exec(db).await?;
    Ok(format!("Successfully deleted {} notes", deleted.rows_affected))
}

