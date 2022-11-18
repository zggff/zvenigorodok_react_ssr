use actix_web::{HttpResponse, Responder, Scope};

use actix_web::web::{self, scope, Query};
use futures::stream::StreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export)]
pub enum ReviewTarget {
    #[serde(alias = "TYRES")]
    Tyres,
    #[serde(alias = "CLEANING")]
    Cleaning,
    HomeMaster,
}

pub fn api(path: &str) -> Scope {
    scope(path).service(get_reviews).service(add_review)
}

// this is stored in the database
#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export)]
pub struct Review {
    text: String,
    user: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    #[ts(type = "Date")]
    date: bson::DateTime,
    target: ReviewTarget,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetReviewsQuery {
    target: Option<ReviewTarget>,
}

#[actix_web::get("/get_reviews")]
async fn get_reviews(
    collection: actix_web::web::Data<mongodb::Collection<Review>>,
    query: Query<GetReviewsQuery>,
) -> impl Responder {
    let filter = query
        .into_inner()
        .target
        .and_then(|target| bson::to_bson(&target).ok())
        .map(|target| doc! {"target": target});
    let cursor = collection.find(filter, None).await;
    match cursor {
        Ok(cursor) => {
            let reviews: Vec<Result<Review, _>> = cursor.collect().await;
            let reviews: Vec<Review> = reviews
                .into_iter()
                .collect::<Result<Vec<Review>, _>>()
                .unwrap_or_default();
            HttpResponse::Ok().json(reviews)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::post("/add_review")]
async fn add_review(
    collection: web::Data<mongodb::Collection<Review>>,
    req_body: web::Json<Review>,
) -> impl Responder {
    let result = collection.insert_one(req_body.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
