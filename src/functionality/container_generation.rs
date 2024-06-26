use azure_core::auth::Secret;
use azure_storage::prelude::*;
use azure_storage::shared_access_signature::service_sas::BlobSharedAccessSignature;
use azure_storage_blobs::prelude::*;
use std::fs::File;
use std::io::Read;
use time::{Duration, OffsetDateTime};
use walkdir::WalkDir;

const ACCOUNT: &str = "wrapmycar";
pub async fn generate_and_upload(container_name: String) -> azure_core::Result<Vec<String>> {
    let access_key_string =
        std::fs::read_to_string("src/key.txt").expect("Failed to read access key from key.txt");
    let access_key_trimmed = access_key_string.trim().to_string();
    let storage_credentials = StorageCredentials::access_key(ACCOUNT, access_key_trimmed.clone());
    let client = ClientBuilder::new(ACCOUNT, storage_credentials);
    client
        .clone()
        .container_client(&container_name)
        .create()
        .public_access(PublicAccess::Blob)
        .await?;

    let base_folder = "src/base/";
    let mut image_file_names: Vec<_> = WalkDir::new(base_folder)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                Some(entry.path().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    image_file_names.sort_by(|a, b| natord::compare_ignore_case(a, b));

    let mut res: Vec<String> = vec![];
    for image_file_name in &image_file_names {
        println!("file {}", image_file_name);
        let mut file = File::open(&image_file_name).expect("Failed to open image file");
        let mut image_data = Vec::new();
        file.read_to_end(&mut image_data)
            .expect("Failed to read image file");
        let blob_name = image_file_name
            .trim_start_matches(base_folder)
            .trim_start_matches('/')
            .to_string();

        let blob_client = client.clone().blob_client(&container_name, &blob_name);

        blob_client
            .put_block_blob(image_data)
            .content_type("image/png")
            .await?;

        let resource: String = format!("/blob/{}/{}/{}", ACCOUNT, container_name, blob_name);
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let signed_token = BlobSharedAccessSignature::new(
            Secret::new(access_key_trimmed.clone()),
            resource,
            permissions,
            OffsetDateTime::now_utc() + Duration::days(30),
            BlobSignedResource::Blob,
        );

        let sas_token = blob_client
            .generate_signed_blob_url(&signed_token)
            .expect("Failed to generate SAS token");
        res.push(sas_token.to_string());
    }
    Ok(res)
}
