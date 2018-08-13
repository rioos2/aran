// Copyright 2018 The Rio Advancement Inc

use api::base::IdGet;
use api::base::{TypeMeta, ObjectMeta, MetaFields, ChildTypeMeta};
use api::schema::type_meta_url;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct InvitationInputs {
    account_id: String,
    origin_id: String,   
    team_id: String, 
    users: Vec<String>,    
}

impl InvitationInputs {
    pub fn new() -> InvitationInputs {
        ::std::default::Default::default()
    }   

    pub fn set_account_id(&mut self, v: ::std::string::String) {
        self.account_id = v;
    }
    pub fn get_account_id(&self) -> ::std::string::String {
        self.account_id.clone()
    }

    pub fn set_origin_id(&mut self, v: ::std::string::String) {
        self.origin_id = v;
    }
    pub fn get_origin_id(&self) -> ::std::string::String {
        self.origin_id.clone()
    }

    pub fn set_team_id(&mut self, v: ::std::string::String) {
        self.team_id = v;
    }
    pub fn get_team_id(&self) -> ::std::string::String {
        self.team_id.clone()
    }

    pub fn set_users(&mut self, v: ::std::vec::Vec<String>) {
        self.users = v;
    }
    pub fn get_users(&self) -> ::std::vec::Vec<String> {
        self.users.clone()
    }
}

impl ChildTypeMeta for InvitationInputs {
    const CHILD_KIND: &'static str = "POST:invitations";
}

/// Build Invitations for each invited users
impl Into<InvitationsList> for Box<InvitationInputs> {
    fn into(self) -> InvitationsList {
        let users = self.get_users();
        let v: Vec<Invitations> = users.into_iter().map(|x| {
            let mut invites = Invitations::new();
            let m = invites.mut_meta(
                ObjectMeta::new(),
                "INVITATIONS".to_string(),
                self.get_account_id(),
            );
            let jackie = self.children();
            invites.set_meta(type_meta_url(jackie), m);
            invites.set_origin_id(self.get_origin_id());
            invites.set_team_id(self.get_team_id());
            invites.set_invite_from(self.get_account_id());
            invites.set_invite_to(x.to_string());
            invites
        }).collect();
        InvitationsList{
            invites: v
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct InvitationsList {
    pub invites: Vec<Invitations>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Invitations {
    #[serde(default)]
    id: String,
    invite_from: String,
    invite_to: String,
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Team
    object_meta: ObjectMeta, ////Standard object metadata
    team_id: String,
    origin_id: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    updated_at: String,
}

impl Invitations {
    pub fn new() -> Invitations {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> Invitations {
        Invitations {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_invite_from(&mut self, v: ::std::string::String) {
        self.invite_from = v;
    }

    pub fn get_invite_from(&self) -> ::std::string::String {
        self.invite_from.clone()
    }

    pub fn set_invite_to(&mut self, v: ::std::string::String) {
        self.invite_to = v;
    }

    pub fn get_invite_to(&self) -> ::std::string::String {
        self.invite_to.clone()
    }

    pub fn set_origin_id(&mut self, v: ::std::string::String) {
        self.origin_id = v;
    }

    pub fn get_origin_id(&self) -> ::std::string::String {
        self.origin_id.clone()
    }

    pub fn set_team_id(&mut self, v: ::std::string::String) {
        self.team_id = v;
    }

    pub fn get_team_id(&self) -> ::std::string::String {
        self.team_id.clone()
    }
   
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }    
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }
    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
    }
}

impl ChildTypeMeta for Invitations {
    const CHILD_KIND: &'static str = "POST:audits";
}

impl MetaFields for Invitations {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}