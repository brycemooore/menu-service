use actix_web::{web, get, post, HttpResponse, Error};
use log::info;
use crate::AppState;
use crate::service;
use crate::vm::item::ItemVM;
use crate::vm::menu::MenuVM;


#[get("/menu/{id}")]
async fn get_menu_by_id(path: web::Path<i32>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let db = state.as_ref().db.clone();
    let menu = service::menu::get_menu_by_id(id, db).await?;
    Ok(HttpResponse::Ok().json(menu))
}

#[post("/menu")]
async fn post_menu(menu: web::Json<MenuVM>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db = state.as_ref().db.clone();
    let menu_vm = menu.into_inner();
    let menu = service::menu::create_menu(menu_vm, db).await?;
    Ok(HttpResponse::Ok().json(menu))
}

#[post("/item")]
async fn post_item(item: web::Json<ItemVM>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db = state.as_ref().db.clone();
    let item_vm = item.into_inner();
    info!("Recieved request to create item: {:?}", item_vm);
    let item = service::item::create_item(item_vm, db).await?;
    Ok(HttpResponse::Ok().json(item))
}
