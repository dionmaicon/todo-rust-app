use std::{sync::Arc, convert::Infallible};

use warp::{Filter, Rejection};

use crate::{security::{UserCtx, utx_from_token}, model::Db};

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

