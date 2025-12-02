use anyhow::Context;
use anyhow::ensure;
use std::path::PathBuf;
use url::Url;

#[derive(argh::FromArgs)]
#[argh(description = "A small CLI to download tiktok videos")]
struct Options {
    #[argh(positional, description = "the post url")]
    pub post: String,

    #[argh(
        option,
        short = 'o',
        default = "PathBuf::from(\".\")",
        description = "the output directory"
    )]
    pub out_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    tokio_rt.block_on(async_main(options))?;

    Ok(())
}

async fn async_main(options: Options) -> anyhow::Result<()> {
    let client = tikwm::Client::new();

    let video_id: u64 = match Url::parse(&options.post) {
        Ok(post_url) => post_url
            .path_segments()
            .context("post url missing path")?
            .next_back()
            .context("missing video id")?
            .parse()?,
        Err(_error) => options.post.parse()?,
    };

    let file_name = format!("{video_id}.mp4");
    let out_path = options.out_dir.join(file_name);

    if tokio::fs::try_exists(&out_path).await? {
        eprintln!("file exists, skipping...");
        return Ok(());
    }

    eprintln!("creating video download...");
    let create_download_response = client
        .create_download_task(&options.post)
        .await
        .context("failed to create video download")?;
    ensure!(create_download_response.id == video_id);

    eprintln!("getting download task result...");
    let download_task_result = client
        .get_task_result(&create_download_response.task_id)
        .await
        .context("failed to create video download")?;

    eprintln!(
        "downloading video from \"{}\"",
        download_task_result.download_url.as_str()
    );
    nd_util::download_to_path(
        &client.client,
        download_task_result.download_url.as_str(),
        out_path,
    )
    .await?;

    Ok(())
}
