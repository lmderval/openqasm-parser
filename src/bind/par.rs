pub struct ParDec {
    name: String,
}

impl ToString for ParDec {
    fn to_string(&self) -> String {
        format!("{:p} ({})", self, self.name)
    }
}

impl ParDec {
    pub fn new(name: String) -> ParDec {
        ParDec { name: name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
