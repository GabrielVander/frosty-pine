use std::collections::HashMap;

use expense_tracking::domain::entities::Store;

#[derive(Debug)]
struct InMemoryCache {
    stores: HashMap<String, StoreModel>,
    brands: HashMap<String, BrandModel>,
    categories: HashMap<String, CategoryModel>,
    products: HashMap<String, ProductModel>,
    items: HashMap<String, ItemModel>,
    transactions: HashMap<String, TransactionModel>,
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

    pub fn get_single_store(&self, key: &String) -> Option<StoreModel> {
        self.stores.get(key).cloned()
    }

    pub fn get_single_brand(&self, key: &String) -> Option<BrandModel> {
        self.brands.get(key).cloned()
    }

    pub fn get_single_category(&self, key: &String) -> Option<CategoryModel> {
        self.categories.get(key).cloned()
    }

    pub fn get_single_product(&self, key: &String) -> Option<ProductModel> {
        self.products.get(key).cloned()
    }

    pub fn get_single_item(&self, key: &String) -> Option<ItemModel> {
        self.items.get(key).cloned()
    }

    pub fn get_single_transaction(&self, key: &String) -> Option<TransactionModel> {
        self.transactions.get(key).cloned()
    }

    pub fn upsert_store(&mut self, store: StoreModel) -> Option<StoreModel> {
        self.stores.insert(store.key.clone(), store)
    }

    pub fn upsert_brand(&mut self, brand: BrandModel) -> Option<BrandModel> {
        self.brands.insert(brand.key.clone(), brand)
    }

    pub fn upsert_category(&mut self, category: CategoryModel) -> Option<CategoryModel> {
        self.categories.insert(category.key.clone(), category)
    }

    pub fn upsert_product(&mut self, product: ProductModel) -> Option<ProductModel> {
        self.products.insert(product.key.clone(), product)
    }

    pub fn upsert_item(&mut self, item: ItemModel) -> Option<ItemModel> {
        self.items.insert(item.key.clone(), item)
    }

    pub fn upsert_transaction(
        &mut self,
        transaction: TransactionModel,
    ) -> Option<TransactionModel> {
        self.transactions
            .insert(transaction.key.clone(), transaction)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StoreModel {
    key: String,
    name: String,
}

impl StoreModel {
    fn new(key: String, name: String) -> Self {
        Self { key, name }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BrandModel {
    key: String,
    name: String,
}

impl BrandModel {
    fn new(key: String, name: String) -> Self {
        Self { key, name }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CategoryModel {
    key: String,
    name: String,
}

impl CategoryModel {
    fn new(new: String, to_string: String) -> Self {
        Self {
            key: new,
            name: to_string,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ProductModel {
    key: String,
    name: String,
    brand_key: String,
    category_key: String,
}

impl ProductModel {
    fn new(key: String, name: String, brand_key: String, category_key: String) -> Self {
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
    key: String,
    product_key: String,
    unit: UnitModel,
    unitary_price: f64,
}

impl ItemModel {
    fn new(key: String, product_key: String, unit: UnitModel, unitary_price: f64) -> Self {
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
    key: String,
    item_keys: Vec<String>,
    store_key: String,
    datetime: String,
}

impl TransactionModel {
    fn new(key: String, item_keys: Vec<String>, store_key: String, datetime: String) -> Self {
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
        let store_model: StoreModel = StoreModel::new(
            "8C2F2504-68B8-4B63-8325-83A9D3FCED52".to_string(),
            "Some Store".to_string(),
        );

        assert_eq!(cache.upsert_store(store_model.clone()), None);

        let all_stores: Vec<StoreModel> = cache.get_all_stores();
        assert_eq!(all_stores.len(), 1);
        assert!(all_stores.contains(&store_model));
    }

    #[test]
    fn upsert_existing_store_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: String = "D2DEDED4-A7EC-444B-9B25-4144DECC0F4C".to_string();
        let existing_store: StoreModel = StoreModel::new(key.clone(), "Some Store".to_string());
        let updated_store: StoreModel =
            StoreModel::new(key.clone(), "Some updated Store".to_string());

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

        let store_1: StoreModel = StoreModel::new(
            "97D688ED-4248-488E-96EE-BD5FE601289A".to_string(),
            "Some Store".to_string(),
        );
        let store_2: StoreModel = StoreModel::new(
            "487DACC8-C56D-49C6-A7CE-C82E0300AB6F".to_string(),
            "Some Other Store".to_string(),
        );
        let store_3: StoreModel = StoreModel::new(
            "E02D350D-280D-403C-8A4B-57220A2EF84A".to_string(),
            "Yet Another Store".to_string(),
        );

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

        let store_1: StoreModel = StoreModel::new(
            "1E1C9D92-A233-4153-BC24-AA20FD75FB8B".to_string(),
            "Some Store".to_string(),
        );
        let store_2: StoreModel = StoreModel::new(
            "E6DE83D0-80B2-498F-923D-3EFCCA9DF7D8".to_string(),
            "Some Other Store".to_string(),
        );
        let store_3: StoreModel = StoreModel::new(
            "DF546FBA-4677-457D-BE74-F19237B9CBA8".to_string(),
            "Yet Another Store".to_string(),
        );

        assert_eq!(cache.upsert_store(store_1.clone()), None);
        assert_eq!(cache.upsert_store(store_2.clone()), None);
        assert_eq!(cache.upsert_store(store_3.clone()), None);

        let store: Option<StoreModel> =
            cache.get_single_store(&"2D195006-AEA7-4DDB-ADD8-A1B070EB01EC".to_string());
        assert_eq!(store, None);
    }

    #[test]
    fn get_existing_store_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "BE9498F4-23CC-4A75-8FD8-0C116F7755F4".to_string();
        let store_1: StoreModel = StoreModel::new(target_key.clone(), "Some Store".to_string());
        let store_2: StoreModel = StoreModel::new(
            "5C844592-7F39-425F-8D29-5F9FA0266561".to_string(),
            "Some Other Store".to_string(),
        );
        let store_3: StoreModel = StoreModel::new(
            "060F221A-349E-40DD-A975-837F6581856C".to_string(),
            "Yet Another Store".to_string(),
        );

        assert_eq!(cache.upsert_store(store_1.clone()), None);
        assert_eq!(cache.upsert_store(store_2.clone()), None);
        assert_eq!(cache.upsert_store(store_3.clone()), None);

        let store: Option<StoreModel> = cache.get_single_store(&target_key);
        assert_eq!(store, Some(store_1));
    }

    #[test]
    fn no_brands_upsert_new_brand_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let brand_model: BrandModel = BrandModel::new(
            "C3DA8CD6-5362-4124-BCBD-FBB73E483EE0".to_string(),
            "Some Brand".to_string(),
        );

        assert_eq!(cache.upsert_brand(brand_model.clone()), None);

        let all_brands: Vec<BrandModel> = cache.get_all_brands();
        assert_eq!(all_brands.len(), 1);
        assert!(all_brands.contains(&brand_model));
    }

    #[test]
    fn upsert_existing_brand_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: String = "C0CAC505-D2B4-4878-B2BC-D789B8B4B07A".to_string();
        let existing_brand: BrandModel = BrandModel::new(key.clone(), "Some Brand".to_string());
        let updated_brand: BrandModel =
            BrandModel::new(key.clone(), "Some updated Brand".to_string());

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

        let brand_1: BrandModel = BrandModel::new(
            "389016A7-BDE8-48AE-9542-8611AD7A6115".to_string(),
            "Some Brand".to_string(),
        );
        let brand_2: BrandModel = BrandModel::new(
            "A12E6B36-2D05-4B83-A42E-B3A6DE1D1417".to_string(),
            "Some Other Brand".to_string(),
        );
        let brand_3: BrandModel = BrandModel::new(
            "D2F5A1BA-D902-4405-A4A6-3708575AA2AA".to_string(),
            "Yet Another Brand".to_string(),
        );

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

        let brand_1: BrandModel = BrandModel::new(
            "1738C808-00CF-40E6-B279-E154013DD1D3".to_string(),
            "Some Brand".to_string(),
        );
        let brand_2: BrandModel = BrandModel::new(
            "508DA259-8D3D-46D6-B641-981254334995".to_string(),
            "Some Other Brand".to_string(),
        );
        let brand_3: BrandModel = BrandModel::new(
            "F5A44D70-4C0D-4EFB-AC15-E03D6C46062D".to_string(),
            "Yet Another Brand".to_string(),
        );

        assert_eq!(cache.upsert_brand(brand_1.clone()), None);
        assert_eq!(cache.upsert_brand(brand_2.clone()), None);
        assert_eq!(cache.upsert_brand(brand_3.clone()), None);

        let brand: Option<BrandModel> =
            cache.get_single_brand(&"C8D32013-6773-43A8-8BA6-FA87A94335B1".to_string());
        assert_eq!(brand, None);
    }

    #[test]
    fn get_existing_brand_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "F233E1AB-2A2C-485B-BE57-0253B95E5767".to_string();
        let brand_1: BrandModel = BrandModel::new(target_key.clone(), "Some Brand".to_string());
        let brand_2: BrandModel = BrandModel::new(
            "50839B13-3EE8-493E-A34A-D44DB255D261".to_string(),
            "Some Other Brand".to_string(),
        );
        let brand_3: BrandModel = BrandModel::new(
            "D86892D5-296C-4278-B85E-8CE061D1FE47".to_string(),
            "Yet Another Brand".to_string(),
        );

        assert_eq!(cache.upsert_brand(brand_1.clone()), None);
        assert_eq!(cache.upsert_brand(brand_2.clone()), None);
        assert_eq!(cache.upsert_brand(brand_3.clone()), None);

        let brand: Option<BrandModel> = cache.get_single_brand(&target_key);
        assert_eq!(brand, Some(brand_1));
    }

    #[test]
    fn no_categories_upsert_new_category_return_none_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();
        let category_model: CategoryModel = CategoryModel::new(
            "493180A6-322E-4031-AAE4-00AFD696497D".to_string(),
            "Some Category".to_string(),
        );

        assert_eq!(cache.upsert_category(category_model.clone()), None);

        let all_categories: Vec<CategoryModel> = cache.get_all_categories();
        assert_eq!(all_categories.len(), 1);
        assert!(all_categories.contains(&category_model));
    }

    #[test]
    fn upsert_existing_category_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: String = "985D9369-2B21-4F59-B228-9E192277A9A6".to_string();
        let existing_category: CategoryModel =
            CategoryModel::new(key.clone(), "Some Category".to_string());
        let updated_category: CategoryModel =
            CategoryModel::new(key.clone(), "Some updated Category".to_string());

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

        let category_1: CategoryModel = CategoryModel::new(
            "CD21C158-E52C-47EF-80A2-CB1985DE4B83".to_string(),
            "Some Category".to_string(),
        );
        let category_2: CategoryModel = CategoryModel::new(
            "493E2116-5AA5-4AC6-99D8-3DA4DAD93E13".to_string(),
            "Some Other Category".to_string(),
        );
        let category_3: CategoryModel = CategoryModel::new(
            "FECD8BA6-6F94-4902-BECE-86D4ED7E3117".to_string(),
            "Yet Another Category".to_string(),
        );

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

        let category_1: CategoryModel = CategoryModel::new(
            "8672F91B-D326-41AC-A9E5-AC00E43E6131".to_string(),
            "Some Category".to_string(),
        );
        let category_2: CategoryModel = CategoryModel::new(
            "3E5B9BDB-FD8B-48CF-94EB-2921A5515615".to_string(),
            "Some Other Category".to_string(),
        );
        let category_3: CategoryModel = CategoryModel::new(
            "DD793F32-E518-456F-9A82-0DC43344BD41".to_string(),
            "Yet Another Category".to_string(),
        );

        assert_eq!(cache.upsert_category(category_1.clone()), None);
        assert_eq!(cache.upsert_category(category_2.clone()), None);
        assert_eq!(cache.upsert_category(category_3.clone()), None);

        let category: Option<CategoryModel> =
            cache.get_single_category(&"7AAAF9D4-3EEF-4549-9210-64D98B297A56".to_string());
        assert_eq!(category, None);
    }

    #[test]
    fn get_existing_category_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "CA470BB5-5F53-477A-AE75-08A1E0EFFA8E".to_string();
        let category_1: CategoryModel =
            CategoryModel::new(target_key.clone(), "Some Category".to_string());
        let category_2: CategoryModel = CategoryModel::new(
            "7B39DF64-6C45-44F1-BAC8-05BDF4DB86A8".to_string(),
            "Some Other Category".to_string(),
        );
        let category_3: CategoryModel = CategoryModel::new(
            "10032AAA-AB41-4718-AB52-E6A168F8E03A".to_string(),
            "Yet Another Category".to_string(),
        );

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
            "C5BBB68C-9E7D-48BD-A4BB-AB6E37855775".to_string(),
            "Some Product".to_string(),
            "DC6DD0BE-E70C-4574-9060-E49DFB167C89".to_string(),
            "29B9E2C8-6C17-41A5-95BD-4CE9B9922941".to_string(),
        );

        assert_eq!(cache.upsert_product(product_model.clone()), None);

        let all_products: Vec<ProductModel> = cache.get_all_products();
        assert_eq!(all_products.len(), 1);
        assert!(all_products.contains(&product_model));
    }

    #[test]
    fn upsert_existing_product_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: String = "0DB3DD3D-CA4A-4CB1-A0C4-839FE1513578".to_string();
        let brand_key: String = "F473D4DE-894D-46FB-9DAB-415D97E94614".to_string();
        let category_key: String = "AD95B9A8-D9C3-4AA3-A565-D67F5CB263C9".to_string();

        let existing_product: ProductModel = ProductModel::new(
            key.clone(),
            "Some Product".to_string(),
            brand_key.clone(),
            category_key.clone(),
        );
        let updated_product: ProductModel = ProductModel::new(
            key.clone(),
            "Some updated Product".to_string(),
            brand_key.clone(),
            category_key.clone(),
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
            "7DD5D642-D192-4B05-89CB-F3E5C2877896".to_string(),
            "Some Product".to_string(),
            "8B3BA55A-9D07-4A92-B825-FED2198E5E91".to_string(),
            "BE6C7F4C-2365-48A7-B333-F30D08A0DC36".to_string(),
        );
        let product_2: ProductModel = ProductModel::new(
            "146196CC-3C37-4229-94DA-BE8507C088BA".to_string(),
            "Some Other Product".to_string(),
            "F39D64B2-5399-4956-88A2-F8CB90B43E2B".to_string(),
            "9C05FE69-7E72-405C-B091-D35B43CBBA28".to_string(),
        );
        let product_3: ProductModel = ProductModel::new(
            "AFFD6DDC-41BB-499A-B76C-B3BAA43FB28B".to_string(),
            "Yet Another Product".to_string(),
            "02BCCE8A-7FA0-47F9-9902-D96E7A11A4F4".to_string(),
            "0C124495-284C-43CD-B339-1A5973B8BAE7".to_string(),
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
            "13048D20-77BA-4F4E-9F32-782EA185B6FA".to_string(),
            "Some Product".to_string(),
            "76A1181A-0827-4E93-B305-D22D0243748B".to_string(),
            "C8CBCD6E-3F04-4A6C-94F5-FB646E6CEC95".to_string(),
        );
        let product_2: ProductModel = ProductModel::new(
            "D654621B-1105-46AD-B002-F2392DDFADC5".to_string(),
            "Some Other Product".to_string(),
            "7A51BA78-9C10-483E-8B3A-2D341D706B2E".to_string(),
            "610FED4B-E931-4A95-BD85-0177F2190748".to_string(),
        );
        let product_3: ProductModel = ProductModel::new(
            "982BF804-08D5-4E78-B0BB-707218C27CD5".to_string(),
            "Yet Another Product".to_string(),
            "B08BC960-39F1-47DB-8010-ED36C7A3E740".to_string(),
            "63676883-EEAC-4460-BEB0-C73E49721174".to_string(),
        );

        assert_eq!(cache.upsert_product(product_1.clone()), None);
        assert_eq!(cache.upsert_product(product_2.clone()), None);
        assert_eq!(cache.upsert_product(product_3.clone()), None);

        let product: Option<ProductModel> =
            cache.get_single_product(&"C373A86C-54C7-4C40-9FBE-B5FFE85AEF1E".to_string());
        assert_eq!(product, None);
    }

    #[test]
    fn get_existing_product_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "4C48A5A6-D802-4EEE-89A6-0AA17CBDE0D6".to_string();
        let product_1: ProductModel = ProductModel::new(
            target_key.clone(),
            "Some Product".to_string(),
            "9A6E88AB-4F75-4C36-BA37-BE0BDB8B57D8".to_string(),
            "1A9ED8D0-DC1B-407B-892A-9B5837A19AEC".to_string(),
        );
        let product_2: ProductModel = ProductModel::new(
            "860AFBB7-02E9-47A2-A12B-B3457796996E".to_string(),
            "Some Other Product".to_string(),
            "B0D91B19-F5A1-42B3-8466-7C37D61FACD9".to_string(),
            "896D9307-B289-47F6-85E3-CB8CD5A0EADF".to_string(),
        );
        let product_3: ProductModel = ProductModel::new(
            "48664AE5-ECB2-4425-8AB6-3340087AE41C".to_string(),
            "Yet Another Product".to_string(),
            "5168ACC4-F235-492F-8A55-488D3FAD1464".to_string(),
            "614D9072-DCAA-4303-82A2-D018C75C7985".to_string(),
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
        let item_model: ItemModel = ItemModel::new(
            "E182A427-94F2-4D4A-AF38-8EA03D2DA667".to_string(),
            "BC9B9C0C-9DED-4FAD-9589-FE8A0A5C3B35".to_string(),
            UnitModel::None,
            85.04,
        );

        assert_eq!(cache.upsert_item(item_model.clone()), None);

        let all_items: Vec<ItemModel> = cache.get_all_items();
        assert_eq!(all_items.len(), 1);
        assert!(all_items.contains(&item_model));
    }

    #[test]
    fn upsert_existing_item_return_previous_and_retrieve() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let key: String = "E2D08989-F879-479B-A80F-EC3D0C4CEDFF".to_string();
        let existing_item: ItemModel = ItemModel::new(
            key.clone(),
            "F543E605-971C-4F00-BC95-9760B053387D".to_string(),
            UnitModel::None,
            80.36,
        );
        let updated_item: ItemModel = ItemModel::new(
            key.clone(),
            "DE20ECB8-1E00-483B-BFEF-9212B73E085C".to_string(),
            UnitModel::Liters(25.38),
            33.03,
        );

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
            "6203F84E-4FC6-49E8-AF44-38076DE40EF7".to_string(),
            "EDD63DF9-7929-49B7-9DE6-5C92DDEF6C2F".to_string(),
            UnitModel::Liters(84.53),
            26.68,
        );
        let item_2: ItemModel = ItemModel::new(
            "5022A853-54BC-4AED-90C4-B0E4F57C3641".to_string(),
            "E01C542D-4629-43E9-A062-59779197A99A".to_string(),
            UnitModel::Quantity(81.0),
            29.65,
        );
        let item_3: ItemModel = ItemModel::new(
            "BC576D27-4F96-4C58-8063-61BCEFF175BA".to_string(),
            "C10BB789-E366-4B4F-96B2-204F13BC2A16".to_string(),
            UnitModel::None,
            40.56,
        );

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

        let item_1: ItemModel = ItemModel::new(
            "349B7647-2F85-4748-86A8-6B36FD76F99C".to_string(),
            "5E9DE23A-927D-44AB-A8B0-1D49CCAB5E1B".to_string(),
            UnitModel::None,
            39.89,
        );
        let item_2: ItemModel = ItemModel::new(
            "01EB9A5F-EF31-45CA-B295-1DF9FDE421C4".to_string(),
            "309C40B6-9AF1-4DD3-97C1-4D99C7AB8F99".to_string(),
            UnitModel::Kilograms(31.32),
            59.95,
        );
        let item_3: ItemModel = ItemModel::new(
            "7EB4C4FF-D5F3-4A88-9D3B-B7ECA4564E62".to_string(),
            "05828A34-55E9-44AB-B43B-CDDDFD225961".to_string(),
            UnitModel::None,
            98.64,
        );

        assert_eq!(cache.upsert_item(item_1.clone()), None);
        assert_eq!(cache.upsert_item(item_2.clone()), None);
        assert_eq!(cache.upsert_item(item_3.clone()), None);

        let item: Option<ItemModel> =
            cache.get_single_item(&"0E598BF1-3B07-4368-8DFA-285031383F9B".to_string());
        assert_eq!(item, None);
    }

    #[test]
    fn get_existing_item_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "832C0CD7-5FB0-4677-9C04-B2A9388026D4".to_string();
        let item_1: ItemModel = ItemModel::new(
            target_key.clone(),
            "27D12468-87C2-4DFD-A139-0CB713373E21".to_string(),
            UnitModel::Liters(73.77),
            11.60,
        );
        let item_2: ItemModel = ItemModel::new(
            "3A6857F9-B61E-491A-8980-434D96E120E7".to_string(),
            "2BE65104-5863-413F-8908-2578E2BBE38F".to_string(),
            UnitModel::None,
            94.96,
        );
        let item_3: ItemModel = ItemModel::new(
            "C42CE802-0A28-4781-94A2-AF27B52D1CF6".to_string(),
            "09EA7BFD-1474-4EE1-B399-99865A395264".to_string(),
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
            "391F52B6-081E-472C-81D4-9D4B3B7D236F".to_string(),
            vec!["34B59923-335C-4346-A5C2-C277854A2FF1".to_string()],
            "7A48C6F3-17F4-4CA1-84E8-FD60D430D80E".to_string(),
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

        let key: String = "C996620D-8309-49DA-93D7-EE821B5C7BA9".to_string();
        let store_key: String = "0E98AC16-BC6B-4BB4-A829-1C16C97669F9".to_string();
        let existing_transaction: TransactionModel = TransactionModel::new(
            key.clone(),
            vec![],
            store_key.clone(),
            "2021-10-26T03:30:48Z".to_string(),
        );
        let updated_transaction: TransactionModel = TransactionModel::new(
            key.clone(),
            vec![],
            store_key.clone(),
            "2021-05-25T21:10:43Z".to_string(),
        );

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
            "4158CEA8-0912-4756-98F6-C2806315A855".to_string(),
            vec![],
            "07A61C17-A412-4591-906A-CC2758CD59B6".to_string(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            "26E2F1C3-F1CC-4CB5-89AF-09A8A669E806".to_string(),
            vec!["EEF610FB-6ED1-42FE-9B2B-593CB57619CA".to_string()],
            "F7143996-2304-4F34-B59E-D322C1D8221A".to_string(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            "3703D43C-62A7-4D9D-AB83-764A87E59E86".to_string(),
            vec![
                "2762C001-EBB7-41AA-8449-33069952E76B".to_string(),
                "965C80E0-0E87-41E2-A1CC-17F1E2618343".to_string(),
                "F858ECED-59C1-4FC9-BB30-2DAB4409CC02".to_string(),
            ],
            "DA24BD4C-A9BC-4211-8792-3D5E55DF6C85".to_string(),
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
            "3A4DB4CA-7602-4CFF-ACDF-DC61502E038A".to_string(),
            vec![],
            "E34896E0-FEC7-424D-8D1B-31F7D109262E".to_string(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            "C7E46DD2-CCF0-4BE5-B16B-8694C1C48933".to_string(),
            vec!["0364A1B0-C6FA-4FFD-9F0D-3E86791091C9".to_string()],
            "CD25CADA-CE2F-4F59-A233-D86F755069D8".to_string(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            "E556D7D3-4A92-455C-BE99-BC065B1BDCC8".to_string(),
            vec![
                "B850A566-A728-4B5E-BBBE-4F5C59740E75".to_string(),
                "6BF01E6C-E087-47A5-B253-C3F2BFEA2C10".to_string(),
                "3B4363FD-7132-499B-80B7-546872CCC643".to_string(),
            ],
            "5239BD51-608E-446B-8943-EDDD53322C63".to_string(),
            "2020-11-18T02:08:09Z".to_string(),
        );

        assert_eq!(cache.upsert_transaction(transaction_1.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_2.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_3.clone()), None);

        let transaction: Option<TransactionModel> =
            cache.get_single_transaction(&"EEDEB385-A734-423B-AF0D-7D68BDC86748".to_string());
        assert_eq!(transaction, None);
    }

    #[test]
    fn get_existing_transaction_return_it() {
        let mut cache: InMemoryCache = InMemoryCache::new();

        let target_key: String = "712FB316-3FC5-4103-A2F4-4BBB8DABB1F8".to_string();
        let transaction_1: TransactionModel = TransactionModel::new(
            target_key.clone(),
            vec![],
            "6A6E46A2-1AC7-4DDA-AED6-99E40FE071B6".to_string(),
            "2017-05-28T21:07:57Z".to_string(),
        );
        let transaction_2: TransactionModel = TransactionModel::new(
            "AE8FBD41-9667-4E93-8DF5-574A92E839D4".to_string(),
            vec!["C5D1EBCA-A1A9-4834-88F8-EF37306F2F8E".to_string()],
            "A454BA36-40CC-497A-9875-E86AF25C9E78".to_string(),
            "2015-11-12T17:44:56Z".to_string(),
        );
        let transaction_3: TransactionModel = TransactionModel::new(
            "AB600EC9-6D0A-408D-9A4C-59FEBD9DD104".to_string(),
            vec![
                "0AA12100-95D2-4D92-936B-167502E938DD".to_string(),
                "4AA5C5C6-FC74-4B48-8EEF-F00B8C606FB3".to_string(),
                "435D6A99-9099-4DED-A8B7-B724805918EC".to_string(),
            ],
            "5D359788-C3BC-473C-93EA-D83C37B564E3".to_string(),
            "2020-11-18T02:08:09Z".to_string(),
        );

        assert_eq!(cache.upsert_transaction(transaction_1.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_2.clone()), None);
        assert_eq!(cache.upsert_transaction(transaction_3.clone()), None);

        let transaction: Option<TransactionModel> = cache.get_single_transaction(&target_key);
        assert_eq!(transaction, Some(transaction_1));
    }
}
