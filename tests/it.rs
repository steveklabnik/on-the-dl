use std::path::Path;

#[tokio::test]
async fn hello_world() {
    let client = reqwest::Client::new();

    let shasum = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8";
    let file = "secret";
    let url = "https://raw.githubusercontent.com/steveklabnik/on-the-dl/main/secret";

    on_the_dl::download(client, url, file, shasum).await.expect("download failed");

    assert!(Path::new(file).try_exists().expect("cannot determine if file exists"));
}
