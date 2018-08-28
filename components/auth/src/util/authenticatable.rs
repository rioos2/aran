// Copyright 2018 The Rio Advancement Inc

//Authenticatable enum helps for various of authentication types
//anyone authenticate their credentials then first create this enum type and pass via arguments on delegate functions
//Example :
//let delegate = AuthenticateDelegate::new(broker.clone());
//let auth_enum = Authenticatable::UserAndPass{
//                   username: "info@megam.io",
//                   password: "sdkjfhkj",
//                };
//let auth = delegate.authenticate(&auth_enum);
use rbac::authorizer::{AccountType, AccountNames};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Authenticatable {
    UserAndPass {
        username: String,
        password: String,
    },
    UserEmailAndToken {
        email: String,
        token: String,
    },
    UserEmailAndWebtoken {
        email: String,
        webtoken: String,
    },
    ServiceAccountNameAndWebtoken {
        name: String,
        webtoken: String,
        key: PathBuf,
    },
    PassTicket {
        token: String,
    },
}

pub trait ToAuth {
    fn to_auth(&self) -> Authenticatable;
}

impl ToAuth for Authenticatable {
    //to_auth method returns self value
    //it validates enum types correct or not
    fn to_auth(&self) -> Authenticatable {
        match *self {
            Authenticatable::UserAndPass {
                username: ref u,
                password: ref p,
            } => Authenticatable::UserAndPass {
                username: u.to_string(),
                password: p.to_string(),
            },
            Authenticatable::UserEmailAndToken {
                email: ref u,
                token: ref p,
            } => Authenticatable::UserEmailAndToken {
                email: u.to_string(),
                token: p.to_string(),
            },
            Authenticatable::UserEmailAndWebtoken {
                email: ref u,
                webtoken: ref p,
            } => Authenticatable::UserEmailAndWebtoken {
                email: u.to_string(),
                webtoken: p.to_string(),
            },
            Authenticatable::ServiceAccountNameAndWebtoken {
                name: ref u,
                webtoken: ref p,
                key: ref k,
            } => Authenticatable::ServiceAccountNameAndWebtoken {
                name: u.to_string(),
                webtoken: p.to_string(),
                key: k.to_path_buf(),
            },
            Authenticatable::PassTicket { token: ref t } => Authenticatable::PassTicket {
                token: t.to_string(),
            },
        }
    }
}

//convert the Authenticatable into TeamType
impl Into<AccountType> for Authenticatable {
    fn into(self) -> AccountType {
        match self {
             Authenticatable::UserAndPass {
                 username: ref u,
                 password: ref _p,
             } => AccountType::new(u.to_string(), AccountNames::USERACCOUNT),

            Authenticatable::ServiceAccountNameAndWebtoken {
                name: ref u,
                webtoken: ref _p,
                key: ref _k,
            } => AccountType::new(u.to_string(), AccountNames::SERVICEACCOUNT),

             Authenticatable::UserEmailAndToken {
                 email: ref u,
                 token: ref _p,
             } => AccountType::new(u.to_string(), AccountNames::USERACCOUNT),

             Authenticatable::UserEmailAndWebtoken {
                 email: ref u,
                 webtoken: ref _p,
             } => AccountType::new(u.to_string(), AccountNames::USERACCOUNT),

            _ => AccountType::new("".to_string(), AccountNames::NONE),
        }
    }
}
