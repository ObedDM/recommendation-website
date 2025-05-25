use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use image::ImageReader;
use base64::{Engine, engine::general_purpose::STANDARD};

use super::errors::{ProfileDeleteError, ProfilePictureError};

pub async fn get_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {
    let users_route: String = "static/images/users/".to_string();
    let file = format!("{}profile_picture_{}.png", users_route, user_id);
    
    let picture_bytes = tokio::fs::read(&file).await
    .map_err(|e| ProfilePictureError::FileReadingFailed(format!("{}.png", user_id), e.to_string(), users_route))?;

    Ok(
        STANDARD.encode(&picture_bytes)
    )
}

pub async fn delete_profile(user_id: Uuid, db: &DatabaseConnection) -> Result<String, ProfileDeleteError> {

    match user::Entity::delete_by_id(user_id).exec(db).await {
        Ok(result) if result.rows_affected == 1 =>
            Ok(format!("User {} successfully deleted", user_id)),
        Ok(_) => Err(ProfileDeleteError::UserNotFound(user_id.to_string())),  
        Err(e) => Err(ProfileDeleteError::UserDeletionFailed(user_id.to_string(), e.to_string()))
    }
}

pub async fn create_default_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {
    let default_route: String = "static/images/default/".to_string();
    let users_route: String = "static/images/users/".to_string();

    let reader = ImageReader::open(format!("{}default_pfp.png", default_route))
    .map_err(|e| ProfilePictureError::FileReadingFailed(format!("{}.png", user_id), e.to_string(), default_route))?;
    
    let default_picture = reader.decode()
    .map_err(|e| ProfilePictureError::ImageDecodingFailed("default_pfp.png".to_string(), e.to_string()))?;

    default_picture.save(format!("{}profile_picture_{}.png", users_route, user_id))
    .map_err(|e| ProfilePictureError::ImageSavingFailed(format!("profile_picture_{}.png", user_id), e.to_string(), users_route))?;

    Ok(
        format!("default picture for {} created successfully", user_id)
    )
}