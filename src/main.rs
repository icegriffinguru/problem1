use exitfailure::ExitFailure;
use std::{fs::File, io::Write};

async fn get(entry: i32) -> Result<String, ExitFailure> {
    let url = format!(
        "https://m6alkgx1qj.execute-api.us-west-2.amazonaws.com/prod/v1/{}",
        entry
    );

    let mut resp;
    loop {
        resp = reqwest::get(url.clone())
        .await?
        .text()
        .await?;

        if resp.len() > 0 {
            break;
        }
    }

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    for i in 0..5 {
        let file_name = format!("output-{}.txt", i + 1);
        let mut file = File::create(file_name)?;

        let mut output = String::new();
        for j in 0..100 {
            let entry = i * 100 + j + 1;
            let res = get(entry).await?;
            // println!("{:#?}", res);
            output.push_str(&res);
            output.push('\n');

            println!("Read line {}", entry);
        }
        // println!("{:#?}", output);
        let _ = file.write(output.as_bytes());
    }
    

    Ok(())
}