use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
}

#[derive(Debug)]
enum DownloadResult {
    Good_img(String),  
    Error_img(String), 
    File_err(String),  
    Copy_err(String),  
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
        Err(e) => ApiResult::ApiError(format!("Request failed: {}", e)),
    }
}

fn download_dog_image(url: &str) -> DownloadResult {
    let response = match ureq::get(url).call() {
        Ok(r) => r,
        Err(e) => return DownloadResult::Error_img(format!("Request failed: {}", e)),
    };

    if response.status() != 200 {
        return DownloadResult::Error_img(format!("HTTP error: {}", response.status()));
    }

    let filename = match url.rsplit('/').next() {
        Some(seg) if !seg.is_empty() => seg,
        _ => return DownloadResult::File_err("Empty filename derived from URL".into()),
    };

    let path = Path::new("downloads").join(filename);

    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => return DownloadResult::File_err(format!("Failed to create file: {}", e)),
    };

    let mut reader = response.into_reader();
    if let Err(e) = copy(&mut reader, &mut file) {
        return DownloadResult::Copy_err(format!("Failed to write file: {}", e));
    }

    DownloadResult::Good_img(path.display().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                match download_dog_image(&dog_image.message) {
                    DownloadResult::Good_img(path) => {
                        println!("📥 Saved to: {}", path);
                    }
                    DownloadResult::Error_img(e) => {
                        println!("❌ Image/HTTP error: {}", e);
                    }
                    DownloadResult::File_err(e) => {
                        println!("❌ File error: {}", e);
                    }
                    DownloadResult::Copy_err(e) => {
                        println!("❌ Copy error: {}", e);
                    }
                }
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
        }
        println!();
    }

    Ok(())
}
