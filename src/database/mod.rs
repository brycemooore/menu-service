use sqlx::{Postgres, Pool, query_as};

use crate::{models::{menu::Menu, menu_category::MenuCategory, item::{Item, NewItem}, item_ingredient::ItemIngredient, menu_category_item::MenuCategoryItem}, vm::{menu::MenuVM, item::ItemVM}};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Postgres>
}

pub struct InsertMenuResponse {
    pub menu: Menu,
    pub categories: Vec<MenuCategory>,
}

pub struct InsertItemResponse {
    pub item: NewItem,
    pub ingredients: Vec<ItemIngredient>,
}

impl Database {

    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool
        }
    }

    pub async fn get_menu_by_id(&self, id: i32) -> Result<Option<Menu>, sqlx::Error> {
        query_as!(Menu, "SELECT * FROM menu WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_categories_by_menu_id(&self, menu_id: i32) -> Result<Vec<MenuCategory>, sqlx::Error> {
        query_as!(MenuCategory, "SELECT * FROM menu_category WHERE menu_id = $1", menu_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_items_by_category_ids(&self, ids: Vec<i32>) -> Result<Vec<Item>, sqlx::Error> {
        query_as!(
            Item, 
            "SELECT i.id, i.name, i.description, i.available, mci.menu_category_id, i.created_at, i.updated_at 
            FROM item AS i 
            JOIN menu_category_item AS mci 
            ON mci.item_id = i.id 
            WHERE mci.menu_category_id = ANY($1)", 
            &ids
        )
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_ingredients_by_item_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemIngredient>, sqlx::Error> {
        query_as!(ItemIngredient, "SELECT * FROM item_ingredient WHERE item_id = ANY($1)", &ids)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn insert_menu(&self, menu_vm: MenuVM) -> Result<InsertMenuResponse, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        let menu = query_as!(
            Menu, 
            "INSERT INTO menu (name, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *", 
            menu_vm.name, menu_vm.is_active, chrono::Utc::now().naive_utc(), chrono::Utc::now().naive_utc()
        )
            .fetch_one(&mut tx)
            .await?;

        let mut categories = Vec::new();
        for c in menu_vm.categories.into_iter() {

            //insert category
            let category = match query_as!(
                MenuCategory, 
                "INSERT INTO menu_category (name, menu_id, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *", 
                c.name, 
                menu.id, 
                chrono::Utc::now().naive_utc(), 
                chrono::Utc::now().naive_utc()
            ).fetch_one(&mut tx).await {
                Ok(c) => c,
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            };
            let category_id: i32 = category.id;
            categories.push(category);

            //insert associations for items
            for i in c.items.into_iter() {
                match query_as!(
                    MenuCategoryItem, 
                    "INSERT INTO menu_category_item (menu_category_id, item_id) VALUES ($1, $2) RETURNING *", 
                    category_id, 
                    i.id
                ).fetch_one(&mut tx).await {
                    Ok(_) => (),
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            }
        }
        tx.commit().await?;
        Ok(InsertMenuResponse {menu, categories})
    }

    pub async fn insert_item(&self, item_vm: ItemVM) -> Result<InsertItemResponse, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let item = match query_as!(
            NewItem, 
            "INSERT INTO item (name, description, available, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *", 
            item_vm.name, 
            item_vm.description, 
            item_vm.available, 
            chrono::Utc::now().naive_utc(), 
            chrono::Utc::now().naive_utc()
        )
            .fetch_one(&mut tx)
            .await {
                Ok(i) => i,
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            };
        
        let mut ingredients = Vec::new();

        for i in item_vm.ingredients.into_iter() {
            let ingredient = match query_as!(
                ItemIngredient, 
                "INSERT INTO item_ingredient (name, available, item_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *", 
                i.name,
                i.available,
                item.id, 
                chrono::Utc::now().naive_utc(), 
                chrono::Utc::now().naive_utc()
            )
                .fetch_one(&mut tx)
                .await {
                    Ok(i) => i,
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                };
            ingredients.push(ingredient);
        }
        tx.commit().await?;
        Ok(InsertItemResponse {item, ingredients})
    }
}