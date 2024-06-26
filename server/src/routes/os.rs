use actix_web::{get, web, Responder, Result};
use crate::models::os_info::OsInfo;

#[get("/os")]
pub async fn return_os_info() -> Result<impl Responder>  {
    let os: OsInfo = OsInfo::new();
    Ok(web::Json(os))
}