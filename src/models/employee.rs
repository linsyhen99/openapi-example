use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
pub struct Employee {
    id: u64,
    name: String,
    age: u16,
    gender: GenderEnum,
    phone_no: u32,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub enum GenderEnum {
    Male,
    Female,
    UnidentifiedAnimal,
}

pub fn get_mock_employee_with_id(emp_id: u64) -> Employee {
    Employee {
        id: emp_id,
        name: "Tahj Mabols".to_string(),
        age: 24,
        gender: GenderEnum::UnidentifiedAnimal,
        phone_no: 12345678,
    }
}

pub fn get_mock_list_of_employees() -> Vec<Employee> {
    let bee_c= Employee {
        id: 0,
        name: "Bee C".to_string(),
        age: 24,
        gender: GenderEnum::UnidentifiedAnimal,
        phone_no: 12345678,
    };
    let corgeweloper = Employee {
        id: 1,
        gender: GenderEnum::Male,
        name: "Corgeweloper".to_string(),
        age: 39,
        phone_no: 12900782,
    };
    let dew = Employee {
        id: 2,
        name: "Dew Analytics".to_string(),
        age: 42,
        gender: GenderEnum::Female,
        phone_no: 31516782,
    };

    vec![bee_c, corgeweloper, dew]
}