use std::net::SocketAddr;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::{get, post};
use tracing::info;
use crate::models::employee::{EmployeeInputParam, get_mock_employee_with_id, get_mock_list_of_employees};
use crate::openapi_generator::generate_openapi_json;

pub async fn serve_endpoint() -> () {
    let routing = Router::new();
    let app = routing.merge(router());

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
        .route("/api/v1/employee", post(create_new_employee_record))
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
(status = 400, description = "Please double check your input parameters"),
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

#[utoipa::path(
post,
path = "/api/v1/employee",
request_body = EmployeeInputParam,
responses(
(status = 200, description = "Successfully created employees", body = String),
(status = 400, description = "Please double check your input parameters"),
(status = 500, description = "Something went wrong")
),
)]
pub async fn create_new_employee_record(
    Json(employee): Json<EmployeeInputParam>,
) -> Response {
    info!("Received employee: {:?}", employee);
    (StatusCode::OK, "Successfully created employee".to_string()).into_response()
}