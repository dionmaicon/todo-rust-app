use serde_json::json;
use warp::{Filter, Rejection, Reply};
use crate::{model::{Db, self}, security, web::todo::todo_rest_filters};
use std::{sync::Arc, path::Path, convert::Infallible};

mod todo;
mod filter_utils;
mod filter_auth;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>)-> Result<(), Error> {
    // Validate web-folder
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNOtFounded( web_folder.to_string()));
    }

    // APIS

    let apis = todo_rest_filters("api", db);

    // static content
    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    
    let static_site = content.or(root_index);
    let routes = apis.or(static_site).recover(handle_rejection);
    
    println!("Starts at 127.0.0.1:/{}", web_port);

    warp::serve(routes).run(([127,0,0,1], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
	// Print to server side
	println!("ERROR - {:?}", err);

	// TODO - Call log API for capture and store

	// Build user message
	let user_message = match err.find::<WebErrorMessage>() {
		Some(err) => err.typ.to_string(),
		None => "Unknown".to_string(),
	};

	let result = json!({ "errorMessage": user_message });
	let result = warp::reply::json(&result);

	Ok(warp::reply::with_status(result, warp::http::StatusCode::BAD_REQUEST))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed because web-folder '{0}' not found")]
    FailStartWebFolderNOtFounded(String),
    #[error("Failed auth missing X-Auth-Token Header")]
    FailAuthMissingXAuth
}

// region: WARP custom errors
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage {typ, message})
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{}", other))
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{}", other))
    }
}

impl From<security::Error> for warp::Rejection {
    fn from(other: security::Error) -> Self {
        WebErrorMessage::rejection("security::Error", format!("{}", other))
    }
}
// endregion: WARP custom errors