// Copyright 2018 The Rio Advancement Inc
use base64;
use rio_core::crypto::{default_rioconfig_key_path, SigKeyPair};
use rio_core::crypto::keys::{PairConf, PairSaverExtn};
use protocol::api::secret::Secret;
use error::{Result, Error};
use protocol::api::base::MetaFields;

/// Security types
const OPAQUE: &'static str = "opaque";
const SSH_AUTH: &'static str = "rioos_sh/ssh-auth";
const SERVICE_ACCOUNT: &'static str = "rioos_sh/service-account-token";
const TOKEN: &'static str = "rioos_sh/token";
const TLS: &'static str = "rioos_sh/tls";
const DOCKERCFG: &'static str = "rioos_sh/dockercfg";
const DOCKERCFG_JSON: &'static str = "rioos_sh/dockerconfigjson";
const KRYPTONITE: &'static str = "rioos_sh/kryptonite";

/// SSH keys
pub const SSH_AUTH_PRIVATE_KEY: &'static str = "rioos_sh/ssh_privatekey";
pub const SSH_AUTH_PUBLIC_KEY: &'static str = "rioos_sh/ssh_pubkey";

#[derive(Debug, Eq, PartialEq)]
enum SecretType {
    SSH,
    COMMON,
    UNKNOWN,
}

impl SecretType {
    pub fn from_str(value: String) -> SecretType {
        match &value[..] {
            OPAQUE => SecretType::COMMON,
            SSH_AUTH => SecretType::SSH,
            SERVICE_ACCOUNT => SecretType::COMMON,
            TOKEN => SecretType::COMMON,
            TLS => SecretType::COMMON,
            DOCKERCFG => SecretType::COMMON,
            DOCKERCFG_JSON => SecretType::COMMON,
            KRYPTONITE => SecretType::COMMON,
            _ => SecretType::UNKNOWN,
        }
    }
}

pub fn parse_key(secret: &Secret) -> Result<Secret> {
    match SecretType::from_str(secret.get_secret_type()) {
        SecretType::SSH => generate_ssh(secret),
        SecretType::COMMON => Ok(secret.clone()),
        SecretType::UNKNOWN => Err(Error::UNKNOWSECRET),
    }
}

fn generate_ssh(secret: &Secret) -> Result<Secret> {
    let mut _secret = secret.clone();

    let pairs = SigKeyPair::mk_signed(
        &_secret.object_meta().name,
        PairConf::with_save(false, _secret.bit_size(), PairSaverExtn::PemX509),
        &default_rioconfig_key_path(None),
    )?;

    let mut data = _secret.get_data().clone();

    &data.insert(
        SSH_AUTH_PUBLIC_KEY.to_string(),
        base64::encode(&pairs.public()?),
    );

    &data.insert(
        SSH_AUTH_PRIVATE_KEY.to_string(),
        base64::encode(&pairs.secret()?),
    );

    _secret.set_data(data);

    Ok(_secret.clone())
}
