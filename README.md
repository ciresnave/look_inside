# look_inside

Did you ever wonder how Rust sees your data type?

Now you can find out with one simple derive macro...

But first, you need to add the look_inside crate to your dependencies in Cargo.toml:

``` TOML
[dependencies]
look_inside = "0.1"
```

Then you simply derive `LookInside` on struct `struct`, `enum`, or `union` and compile.  `LookInside` will panic during compile and show you the full abstract syntax tree it breaks down into.  Want to know the AST for a type other than a `struct`, `enum`, or `union`?  No problem.  Use your type in a `struct`, `enum`, or `union`.

Struct example:

``` Rust
use look_inside::LookInside;

#[Derive(LookInside)]
struct MyStruct {
    oneThing: u8,
    twoThing: String,
    threeThing: Vec<u16>,
}
```

Enum example:

``` Rust
use look_inside::LookInside;

#[Derive(LookInside)]
enum MyEnum {
    variantOne(u8),
    variantTwo(u16),
    variantThree(MyStruct)
}
```

Union example:

``` Rust
use look_inside::LookInside;

#[Derive(LookInside)]
union MyUnion {
    aU8: u8,
    aU16: u16,
    myStruct: MyStruct,
}
```

For those who want to understand how this all works...look at lib.rs.  There is literally NOTHING to this.  

If it's so simple, why did I make this crate?  Simple, I wanted to see inside MY types so I can make better derive macros.  Having this in a crate makes it easy to add it to any project I'm working on just long enough to inspect things and then remove it again.
