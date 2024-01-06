use crate::*;

/// The `UnitField` struct has a public field `attributes` of type `Vec<Attribute>`.
///
/// Properties:
///
/// * `attributes`: A vector of Attribute structs.
pub struct UnitField {
    /// The attributes applied to this field.
    pub attributes: Vec<Attribute>,
}

impl UnitField {
    /// The function "new" creates a new instance of a struct with the given field attributes.
    ///
    /// Arguments:
    ///
    /// * `field`: The `field` parameter is of type `Field`.
    pub fn new(field: Field) -> Self {
        Self {
            attributes: field.attrs,
        }
    }
}

/// Represents an unnamed field in a struct or enum.
pub struct UnnamedField {
    /// The attributes applied to this field.
    pub attributes: Vec<Attribute>,

    /// The visibility of this field.  
    pub visibility: Visibility,

    /// The type of this field.
    pub field_type: Type,
}

impl UnnamedField {
    /// The function "new" creates a new instance of a struct with the given field attributes,
    /// visibility, and field type.
    ///
    /// Arguments:
    ///
    /// * `field`: The `field` parameter is of type `Field`.
    pub fn new(field: Field) -> Self {
        Self {
            attributes: field.attrs,
            visibility: field.vis,
            field_type: field.ty,
        }
    }
}

/// Represents a named field in a struct or enum.
pub struct NamedField {
    /// The attributes applied to this field.
    pub attributes: Vec<Attribute>,

    /// The visibility of this field.
    pub visibility: Visibility,

    /// The identifier name of this field.
    pub identifier: Ident,

    /// The type of this field.
    pub field_type: Type,
}

impl NamedField {
    /// Constructs a new NamedField from a syn::Field.
    ///
    /// # Arguments
    ///
    /// * `field` - The input syn::Field to construct the NamedField from.
    pub fn new(field: Field) -> Self {
        Self {
            attributes: field.attrs,
            visibility: field.vis,
            identifier: field.ident.expect("NamedField has no identifier!"),
            field_type: field.ty,
        }
    }
}
