use crate::{fields::*, *};

/// Represents a Rust union definition.
pub struct Union {
    /// The attributes applied to the union.
    pub attributes: Vec<Attribute>,
    /// The visibility of the union, e.g. `pub`.
    pub visibility: Visibility,
    /// The identifier name of the union.
    pub identifier: Ident,
    /// The generic parameters of the union, if any.
    pub generics: Generics,
    /// The named fields belonging to the union.
    pub fields: Vec<NamedField>,
}

impl Union {
    /// Creates a new Union from a DeriveInput.
    ///
    /// Parses the input, extracting fields if it is a union.
    /// Panics if the input is not a union.
    pub fn new(input: DeriveInput) -> Self {
        let mut fields: Vec<NamedField> = Vec::new();
        if let syn::Data::Union(new_input) = input.data {
            let possibly_named_fields = new_input.fields;
            for field in possibly_named_fields.named {
                fields.push(NamedField::new(field));
            }
        } else {
            panic!("Only structs may be parsed into StructWithUnnamedFields");
        }
        Self {
            attributes: input.attrs,
            visibility: input.vis,
            identifier: input.ident,
            generics: input.generics,
            fields,
        }
    }
}
