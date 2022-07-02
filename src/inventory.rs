pub use inventory::submit;

use crate::Zephyr;

pub struct AddClassToInventory {
    class: &'static str,
}

impl AddClassToInventory {
    pub const fn new(class: &'static str) -> Self {
        Self { class }
    }
}

inventory::collect!(AddClassToInventory);

impl Zephyr {
    pub fn generate_from_inventory(&self) -> String {
        self.generate_classes(
            inventory::iter::<AddClassToInventory>
                .into_iter()
                .map(|a| a.class),
        )
    }
}

#[macro_export]
macro_rules! register_class {
    ($c:literal) => {{
        const C: &'static str = $c;
        $crate::inventory::submit! {
            $crate::inventory::AddClassToInventory::new(C)
        }
        C
    }};
}
