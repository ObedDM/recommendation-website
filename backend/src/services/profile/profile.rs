use axum::extract::Multipart;
use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;
use image::ImageReader;
use base64::{Engine, engine::general_purpose::STANDARD};

use super::errors::{ProfileDeleteError, ProfilePictureError};

const DEFAULT_ROUTE: &str = "static/images/default/";
const USERS_ROUTE: &str = "static/images/users/";


pub async fn get_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {
    let file = format!("{}profile_picture_{}.png", USERS_ROUTE, user_id);
    
    let picture_bytes = tokio::fs::read(&file).await
    .map_err(|e| ProfilePictureError::FileReadingFailed(format!("{}.png", user_id), e.to_string(), USERS_ROUTE.to_string()))?;

    Ok(
        STANDARD.encode(&picture_bytes)
    )
}

pub async fn delete_user_profile(user_id: Uuid, db: &DatabaseConnection) -> Result<String, ProfileDeleteError> {

    match user::Entity::delete_by_id(user_id).exec(db).await {
        Ok(result) if result.rows_affected == 1 =>
            Ok(format!("User {} successfully deleted", user_id)),
        Ok(_) => Err(ProfileDeleteError::UserNotFound(user_id.to_string())),  
        Err(e) => Err(ProfileDeleteError::UserDeletionFailed(user_id.to_string(), e.to_string()))
    }
}

pub async fn update_profile_picture(user_id: Uuid, mut new_picture: Multipart) -> Result<String, ProfilePictureError> {
    let picture_data: Vec<u8>;  

    let field = new_picture.next_field().await
    .map_err(|e| ProfilePictureError::MultipartError((e.to_string())))?
    .ok_or_else(|| ProfilePictureError::MissingField("image".to_string()))?;

    picture_data = field.bytes().await
    .map_err(|e| ProfilePictureError::ByteReadError(e.to_string()))?
    .to_vec();

    let image_path = format!("{}profile_picture_{}.png", USERS_ROUTE, user_id);
    let mut file = File::create(&image_path).await.unwrap();

    file.write_all(&picture_data).await
    .map_err(|e| ProfilePictureError::ErrorUpdatingFile(image_path.clone(), e.to_string(), USERS_ROUTE.to_string()))?;

    Ok(
        format!("image: {} updated successfully", image_path)
    )
}

pub async fn create_default_profile_picture(user_id: Uuid) -> Result<String, ProfilePictureError> {

    let reader = ImageReader::open(format!("{}default_pfp.png", DEFAULT_ROUTE))
    .map_err(|e| ProfilePictureError::FileReadingFailed(format!("{}.png", user_id), e.to_string(), DEFAULT_ROUTE.to_string()))?;
    
    let default_picture = reader.decode()
    .map_err(|e| ProfilePictureError::ImageDecodingFailed("default_pfp.png".to_string(), e.to_string()))?;

    default_picture.save(format!("{}profile_picture_{}.png", USERS_ROUTE, user_id))
    .map_err(|e| ProfilePictureError::ImageSavingFailed(format!("profile_picture_{}.png", user_id), e.to_string(), USERS_ROUTE.to_string()))?;

    Ok(
        format!("default picture for {} created successfully", user_id)
    )
}