// Copyright 2018 The Rio Advancement Inc

use base64;
use error::{Error, Result};
use protocol::api::base::MetaFields;
use protocol::api::secret::Secret;
use rio_core::crypto::{default_rioconfig_key_path, SigKeyPair};
use rio_core::crypto::keys::{PairConf, PairSaverExtn};

/// Security types
const OPAQUE: &'static str = "opaque";
const SSH_X509: &'static str = "rioos_sh_ssh_x509";
const SSH_RSA: &'static str = "rioos_sh_ssh_rsa";
const SSH_DSA: &'static str = "rioos_sh_ssh_dsa";
const SSH_ED25519: &'static str = "rioos_sh_ssh_ed25519";
const KRYPTONITE: &'static str = "rioos_sh_kryptonite";
const SERVICE_ACCOUNT: &'static str = "rioos_sh_service_account_token";
const TOKEN: &'static str = "rioos_sh_token";
const TLS: &'static str = "rioos_sh_tls";
const DOCKERCFG: &'static str = "rioos_sh_dockercfg";
const DOCKERCFG_JSON: &'static str = "rioos_sh_dockerconfigjson";

/// SSH keys
const SSH_AUTH_PRIVATE_KEY: &'static str = "rioos_sh/ssh_privatekey";
const SSH_AUTH_PUBLIC_KEY: &'static str = "rioos_sh/ssh_pubkey";

#[derive(Debug, Eq, PartialEq)]
enum SecretType {
    RSA,
    DSA,
    X509,
    ED25519,
    COMMON,
    UNKNOWN,
}

impl SecretType {
    pub fn from_str(value: String) -> SecretType {
        match &value[..] {
            OPAQUE => SecretType::COMMON,
            SSH_RSA => SecretType::RSA,
            SSH_X509 => SecretType::X509,
            SERVICE_ACCOUNT => SecretType::COMMON,
            TOKEN => SecretType::COMMON,
            TLS => SecretType::COMMON,
            SSH_DSA => SecretType::DSA,
            SSH_ED25519 => SecretType::ED25519,
            DOCKERCFG => SecretType::COMMON,
            DOCKERCFG_JSON => SecretType::COMMON,
            KRYPTONITE => SecretType::COMMON,
            _ => SecretType::UNKNOWN,
        }
    }
}

pub fn parse_key(secret: &Secret) -> Result<Secret> {
    match SecretType::from_str(secret.get_secret_type()) {
        SecretType::X509 => generate_ssh(secret, secret.bit_size(), PairSaverExtn::PemX509),
        SecretType::RSA => generate_ssh(secret, secret.bit_size(), PairSaverExtn::PubRSA),
        SecretType::COMMON => Ok(secret.clone()),
        SecretType::DSA => generate_ssh(secret, secret.bit_size(), PairSaverExtn::DSA),
        SecretType::ED25519 => generate_ssh(secret, None, PairSaverExtn::ED25519),
        SecretType::UNKNOWN => Err(Error::UNKNOWSECRET),
    }
}

fn generate_ssh(secret: &Secret, bit_len: Option<u32>, extn: PairSaverExtn) -> Result<Secret> {
    let mut _secret = secret.clone();

    let pairs = SigKeyPair::mk_signed(
        &_secret.object_meta().name,
        PairConf::with_save(false, bit_len, extn),
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
