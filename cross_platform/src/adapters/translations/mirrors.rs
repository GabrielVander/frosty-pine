pub use expense_tracking::domain::entities::Brand;

#[flutter_rust_bridge::frb(mirror(Brand))]
struct _Brand {
    pub name: String,
}
