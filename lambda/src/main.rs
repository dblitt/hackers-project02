//! This is an example function that leverages the Lambda Rust runtime HTTP support
//! and the [axum](https://docs.rs/axum/latest/axum/index.html) web framework.  The
//! runtime HTTP support is backed by the [tower::Service](https://docs.rs/tower-service/0.3.2/tower_service/trait.Service.html)
//! trait.  Axum's applications are also backed by the `tower::Service` trait.  That means
//! that it is fairly easy to build an Axum application and pass the resulting `Service`
//! implementation to the Lambda runtime to run as a Lambda function.  By using Axum instead
//! of a basic `tower::Service` you get web framework niceties like routing, request component
//! extraction, validation, etc.
use aws_sdk_dynamodb::error::DisplayErrorContext;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime,Utc};
use lambda_http::{run, tracing, Error};
use models::checkin::{self, Checkin, CreateCheckinRequest};
use models::user::{CreateUserRequest, User};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;
use std::env::set_var;
use db::get_dynamodb_client;

// pub mod database_service;
mod db;
// pub mod checkin;
// pub mod handler;
// pub mod handler_params;
mod models;
// use database_service::DatabaseService;

pub async fn create_user(
    req: CreateUserRequest,
) -> Result<String> {
    let client = get_dynamodb_client().await?;

    let user_id = Uuid::new_v4().to_string();
    let user = User {
        pk: format!("USER#{}", user_id),
        sk: "METADATA".to_string(),
        name: req.name.clone(),
        email: req.email.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // let item = serde_dynamodb::to_hashmap(&user).unwrap();
    let item = serde_dynamo::to_item(user)?;

    let put_item = client.put_item()
        .table_name("BarBuddyTableTwo")
        .set_item(Some(item));

    let thing = put_item.send().await;// {
    //     Ok(_) => Ok(user_id),
    //     Err(e) => {
    //         eprintln!("DynamoDB PutItem Error: {}", e);
    //         // HttpResponse::InternalServerError().json(json!({"error": "Could not create user"}))
    //         return e;
    //     }
    // }

    match thing
    {
        Ok(_output) => {
            Ok(user_id)
        }
        Err(err) => {
            err.to_string();
            println!("ERROR: {}", DisplayErrorContext(&err));
            return Err(err.into())
        }
    }
}

fn extract_user_id(pk: &str) -> Option<String> {
    pk.strip_prefix("USER#").map(|s| s.to_string())
}

pub async fn create_checkin(
    req: CreateCheckinRequest,
) -> Result<String> {
    let client = get_dynamodb_client().await?;

    let checkin_id = Uuid::new_v4().to_string();
    let checkin = Checkin {
        pk: format!("USER#{}", req.user_id),
        sk: format!("CHECKIN#{}", checkin_id),//"METADATA".to_string(),
        checkin_id: checkin_id.clone(),
        location: req.location.clone(),
        time: chrono::Utc::now().to_rfc3339(),
    };

    // let item = serde_dynamodb::to_hashmap(&user).unwrap();
    let item = serde_dynamo::to_item(checkin)?;

    let put_item = client.put_item()
        .table_name("BarBuddyTableTwo")
        .set_item(Some(item));

    let thing = put_item.send().await;// {
    //     Ok(_) => Ok(user_id),
    //     Err(e) => {
    //         eprintln!("DynamoDB PutItem Error: {}", e);
    //         // HttpResponse::InternalServerError().json(json!({"error": "Could not create user"}))
    //         return e;
    //     }
    // }

    match thing
    {
        Ok(_output) => {
            Ok(checkin_id)
        }
        Err(err) => {
            err.to_string();
            println!("ERROR: {}", DisplayErrorContext(&err));
            return Err(err.into())
        }
    }
}

// #[derive(Deserialize)]
// struct PostCheckinBody {
//     user_id: String,
//     location: String,
// }

fn deduplicate_by_user_id(orders: Vec<Checkin>) -> Vec<Checkin> {
    let mut latest_orders: HashMap<String, Checkin> = HashMap::new();

    for order in orders {
        if let Some(user_id) = extract_user_id(&order.pk) {
            // Parse the `created_at` timestamp
            let created_at = DateTime::parse_from_rfc3339(&order.time)
                .unwrap_or_else(|_| Utc::now().into()); // Handle parse errors gracefully

            // Check if the user already has an order recorded
            if let Some(existing_order) = latest_orders.get(&user_id) {
                let existing_created_at = DateTime::parse_from_rfc3339(&existing_order.time)
                    .unwrap_or_else(|_| Utc::now().into());

                // Keep the most recent order
                if created_at > existing_created_at {
                    latest_orders.insert(user_id.clone(), order);
                }
            } else {
                // Insert the first order for this user
                latest_orders.insert(user_id.clone(), order);
            }
        }
    }

    // Collect the latest orders into a Vec
    latest_orders.into_values().collect()
}

pub async fn get_all_checkins() -> Result<Vec<Checkin>> {
    let client = get_dynamodb_client().await?;
    let scan_output = client.scan()
        .table_name("BarBuddyTableTwo")  // Replace with your table name
        .filter_expression("begins_with(sk, :sk_prefix)")
        .expression_attribute_values(":sk_prefix", AttributeValue::S("CHECKIN#".to_string()))
        .send()
        .await?;

    let mut checkins: Vec<Checkin> = Vec::new();
    if let Some(items) = scan_output.items {
        // for item in items {
        //     serde_dynamo::
        //     match parse_order(item) {
        //         Ok(order) => orders.push(order),
        //         Err(err) => eprintln!("Error parsing order: {}", err),
        //     }
        // }
        checkins = serde_dynamo::from_items(items)?;
    }



    Ok(checkins)
}


#[derive(Deserialize, Serialize)]
struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

fn filter_out_good_orders(orders: Vec<Checkin>) -> Vec<Checkin> {
    orders
        .into_iter()
        .filter(|order| order.location.to_lowercase() != "home")
        .collect()
}

async fn get_foo_internal() -> Result<Json<Value>> {
    let mut checkins = get_all_checkins().await?;
    checkins = deduplicate_by_user_id(checkins);
    checkins = filter_out_good_orders(checkins);
    for checkin in &checkins {
        println!("{:#?}", checkin);
    }
    // let user_id = create_user(CreateUserRequest{name:"test name".to_string(), email:"email@email.com".to_string()}).await?;
    Ok(Json(json!({ "msg": format!("I am GET /foo PLUS the user_id is jk") })))
}

async fn get_foo() -> (StatusCode, Json<Value>) {
    result_to_response(get_foo_internal().await)
}

#[derive(Deserialize, Serialize)]
struct GetAllCheckinsEntry {
    user_id: String,
    location: String,
    time: String,
}

async fn get_all_checkins_route_internal() -> Result<Json<Value>> {
    let mut checkins = get_all_checkins().await?;
    checkins = deduplicate_by_user_id(checkins);
    checkins = filter_out_good_orders(checkins);
    let mut checkins_struct_vec: Vec<GetAllCheckinsEntry> = Vec::new();
    for checkin in &checkins {
        println!("{:#?}", checkin);
        if let Some(user_id) = extract_user_id(&checkin.pk) {
            checkins_struct_vec.push(GetAllCheckinsEntry{user_id: user_id, location: checkin.location.clone(), time: checkin.time.clone()});
        }
    }
    // let user_id = create_user(CreateUserRequest{name:"test name".to_string(), email:"email@email.com".to_string()}).await?;
    Ok(Json(json!({ "success": true, "checkins":checkins_struct_vec })))
}

async fn get_all_checkins_route() -> (StatusCode, Json<Value>) {
    result_to_response(get_all_checkins_route_internal().await)
}

async fn post_checkin_internal(body: CreateCheckinRequest) -> Result<Json<Value>> {
    let checkin_id = create_checkin(body).await?;
    Ok(Json(json!({"success":true,"checkin_id":checkin_id})))
}

async fn post_checkin(Json(body): Json<CreateCheckinRequest>) -> (StatusCode, Json<Value>) {
    result_to_response(post_checkin_internal(body).await)
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    // required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/checkin", post(post_checkin))
        .route("/getallcheckins", get(get_all_checkins_route))
        .route("/health/", get(health_check));

    run(app).await
}

fn result_to_response(result: Result<Json<Value>>) -> (StatusCode, Json<Value>) {
    match result {
        Ok(json) => (StatusCode::OK, json),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": true,
                "message": error.to_string()
            })),
        ),
    }
}
