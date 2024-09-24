use actix_web::web::Data;
use actix_web::{web, HttpResponse};


use crate::DBPool;
use crate::models::node::{NodeToDB, NodeFromDB};
use crate::constants::constants::CONTENT_TYPE_JSON;
use crate::controllers::node_controllers::{get_notes_from_db, get_nodes_from_api, create_nodes};


#[get("/")]
pub async fn populate(pool: Data<DBPool>) -> HttpResponse {

    let nodes: Vec<NodeToDB> = match get_nodes_from_api().await {
        Ok(nodes) => nodes,
        Err(e) => {
            println!("Error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let nodes_clone = nodes.clone();
    
    match  web::block(move || create_nodes(nodes_clone, &pool)).await {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }

    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_JSON)
        .json(nodes)
}


// list nodes
#[get("/nodes")]
pub async fn get_nodes(pool: Data<DBPool>) -> HttpResponse {

    let nodes: Vec<NodeFromDB> = match get_notes_from_db(&pool) {
            Ok(nodes) => nodes,
            Err(e) => {
                println!("Error: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };
    
    HttpResponse::Ok()
    .content_type(CONTENT_TYPE_JSON)
    .json(nodes)
}

