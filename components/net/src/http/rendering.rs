use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use util::errors::Bad;
use std::path::PathBuf;
use handlebars::{to_json, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use core::fs::{read_from_file, rioconfig_config_path};
use http::schema::ApiSchema;
use serde::Serialize;
use serde_json;
use serde_json::value::Map;
use std::str;
use chrono::Local;

lazy_static! {
    static  ref RIOSTATUS_TEMPLATE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("template/riostatus.hbs").to_str().unwrap());
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ResponseList<T> {
    api_version: String,
    kind: String,
    pub items: T,
}

/// Return an Modifier<Result> containing the body of a NetError and the appropriate HTTP response status
/// as json
/// This is used by BeforeMiddlerware and others where an error needs to be communicated as json.
pub fn render_json_error(err: &Bad, status: status::Status) -> IronError {
    let encoded = serde_json::to_string(err).unwrap();
    let headers = Header(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));

    IronError::new(err.clone(), (status, encoded, headers))
}

pub fn render_json<T: Serialize>(status: status::Status, response: &T) -> Response {
    let encoded = serde_json::to_string(response).unwrap();
    let headers = Header(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));

    Response::with((status, encoded, headers))
}

pub fn render_html<T: Serialize>(_status: status::Status, response: &T, _title: String) -> Response {
    let value = serde_json::to_value(response).unwrap();    
    let mut data = Map::new();
    let headers = Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("status_symbol", Box::new(status_helper));

    let date = Local::now();
    data.insert("time".to_string(), to_json(&date.to_rfc2822()));

    match read_from_file(&RIOSTATUS_TEMPLATE) {
        Ok(res) => {
            data.insert("packages".to_string(), to_json(&value["master"]));
            data.insert("nodes".to_string(), to_json(&value["nodes"]));
            let r = handlebars.render_template(&res, &data).unwrap();
            
            Response::with((status::Ok, r, headers))
        },
        Err(_err) => {
            let r = "<h3>Rio status template file not found</h3>";
            Response::with((status::Ok, r, headers))
        }
    }    
}

// define a custom helper
fn status_helper(
    h: &Helper,
    _: &Handlebars,
    _: &mut RenderContext,
    out: &mut Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = try!(
        h.param(0,)
            .ok_or(RenderError::new("Param 0 is required for format helper.",),)
    );   

    let mut rendered = format!("<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" width=\"90.594px\" height=\"59.714px\" viewBox=\"0 0 90.594 59.714\" enable-background=\"new 0 0 90.594 59.714\" xml:space=\"preserve\" class=\"icon\">
        <polyline class=\"up\" fill=\"none\" stroke=\"#666\" stroke-width=\"5\" stroke-miterlimit=\"10\" points=\"1.768,23.532 34.415,56.179 88.826,1.768\"/>
         </svg>");

    if param.value().render() != "up".to_string() {
        rendered = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 414.298 414.299\" class=\"icon\"><path d=\"M3.663 410.637c2.44 2.44 5.64 3.66 8.84 3.66 3.198 0 6.397-1.22 8.838-3.66l185.81-185.81 185.81 185.81c2.44 2.44 5.64 3.662 8.84 3.662 3.198 0 6.397-1.222 8.84-3.662 4.88-4.88 4.88-12.796 0-17.68L224.827 207.15l185.81-185.81c4.882-4.88 4.882-12.795 0-17.677-4.88-4.88-12.795-4.88-17.678 0L207.15 189.47 21.34 3.664c-4.882-4.882-12.796-4.882-17.678 0-4.882 4.88-4.882 12.796 0 17.678l185.81 185.81L3.662 392.96c-4.88 4.88-4.88 12.796 0 17.677z\"/></svg>");
    }
    
    out.write(rendered.as_ref())?;
    Ok(())
}

pub fn render_json_list<T>(status: status::Status, ident: ApiSchema, response: &T) -> Response
where
    T: Serialize,
{
    let encoded = serde_json::to_string(&ResponseList {
        api_version: ident.version,
        kind: ident.kind,
        items: response,
    }).unwrap();
    let headers = Header(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));
    Response::with((status, encoded, headers))
}
