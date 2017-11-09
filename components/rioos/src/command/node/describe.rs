pub use error::{Error, Result};

use common::ui::UI;
use api_client::Client;
use {PRODUCT, VERSION};
use protocol::nodesrv;
use super::super::common::condition_table;
use human_size::Size;



pub fn start(ui: &mut UI, url: &str, token: String, email: String, id: String) -> Result<()> {
    ui.begin(&format!("Constructing a {} node for you...", id))?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

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
    let cpu = vec![
        vec![
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
    ];
    let mut x = result
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



    let memory = vec![
        vec![
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
    ];

    let mut sto = result
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

    let storage = vec![
        vec![
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
    ui.para(&format!("IP Address: {}", addr))?;
    ui.para(&format!(
        "Node Os image: {} - {}",
        result.get_status().get_node_info().get_os_image(),
        result.get_status().get_node_info().get_architecture()
    ))?;
    ui.para(
        &format!("Status: {}", result.get_status().get_phase()),
    )?;
    ui.br()?;
    ui.heading("Status conditions")?;
    let title = row!["Condition Type", "Status", "Reason", "last Transition Time"];
    condition_table(conditions.to_owned(), title);
    ui.br()?;


    ui.heading("CPU :")?;
    let title1 = row!["Used", "Free", "Total"];
    condition_table(cpu, title1);
    ui.br()?;


    ui.heading("Memory:")?;
    let title2 = row!["Used", "Free", "Total"];
    condition_table(memory, title2);
    ui.br()?;


    ui.heading("Storage :")?;
    let title3 = row!["Used", "Free", "Total"];
    condition_table(storage, title3);
    ui.br()?;

    ui.para(&format!("Hrs Ago: {}", result.get_created_at()))?;

    ui.para(
        "For more information on digitalclouds node: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}

pub fn check(a: Vec<nodesrv::Addresses>, param: &str) -> String {
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
    let mut y = 0.00;
    while x > 1024.00 {
        i = i + 1;
        x = x / 1024.00;

    }
    let mut size = "".to_string();

    match i {
        0 => size = " B".to_string(),
        1 => size = " KB".to_string(),
        2 => size = " MiB".to_string(),
        3 => size = " GiB".to_string(),
        4 => size = " TiB".to_string(),
        _ => size = " PiB".to_string(),

    }
    format!("{} {}", x, size)
}
