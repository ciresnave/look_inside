use crate::{fields::*, *};

/// Represents an enum definition.
pub struct Enum {
    /// The attributes applied to the enum.
    pub attributes: Vec<Attribute>,

    /// The visibility of the enum.
    pub visibility: Visibility,

    /// The identifier name of the enum.
    pub ident: Ident,

    /// The generic parameters on the enum.
    pub generics: Generics,

    /// The variant cases of the enum.
    pub variants: Vec<Variant>,
}

impl Enum {
    /// Creates a new `Enum` struct from a `syn::DeriveInput`.
    ///
    /// Parses the variants from the input data if it is an enum.
    /// Panics if the input is not an enum.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse into an `Enum`.
    pub fn new(input: syn::DeriveInput) -> Self {
        let variants;
        if let syn::Data::Enum(new_input) = input.data {
            variants = Vec::from_iter(new_input.variants.into_iter());
        } else {
            panic!("Enum struct can only be used to parse enums!");
        }
        Self {
            attributes: input.attrs,
            visibility: input.vis,
            ident: input.ident,
            generics: input.generics,
            variants,
        }
    }
}

/// Represents a unit variant for an enum.
pub struct EnumUnitVariant {
    /// The attributes applied to the variant.
    pub attributes: Vec<Attribute>,

    /// The identifier name of the variant.
    pub identifier: Ident,
}

impl EnumUnitVariant {
    /// Creates a new `EnumUnitVariant` from the given `Variant`.
    ///
    /// Extracts the attributes and identifier from the variant.
    pub fn new(variant: Variant) -> Self {
        Self {
            attributes: variant.attrs,
            identifier: variant.ident,
        }
    }
}

/// Represents a variant of an enum that contains unnamed fields.  
///
/// Contains the variant's attributes, identifier name, and a
/// vector of the unnamed fields. Used when parsing an enum
/// variant to extract its definition into a structured form.
pub struct EnumVariantWithUnnamedFields {
    /// The attributes applied to the variant.
    pub attributes: Vec<Attribute>,
    /// The identifier name of the variant.
    pub identifier: Ident,
    /// A vector containing the unnamed fields of the variant.
    pub fields: Vec<UnnamedField>,
}

impl EnumVariantWithUnnamedFields {
    /// Creates a new `EnumVariantWithUnnamedFields` from the given `Variant`.
    ///
    /// Extracts the attributes, identifier, and unnamed fields from the variant.
    /// Initializes a new vector and populates it by iterating through the variant's
    /// fields and creating an `UnnamedField` for each one.
    pub fn new(variant: Variant) -> Self {
        let mut fields = Vec::new();
        for field in variant.fields {
            fields.push(UnnamedField::new(field))
        }
        Self {
            attributes: variant.attrs,
            identifier: variant.ident,
            fields,
        }
    }
}

/// Represents a variant of an enum that contains named fields.
///
/// Contains the variant's attributes, identifier name, and a
/// vector of the named fields. Used when parsing an enum
/// variant to extract its definition into a structured form.
pub struct EnumVariantWithNamedFields {
    /// The attributes applied to the variant.
    pub attributes: Vec<Attribute>,
    /// The identifier name of the variant.
    pub identifier: Ident,
    /// A vector containing the named fields of the variant.
    pub fields: Vec<NamedField>,
}

impl EnumVariantWithNamedFields {
    /// Creates a new `EnumVariantWithNamedFields` from the given `Variant`.
    ///
    /// Extracts the attributes, identifier, and named fields from the variant.
    /// Initializes a new vector and populates it by iterating through the variant's
    /// fields and creating a `NamedField` for each one.
    pub fn new(variant: Variant) -> Self {
        let mut fields = Vec::new();
        for field in variant.fields {
            fields.push(NamedField::new(field))
        }
        Self {
            attributes: variant.attrs,
            identifier: variant.ident,
            fields,
        }
    }
}

/// Represents a variant of an enum that contains unit fields.
///
/// Contains the variant's attributes, identifier name, and a
/// vector of the unit fields. Used when parsing an enum
/// variant to extract its definition into a structured form.
pub struct EnumVariantWithUnitFields {
    /// The attributes applied to the variant.
    pub attributes: Vec<Attribute>,
    /// The identifier name of the variant.
    pub identifier: Ident,
    /// A vector containing the unit fields of the variant.
    pub fields: Vec<UnitField>,
}

impl EnumVariantWithUnitFields {
    /// Creates a new `EnumVariantWithUnitFields` from the given `Variant`.
    ///
    /// Extracts the attributes, identifier, and unit fields from the variant.
    /// Initializes a new vector and populates it by iterating through the variant's
    /// fields and creating a `UnitField` for each one.
    pub fn new(variant: Variant) -> Self {
        let mut fields = Vec::new();
        for field in variant.fields {
            fields.push(UnitField::new(field))
        }
        Self {
            attributes: variant.attrs,
            identifier: variant.ident,
            fields,
        }
    }
}
