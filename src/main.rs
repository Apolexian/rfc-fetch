extern crate clap;
use clap::{App, Arg};
use std::fs::File;
use std::io::copy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("RFC fetch")
        .version("0.1.0")
        .author("Ivan Nikitin <ivan@niktivan.org>")
        .about("Fetch and save RFCs")
        .arg(
            Arg::with_name("NUMBER")
                .short("n")
                .long("number")
                .value_name("NUMBER")
                .help("RFC number to fetch")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();
    let rfcs_to_fetch: Vec<&str> = matches.values_of("NUMBER").unwrap().collect();
    for rfc in rfcs_to_fetch {
        let url = format!("https://www.rfc-editor.org/rfc/rfc{rfc}.txt", rfc = rfc);
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        let path = format!("RFC{}.txt", rfc);
        let mut dest = File::create(path)?;
        copy(&mut content.as_bytes(), &mut dest)?;
        println!("Created file for RFC{}", rfc);
    }
    Ok(())
}
