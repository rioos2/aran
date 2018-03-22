#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PassTicket {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the origin
    passticket: String, //Standard type metadata: kind: Origin
    #[serde(default)]
    created_at: String, //when origin created
}
impl PassTicket {
    pub fn new() -> PassTicket {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn set_passticket(&mut self, v: ::std::string::String) {
        self.passticket = v;
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }
}
