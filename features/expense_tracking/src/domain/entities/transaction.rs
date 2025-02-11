use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;
use uuid_b64::UuidB64;

use crate::domain::entities::Item;
use crate::domain::entities::Store;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: UuidB64,
    items: Vec<Item>,
    store: Store,
    datetime: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        id: Option<UuidB64>,
        items: Vec<Item>,
        store: Store,
        datetime: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| UuidB64::from(Uuid::new_v4())),
            items,
            store,
            datetime,
        }
    }

    pub fn calculate_total(&self) -> f64 {
        let decimal_places: i32 = 2;
        let sum: f64 = self.items.iter().map(|i| i.calculate_full_price()).sum();

        sum.mul(10.0_f64.powi(decimal_places)).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::super::Brand;
    use super::super::Category;
    use super::super::Product;
    use super::super::Unit;
    use super::Item;
    use super::Store;
    use super::Transaction;
    use chrono::DateTime;

    macro_rules! calculate_total {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (items_data, expected) = $value;
                let store: Store = Store::default();
                let brand: Brand = Brand::default();
                let category: Category = Category::default();
                let product: Product = Product::new(None, "Product".into(), brand, category);
                let items: Vec<Item> = items_data.map(|i: (Unit, f64)| Item::new(None, product.clone(), i.0, i.1)).into();

                let transaction: Transaction = Transaction::new(None, items, store, DateTime::default());
                let result: f64 = transaction.calculate_total();

                assert_eq!(
                    result, expected,
                    "Expected {}, but got {}",
                    expected, result
                )
            }
        )*
        }
    }

    calculate_total! {
        given_no_items_then_return_zero: ([], 0.),
        given_one_item_no_unit_for_free_then_return_zero: ([(Unit::None, 0.)], 0.),
        given_multiple_items_no_unit_for_free_then_return_zero: ([(Unit::None, 0.), (Unit::None, 0.), (Unit::None, 0.)], 0.),
        given_one_item_no_unit_then_return_price: ([(Unit::None, 9.46)], 9.46),
        given_multiple_items_no_unit_and_multiple_prices_then_return_sum: ([(Unit::None, 76.12), (Unit::None, 7.16), (Unit::None, 19.73)], 103.01),
        given_one_item_multiple_units_for_free_then_return_zero: ([(Unit::Quantity(12.), 0.)], 0.),
        given_multiple_items_multiple_units_for_free_then_return_zero: ([(Unit::Quantity(63.), 0.), (Unit::Quantity(14.), 0.), (Unit::Quantity(80.), 0.)], 0.),
        given_multiple_items_no_units_then_return_zero: ([(Unit::Quantity(0.), 50.83), (Unit::Quantity(0.), 49.49), (Unit::Quantity(0.), 15.08)], 0.),
        given_one_item_one_unit_then_return_price: ([(Unit::Quantity(1.), 80.48)], 80.48),
        given_multiple_items_one_unit_then_return_sum: ([(Unit::Quantity(1.), 50.83), (Unit::Quantity(1.), 49.49), (Unit::Quantity(1.), 15.08)], 115.4),
        given_multiple_items_multiple_units_then_return_sum: ([(Unit::Quantity(80.), 50.83), (Unit::Quantity(62.), 49.49), (Unit::Quantity(81.), 15.08)], 8356.26),
        given_one_item_with_multiple_kilo_for_free_then_return_zero: ([(Unit::Kilograms(19.14), 0.)], 0.),
        given_multiple_items_with_multiple_kilos_for_free_then_return_zero: ([(Unit::Kilograms(4.90), 0.), (Unit::Kilograms(10.18), 0.), (Unit::Kilograms(39.34), 0.)], 0.),
        given_multiple_items_with_no_kilos_then_return_zero: ([(Unit::Kilograms(0.), 50.83), (Unit::Kilograms(0.), 49.49), (Unit::Kilograms(0.), 15.08)], 0.),
        given_one_item_with_one_kilo_then_return_price: ([(Unit::Kilograms(1.), 7.05)], 7.05),
        given_multiple_items_with_one_kilo_then_return_sum: ([(Unit::Kilograms(1.), 50.83), (Unit::Kilograms(1.), 49.49), (Unit::Kilograms(1.), 15.08)], 115.4),
        given_multiple_items_with_multiple_kilos_then_return_sum: ([(Unit::Kilograms(80.), 50.83), (Unit::Kilograms(62.), 49.49), (Unit::Kilograms(81.), 15.08)], 8356.26),
        given_one_item_with_multiple_liters_for_free_then_return_zero: ([(Unit::Liters(73.17), 0.)], 0.),
        given_multiple_items_with_multiple_liters_for_free_then_return_zero: ([(Unit::Liters(53.08), 0.), (Unit::Liters(62.28), 0.), (Unit::Liters(48.22), 0.)], 0.),
        given_multiple_items_with_no_liters_then_return_zero: ([(Unit::Liters(0.), 51.19), (Unit::Liters(0.), 64.59), (Unit::Liters(0.), 55.56)], 0.),
        given_one_item_with_one_liter_then_return_price: ([(Unit::Liters(1.), 38.83)], 38.83),
        given_multiple_items_with_one_liter_then_return_sum: ([(Unit::Liters(1.), 50.83), (Unit::Liters(1.), 49.49), (Unit::Liters(1.), 15.08)], 115.4),
        given_multiple_items_with_multiple_liters_then_return_sum: ([(Unit::Liters(80.), 50.83), (Unit::Liters(62.), 49.49), (Unit::Liters(81.), 15.08)], 8356.26),
    }
}
