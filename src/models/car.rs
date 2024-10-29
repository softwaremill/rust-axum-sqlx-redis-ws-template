use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema)]
pub struct Car {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub year: Option<i16>,
}

pub type CarList = Vec<Car>;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewCar {
    pub name: String,
    pub color: Option<String>,
    pub year: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarQuery {
    pub name: Option<String>,
}