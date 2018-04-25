use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use util::errors::Bad;
use horrorshow::{Raw, Template};

use http::schema::ApiSchema;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use serde_json::Value::Array;
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

pub fn render_html<T: Serialize>(_status: status::Status, response: &T, title: String) -> Response {
    let data = serde_json::to_value(response).unwrap();
    let master_obj = data["master"].as_object().unwrap();
    let external_obj = data["external"].as_object().unwrap();
    let support_obj = data["support"].as_object().unwrap();
    let v = Vec::new();
    let nodes = match data["master"]["nodes"] {
        Array(ref x) => x,
        _ => {            
            &v
        },
    };    
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
                h2(style = labels_sep_by!(";"; "color:DodgerBlue" => true, "font-weight: bold", "text-align: center")) :  "Rioos Product status";
                div(style = labels_sep_by!(";"; "width: 25%", "float:left")) {
                    h3(style = labels_sep_by!(";"; "color:MediumSeaGreen" => true, "font-weight: bold")) : "Master Packages Status"; 
                    table {
                        tr {
                            th : "Package Name";
                            th : "Status";
                        }
                        @ for (key, value) in master_obj.iter() {
                            @ if key != "nodes" {
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
                div(style = labels_sep_by!(";"; "width: 25%", "float:left")) {
                    h3(style = labels_sep_by!(";"; "color:MediumSeaGreen" => true, "font-weight: bold")) : "Supporting Packages Status"; 
                    table {
                        tr {
                            th : "Package Name";
                            th : "Status";
                        }
                        @ for (key, value) in support_obj.iter() {
                            @ if key != "nodes" {
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
                div(style = labels_sep_by!(";"; "width: 25%", "float:right")) {    
                    h3(style = labels_sep_by!(";"; "color:MediumSeaGreen" => true, "font-weight: bold")) : "Nodes(hosts) Status"; 
                    table {
                        tr {
                            th : "Node IP";
                            th : "Status";
                        }
                        @ if nodes.len() > 0 {
                            @ for node in nodes.iter() {
                                tr {
                                    td : format!("{}", match node["ip"] {
                                        Value::String(ref v) => format!("{}", v),
                                        _ => format!("")
                                    });
                                    td : format!("{}", match node["status"] {
                                        Value::String(ref v) => format!("{}", v),
                                        _ => format!("")
                                    });
                                }    
                            }
                        } else {
                            tr {
                                td(style = labels_sep_by!(";"; "color:red" => true, "font-weight: bold")) : "There are no node(host) systems connected."; 
                            }
                        }                                    
                    }
                }    
                div(style = labels_sep_by!(";"; "width: 25%", "float:right")) {
                    h3(style = labels_sep_by!(";"; "color:MediumSeaGreen" => true, "font-weight: bold")) : "External Packages Status"; 
                    table {
                        tr {
                            th : "Package Name";
                            th : "Status";
                        }
                        @ for (key, value) in external_obj.iter() {
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
