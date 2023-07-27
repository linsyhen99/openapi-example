use std::net::SocketAddr;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use tracing::info;
use crate::models::employee::{get_mock_employee_with_id, get_mock_list_of_employees};
use crate::openapi_generator::generate_openapi_json;

pub async fn serve_endpoint() -> () {
    let app = Router::new();

    let address = SocketAddr::from(([0, 0, 0, 0], 6969));
    info!("Server listening on {}", address);

    let _ = generate_openapi_json();
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/api/v1/employees", get(get_all_employees))
        .route("/api/v1/employee/:id", get(get_employee_with_id))
}

#[utoipa::path(
get,
path = "/api/v1/employees",
responses(
(status = 200, description = "Successfully retrieved employees", body = [Vec < Employee >]),
(status = 500, description = "Something went wrong")
)
)]
pub async fn get_all_employees() -> Response {
    let employees = get_mock_list_of_employees();

    (StatusCode::OK, Json(employees)).into_response()
}

#[utoipa::path(
get,
path = "/api/v1/employee/{id}",
responses(
(status = 200, description = "Successfully retrieved employees", body = [Employee]),
(status = 500, description = "Something went wrong")
),
params(
("id" = u64, Path, description = "The employee id")
)
)]
pub async fn get_employee_with_id(
    Path(employee_id): Path<u64>,
) -> Response {
    let employee = get_mock_employee_with_id(employee_id);

    (StatusCode::OK, Json(employee)).into_response()
}