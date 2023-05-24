use std::collections::HashMap;

use log::warn;

use crate::{database::{Database, InsertMenuResponse}, vm::{menu::MenuVM, menu_category::MenuCategoryVM, item::ItemVM}, dto::{item::ItemDTO, item_ingredient::ItemIngredientDTO}, error::DatabaseError, models::{menu::Menu, menu_category::MenuCategory,  item::Item, item_ingredient::ItemIngredient}};

 pub async fn get_menu_by_id(id: i32, db: Database) -> Result<MenuVM, DatabaseError> {
  
    let menu: Menu = match db.get_menu_by_id(id).await {
        Ok(Some(menu)) => menu, 
        Ok(None) => return Err(DatabaseError::NotFound(id.to_string())),
        Err(e) => {
            warn!("Error getting menu by id {:?}: {}", id, e);
            return Err(DatabaseError::InternalError);
        }
    };

    let categories = match db.get_categories_by_menu_id(id).await {
        Ok(categories) => categories,
        Err(e) => {
            warn!("Error getting categories by menu id {:?}: {}", id, e);
            return Err(DatabaseError::InternalError);
        }
    };

    let items = match db.get_items_by_category_ids(categories.iter().map(|c| c.id).collect()).await {
        Ok(items) => items,
        Err(e) => {
            warn!("Error getting items by category ids {}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    let ingredients = match db.get_ingredients_by_item_ids(items.iter().map(|i| i.id).collect()).await {
        Ok(ingredients) => ingredients,
        Err(e) => {
            warn!("Error getting ingredients by item ids {}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    Ok(build_menu_vm(menu, categories, items, ingredients))
}

pub async fn create_menu(menu_vm: MenuVM, db: Database) -> Result<MenuVM, DatabaseError> {
    let insert_menu_response: InsertMenuResponse = match db.insert_menu(menu_vm).await {
        Ok(insert_menu_response) => insert_menu_response,
        Err(e) => {
            warn!("Error inserting menu {}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    let items = match db.get_items_by_category_ids(insert_menu_response.categories.iter().map(|c| c.id).collect()).await {
        Ok(items) => items,
        Err(e) => {
            warn!("Error getting items by category ids {}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    let ingredients = match db.get_ingredients_by_item_ids(items.iter().map(|i| i.id).collect()).await {
        Ok(ingredients) => ingredients,
        Err(e) => {
            warn!("Error getting ingredients by item ids {}", e);
            return Err(DatabaseError::InternalError);
        }
    };

    Ok(build_menu_vm(insert_menu_response.menu, insert_menu_response.categories, items, ingredients))
}

fn build_menu_vm(menu: Menu, categories: Vec<MenuCategory>, items: Vec<Item>, ingredients: Vec<ItemIngredient>) -> MenuVM {
    let mut menu_vm = MenuVM::from(menu);
    let mut item_map: HashMap<i32, ItemDTO> = items.into_iter().map(|i|ItemDTO::from(i))
    .fold(HashMap::new(), |mut map, item| {
        map.insert(item.id.unwrap(), item);
        map
    });

    for ingredient in ingredients.into_iter() {
        if let Some(item) = item_map.get_mut(&ingredient.item_id) {
            item.ingredients.push(ItemIngredientDTO::from(ingredient));
        }
    }

    let mut category_map: HashMap<i32, MenuCategoryVM> = categories.into_iter().map(|c|MenuCategoryVM::from(c))
    .fold(HashMap::new(), |mut map, category| {
        map.insert(category.id.unwrap(), category);
        map
    });

    for item in item_map.into_iter().map(|(_, item)| item) {
        if let Some(category) = category_map.get_mut(&item.menu_category_id) {
            category.items.push(ItemVM::from(item));
        }
    }

    menu_vm.categories = category_map.into_iter().map(|(_, category)| category).collect();

    menu_vm
}