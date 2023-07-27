use utoipa::OpenApi;
use crate::models::employee::{Employee, GenderEnum, EmployeeInputParam};
use crate::controllers::endpoints::__path_get_all_employees;
use crate::controllers::endpoints::__path_get_employee_with_id;
use crate::controllers::endpoints::__path_create_new_employee_record;

#[derive(OpenApi)]
#[openapi(
components(schemas(Employee, GenderEnum, EmployeeInputParam)),
info(description = "This is a sample generated openapi documentation for reference"),
paths(get_all_employees, get_employee_with_id, create_new_employee_record)
)]
pub struct ApiDoc;

pub fn generate_openapi_json() -> () {
    std::fs::write("sample-openapi.json", ApiDoc::openapi().to_pretty_json().unwrap())
        .expect("Unable to create file");
}