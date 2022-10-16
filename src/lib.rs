use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config};
use surf::Url;
use std::error::Error;
use serde_json::Value;
use std::fs::File;
use std::thread;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let client: Client = Config::new()
        .set_base_url(Url::parse("https://api-v3.mbta.com")?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;
        
    let mut count = 0;

    //loop {
        let mut res = client.get("/vehicles?filter[id]=G-10121,O-54731841,G-10063,R-54731200,B-5473183C,G-10026,G-10039").await?;
        let result = &res.body_json::<Value>().await?;
        /*
         * need to figure out how to make rust trust that this will always be a vector
        for train in result["data"] {
            write_file_end(match_id_to_file(train["id"]));
        }
        */
        count += 1;
        println!("Updated {} times", count);
        thread::sleep(Duration::from_secs(15));
    //}

    println!("the first train is {}", result["data"][0]["id"]);
    Ok(())
}

fn match_id_to_file(id: String) {
    unimplemented!();
}

fn write_file_end(file: File, data: String) {
    unimplemented!();
}
