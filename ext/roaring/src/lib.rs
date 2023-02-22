use magnus::{self as magnus, define_class, define_module, function, method, prelude::*, Error};

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}

#[derive(Debug)]
#[magnus::wrap(class = "Roaring::Bitmap", mark)]
struct Bitmap {
    data: roaring::RoaringBitmap,
}

impl Bitmap {
    fn new() -> Self {
        Self {
            data: roaring::RoaringBitmap::new(),
        }
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let bitmap_class = define_class("Roaring::Bitmap", Default::default())?;
    bitmap_class.define_singleton_method("new", function!(Bitmap::new, 0))?;
    Ok(())
}
