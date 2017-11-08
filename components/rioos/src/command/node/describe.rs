pub use error::{Error, Result};

use common::ui::UI;
use api_client::{Client};
use {PRODUCT, VERSION};

pub fn start(ui: &mut UI, url: &str, token: String, email: String, id: String) -> Result<()> {
    ui.begin(&format!(
        "Constructing a {} node for you...",
        id
    ))?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let result = rio_client.node_describe(&token, &email, &id)?;

    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
//     let ip_addr: String = result.get_status().get_addresses().iter().map(|x| {
//     let data = match &x.get_node_type() {
//     "EnternalIP" =>  &x.get_address(),
//     "InternalIP" =>  &x.get_address(),
//     "Hostname" => &x.get_address(),
// };
// return data.to_string()
//     // else if x.get_node_type().contains("InternalIP") {
//     //     return x.get_address()
//     // }
//     // else if x.get_node_type().contains("Hostname") {
//     //     return x.get_address()
//     // }
//     //     "".to_string()
// }).collect();
// ui.para(&format!("Ip Address: {}", ip_addr))?;
ui.para(&format!("Node Os image: {} - {}",result.get_status().get_node_info().get_os_image(),result.get_status().get_node_info().get_architecture()))?;

ui.para(&format!("Status: {}",result.get_status().get_phase()))?;
ui.heading("Capacity:")?;
ui.para(&format!("      cpu: {}",result.get_status().get_capacity().get("cpu").unwrap()))?;
ui.para(&format!("      memory: {}",result.get_status().get_capacity().get("memory").unwrap()))?;
ui.para(&format!("      storage: {}",result.get_status().get_capacity().get("storage").unwrap()))?;

    ui.para(
        "For more information on digitalclouds node: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
