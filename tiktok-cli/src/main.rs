use anyhow::ensure;
use anyhow::Context;
use std::path::PathBuf;
use tiktok::Url;

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
    let client = tiktok::Client::new();

    let video_id = match Url::parse(&options.post) {
        Ok(post_url) => post_url
            .path_segments()
            .context("post url missing path")?
            .next_back()
            .context("missing video id")?
            .parse()?,
        Err(_error) => options.post.parse()?,
    };

    eprintln!("fetching feed...");
    let feed_cursor = client
        .get_feed(Some(video_id))
        .await
        .context("failed to get post")?;
    let post = feed_cursor.aweme_list.first().context("missing post")?;
    ensure!(post.aweme_id == video_id);

    let video_url = &post
        .video
        .download_addr
        .url_list
        .first()
        .context("missing download url")?;

    eprintln!("downloading video from \"{}\"", video_url.as_str());

    let file_name = format!("{video_id}.mp4");
    let out_path = options.out_dir.join(file_name);
    nd_util::download_to_path(&client.client, video_url.as_str(), out_path).await?;

    Ok(())
}
