pub trait ToLowerCase {
    fn to_lowercase(&self) -> Vec<String>;
}

impl ToLowerCase for Vec<String> {
    fn to_lowercase(&self) -> Vec<String> {
        self.iter().map(|x| x.to_lowercase()).collect::<Vec<_>>()
    }
}
