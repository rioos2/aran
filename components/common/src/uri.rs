// Copyright 2018 The Rio Advancement Inc
//

use regex::Regex;
use hyper::uri::RequestUri;

pub struct URI {
	url: String,
}

impl URI {
	pub fn new(uri: RequestUri) -> Self {
		let url = match uri {
                 	RequestUri::AbsolutePath(obj) => obj,
                    _ => {
                            println!("Invalid url");
                            "".to_string()
                        },
                   }; 
        return URI{
        	url: url
        };
	}

	pub fn id(self) -> String {
		let re = Regex::new("/(\\w+)/watch").expect("regex");
        let captures = re.captures(&self.url).expect("captures");
        let id: String = captures.get(1).expect("1").as_str().parse().expect("parse");
        return id;
	}
}