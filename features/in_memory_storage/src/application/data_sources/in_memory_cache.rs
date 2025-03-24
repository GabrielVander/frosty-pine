use std::collections::HashMap;

use expense_tracking::domain::entities::Store;
use uuid_b64::UuidB64;

#[derive(Debug)]
struct InMemoryCache {
    stores: HashMap<UuidB64, StoreModel>,
    brands: HashMap<UuidB64, BrandModel>,
    categories: HashMap<UuidB64, CategoryModel>,
    products: HashMap<UuidB64, ProductModel>,
    items: HashMap<UuidB64, ItemModel>,
    transactions: HashMap<UuidB64, TransactionModel>,
}

impl InMemoryCache {
    fn new() -> Self {
        Self {
            stores: HashMap::new(),
            brands: HashMap::new(),
            categories: HashMap::new(),
            products: HashMap::new(),
            items: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn get_all_stores(&self) -> Vec<StoreModel> {
        self.stores.clone().into_values().collect()
    }

    pub fn get_all_brands(&self) -> Vec<BrandModel> {
        self.brands.clone().into_values().collect()
    }

    pub fn get_all_categories(&self) -> Vec<CategoryModel> {
        self.categories.clone().into_values().collect()
    }

    pub fn get_all_products(&self) -> Vec<ProductModel> {
        self.products.clone().into_values().collect()
    }

    pub fn get_all_items(&self) -> Vec<ItemModel> {
        self.items.clone().into_values().collect()
    }
    pub fn get_all_transactions(&self) -> Vec<TransactionModel> {
        self.transactions.clone().into_values().collect()
    }

    pub fn get_single_store(&self, key: &UuidB64) -> Option<StoreModel> {
        self.stores.get(&key).cloned()
    }

    pub fn get_single_brand(&self, key: &UuidB64) -> Option<BrandModel> {
        self.brands.get(&key).cloned()
    }

    pub fn get_single_category(&self, key: &UuidB64) -> Option<CategoryModel> {
        self.categories.get(&key).cloned()
    }

    pub fn get_single_product(&self, key: &UuidB64) -> Option<ProductModel> {
        self.products.get(&key).cloned()
    }

    pub fn get_single_item(&self, key: &UuidB64) -> Option<ItemModel> {
        self.items.get(&key).cloned()
    }

    pub fn get_single_transaction(&self, key: &UuidB64) -> Option<TransactionModel> {
        self.transactions.get(&key).cloned()
    }

    pub fn upsert_store(&mut self, store: StoreModel) -> Option<StoreModel> {
        self.stores.insert(store.key, store)
    }

    pub fn upsert_brand(&mut self, brand: BrandModel) -> Option<BrandModel> {
        self.brands.insert(brand.key, brand)
    }

    pub fn upsert_category(&mut self, category: CategoryModel) -> Option<CategoryModel> {
        self.categories.insert(category.key, category)
    }

    pub fn upsert_product(&mut self, product: ProductModel) -> Option<ProductModel> {
        self.products.insert(product.key, product)
    }

    pub fn upsert_item(&mut self, item: ItemModel) -> Option<ItemModel> {
        self.items.insert(item.key, item)
    }

    pub fn upsert_transaction(
        &mut self,
        transaction: TransactionModel,
    ) -> Option<TransactionModel> {
        self.transactions.insert(transaction.key, transaction)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StoreModel {
    key: UuidB64,
    name: String,
}

impl StoreModel {
    fn new(key: UuidB64, name: String) -> Self {
        Self { key, name }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BrandModel {
    key: UuidB64,
    name: String,
}

impl BrandModel {
    fn new(key: UuidB64, name: String) -> Self {
        Self { key, name }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CategoryModel {
    key: UuidB64,
    name: String,
}

impl CategoryModel {
    fn new(new: UuidB64, to_string: String) -> Self {
        Self {
            key: new,
            name: to_string,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ProductModel {
    key: UuidB64,
    name: String,
    brand_key: UuidB64,
    category_key: UuidB64,
}

impl ProductModel {
    fn new(key: UuidB64, name: String, brand_key: UuidB64, category_key: UuidB64) -> Self {
        Self {
            key,
            name,
            brand_key,
            category_key,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemModel {
    key: UuidB64,
    product_key: UuidB64,
    unit: UnitModel,
    unitary_price: f64,
}

impl ItemModel {
    fn new(key: UuidB64, product_key: UuidB64, unit: UnitModel, unitary_price: f64) -> Self {
        Self {
            key,
            product_key,
            unit,
            unitary_price,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum UnitModel {
    None,
    Quantity(f64),
    Kilograms(f64),
    Liters(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct TransactionModel {
    key: UuidB64,
    item_keys: Vec<UuidB64>,
    store_key: UuidB64,
    datetime: String,
}

impl TransactionModel {
    fn new(key: UuidB64, item_keys: Vec<UuidB64>, store_key: UuidB64, datetime: String) -> Self {
        Self {
            key,
            item_keys,
            store_key,
            datetime,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use expense_tracking::domain::entities::Store;
    use uuid_b64::UuidB64;

    use crate::application::data_sources::in_memory_cache::{
        BrandModel, CategoryModel, ItemModel, ProductModel, TransactionModel, UnitModel,
    };

    use super::{InMemoryCache, StoreModel};

    #[test]
    fn new_cache_no_data() {
        let cache: InMemoryCache = InMemoryCache::new();

        assert!(cache.get_all_stores().is_empty());
        assert!(cache.get_all_brands().is_empty());
        assert!(cache.get_all_categories().is_empty());
        assert!(cache.get_all_products().is_empty());
    }

    #[test]
    fn no_stores_upsert_new_store_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let store_model: StoreModel = StoreModel::new(UuidB64::new(), "Some Store".to_string());

        assert_eq!(cache.upsert_store(store_model.clone()), None);

        let all_stores: Vec<StoreModel> = cache.get_all_stores();
        assert_eq!(all_stores.len(), 1);
        assert!(all_stores.contains(&store_model));
    }

    #[test]
    fn upsert_existing_store_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let existing_store: StoreModel = StoreModel::new(key, "Some Store".to_string());
        let updated_store: StoreModel = StoreModel::new(key, "Some updated Store".to_string());

        assert_eq!(cache.upsert_store(existing_store.clone()), None);
        assert_eq!(
            cache.upsert_store(updated_store.clone()),
            Some(existing_store.clone())
        );

        let all_stores: Vec<StoreModel> = cache.get_all_stores();
        assert_eq!(all_stores.len(), 1);
        assert!(all_stores.contains(&updated_store));
    }

    #[test]
    fn get_all_existing_stores() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let store_1: StoreModel = StoreModel::new(UuidB64::new(), "Some Store".to_string());
        let store_2: StoreModel = StoreModel::new(UuidB64::new(), "Some Other Store".to_string());
        let store_3: StoreModel = StoreModel::new(UuidB64::new(), "Yet Another Store".to_string());

        assert!(cache.get_all_stores().is_empty());

        assert_eq!(cache.upsert_store(store_1.clone()), None);
        assert_eq!(cache.upsert_store(store_2.clone()), None);
        assert_eq!(cache.upsert_store(store_3.clone()), None);

        let stores: Vec<StoreModel> = cache.get_all_stores();
        assert_eq!(stores.len(), 3);
        assert!(stores.contains(&store_1));
        assert!(stores.contains(&store_2));
        assert!(stores.contains(&store_3));
    }

    #[test]
    fn get_non_existing_store_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let store_1: StoreModel = StoreModel::new(UuidB64::new(), "Some Store".to_string());
        let store_2: StoreModel = StoreModel::new(UuidB64::new(), "Some Other Store".to_string());
        let store_3: StoreModel = StoreModel::new(UuidB64::new(), "Yet Another Store".to_string());

        assert_eq!(cache.upsert_store(store_1.clone()), None);
        assert_eq!(cache.upsert_store(store_2.clone()), None);
        assert_eq!(cache.upsert_store(store_3.clone()), None);

        let store: Option<StoreModel> = cache.get_single_store(&UuidB64::new());
        assert_eq!(store, None);
    }

    #[test]
    fn get_existing_store_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let store_1: StoreModel = StoreModel::new(target_key, "Some Store".to_string());
        let store_2: StoreModel = StoreModel::new(UuidB64::new(), "Some Other Store".to_string());
        let store_3: StoreModel = StoreModel::new(UuidB64::new(), "Yet Another Store".to_string());

        assert_eq!(cache.upsert_store(store_1.clone()), None);
        assert_eq!(cache.upsert_store(store_2.clone()), None);
        assert_eq!(cache.upsert_store(store_3.clone()), None);

        let store: Option<StoreModel> = cache.get_single_store(&target_key);
        assert_eq!(store, Some(store_1));
    }

    #[test]
    fn no_brands_upsert_new_brand_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let brand_model: BrandModel = BrandModel::new(UuidB64::new(), "Some Brand".to_string());

        assert_eq!(cache.upsert_brand(brand_model.clone()), None);

        let all_brands: Vec<BrandModel> = cache.get_all_brands();
        assert_eq!(all_brands.len(), 1);
        assert!(all_brands.contains(&brand_model));
    }

    #[test]
    fn upsert_existing_brand_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let existing_brand: BrandModel = BrandModel::new(key, "Some Brand".to_string());
        let updated_brand: BrandModel = BrandModel::new(key, "Some updated Brand".to_string());

        assert_eq!(cache.upsert_brand(existing_brand.clone()), None);
        assert_eq!(
            cache.upsert_brand(updated_brand.clone()),
            Some(existing_brand.clone())
        );

        let all_brands: Vec<BrandModel> = cache.get_all_brands();
        assert_eq!(all_brands.len(), 1);
        assert!(all_brands.contains(&updated_brand));
    }

    #[test]
    fn get_all_existing_brands() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let brand_1: BrandModel = BrandModel::new(UuidB64::new(), "Some Brand".to_string());
        let brand_2: BrandModel = BrandModel::new(UuidB64::new(), "Some Other Brand".to_string());
        let brand_3: BrandModel = BrandModel::new(UuidB64::new(), "Yet Another Brand".to_string());

        assert!(cache.get_all_brands().is_empty());

        assert_eq!(cache.upsert_brand(brand_1.clone()), None);
        assert_eq!(cache.upsert_brand(brand_2.clone()), None);
        assert_eq!(cache.upsert_brand(brand_3.clone()), None);

        let brands: Vec<BrandModel> = cache.get_all_brands();
        assert_eq!(brands.len(), 3);
        assert!(brands.contains(&brand_1));
        assert!(brands.contains(&brand_2));
        assert!(brands.contains(&brand_3));
    }

    #[test]
    fn get_non_existing_brand_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let brand_1: BrandModel = BrandModel::new(UuidB64::new(), "Some Brand".to_string());
        let brand_2: BrandModel = BrandModel::new(UuidB64::new(), "Some Other Brand".to_string());
        let brand_3: BrandModel = BrandModel::new(UuidB64::new(), "Yet Another Brand".to_string());

        assert_eq!(cache.upsert_brand(brand_1.clone()), None);
        assert_eq!(cache.upsert_brand(brand_2.clone()), None);
        assert_eq!(cache.upsert_brand(brand_3.clone()), None);

        let brand: Option<BrandModel> = cache.get_single_brand(&UuidB64::new());
        assert_eq!(brand, None);
    }

    #[test]
    fn get_existing_brand_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let brand_1: BrandModel = BrandModel::new(target_key, "Some Brand".to_string());
        let brand_2: BrandModel = BrandModel::new(UuidB64::new(), "Some Other Brand".to_string());
        let brand_3: BrandModel = BrandModel::new(UuidB64::new(), "Yet Another Brand".to_string());

        assert_eq!(cache.upsert_brand(brand_1.clone()), None);
        assert_eq!(cache.upsert_brand(brand_2.clone()), None);
        assert_eq!(cache.upsert_brand(brand_3.clone()), None);

        let brand: Option<BrandModel> = cache.get_single_brand(&target_key);
        assert_eq!(brand, Some(brand_1));
    }

    #[test]
    fn no_categories_upsert_new_category_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let category_model: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Category".to_string());

        assert_eq!(cache.upsert_category(category_model.clone()), None);

        let all_categories: Vec<CategoryModel> = cache.get_all_categories();
        assert_eq!(all_categories.len(), 1);
        assert!(all_categories.contains(&category_model));
    }

    #[test]
    fn upsert_existing_category_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let existing_category: CategoryModel = CategoryModel::new(key, "Some Category".to_string());
        let updated_category: CategoryModel =
            CategoryModel::new(key, "Some updated Category".to_string());

        assert_eq!(cache.upsert_category(existing_category.clone()), None);
        assert_eq!(
            cache.upsert_category(updated_category.clone()),
            Some(existing_category.clone())
        );

        let all_categories: Vec<CategoryModel> = cache.get_all_categories();
        assert_eq!(all_categories.len(), 1);
        assert!(all_categories.contains(&updated_category));
    }

    #[test]
    fn get_all_existing_categories() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let category_1: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Category".to_string());
        let category_2: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Other Category".to_string());
        let category_3: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Yet Another Category".to_string());

        assert!(cache.get_all_categories().is_empty());

        assert_eq!(cache.upsert_category(category_1.clone()), None);
        assert_eq!(cache.upsert_category(category_2.clone()), None);
        assert_eq!(cache.upsert_category(category_3.clone()), None);

        let categories: Vec<CategoryModel> = cache.get_all_categories();
        assert_eq!(categories.len(), 3);
        assert!(categories.contains(&category_1));
        assert!(categories.contains(&category_2));
        assert!(categories.contains(&category_3));
    }

    #[test]
    fn get_non_existing_category_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let category_1: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Category".to_string());
        let category_2: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Other Category".to_string());
        let category_3: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Yet Another Category".to_string());

        assert_eq!(cache.upsert_category(category_1.clone()), None);
        assert_eq!(cache.upsert_category(category_2.clone()), None);
        assert_eq!(cache.upsert_category(category_3.clone()), None);

        let category: Option<CategoryModel> = cache.get_single_category(&UuidB64::new());
        assert_eq!(category, None);
    }

    #[test]
    fn get_existing_category_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let category_1: CategoryModel = CategoryModel::new(target_key, "Some Category".to_string());
        let category_2: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Some Other Category".to_string());
        let category_3: CategoryModel =
            CategoryModel::new(UuidB64::new(), "Yet Another Category".to_string());

        assert_eq!(cache.upsert_category(category_1.clone()), None);
        assert_eq!(cache.upsert_category(category_2.clone()), None);
        assert_eq!(cache.upsert_category(category_3.clone()), None);

        let category: Option<CategoryModel> = cache.get_single_category(&target_key);
        assert_eq!(category, Some(category_1));
    }

    #[test]
    fn no_products_upsert_new_product_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let product_model: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );

        assert_eq!(cache.upsert_product(product_model.clone()), None);

        let all_products: Vec<ProductModel> = cache.get_all_products();
        assert_eq!(all_products.len(), 1);
        assert!(all_products.contains(&product_model));
    }

    #[test]
    fn upsert_existing_product_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let brand_key: UuidB64 = UuidB64::new();
        let category_key: UuidB64 = UuidB64::new();

        let existing_product: ProductModel =
            ProductModel::new(key, "Some Product".to_string(), brand_key, category_key);
        let updated_product: ProductModel = ProductModel::new(
            key,
            "Some updated Product".to_string(),
            brand_key,
            category_key,
        );

        assert_eq!(cache.upsert_product(existing_product.clone()), None);
        assert_eq!(
            cache.upsert_product(updated_product.clone()),
            Some(existing_product.clone())
        );

        let all_products: Vec<ProductModel> = cache.get_all_products();
        assert_eq!(all_products.len(), 1);
        assert!(all_products.contains(&updated_product));
    }

    #[test]
    fn get_all_existing_products() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let product_1: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_2: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Other Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_3: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Yet Another Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );

        assert!(cache.get_all_products().is_empty());

        assert_eq!(cache.upsert_product(product_1.clone()), None);
        assert_eq!(cache.upsert_product(product_2.clone()), None);
        assert_eq!(cache.upsert_product(product_3.clone()), None);

        let products: Vec<ProductModel> = cache.get_all_products();
        assert_eq!(products.len(), 3);
        assert!(products.contains(&product_1));
        assert!(products.contains(&product_2));
        assert!(products.contains(&product_3));
    }

    #[test]
    fn get_non_existing_product_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let product_1: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_2: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Other Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_3: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Yet Another Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );

        assert_eq!(cache.upsert_product(product_1.clone()), None);
        assert_eq!(cache.upsert_product(product_2.clone()), None);
        assert_eq!(cache.upsert_product(product_3.clone()), None);

        let product: Option<ProductModel> = cache.get_single_product(&UuidB64::new());
        assert_eq!(product, None);
    }

    #[test]
    fn get_existing_product_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let product_1: ProductModel = ProductModel::new(
            target_key,
            "Some Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_2: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Some Other Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );
        let product_3: ProductModel = ProductModel::new(
            UuidB64::new(),
            "Yet Another Product".to_string(),
            UuidB64::new(),
            UuidB64::new(),
        );

        assert_eq!(cache.upsert_product(product_1.clone()), None);
        assert_eq!(cache.upsert_product(product_2.clone()), None);
        assert_eq!(cache.upsert_product(product_3.clone()), None);

        let product: Option<ProductModel> = cache.get_single_product(&target_key);
        assert_eq!(product, Some(product_1));
    }

    #[test]
    fn no_items_upsert_new_item_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let item_model: ItemModel =
            ItemModel::new(UuidB64::new(), UuidB64::new(), UnitModel::None, 85.04);

        assert_eq!(cache.upsert_item(item_model.clone()), None);

        let all_items: Vec<ItemModel> = cache.get_all_items();
        assert_eq!(all_items.len(), 1);
        assert!(all_items.contains(&item_model));
    }

    #[test]
    fn upsert_existing_item_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let existing_item: ItemModel = ItemModel::new(key, UuidB64::new(), UnitModel::None, 80.36);
        let updated_item: ItemModel =
            ItemModel::new(key, UuidB64::new(), UnitModel::Liters(25.38), 33.03);

        assert_eq!(cache.upsert_item(existing_item.clone()), None);
        assert_eq!(
            cache.upsert_item(updated_item.clone()),
            Some(existing_item.clone())
        );

        let all_items: Vec<ItemModel> = cache.get_all_items();
        assert_eq!(all_items.len(), 1);
        assert!(all_items.contains(&updated_item));
    }

    #[test]
    fn get_all_existing_items() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let item_1: ItemModel = ItemModel::new(
            UuidB64::new(),
            UuidB64::new(),
            UnitModel::Liters(84.53),
            26.68,
        );
        let item_2: ItemModel = ItemModel::new(
            UuidB64::new(),
            UuidB64::new(),
            UnitModel::Quantity(81.0),
            29.65,
        );
        let item_3: ItemModel =
            ItemModel::new(UuidB64::new(), UuidB64::new(), UnitModel::None, 40.56);

        assert!(cache.get_all_items().is_empty());

        assert_eq!(cache.upsert_item(item_1.clone()), None);
        assert_eq!(cache.upsert_item(item_2.clone()), None);
        assert_eq!(cache.upsert_item(item_3.clone()), None);

        let items: Vec<ItemModel> = cache.get_all_items();
        assert_eq!(items.len(), 3);
        assert!(items.contains(&item_1));
        assert!(items.contains(&item_2));
        assert!(items.contains(&item_3));
    }

    #[test]
    fn get_non_existing_item_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let item_1: ItemModel =
            ItemModel::new(UuidB64::new(), UuidB64::new(), UnitModel::None, 39.89);
        let item_2: ItemModel = ItemModel::new(
            UuidB64::new(),
            UuidB64::new(),
            UnitModel::Kilograms(31.32),
            59.95,
        );
        let item_3: ItemModel =
            ItemModel::new(UuidB64::new(), UuidB64::new(), UnitModel::None, 98.64);

        assert_eq!(cache.upsert_item(item_1.clone()), None);
        assert_eq!(cache.upsert_item(item_2.clone()), None);
        assert_eq!(cache.upsert_item(item_3.clone()), None);

        let item: Option<ItemModel> = cache.get_single_item(&UuidB64::new());
        assert_eq!(item, None);
    }

    #[test]
    fn get_existing_item_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let item_1: ItemModel =
            ItemModel::new(target_key, UuidB64::new(), UnitModel::Liters(73.77), 11.60);
        let item_2: ItemModel =
            ItemModel::new(UuidB64::new(), UuidB64::new(), UnitModel::None, 94.96);
        let item_3: ItemModel = ItemModel::new(
            UuidB64::new(),
            UuidB64::new(),
            UnitModel::Quantity(73.0),
            91.64,
        );

        assert_eq!(cache.upsert_item(item_1.clone()), None);
        assert_eq!(cache.upsert_item(item_2.clone()), None);
        assert_eq!(cache.upsert_item(item_3.clone()), None);

        let item: Option<ItemModel> = cache.get_single_item(&target_key);
        assert_eq!(item, Some(item_1));
    }

    #[test]
    fn no_transactions_upsert_new_transaction_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let transaction_model: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new()],
            UuidB64::new(),
            "2023-08-18T11:16:39Z".to_string(),
        );

        assert_eq!(cache.upsert_transaction(transaction_model.clone()), None);

        let all_transactions: Vec<TransactionModel> = cache.get_all_transactions();
        assert_eq!(all_transactions.len(), 1);
        assert!(all_transactions.contains(&transaction_model));
    }

    #[test]
    fn upsert_existing_transaction_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: UuidB64 = UuidB64::new();
        let store_key: UuidB64 = UuidB64::new();
        let existing_transaction: TransactionModel =
            TransactionModel::new(key, vec![], store_key, "2021-10-26T03:30:48Z".to_string());
        let updated_transaction: TransactionModel =
            TransactionModel::new(key, vec![], store_key, "2021-05-25T21:10:43Z".to_string());

        assert_eq!(cache.upsert_transaction(existing_transaction.clone()), None);
        assert_eq!(
            cache.upsert_transaction(updated_transaction.clone()),
            Some(existing_transaction.clone())
        );

        let all_transactions: Vec<TransactionModel> = cache.get_all_transactions();
        assert_eq!(all_transactions.len(), 1);
        assert!(all_transactions.contains(&updated_transaction));
    }

    #[test]
    fn get_all_existing_transactions() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let transaction_1: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![],
            UuidB64::new(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new()],
            UuidB64::new(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new(), UuidB64::new(), UuidB64::new()],
            UuidB64::new(),
            "2020-11-18T02:08:09Z".to_string(),
        );

        assert!(cache.get_all_transactions().is_empty());

        assert_eq!(cache.upsert_transaction(transaction_1.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_2.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_3.clone()), None);

        let transactions: Vec<TransactionModel> = cache.get_all_transactions();
        assert_eq!(transactions.len(), 3);
        assert!(transactions.contains(&transaction_1));
        assert!(transactions.contains(&transaction_2));
        assert!(transactions.contains(&transaction_3));
    }

    #[test]
    fn get_non_existing_transaction_return_none() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let transaction_1: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![],
            UuidB64::new(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new()],
            UuidB64::new(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new(), UuidB64::new(), UuidB64::new()],
            UuidB64::new(),
            "2020-11-18T02:08:09Z".to_string(),
        );

        assert_eq!(cache.upsert_transaction(transaction_1.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_2.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_3.clone()), None);

        let transaction: Option<TransactionModel> = cache.get_single_transaction(&UuidB64::new());
        assert_eq!(transaction, None);
    }

    #[test]
    fn get_existing_transaction_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: UuidB64 = UuidB64::new();
        let transaction_1: TransactionModel = TransactionModel::new(
            target_key,
            vec![],
            UuidB64::new(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new()],
            UuidB64::new(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            UuidB64::new(),
            vec![UuidB64::new(), UuidB64::new(), UuidB64::new()],
            UuidB64::new(),
            "2020-11-18T02:08:09Z".to_string(),
        );

        assert_eq!(cache.upsert_transaction(transaction_1.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_2.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_3.clone()), None);

        let transaction: Option<TransactionModel> = cache.get_single_transaction(&target_key);
        assert_eq!(transaction, Some(transaction_1));
    }
}
