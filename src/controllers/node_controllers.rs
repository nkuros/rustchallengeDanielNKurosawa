
use actix_web::web::Data;


use diesel::result::Error;
use diesel::RunQueryDsl;

use crate::DBPool;
use crate::models::node::{NodeToDB, NodeFromDB, NodeDB};
use crate::constants::constants::API_ADDRESS;
use crate::errors::errors::ERROR_CONNECTION_POOL;
use crate::database::schema::nodes::table;


pub fn get_notes_from_db(pool: &Data<DBPool>) -> Result<Vec<NodeFromDB>, Error> {
    use crate::database::schema::nodes::dsl::nodes;
    let conn = &mut pool.get().expect(ERROR_CONNECTION_POOL);
    let nodes_from_db = match nodes.load::<NodeDB>(conn){
        Ok(nodedb) => nodedb,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e);
        }
    };
    let ans = nodes_from_db.iter().map(|node| node.from_db()).collect();
    Ok(ans)
}


pub async fn get_nodes_from_api() -> Result<Vec<NodeToDB>, reqwest::Error> {
    let response = match  reqwest::get(API_ADDRESS).await{
        Ok(response) => response,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e);
        }
    };
        
    let nodes: Vec<NodeToDB> = match response.json().await {
        Ok(nodes) => nodes,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e);
        }
    };
    
    Ok(nodes)
    
}

pub fn create_nodes(nodes: Vec<NodeToDB>, pool: &Data<DBPool>) -> Result<() ,Error> {
    let conn = &mut pool.get().expect(ERROR_CONNECTION_POOL);
    for node in nodes {
        let node_db = node.to_node_db();
        let _ = diesel::insert_into(table).values(&node_db).execute(conn);

    }

    Ok(())
}