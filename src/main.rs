use exitfailure::ExitFailure;
use reqwest::StatusCode;
use std::{fs::{File, OpenOptions}, io::{Write, Read}, thread, path::Path};
use futures::executor::block_on;

async fn get(entry: i32) -> Result<String, ExitFailure> {
    let url = format!(
        "https://m6alkgx1qj.execute-api.us-west-2.amazonaws.com/prod/v2/{}",
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

fn count_lines(data: &String) -> Result<i32, ExitFailure> {
    let mut count = 0;
    for c in data.chars() {
        if c == '\n' {
            count += 1;
        }
    }

    Ok(count)
}

// async fn read_and_append(file: &mut File, entry: i32) -> Result<(), ExitFailure> {
//     let res = get(entry).await?;
//     // let _ = file.write_all(res.as_bytes());
//     // let _ = file.write_all(b"\n");

//     Ok(())
// }

async fn process_in_thread(thread_id: i32) -> Result<(), ExitFailure> {
    // println!("thread {}", thread_id);

    let file_path = format!("output-{}.txt", thread_id + 1);

    let mut data = String::new();

    let mut file = if Path::new(&file_path).exists() {
        OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap()
    } else {
        OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap()
    };

    let _ = file.read_to_string(&mut data).unwrap_or_default();

    let number_of_lines = count_lines(&data).unwrap();
    println!("Thread: {} === Number Of Lines: {}", thread_id, number_of_lines);

    for i in number_of_lines..100 {
        let entry = (thread_id - 1) * 100 + i;
        // read_and_append(&mut file, entry).await?;

        let res = get(entry).await?;
        let _ = file.write_all(res.as_bytes());
        let _ = file.write_all(b"\n");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let mut threads = Vec::new();

    for thread_id in 1..=5 {
        
        let thread = thread::spawn(move || block_on(process_in_thread(thread_id)));
        threads.push(thread);
    }
    
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    Ok(())
}