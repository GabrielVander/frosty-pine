use uuid::Uuid;
use uuid_b64::UuidB64;

use crate::domain::entities::Product;
use crate::domain::entities::Unit;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item {
    id: UuidB64,
    product: Product,
    unit: Unit,
    unitary_price: f64,
}

impl Item {
    pub fn new(id: Option<UuidB64>, product: Product, unit: Unit, unitary_price: f64) -> Self {
        Self {
            id: id.unwrap_or_else(|| UuidB64::from(Uuid::new_v4())),
            product,
            unit,
            unitary_price,
        }
    }

    pub fn calculate_full_price(&self) -> f64 {
        match self.unit {
            Unit::None => self.unitary_price,
            Unit::Quantity(amount) => amount * self.unitary_price,
            Unit::Kilograms(weight) => weight * self.unitary_price,
            Unit::Liters(volume) => volume * self.unitary_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Brand;
    use super::super::Category;
    use super::Item;
    use super::Product;
    use super::Unit;

    macro_rules! calculate_full_price_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (unit, unitary_price, expected) = $value;
                let brand: Brand = Brand::default();
                let category: Category = Category::default();
                let product: Product = Product::new(None, "".into(), brand, category);

                let item: Item = Item::new(None, product, unit, unitary_price);
                let result: f64 = item.calculate_full_price();

                assert_eq!(
                    result, expected,
                    "Expected {}, but got {}",
                    expected, result
                )
            }
        )*
        }
    }

    calculate_full_price_tests! {
        none_should_return_same_as_price_0: (Unit::None, 37.99, 37.99),
        none_should_return_same_as_price_1: (Unit::None, 12.53, 12.53),
        none_should_return_same_as_price_2: (Unit::None, 0., 0.),
        quantity_should_return_zero_0: (Unit::Quantity(0.), 0., 0.),
        quantity_should_return_zero_1: (Unit::Quantity(0.), 90.35, 0.),
        quantity_should_return_zero_2: (Unit::Quantity(0.), 96.22, 0.),
        quantity_should_return_zero_3: (Unit::Quantity(63.),  0., 0.),
        quantity_should_return_zero_4: (Unit::Quantity(89.),  0., 0.),
        quantity_should_return_same_as_price_0: (Unit::Quantity(1.), 49.3, 49.3),
        quantity_should_return_same_as_price_1: (Unit::Quantity(1.), 61.68, 61.68),
        quantity_should_return_same_as_amount_0: (Unit::Quantity(73.), 1., 73.),
        quantity_should_return_same_as_amount_1: (Unit::Quantity(94.), 1., 94.),
        kilograms_should_return_zero_0: (Unit::Kilograms(0.), 0., 0.),
        kilograms_should_return_zero_1: (Unit::Kilograms(0.), 32.18, 0.),
        kilograms_should_return_zero_2: (Unit::Kilograms(0.), 35.69, 0.),
        kilograms_should_return_zero_3: (Unit::Kilograms(45.),  0., 0.),
        kilograms_should_return_zero_4: (Unit::Kilograms(38.),  0., 0.),
        kilograms_should_return_same_as_price_0: (Unit::Kilograms(1.), 66.28, 66.28),
        kilograms_should_return_same_as_price_1: (Unit::Kilograms(1.), 76.88, 76.88),
        kilograms_should_return_same_as_amount_0: (Unit::Kilograms(85.), 1., 85.),
        kilograms_should_return_same_as_amount_1: (Unit::Kilograms(19.), 1., 19.),
        liters_should_return_zero_0: (Unit::Liters(0.), 0., 0.),
        liters_should_return_zero_1: (Unit::Liters(0.), 31.56, 0.),
        liters_should_return_zero_2: (Unit::Liters(0.), 53.62, 0.),
        liters_should_return_zero_3: (Unit::Liters(78.),  0., 0.),
        liters_should_return_zero_4: (Unit::Liters(31.),  0., 0.),
        liters_should_return_same_as_price_0: (Unit::Liters(1.), 79.98, 79.98),
        liters_should_return_same_as_price_1: (Unit::Liters(1.), 84.35, 84.35),
        liters_should_return_same_as_amount_0: (Unit::Liters(44.), 1., 44.),
        liters_should_return_same_as_amount_1: (Unit::Liters(2.), 1., 2.),
    }
}
