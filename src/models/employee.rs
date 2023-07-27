use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct Employee {
    #[schema(example = "The unique ID that represents the employee")]
    id: u64,
    #[schema(example = "Bee Cheng")]
    name: String,
    #[schema(example = 24)]
    age: u16,
    #[schema(example = "Male, Female, UnidentifiedAnimal")]
    gender: GenderEnum,
    #[schema(example = "91929394")]
    phone_no: u32,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub enum GenderEnum {
    Male,
    Female,
    UnidentifiedAnimal,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct EmployeeInputParam {
    #[schema(example = "Tahj Mabo1s", required=true)]
    name: String,
    #[schema(example = 24, default = 10)]
    age: u16,
    #[schema(example = "Male, Female, UnidentifiedAnimal", default = "Male")]
    gender: GenderEnum,
    #[schema(example = "91929394", required=true)]
    phone_no: u32,
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
    let bee_c = Employee {
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