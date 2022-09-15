extern crate serde;
extern crate serde_json;

// standard library imports
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use reqwest::blocking::Client;
use serde_json::Value;

pub fn set_api_key(args: Vec<String>) {
    let mut keyfile = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(".creds.txt")
        .unwrap();

    keyfile.write_all(args[1].as_bytes())
        .expect("couldn't write the file");
}

pub fn get_api_key() {
    let key = fs::read_to_string(".creds.txt")
        .expect("couldn't read the file");

    println!("api-key: {key}");
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct JSONResponse {
   message: String,
   uuid: String,
   result: String,
   api: String,
   visibility: String,
   options: HashMap::<String, String>,
   url: String,
}

pub fn follow_url(args: String) {
    println!("url: {:#?}", args);

    let client = Client::new();
    let resp = client.get(args)
       .send();

    match resp {
        Ok(resp) => if resp.status() == 200 {
            let resp_text = resp.text().unwrap();
            let resp_json: Value = serde_json::from_str(&resp_text).unwrap();
            let resp_json = resp_json.as_object().unwrap();

            let malicious = resp_json["verdicts"]["overall"]["malicious"].as_str();
            
            match malicious.is_none() {
                false => println!("bad"),
                true => println!("good"),
                _ => todo!(),
            }
            println!("json as object: {:#?}", malicious);
        }
        else {
            println!("query failed: {:#?}", resp.status());
        }
        Err(error) => panic!("panic at da disco: {:#?}", error),
    };
}

pub fn query() {
    let mut query_data = HashMap::new();
    query_data.insert("url", "https://google.com");
    query_data.insert("visibility", "public");
    query_data.insert("tags", "test");

    // establish http client + send query
    let client = Client::new();
    let resp = client.post("https://urlscan.io/api/v1/scan/")
        .header("Content-Type", "application/json")
        .header("API-Key", "INSERT_API_KEY")
        .json(&query_data)
        .send();

    match resp {
        Ok(resp) => if resp.status() == 200 {
            println!("received {:#?} back", resp.status());
            let resp_json: JSONResponse = resp.json().unwrap();
            // println!("query located @ {:#?}", resp_json.result);
            let mut final_url = resp_json.result;
            final_url.insert_str(19, "api/v1/");
            println!("final url: {:#?}", final_url);
            follow_url(final_url);
        }
        else {
            println!("query failed... response code: {:?}", resp.status());
        }
        Err(error) => panic!("panic at da disco: {:?}", error),
    };
}
