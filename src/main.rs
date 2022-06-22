use exitfailure::ExitFailure;
use reqwest::StatusCode;
use std::{fs::File, io::Write, thread};
use futures::executor::block_on;

async fn get(entry: i32) -> Result<String, ExitFailure> {
    let url = format!(
        "https://m6alkgx1qj.execute-api.us-west-2.amazonaws.com/prod/v1/{}",
        entry
    );

    loop {
        let resp = reqwest::get(url.clone())
        .await?;

        // println!("status: {}", temp.status());

        if resp.status() == StatusCode::OK {
            let text = resp.text().await?;
            return Ok(text);
        }
    }
}

async fn process_in_thread(thread_id: i32) -> Result<(), ExitFailure> {
    println!("thread {}", thread_id);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let mut threads = Vec::new();

    for thread_id in 0..5 {
        let thread = thread::spawn(move || block_on(process_in_thread(thread_id)));
        threads.push(thread);
    }
    
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    Ok(())
}