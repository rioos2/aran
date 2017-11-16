pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;
use {PRODUCT, VERSION};

use super::super::common::pretty_table;

pub fn start(ui: &mut UI, url: &str, token: String, email: String, name: String) -> Result<()> {
    ui.begin(&format!(
        "Constructing a {} digitalcloud for you...",
        name
    ))?;
    ui.br()?;

    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    let result = rio_client.describe_deploy(&token, &email, &name)?;

    ui.heading("OverView")?;
    ui.para(&format!("Id: {}", result.get_id()))?;
    ui.para(&format!("Name: {}", result.get_name()))?;
    ui.para(&format!(
        "Replicas: {}",
        result.get_replicas().to_string()
    ))?;
    ui.para(
        &format!("Status: {}", result.get_status().get_phase()),
    )?;
    ui.para(&format!(
        "Located: {}",
        result.get_properties().clone().get_region()
    ))?;
    ui.para(&format!("Origin: {}", result.get_origin()))?;
    ui.para(&format!("Hrs ago: {}", result.get_created_at()))?;


    let hs_result = rio_client.get_hs_by_asmfac_id(
        &token,
        &email,
        &result.get_id(),
    )?;

    ui.heading("Horizontal Scaling")?;
    ui.para(&format!("Id: {}", hs_result.get_id()))?;
    ui.para(&format!("Name: {}", hs_result.get_name()))?;
    ui.para(&format!(
        "Scale Type: {}",
        hs_result.get_scale_type().to_string()
    ))?;
    ui.para(&format!(
        "Representation Skew: {}",
        hs_result.get_representation_skew()
    ))?;
    ui.para(&format!("State: {}", hs_result.get_state()))?;
    ui.para(&format!("Origin: {}", hs_result.get_origin()))?;

    ui.para(&format!(
        "Min Replicas: {}",
        hs_result.get_spec().get_min_replicas()
    ))?;
    ui.para(&format!(
        "Max Replicas: {}",
        hs_result.get_spec().get_max_replicas()
    ))?;
    ui.para(&format!(
        "Last Scale Time: {}",
        hs_result.get_status().get_last_scale_time()
    ))?;
    ui.para(&format!(
        "Current Replicas: {}",
        hs_result.get_status().get_current_replicas()
    ))?;
    ui.para(&format!(
        "Desired Replicas: {}",
        hs_result.get_status().get_desired_replicas()
    ))?;

    ui.heading("Metric Resource")?;
    let resorce_title = row![
        "Name",
        "Min Target Value",
        "Max Target Value",
        "Scale Up By",
        "Scale Up Wait Time",
        "Scale Down By",
        "Scale Down Wait Time"
    ];
    let metric = hs_result
        .get_spec()
        .get_metrics()
        .iter_mut()
        .map(|x| {
            vec![
                x.get_metric_resource().get_name(),
                x.get_metric_resource().get_min_target_value(),
                x.get_metric_resource().get_max_target_value(),
                x.get_metric_resource()
                    .get_metric_time_spec()
                    .get_scale_up_by(),
                x.get_metric_resource()
                    .get_metric_time_spec()
                    .get_scale_up_wait_time(),
                x.get_metric_resource()
                    .get_metric_time_spec()
                    .get_scale_down_by(),
                x.get_metric_resource()
                    .get_metric_time_spec()
                    .get_scale_down_wait_time(),
            ]
        })
        .collect::<Vec<_>>();

    pretty_table(metric.to_owned(), resorce_title);

    let replicas = rio_client.get_assembly_by_id(
        &token,
        &email,
        &result.get_id(),
    )?;

    ui.heading("Replicas")?;
    let title = row!["Id", "Name", "Status", "Origin", "Hrs ago"];

    pretty_table(replicas.to_owned(), title);

    ui.br()?;

    ui.end(
        format!("{} records listed.", replicas.to_owned().len()),
    )?;

    ui.para(
        "For more information on digitalclouds deployments: \
        https://www.rioos.sh/docs/reference/deployment/",
    )?;

    Ok(())
}
