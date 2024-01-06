use crate::{fields::*, *};

/// Struct to represent a struct with named fields
pub struct StructWithNamedFields {
    /// Vector of attributes
    pub attributes: Vec<Attribute>,
    /// Visibility of struct  
    pub visibility: Visibility,
    /// Identifier of struct
    pub ident: Ident,
    /// Generic parameters of struct
    pub generics: Generics,
    /// Vector of named fields  
    pub fields: Vec<NamedField>,
}

impl StructWithNamedFields {
    /// Creates a new StructWithNamedFields from a syn::DeriveInput
    ///
    /// # Panics
    /// Panics if the input is not a struct with named fields
    pub fn new(input: syn::DeriveInput) -> Self {
        let mut fields: Vec<NamedField> = Vec::new();
        if let syn::Data::Struct(new_input) = input.data {
            if let syn::Fields::Named(named_fields) = new_input.fields {
                for named_field in named_fields.named.into_iter() {
                    fields.push(NamedField::new(named_field));
                }
            } else {
                panic!("Only structs with named fields may be parsed into StructWithNamedFields");
            }
        } else {
            panic!("Only structs may be parsed into StructWithNamedFields");
        }
        Self {
            attributes: input.attrs,
            visibility: input.vis,
            ident: input.ident,
            generics: input.generics,
            fields,
        }
    }
}

/// Struct to represent a struct with unnamed fields
pub struct StructWithUnnamedFields {
    /// Vector of attributes
    pub attributes: Vec<Attribute>,
    /// Visibility of struct
    pub visibility: Visibility,
    /// Identifier of struct
    pub ident: Ident,
    /// Generic parameters of struct
    pub generics: Generics,
    /// Vector of unnamed fields
    pub fields: Vec<UnnamedField>,
}

impl StructWithUnnamedFields {
    /// Creates a new StructWithUnnamedFields from a syn::DeriveInput
    ///
    /// # Panics
    /// Panics if the input is not a struct with unnamed fields
    pub fn new(input: syn::DeriveInput) -> Self {
        let mut fields: Vec<UnnamedField> = Vec::new();
        if let syn::Data::Struct(new_input) = input.data {
            let possibly_unnamed_fields = new_input.fields;
            if let syn::Fields::Unnamed(unnamed_fields) = possibly_unnamed_fields {
                for field in unnamed_fields.unnamed {
                    fields.push(UnnamedField::new(field));
                }
            } else {
                panic!(
                    "Only structs with unnamed fields may be parsed into StructWithUnnamedFields"
                );
            }
        } else {
            panic!("Only structs may be parsed into StructWithUnnamedFields");
        }
        Self {
            attributes: input.attrs,
            visibility: input.vis,
            ident: input.ident,
            generics: input.generics,
            fields,
        }
    }
}

/// Struct to represent a unit struct
pub struct UnitStruct {
    /// Vector of attributes
    pub attributes: Vec<Attribute>,
    /// Visibility of struct
    pub visibility: Visibility,
    /// Identifier of struct
    pub ident: Ident,
    /// Generic parameters of struct
    pub generics: Generics,
}

impl UnitStruct {
    /// Creates a new UnitStruct from a syn::DeriveInput
    ///
    /// # Panics
    /// Panics if the input is not a unit struct
    pub fn new(input: syn::DeriveInput) -> Self {
        Self {
            attributes: input.attrs,
            visibility: input.vis,
            ident: input.ident,
            generics: input.generics,
        }
    }
}
