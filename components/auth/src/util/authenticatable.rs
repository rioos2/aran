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
pub enum Authenticatable<'a> {
    UserAndPass {
        username: &'a str,
        password: &'a str,
    },
    UserEmailAndToken { email: &'a str, token: &'a str },
    UserEmailAndWebtoken { email: &'a str, webtoken: &'a str },
    ServiceAccountNameAndWebtoken {
        name: &'a str,
        webtoken: &'a str,
        key: &'a str,
    },
    OtpAuth { token: &'a str },
}

pub trait ToAuth {
    fn to_auth(&self) -> Authenticatable;
}

impl<'a> ToAuth for Authenticatable<'a> {
    //to_auth method returns self value
    //it validates enum types correct or not
    fn to_auth(&self) -> Authenticatable {
        match *self {
            Authenticatable::UserAndPass {
                username: u,
                password: p,
            } => Authenticatable::UserAndPass {
                username: u,
                password: p,
            },
            Authenticatable::UserEmailAndToken { email: u, token: p } => Authenticatable::UserEmailAndToken { email: u, token: p },
            Authenticatable::UserEmailAndWebtoken {
                email: u,
                webtoken: p,
            } => Authenticatable::UserEmailAndWebtoken {
                email: u,
                webtoken: p,
            },
            Authenticatable::ServiceAccountNameAndWebtoken {
                name: u,
                webtoken: p,
                key: k,
            } => Authenticatable::ServiceAccountNameAndWebtoken {
                name: u,
                webtoken: p,
                key: k,
            },
            Authenticatable::OtpAuth { token: t } => Authenticatable::OtpAuth { token: t },
        }
    }
}
