/// Download a file with a checksum
/// 
/// This library has one function, and it's used like this:
/// 
/// ```ignore
/// let client = reqwest::Client::new();
/// 
/// let shasum = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8";
/// let file = "/tmp/foo";
/// let url = "http://example.com/foo";
/// 
/// on_the_dl::download(client, url, file, shasum).await?;
/// ```
/// 
/// If the file already exists, and the sha256 matches, then nothing will
/// happen. If the file doesn't exist, or the sha256 doesn't match, then it will
/// download the file, put it at that location, and check that the sha256
/// matches.

#[derive(Debug)]
pub struct Error;

pub async fn download(client: reqwest::Client, url: &str, file: &str, shasum: &str) -> Result<(), Error> {
    Ok(())
}
