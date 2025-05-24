use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use axum::extract::multipart;
use image::ImageReader;

use super::errors::{ProfileDeleteError, ProfilePictureError};

pub async fn get_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {
    todo!()
}

pub async fn delete_profile(user_id: Uuid, db: &DatabaseConnection) -> Result<String, ProfileDeleteError> {

    match user::Entity::delete_by_id(user_id).exec(db).await {
        Ok(result) if result.rows_affected == 1 =>
            Ok(format!("User {} succesfully deleted", user_id)),
        Ok(_) => Err(ProfileDeleteError::UserNotFound(user_id.to_string())),  
        Err(e) => Err(ProfileDeleteError::UserDeletionFailed(user_id.to_string(), e.to_string()))
    }
}

pub async fn create_default_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {
    let default_route: String = "static/images/default/".to_string();
    let users_route: String = "static/images/users/".to_string();

    let reader = ImageReader::open("static/images/default/default_pfp.png")
    .map_err(|e| ProfilePictureError::FileOpeningFailed(e.to_string(), default_route))?;
    
    let default_picture = reader.decode()
    .map_err(|e| ProfilePictureError::ImageDecodingFailed("default_pfp.png".to_string(), e.to_string()))?;

    default_picture.save(format!("{}profile_picture_{}.png", users_route, user_id))
    .map_err(|e| ProfilePictureError::ImageSavingFailed(format!("profile_picture_{}.png", user_id), e.to_string(), users_route))?;

    Ok(
        format!("default picture for {} created successfully", user_id)
    )
}