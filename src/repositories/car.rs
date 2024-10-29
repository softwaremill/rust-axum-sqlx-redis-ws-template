use crate::db::postgres::Db;
use crate::models::car::{NewCar, Car, CarQuery, CarList};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

pub struct CarRepositoryImpl {
    pool: Db,
}
impl CarRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait CarRepository {
    async fn find_all(&self, conditions: &CarQuery) -> Result<CarList>;
    async fn create(&self, car_data: &NewCar) -> Result<Car>;
    async fn update(&self, car_data: &Car) -> Result<Car>;
    async fn delete(&self, car_id: i32) -> Result<u64>;
    async fn find_by_id(&self, car_id: i32) -> Result<Car>;
}

#[async_trait]
impl CarRepository for CarRepositoryImpl {
    async fn find_all(&self, conditions: &CarQuery) -> Result<CarList> {
        let mut query = sqlx::query_as::<_, Car>("SELECT * FROM cars");
        if let Some(name) = &conditions.name {
            query = sqlx::query_as::<_, Car>("SELECT * FROM cars WHERE NAME LIKE $1")
                .bind(format!("%{}%", name))
        }
        let result = query
            .fetch_all(&*self.pool)
            .await?;
        Ok(result)
    }

    async fn create(&self, car_data: &NewCar) -> Result<Car> {
        let created_car = sqlx::query_as::<_, Car>(
            r#"
            INSERT INTO cars (name, color, year)
            VALUES ($1, $2, $3)
            RETURNING id, name, color, year
            "#,
        )
            .bind(&car_data.name)
            .bind(&car_data.color)
            .bind(car_data.year)
            .fetch_one(&*self.pool)
            .await?;
        Ok(created_car)
    }

    async fn update(&self, car_data: &Car) -> Result<Car> {
        let updated_car = sqlx::query_as::<_, Car>(
            r#"
            UPDATE cars
            SET name = $2, color = $3, year = $4
            WHERE id = $1
            RETURNING id, name, color, year
            "#,
        )
            .bind(car_data.id)
            .bind(&car_data.name)
            .bind(&car_data.color)
            .bind(car_data.year)
            .fetch_one(&*self.pool)
            .await?;
        Ok(updated_car)
    }

    async fn delete(&self, car_id: i32) -> Result<u64> {
        let query = sqlx::query("DELETE FROM cars WHERE id = $1")
            .bind(car_id)
            .execute(&*self.pool).await?;
        Ok(query.rows_affected())
    }

    async fn find_by_id(&self, car_id: i32) -> Result<Car> {
        let row = sqlx::query_as::<_, Car>("SELECT * FROM cars WHERE id = $1")
            .bind(car_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(row)
    }
}