use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config};
use surf::Url;
use std::error::Error;
use serde_json::Value;
use std::fs::File;
use std::thread;

const FILE_PATH: &str = "lines.json";

pub async fn run() -> Result<(), Box<dyn Error>> {
    let client: Client = Config::new()
        .set_base_url(Url::parse("https://api-v3.mbta.com")?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;
        
    let mut count = 0;

    //loop {
        let mut res = client.get("/vehicles?filter[route_type]=0,1").await?;
        let result = &mut res.body_json::<Value>().await?;
        // need to figure out how to make rust trust that this will always be a vector
        for train in result["data"].as_array_mut().expect("aaa") {
            let index = match train["relationships"]["route"]["data"]["id"] {
                "Green-A" => 0,
                "Green-B" => 1,
                "Green-C" => 2,
                "Green-D" => 3,
                "Green-E" => 4,
                "Red" => 5,
                "Orange" => 6,
                "Mattapan" => 7,
                "Blue" => 8,
            };


            //println!("train: {:?}", train);
            //let str = "";
            //write_file_end(0 , &str);
        }
        count += 1;
        println!("Updated {} times", count);
        //thread::sleep(Duration::from_secs(15));
    //}

    //println!("the first train is {}", result["data"][0]["id"]);
    //println!("{}", result);
    Ok(())
}

// Matches train ID to an element in the json list
fn match_id(id: String) -> u8 {
    unimplemented!();
}

fn write_file_end(index: u8, data: &str) {
    unimplemented!();
}
