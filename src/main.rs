use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

#[derive(Debug)]
enum DownloadError {
    HttpError(String),
    IoError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        }
    }
}

fn download_image(url: &str, index: usize) -> Result<(), DownloadError> {
    use std::fs::File;
    use std::io::{Read, Write};

    let response_result = ureq::get(url).call();

    match response_result {
        Ok(response) => {
            if response.status() == 200 {
                let mut reader = response.into_reader();
                let mut bytes: Vec<u8> = Vec::new();

                if let Err(e) = reader.read_to_end(&mut bytes) {
                    return Err(DownloadError::IoError(format!(
                        "Failed to read image data: {}",
                        e
                    )));
                }

                let filename = format!("dog_{}.jpg", index);
                let mut file = File::create(&filename)
                    .map_err(|e| DownloadError::IoError(format!("Failed to create {}: {}", filename, e)))?;
                file.write_all(&bytes)
                    .map_err(|e| DownloadError::IoError(format!("Failed to write {}: {}", filename, e)))?;

                println!("💾 Saved image as {}", filename);
                Ok(())
            } else {
                Err(DownloadError::HttpError(format!(
                    "HTTP error while downloading: {}",
                    response.status()
                )))
            }
        }
        Err(e) => Err(DownloadError::HttpError(format!(
            "Request failed: {}",
            e
        ))),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🐶 Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                match download_image(&dog_image.message, i as usize) {
                    Ok(()) => println!("✅ Downloaded image #{}", i),
                    Err(e) => println!("❌ Download error for image #{}: {:?}", i, e),
                }
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}

