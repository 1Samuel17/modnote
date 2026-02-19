use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DbConn, DbErr, Set};
use crate::entities::{notebooks};


// Create
pub async fn create_notebook(db: &DbConn, title: &String, desc: &Option<String>) -> Result<notebooks::Model, DbErr> {
    let new_notebook = notebooks::ActiveModel {
        id: NotSet,
        notebook_name: Set(title.to_owned()),
        description: Set(desc.to_owned().unwrap_or_default()),
    };

    new_notebook.insert(db).await
}

// Read

// Update

// Delete