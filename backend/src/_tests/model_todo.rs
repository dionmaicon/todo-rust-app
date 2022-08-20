use warp::hyper::StatusCode;

use crate::{model::{init_db, TodoPatch}, security::utx_from_token};
use std::error::Error;

use super::{TodoMac, TodoStatus};

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let todos = TodoMac::list(&db, &utx).await?;
    
    assert_eq!(2, todos.len(), "TODO ");
    
    Ok(())
}
#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    
    let utx = utx_from_token(&db, "123").await?;

    let data_fx = TodoPatch {
        title: Some("test - model todo create 1".to_string()),
        status: Some(TodoStatus::Open),
        ..Default::default()
    };
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    assert_eq!(todo_created.id, 1000);
    assert_eq!(todo_created.cid, 123);
    
    Ok(())
}

#[tokio::test]
async fn model_todo_get() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    
    let utx = utx_from_token(&db, "123").await?;

    let data_fx = TodoPatch {
        title: Some("test - model todo create 1".to_string()),
        status: Some(TodoStatus::Open),
        ..Default::default()
    };
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    
    let todo = TodoMac::get(&db, &utx, todo_created.id).await?;
        
    assert_eq!(todo_created.id, todo.id);
    assert_eq!(todo_created.title, todo.title);
    
    Ok(())
}

#[tokio::test]
async fn model_todo_update() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    
    let utx = utx_from_token(&db,"123").await?;

    let data_fx = TodoPatch {
        title: Some("test - model todo create 1".to_string()),
        status: Some(TodoStatus::Open),
        ..Default::default()
    };
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    
    let todo_patch = TodoPatch { title: Some("test updated 2".to_string()), ..Default::default() };
    let todo_updated = TodoMac::update(&db, &utx, todo_created.id, todo_patch.clone()).await?;
        
    assert_eq!(todo_updated.id, 1000);
    assert_eq!(todo_updated.title, "test updated 2");
    
    Ok(())
}


#[tokio::test]
async fn model_todo_delete() -> Result<(), Box<dyn Error>> {
    let db = init_db().await?;
    
    let utx = utx_from_token(&db, "123").await?;

    let data_fx = TodoPatch {
        title: Some("test - model todo create 1".to_string()),
        status: Some(TodoStatus::Open),
        ..Default::default()
    };
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    
    let todo_deleted = TodoMac::delete(&db, &utx, todo_created.id).await?;
    
    
    assert_eq!(todo_deleted.id, 1000);
    Ok(())
}


