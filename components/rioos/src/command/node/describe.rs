#[warn(unused_assignments)]
pub use error::{Error, Result};
use common::ui::UI;
use api_client::Client;
use protocol::api::node;
use super::super::common::condition_table;
use human_size::Size;
use protocol::api::base::MetaFields;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String, id: String) -> Result<()> {
    ui.begin(&format!("Constructing a {} node for you...", id))?;
    ui.br()?;

    let result = rio_client.node_describe(&token, &email, &id)?;
    let ips = vec!["ExternalIP", "InternalIP", "Hostname"];
    let mut addr = "".to_string();
    for ip in &ips {
        addr = check(result.get_status().get_addresses().to_vec().clone(), ip);
        if !addr.is_empty() {
            break;
        }
    }
    let conditions = result
        .get_status()
        .get_conditions()
        .iter()
        .map(|x| {
            vec![
                x.get_condition_type(),
                x.get_status(),
                x.get_reason(),
                x.get_last_transition_time(),
            ]
        })
        .collect::<Vec<_>>();

    let x = result
        .get_status()
        .get_capacity()
        .get("memory")
        .unwrap()
        .parse::<Size>()
        .unwrap()
        .into_bytes() -
        result
            .get_status()
            .get_allocatable()
            .get("memory")
            .unwrap()
            .parse::<Size>()
            .unwrap()
            .into_bytes();

    let sto = result
        .get_status()
        .get_capacity()
        .get("storage")
        .unwrap()
        .parse::<Size>()
        .unwrap()
        .into_bytes() -
        result
            .get_status()
            .get_allocatable()
            .get("storage")
            .unwrap()
            .parse::<Size>()
            .unwrap()
            .into_bytes();

    let cpu = vec![
        vec![
            "Cpu".to_string(),
            (result
                 .get_status()
                 .get_capacity()
                 .get("cpu")
                 .unwrap()
                 .parse::<u8>()
                 .unwrap() -
                 result
                     .get_status()
                     .get_allocatable()
                     .get("cpu")
                     .unwrap()
                     .parse::<u8>()
                     .unwrap())
                .to_string(),
            result
                .get_status()
                .get_allocatable()
                .get("cpu")
                .unwrap()
                .parse::<u8>()
                .unwrap()
                .to_string(),
            result
                .get_status()
                .get_capacity()
                .get("cpu")
                .unwrap()
                .parse::<u8>()
                .unwrap()
                .to_string(),
        ],
        vec![
            "Memory".to_string(),
            bytes_to_human(x),
            result
                .get_status()
                .get_allocatable()
                .get("memory")
                .unwrap()
                .to_string(),
            result
                .get_status()
                .get_capacity()
                .get("memory")
                .unwrap()
                .to_string(),
        ],
        vec![
            "Storage".to_string(),
            bytes_to_human(sto),
            result
                .get_status()
                .get_allocatable()
                .get("storage")
                .unwrap()
                .to_string(),
            result
                .get_status()
                .get_capacity()
                .get("storage")
                .unwrap()
                .to_string(),
        ],
    ];
    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
    ui.para(&format!("Name: {}", result.object_meta().name))?;
    ui.para(&format!("IP Address: {}", addr))?;
    ui.para(&format!(
        "Os image: {} - {}",
        result.get_status().get_node_info().get_os_image(),
        result.get_status().get_node_info().get_architecture()
    ))?;
    ui.para(
        &format!("Status: {}", result.get_status().get_phase()),
    )?;
    ui.para(&format!("Hrs Ago: {}", result.get_created_at()))?;
    ui.heading("Conditions")?;
    let title = row!["Type", "Status", "Reason", "LastTransitionTime"];
    condition_table(conditions.to_owned(), title);
    ui.br()?;

    ui.heading("Capacity")?;
    let title1 = row!["Resource", "Used", "Free", "Total"];
    condition_table(cpu, title1);
    ui.br()?;

    ui.para(
        "For more information on node: \
         https://bit.ly/rioos_sh_usersguide",
    )?;

    Ok(())
}

pub fn check(a: Vec<node::Addresses>, param: &str) -> String {
    let iter: String = a.iter()
        .take_while(|x| x.get_node_type().contains(param))
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.get_address().clone())
        .collect();
    iter
}

pub fn bytes_to_human(mut x: f64) -> String {
    let mut i = 0;
    while x > 1024.00 {
        i = i + 1;
        x = x / 1024.00;
    }
    let size: String;
    match i {
        0 => size = " B".to_string(),
        1 => size = " KB".to_string(),
        2 => size = " MiB".to_string(),
        3 => size = " GiB".to_string(),
        4 => size = " TiB".to_string(),
        _ => size = " PiB".to_string(),
    }
    format!("{:.2} {}", x, size)
}
