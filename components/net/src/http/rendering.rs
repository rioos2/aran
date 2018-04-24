use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use util::errors::Bad;
use horrorshow::{Raw, RenderBox, Template};

use http::schema::ApiSchema;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::str;

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

pub fn render_html<T: Serialize>(status: status::Status, response: &T, title: String) -> Response {
    let data = serde_json::to_value(response).unwrap();
    let obj = data.as_object().unwrap();
    
    let markup = (html! {
        : Raw("<!DOCTYPE html>");
        html {
            head {
                title : title
            }
            body {      
               : Raw("<style>
                        table {
                            border-collapse: collapse;
                        }
                        th, td {
                            border: 1px solid #ccc;
                            padding: 10px;
                            text-align: left;
                        }
                        tr:nth-child(even) {
                            background-color: #eee;
                        }
                        tr:nth-child(odd) {
                            background-color: #fff;
                        }            
                </style>"); 
                h2(style = labels_sep_by!(";"; "color:DodgerBlue" => true, "font-weight: bold", "text-align: center")) :  "Rioos Packages status";
                h3(style = labels_sep_by!(";"; "color:MediumSeaGreen" => true, "font-weight: bold")) : "Master Packages Status"; 
                table {
                    tr {
                        th : "Package Name";
                        th : "Status";
                    }
                    @ for (key, value) in obj.iter() {
                        tr {
                            td : key;
                            td : format!("{}", match *value {
                                    Value::String(ref v) => format!("{}", v),
                                    _ => format!("")
                            });
                        }                            
                    }                    
                }
            }
        }
    }).into_string()
        .unwrap();
    let headers = Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    Response::with((status::Ok, markup, headers))
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
