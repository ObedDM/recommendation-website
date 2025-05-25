use axum::{extract::{Multipart, State}, response::Json};
use axum_extra::TypedHeader;
use headers::Cookie;
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};

use crate::{services::profile::{errors::{ProfileDeleteError, ProfilePictureError}, profile::{delete_user_profile, get_profile_picture, update_profile_picture}}, utils::jwt_helper::jwt_helper};

pub async fn get_profile(State(db): State<DatabaseConnection>, TypedHeader(cookie_header): TypedHeader<Cookie>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match jwt_helper(cookie_header, &db).await {
        Ok(payload) => {
            log::info!("User token validated");
            
            match payload.id {
                Some(user_id) => {

                    //gets profile picture and sends it as base64 **Change to multipart later
                    let mut user_picture = match get_profile_picture(user_id).await {
                        Ok(picture) => picture,
                        
                        Err(e @ ProfilePictureError::FileReadingFailed(_, _, _)) => {
                            log::error!("{}", e);
                            
                            "null".to_string()
                        },

                        Err(e @ _) => {
                            log::error!("{}", e);
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                            ))
                        },
                    };
                    
                    log::info!("Profile info succesfully retrieved");

                    if user_picture != "null" {
                        user_picture = format!("data:image/png;base64,{}", user_picture);
                    }

                    Ok((
                        StatusCode::OK,
                        Json(json!({
                            "message": "successful",
                            "id": user_id,
                            "username": payload.username,
                            "picture": user_picture,
                        }))
                    ))
                }

                None => {
                    log::error!("Invalid token: ID missing in payload");
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Invalid token" }))
                    ))
                }
            }
        },
        Err((error, message)) => Err((error, message))
    }
}



pub async fn put_picture(State(db): State<DatabaseConnection>, TypedHeader(cookie_header): TypedHeader<Cookie>, mut image: Multipart) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    
    match jwt_helper(cookie_header, &db).await {
        Ok(payload) => {
            log::info!("User token validated");

            match payload.id {
                Some(user_id) => {
                    match update_profile_picture(user_id, image).await {
                        Ok(message) => {
                            log::info!("{}", message);
                            Ok((
                                StatusCode::OK,
                                Json(json!({ "message": "User profile picture updated successfully" }))
                            ))
                        },
                        
                        Err(e @ ProfilePictureError::MultipartError(_)) => {
                            log::warn!("{}", e);
                            Err((
                                StatusCode::BAD_REQUEST,
                                Json(json!({ "message": "Missing image field" }))
                            ))
                        },

                        Err(e @ ProfilePictureError::ByteReadError(_)) => {
                            log::error!("{}", e);
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                            ))
                        },
                        
                        Err(e @ ProfilePictureError::ErrorUpdatingFile(_, _, _)) => {
                            log::error!("{}", e);
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                            ))
                        },

                        Err(e @ _) => {
                            log::error!("{}", e);
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                            ))
                        }
                    }
                },

                None => {
                    log::error!("Invalid token: ID missing in payload");
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Invalid token" }))
                    ))
                }
            }
        },

        Err((error, message)) => Err((error, message))
    }
}

pub async fn delete(State(db): State<DatabaseConnection>, TypedHeader(cookie_header): TypedHeader<Cookie>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    
    match jwt_helper(cookie_header, &db).await {
        Ok(payload) => {
            log::info!("User token validated");

            match payload.id {
                Some(user_id) => {
                    match delete_user_profile(user_id, &db).await {
                        Ok(message) => {
                            log::info!("{}", message);
                            Ok((
                                StatusCode::OK,
                                Json(json!({ "message": "User deleted" }))
                            ))
                        },
                        
                        Err(e @ ProfileDeleteError::UserDeletionFailed(_, _)) => {
                            log::error!("{}", e);
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                            ))
                        },
                        
                        Err(e @ ProfileDeleteError::UserNotFound(_)) => {
                            log::warn!("{}", e);
                            Err((
                                StatusCode::BAD_REQUEST,
                                Json(json!({ "message": "Invalid user" }))
                            ))
                        }
                    }
                },

                None => {
                    log::error!("Invalid token: ID missing in payload");
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Invalid token" }))
                    ))
                }
            }
        },

        Err((error, message)) => Err((error, message))
    }
}