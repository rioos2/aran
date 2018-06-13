use api_client::Client;
use base64::decode;
use common::ui::UI;
pub use error::{Error, Result};
use protocol::api::base::{hours_ago, MetaFields};
const SSH_AUTH_PRIVATE_KEY: &'static str = "rioos_sh/ssh_privatekey";
const SSH_AUTH_PUBLIC_KEY: &'static str = "rioos_sh/ssh_pubkey";

pub fn start(
    ui: &mut UI,
    rio_client: Client,
    token: String,
    email: String,
    id: String,
) -> Result<()> {
    ui.begin(&format!("Constructing a {} secret for you...", id))?;
    ui.br()?;

    let result = rio_client.describe_secret(&token, &email, &id)?;

    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
    ui.para(&format!("Name: {}", result.object_meta().name))?;
    ui.para(&format!("Secret Type: {}", result.get_secret_type()))?;

    let x = "".to_string();
    let public = result.get_data().get(SSH_AUTH_PUBLIC_KEY).unwrap_or(&x);
    ui.para(&format!(
        "Public Key : {:?}",
        String::from_utf8(decode(public).unwrap()[..].to_vec()).unwrap()
    ))?;
    let private = result.get_data().get(SSH_AUTH_PRIVATE_KEY).unwrap_or(&x);

    ui.para(&format!(
        "Private Key : {:?}",
        String::from_utf8(decode(private).unwrap()[..].to_vec()).unwrap()
    ))?;

    ui.para(&format!("Hrs ago: {}", hours_ago(result.get_created_at())))?;

    ui.para(
        "For more information on datacenter: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}
