use crate::*;

pub trait ToStrings {
    fn to_strings(&self) -> Vec<String>;
}

impl ToStrings for Vec<Ident> {
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>()
    }
}

impl ToStrings for Vec<Type> {
    fn to_strings(&self) -> Vec<String> {
        self.iter()
            .map(|x| x.into_token_stream().to_string())
            .collect::<Vec<_>>()
    }
}

impl ToStrings for Punctuated<LitStr, Token![,]> {
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|x| x.value()).collect::<Vec<_>>()
    }
}
