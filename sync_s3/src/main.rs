use aws_config::SdkConfig as Config;
use aws_sdk_s3::{
    operation::{get_object::GetObjectOutput, put_object::PutObjectOutput},
    primitives::ByteStream,
    Client,
};
use clap::Parser;
use std::{error::Error, str::Chars};
use tracing::{event, instrument, Level};
use tracing_subscriber;

#[derive(Parser, Debug)]
/// Command line arguments for the executable
pub struct Args {
    /// The name of the source s3 bucket. Follows standard s3://<bucket_name> syntax
    pub source: String,
    /// The name of the source s3 profile. Needs to be configured in your ~/.aws/config file
    pub src_profile: String,
    /// The name of the destination s3 bucket. Follows standard s3://<bucket_name> syntax
    pub destination: String,
    /// The name of the destination s3 profile. Needs to be configured in your ~/.aws/config file
    pub dest_profile: String,
}

#[derive(Debug, Clone)]
/// Parsed struct for s3 buckets passed to the program
pub struct Bucket {
    /// The name of the s3 bucket. Omits the s3:// part.
    pub name: String,
    /// The path added to the base name. Specifies the subfolder.
    pub path: Option<String>,
}

impl Bucket {
    #[instrument]
    /// Returns a new Bucket struct using the input provided
    /// 
    /// # Arguments
    /// 
    /// * `bucket_name`: `&str` A string slice for the full name of the s3 bucket
    /// 
    pub fn new(bucket_name: &str) -> Bucket {
        // remove s3 if present and split the first path into name, the rest goes to path
        let mut name: String = bucket_name.replace("s3://", "");
        let raw_path: String = name.split_off(name.find('/').unwrap_or_else(|| name.len()));
        let mut path: Option<String> = None;
        if raw_path.len() > 0 {
            let mut path_chars: Chars = raw_path.chars();
            path_chars.next();
            if let Option::Some(x) = path_chars.clone().last() {
                if x == '/' {
                    path_chars.next_back();
                }
            }
            path = Some(path_chars.as_str().to_string());
        }

        Bucket { name, path }
    }
}

#[instrument]
/// Returns a result containing a vector of Strings detailing the names of objects in a given s3 bucket
/// 
/// # Arguments
/// 
/// * `s3_client`: `&SdkClient` An S3 client connected to a profile
/// 
/// * `bucket`: `&Bucket` A fully constructed Bucket struct with the name of the desired S3 bucket and path
/// 
pub async fn list_bucket(
    s3_client: &Client,
    bucket: &Bucket,
) -> Result<Vec<String>, aws_sdk_s3::Error> {
    let mut last_key: Option<String> = Some(String::default());
    let mut curr_key: Option<String> = None;
    let mut items: Vec<String> = vec![];
    while last_key != curr_key {
        last_key = curr_key.clone();

        items.extend(
            s3_client
                .list_objects_v2()
                .bucket(&bucket.name)
                .set_prefix(bucket.path.clone())
                .set_start_after(curr_key.clone())
                .send()
                .await?
                .contents()
                .unwrap_or_else(|| &[])
                .into_iter()
                .map(|obj| {
                    obj.key()
                        .unwrap_or("")
                        .to_string()
                        .split_at(bucket.path.clone().unwrap_or("".into()).len())
                        .1
                        .replacen('/', "", 1)
                        .to_string()
                }),
        );

        if let Option::Some(x) = items.last() {
            curr_key = Some(x.to_owned());
        }
    }
    Ok(items)
}

#[instrument]
/// Returns a configured S3 client attached to the provided profile, wrapped in a result
/// 
/// # Arguments
/// 
/// * `profile_name`: `&str` A string slice containing the name of the profile to be used
/// 
pub async fn get_s3_client(profile_name: &str) -> Result<Client, aws_sdk_s3::Error> {
    let config: Config = aws_config::from_env()
        .profile_name(profile_name)
        .load()
        .await;
    Ok(Client::new(&config))
}

#[instrument]
/// Filters two lists of S3 objects to find ones missing in the destination S3 bucket. Returns a vector of strings wrapped in a result.
/// 
/// # Arguments
/// 
/// * `args`: `(&Bucket, &Client, &Bucket, &Client)` Connection arguments for the source and destination s3 buckets, including profiles
/// 
pub async fn get_missing_keys(
    args: (&Bucket, &Client, &Bucket, &Client),
) -> Result<Vec<String>, Box<dyn Error>> {
    let (source_bucket, source_client, destination_bucket, destination_client) = args;
    let source_keys: Vec<String> = list_bucket(source_client, source_bucket).await?;
    event!(Level::INFO, source_key_count = source_keys.len());
    let destination_keys: Vec<String> = list_bucket(destination_client, destination_bucket).await?;
    event!(Level::INFO, source_key_count = destination_keys.len());

    Ok(source_keys
        .into_iter()
        .filter_map(|x| {
            if destination_keys.contains(&x) == false {
                Some(x)
            } else {
                None
            }
        })
        .filter(|it| it.ne(""))
        .collect::<Vec<String>>())
}

#[instrument]
/// Retreives a given object from an S3 bucket as custom output struct wrapped in result
/// 
/// # Arguments
/// 
/// * `s3_client`: `&Client` A borrowed AWS client
/// 
/// * `bucket_name`: `&str` Name of the bucket that contains the object
/// 
/// * `key`: `&str` Name of the object to be retreived
/// 
pub async fn get_object(
    s3_client: &Client,
    bucket_name: &str,
    key: &str,
) -> Result<GetObjectOutput, aws_sdk_s3::Error> {
    event!(Level::INFO, key_name = key, "Getting object byte stream");
    let output = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;
    event!(
        Level::INFO,
        key_name = key,
        length = output.content_length(),
        "Retreived byte stream"
    );
    Ok(output)
}

#[instrument]
/// Uploads an object's ByteStream to a specified S3 location
/// 
/// # Arguments
/// 
/// * `s3_client`: `&Client` Borrowed S3 Client with profile that contains the target S3 bucket
/// 
/// * `bucket_name`: `&str` String slice of the desired S3 bucket
/// 
/// * `key`: `&str` String slice of the object's new name
/// 
/// * `body`: `ByteStream` ByteStream of data to be uploaded.
/// 
pub async fn put_object(
    s3_client: &Client,
    bucket_name: &str,
    key: &str,
    body: ByteStream,
) -> Result<PutObjectOutput, aws_sdk_s3::Error> {
    event!(Level::INFO, key_name = key, "Putting data in destination");
    let output = s3_client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .send()
        .await?;
    event!(Level::INFO, key_name = key, "Put object in destination");
    Ok(output)
}

#[instrument]
/// Formats a key and path in a usable way. Used primarily for upload processes
/// 
/// # Arguments
/// 
/// * `parts`: `[&Option<String>; 2]` An array containing the path and key name wrapped in an Option reference
/// 
pub fn format_key(parts: [&Option<String>; 2]) -> String {
    let filtered_parts: Vec<String> = parts
        .into_iter()
        .filter(|it| it.is_some())
        .map(|it| it.clone().unwrap())
        .filter(|it| it.ne(""))
        .collect();

    return if filtered_parts.len() > 1 {
        filtered_parts.join("/")
    } else {
        filtered_parts.get(0).unwrap().to_owned()
    };
}

#[instrument]
/// Connects to the S3 clients  using profiles, gets missing keys in destination, and uploads missing data to destination bucket
/// 
/// # Arguments
/// 
/// * `args`: `Args` Command line arguments with source and destination bucket and profiles
/// 
pub async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let source_client: Client = get_s3_client(&args.src_profile).await?;
    let destination_client: Client = get_s3_client(&args.dest_profile).await?;
    let source_bucket: Bucket = Bucket::new(&args.source);
    let destination_bucket: Bucket = Bucket::new(&args.destination);
    let missing_keys: Vec<String> = get_missing_keys((
        &source_bucket,
        &source_client,
        &destination_bucket,
        &destination_client,
    ))
    .await?;
    event!(Level::INFO, missing_keys_count = missing_keys.len());

    for missing_key in missing_keys.into_iter() {
        let get_key: String = format_key([&source_bucket.path, &Some(missing_key.clone())]);
        let put_key: String = format_key([&destination_bucket.path, &Some(missing_key.clone())]);
        let get_object_output: GetObjectOutput =
            get_object(&source_client, &source_bucket.name, &get_key).await?;
        let _put_object_output: PutObjectOutput = put_object(
            &destination_client,
            &destination_bucket.name,
            &put_key,
            get_object_output.body,
        )
        .await?;
    }

    Ok(())
}

#[tokio::main()]
/// Retreives command line arguments, sets logging defaults, and begins program
pub async fn main() {
    let args: Args = Args::parse();
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    match run(args).await {
        Ok(_) => event!(Level::INFO, "Successfully ran program"),
        Err(e) => {
            event!(Level::ERROR, error = e);
            panic!("An unrecoverable error was detected")
        }
    }
}
