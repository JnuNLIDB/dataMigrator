mod schema;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::fmt::Write;

use backoff::future::retry;
use backoff::ExponentialBackoff;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Row};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::schema::Root;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(400)
        .connect(&url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS authors (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT UNIQUE NOT NULL,
        geography TEXT,
        title TEXT,
        identity TEXT,
        \"from\" TEXT,
        orob TEXT,
        orob_region TEXT
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS source (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT UNIQUE NOT NULL,
        geography TEXT,
        geopolitical TEXT,
        orob TEXT,
        orob_region TEXT,
        country TEXT,
        \"from\" TEXT
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS article (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        title TEXT UNIQUE NOT NULL,
        time INT NOT NULL
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS source_article (\
        source_id INT NOT NULL,
        article_id INT NOT NULL,
        CONSTRAINT fk_source
            FOREIGN KEY (source_id)
            REFERENCES source(id)
            ON DELETE CASCADE,
        CONSTRAINT fk_article
            FOREIGN KEY (article_id)
            REFERENCES article(id)
            ON DELETE CASCADE
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS opinion (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        author_id INT NOT NULL,
        start INT NOT NULL,
        \"end\" INT NOT NULL,
        text TEXT UNIQUE NOT NULL,
        article_id INT NOT NULL,
        CONSTRAINT fk_author
            FOREIGN KEY (author_id)
            REFERENCES authors(id)
            ON DELETE CASCADE,
        CONSTRAINT fk_article
            FOREIGN KEY (article_id)
            REFERENCES article(id)
            ON DELETE CASCADE
    );",
    )
    .execute(&pool)
    .await?;

    let path = Path::new("./report_2023_new.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let roots = serde_json::from_reader::<BufReader<File>, Vec<Root>>(reader).unwrap();

    let mut futs = FuturesUnordered::new();
    let pb = ProgressBar::new(roots.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for x in roots {
        let pool = pool.clone();
        let fut = async move {
            let mut source_ids = Vec::with_capacity(x.source.len());
            for source in x.source {
                if source.name.is_none() {
                    continue;
                }
                let id = retry(ExponentialBackoff::default(), || async {
                    Ok(
                        match sqlx::query(
                            "INSERT INTO source (name, geography, country, geopolitical, orob, orob_region, \"from\") \
                        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (name) DO NOTHING RETURNING id",
                        )
                            .bind(&source.name)
                            .bind(&source.geography)
                            .bind(&source.country)
                            .bind(&source.geography)
                            .bind(&source.orob)
                            .bind(&source.orob_region)
                            .bind(&source.get_from())
                            .fetch_one(&pool)
                            .await
                        {
                            Ok(row) => row.get::<i32, _>("id"),
                            Err(_e) => sqlx::query("SELECT id FROM source WHERE name = $1")
                                .bind(&source.name)
                                .fetch_one(&pool)
                                .await?
                                .get::<i32, _>("id"),
                        }
                    )
                }).await?;
                source_ids.push(id);
            }

            let update_time = match x.update_time {
                Some(time) => parse_time(time.as_str()).unwrap_or(0),
                None => 0,
            };

            let article_id = retry(ExponentialBackoff::default(), || async {
                Ok(
                    match sqlx::query(
                        "INSERT INTO article (title, time) \
                        VALUES ($1, $2) ON CONFLICT (title) DO NOTHING RETURNING id",
                    )
                    .bind(&x.headline)
                    .bind(update_time)
                    .fetch_one(&pool)
                    .await
                    {
                        Ok(row) => row.get::<i32, _>("id"),
                        Err(_e) => sqlx::query("SELECT id FROM article WHERE title = $1")
                            .bind(&x.headline)
                            .fetch_one(&pool)
                            .await?
                            .get::<i32, _>("id"),
                    },
                )
            })
            .await?;

            let author_id = retry(ExponentialBackoff::default(), || async {
                Ok(
                    match sqlx::query(
                        "INSERT INTO authors (name, title, geography, identity, \"from\", orob, orob_region) \
                        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (name) DO NOTHING RETURNING id",
                    )
                        .bind(&x.people.name)
                        .bind(&x.people.title)
                        .bind(&x.people.geography)
                        .bind(&x.people.get_identity())
                        .bind(&x.people.get_from())
                        .bind(&x.people.orob)
                        .bind(&x.people.orob_region)
                        .fetch_one(&pool)
                        .await
                    {
                        Ok(row) => row.get::<i32, _>("id"),
                        Err(_e) => sqlx::query("SELECT id FROM authors WHERE name = $1")
                            .bind(&x.people.name)
                            .fetch_one(&pool)
                            .await?
                            .get::<i32, _>("id"),
                    }
                )
            }).await?;

            for source_id in source_ids {
                retry(ExponentialBackoff::default(), || async {
                    Ok(sqlx::query(
                        "INSERT INTO source_article (source_id, article_id) \
                    VALUES ($1, $2) ON CONFLICT DO NOTHING",
                    )
                    .bind(&source_id)
                    .bind(&article_id)
                    .execute(&pool)
                    .await?)
                })
                .await?;
            }
            for op in x.people.opinion {
                retry(ExponentialBackoff::default(), || async {
                    Ok(sqlx::query(
                        "INSERT INTO opinion (author_id, score, start, \"end\", text, article_id) \
                                VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
                    )
                    .bind(&author_id)
                    .bind(&op.score)
                    .bind(&op.start)
                    .bind(&op.end)
                    .bind(&op.text)
                    .bind(&article_id)
                    .execute(&pool)
                    .await?)
                })
                .await?;
            }
            Ok::<(), Error>(())
        };
        futs.push(fut);
        pb.inc(1);
        if futs.len() >= 400 {
            futs.next().await.unwrap().unwrap();
        }
    }
    while let Some(_) = futs.next().await {}
    Ok(())
}

fn parse_time(time: &str) -> Option<i64> {
    Some(
        chrono::NaiveDate::parse_from_str(time, "%Y-%m-%d")
            .ok()?
            .and_hms_opt(0, 0, 0)?
            .timestamp(),
    )
}

// fn traverse_and_move(path: &Path, target: &Path) {
//     let mut entries = fs::read_dir(path).unwrap();
//     while let Some(entry) = entries.next() {
//         let entry_path = entry.unwrap().path();
//         if entry_path.is_dir() {
//             traverse_and_move(&entry_path, target);
//         } else {
//             let file_name = entry_path.file_name().unwrap().to_str().unwrap();
//             let mut target_path = target.join(file_name);
//             let mut i = 1;
//             while target_path.exists() {
//                 target_path = target.join(format!("{}-{}", i, file_name));
//                 i += 1;
//             }
//             fs::rename(entry_path, target_path).unwrap();
//         }
//     }
// }
