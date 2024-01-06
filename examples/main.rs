use look_inside::LookInside;
use syn;

#[derive(LookInside)]
struct StructInternals {
    attributes: Vec<syn::Attribute>,
    visibility: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    fields: syn::Fields,
}

#[derive(LookInside)]
enum MyTypes {
    A(i32),
    B(String),
    C { x: i32, y: i32 },
    D,
}

#[derive(LookInside)]
union MyUnion {
    a: i32,
    b: std::mem::ManuallyDrop<TestStruct>,
}

fn main() {
    //
}
