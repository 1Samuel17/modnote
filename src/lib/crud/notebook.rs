use crate::entities::notebooks;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DbConn, DbErr, Set};

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
