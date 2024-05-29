use actix_web::{get, web, Responder, Result};

use crate::models::memory_info::MemoryInfo;

#[get("/memory")]
pub async fn return_memory_info() -> Result<impl Responder> {
    let memory: MemoryInfo = MemoryInfo::new();
    Ok(web::Json(memory))
}