# look_inside

This crate provides the `LookInside` custom derive macro that allows
introspecting the fields of structs and enums at compile time.

It also provides modules for working with different types:

- `enums` - Provides functionality for enum types.
- `fields` - Provides functionality for struct fields.
- `structs` - Provides functionality for struct types.
- `unions` - Provides functionality for union types.

The `LookInside` macro generates an implementation of the `LookInside`
trait which exposes the fields and variants in a structured way at
compile time. This allows iterating and matching on fields and variants
in macros.

Key items exported:

- `LookInside` - The custom derive macro.
- `Ident`, `Type`, etc. - Re-exports of useful types from Syn.
This crate provides a convenient way to introspect struct and enum fields at
compile time using the `LookInside` custom derive macro.

To use it to view the structure of your structs, enums, and unions, add
`#[derive(LookInside)]` to your struct or enum definition. This will
generate an implementation of the `LookInside` trait for that type, which exposes
its fields and variants in a structured way in a panic message.

For example:

``` Rust
#[derive(LookInside)]
struct MyStruct {
    field1: u32,
    field2: String,
}
```

Alternatively, this crate also contains the code to take a DeriveInput from the
syn crate and return structured representations of your types for use in your code.

Here's a simple example of how to print the fields of a struct:

``` Rust
let my_struct = MyStruct { field1: 42, field2: "foo".to_string() };

// Access fields:
let field1 = my_struct.look_inside().fields().field1;

// Iterate fields:
for field in my_struct.look_inside().fields() {
    println!("{}", field.ident);
}
```

This provides a convenient way to introspect and process structs, unions,
and enums generically in macros and procedural macros.
