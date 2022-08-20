use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use serde_json::json;
use warp::{reject::Reject, reply::Json, Filter, Rejection};

use crate::{
    model::{Db, TodoMac, TodoPatch},
    security::{utx_from_token, UserCtx},
};

use super::{filter_auth::do_auth, filter_utils::with_db};

pub fn todo_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todo_paths = warp::path(base_path).and(warp::path("todos"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    // LIST 'GET todos/'
    let list = todo_paths
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(todo_list);

    // GET One 'GET todos/:id'
    let get = todo_paths
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_get);

    // CREATE 'POST todos/'
    let create = todo_paths
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(todo_create);

    // PATCH 'PATCH todos/:id'
    let update = todo_paths
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(todo_udpate);

    // DELETE 'DELETE todos/:id'
    let delete = todo_paths
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_delete);

    list.or(create).or(get).or(update).or(delete)
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::list(&db, &utx).await?;
    json_response(todos)
}

async fn todo_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::get(&db, &utx, id).await?;
    json_response(todo)
}

async fn todo_create(db: Arc<Db>, utx: UserCtx, patch: TodoPatch) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::create(&db, &utx, patch).await?;
    json_response(todo)
}

async fn todo_udpate(
    db: Arc<Db>,
    utx: UserCtx,
    id: i64,
    patch: TodoPatch,
) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::update(&db, &utx, id, patch).await?;
    json_response(todo)
}

async fn todo_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::delete(&db, &utx, id).await?;
    json_response(todo)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

// region: Tests
#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod test;
// endregion: Tests
