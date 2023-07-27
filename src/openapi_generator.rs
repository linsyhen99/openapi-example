use utoipa::OpenApi;
use crate::models::employee::Employee;
use crate::endpoints::__path_get_all_employees;
use crate::endpoints::__path_get_employee_with_id;

#[derive(OpenApi)]
#[openapi(components(schemas(Employee)), paths(get_all_employees, get_employee_with_id))]
pub struct ApiDoc;

pub fn generate_openapi_json() -> () {
    std::fs::write("sample-openapi.json", ApiDoc::openapi().to_pretty_json().unwrap())
        .expect("Unable to create file");
}