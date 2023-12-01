use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub table_number: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemSchema {
    pub name: String,
    pub table_number: Option<String>,
    pub preparation_time_minutes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemSchema {
    pub name: Option<String>,
    pub table_number: Option<String>,
    pub preparation_time_minutes: Option<String>
}
