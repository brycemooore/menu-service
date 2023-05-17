use log::warn;

use crate::{database::Database, vm::{item::ItemVM, item_ingredient::ItemIngredientVM}, models::{item_ingredient::ItemIngredient, item::NewItem}, error::DatabaseError};

pub async fn create_item(item_vm: ItemVM, db: Database) -> Result<ItemVM, DatabaseError> {
    let insert_item_response = match db.insert_item(item_vm).await {
        Ok(insert_item_response) => insert_item_response,
        Err(e) => {
            warn!("Error inserting item: {:?}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    Ok(build_item_vm(insert_item_response.item, insert_item_response.ingredients))
}

pub fn build_item_vm(item: NewItem, ingredients: Vec<ItemIngredient>) -> ItemVM {
    let mut item_vm = ItemVM::from(item);
    item_vm.ingredients = ingredients.into_iter().map(|i| ItemIngredientVM::from(i)).collect();
    item_vm
}