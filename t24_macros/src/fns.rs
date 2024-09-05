use syn::*;

pub fn ident_enum_variants(input: &DeriveInput) -> Vec<Ident> {
    match input.data {
        Data::Enum(DataEnum { ref variants, .. }) => {
            variants.iter().map(|x| x.ident.clone()).collect::<Vec<_>>()
        }
        _ => panic!("this is not an instrument struct"),
    }
}

pub fn to_string_refs(strings: &Vec<String>) -> Vec<&str> {
    strings.iter().map(|x| x.as_ref()).collect::<Vec<_>>()
}
