#[derive(Debug, Clone, Default, PartialEq)]
pub enum Unit {
    #[default]
    None,
    Quantity(f64),
    Kilograms(f64),
    Liters(f64),
}
