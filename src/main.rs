#![feature(result_option_inspect)]

mod schema;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::fmt::Write;

use backoff::future::retry;
use backoff::ExponentialBackoff;
use sqlx::{Error, Pool, Row, Sqlite};
use std::fs::File;

use std::io::BufReader;
use std::path::Path;

use crate::schema::Root;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::{error, warn};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_module("sqlx", log::LevelFilter::Error)
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(400)
        .connect("./data.db")
        .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS country (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            geography TEXT,
            belt_and_road BOOLEAN
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            country_id INTEGER,
            title TEXT,
            origin TEXT,
            identity TEXT,
            FOREIGN KEY (country_id) REFERENCES country(id)
        )",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS source (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT UNIQUE NOT NULL,
        country_id INTEGER,
        origin TEXT,
        FOREIGN KEY (country_id) REFERENCES country(id)
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS article (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT UNIQUE NOT NULL,
        time INTEGER NOT NULL
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS source_article (\
        source_id INTEGER NOT NULL,
        article_id INTEGER NOT NULL,
        UNIQUE (source_id, article_id),
        FOREIGN KEY (source_id) REFERENCES source(id),
        FOREIGN KEY (article_id) REFERENCES article(id)
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS opinion (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author_id INTEGER NOT NULL,
        text TEXT UNIQUE NOT NULL,
        article_id INTEGER NOT NULL,
        FOREIGN KEY (author_id) REFERENCES people(id),
        FOREIGN KEY (article_id) REFERENCES article(id)
    );",
    )
    .execute(&pool)
    .await?;

    // Iterate over file in data folder
    warn!("Start processing files in data folder");
    let data_dir = Path::new("./data_new");
    let iter = data_dir.read_dir().unwrap().filter_map(|entry| {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            Some(entry.path())
        } else {
            None
        }
    });
    for path in iter {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        warn!("Start processing file: {}", path.to_str().unwrap());
        let roots = serde_json::from_reader::<BufReader<File>, Vec<Root>>(reader).unwrap();
        process_roots(&pool, roots).await.unwrap();
    }
    Ok(())
}

async fn process_roots(pool: &Pool<Sqlite>, roots: Vec<Root>) -> Result<(), Error> {
    let mut futs = FuturesUnordered::new();
    let pb = ProgressBar::new(roots.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for x in roots {
        let pool = pool.clone();
        let pb = pb.clone();
        let fut = async move {
            let mut source_ids = Vec::with_capacity(x.source.len());
            for source in x.source {
                if source.name.is_none() {
                    continue;
                }
                let country_id = match source.country {
                    Some(_) => Some(
                        retry(ExponentialBackoff::default(), || async {
                            sqlx::query(
                                "INSERT OR IGNORE INTO country (name, geography, belt_and_road) \
                                        VALUES ($1, $2, $3)",
                            )
                                .bind(&source.country)
                                .bind(&source.geography)
                                .bind(source.orob.is_some())
                                .execute(&pool)
                                .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?;
                            Ok(
                                sqlx::query("SELECT id FROM country WHERE name = $1")
                                    .bind(&source.country)
                                    .fetch_one(&pool)
                                    .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?
                                    .get::<i32, _>("id")
                            )
                        }).await?
                    ),
                    None => None,
                };
                let id = retry(ExponentialBackoff::default(), || async {
                    sqlx::query(
                        "INSERT OR IGNORE INTO source (name, country_id, origin) \
                            VALUES ($1, $2, $3) ON CONFLICT (name) DO NOTHING RETURNING id",
                    )
                        .bind(&source.name)
                        .bind(country_id)
                        .bind(&source.get_from())
                        .execute(&pool)
                        .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?;
                    Ok(
                        sqlx::query("SELECT id FROM source WHERE name = $1")
                            .bind(&source.name)
                            .fetch_one(&pool)
                            .await
                            .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?
                            .get::<i32, _>("id")
                    )
                })
                .await?;
                source_ids.push(id);
            }

            let update_time = match x.update_time {
                Some(time) => parse_time(time.as_str()).unwrap_or(0),
                None => 0,
            };

            let article_id = retry(ExponentialBackoff::default(), || async {
                sqlx::query(
                    "INSERT OR IGNORE INTO article (title, time) \
                        VALUES ($1, $2)",
                )
                    .bind(&x.headline)
                    .bind(update_time)
                    .execute(&pool)
                    .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?;
                Ok(
                    sqlx::query("SELECT id FROM article WHERE title = $1")
                        .bind(&x.headline)
                        .fetch_one(&pool)
                        .await
                        .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?
                        .get::<i32, _>("id")
                )
            })
            .await?;

            let people_country_id = match x.people.country {
                Some(_) => Some(
                    retry(ExponentialBackoff::default(), || async {
                        sqlx::query(
                            "INSERT OR IGNORE INTO country (name, geography, belt_and_road) \
                                    VALUES ($1, $2, $3)",
                        )
                            .bind(&x.people.country)
                            .bind(&x.people.geography)
                            .bind(x.people.orob.is_some())
                            .execute(&pool)
                            .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?;
                        Ok(
                            sqlx::query("SELECT id FROM country WHERE name = $1")
                                .bind(&x.people.country)
                                .fetch_one(&pool)
                                .await
                                .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?
                                .get::<i32, _>("id")
                        )
                    })
                    .await?,
                ),
                None => None,
            };

            let author_id = retry(ExponentialBackoff::default(), || async {
                sqlx::query(
                    "INSERT OR IGNORE INTO people (name, country_id, origin, title, identity) \
                        VALUES ($1, $2, $3, $4, $5)",
                )
                    .bind(&x.people.name)
                    .bind(people_country_id)
                    .bind(&x.people.get_from())
                    .bind(&x.people.title)
                    .bind(&x.people.get_identity())
                    .execute(&pool)
                    .await.inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?;
                Ok(
                    sqlx::query("SELECT id FROM people WHERE name = $1")
                        .bind(&x.people.name)
                        .fetch_one(&pool)
                        .await
                        .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e))).unwrap()
                        .get::<i32, _>("id")
                )
            })
            .await?;

            for source_id in source_ids {
                retry(ExponentialBackoff::default(), || async {
                    Ok(sqlx::query(
                        "INSERT OR IGNORE INTO source_article (source_id, article_id) \
                    VALUES ($1, $2)",
                    )
                    .bind(source_id)
                    .bind(&article_id)
                    .execute(&pool)
                    .await
                    .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?)
                })
                .await?;
            }

            for op in x.people.opinion {
                retry(ExponentialBackoff::default(), || async {
                    let result = sqlx::query(
                        "INSERT OR IGNORE INTO opinion (author_id, text, article_id) \
                                VALUES ($1, $2, $3)",
                    )
                    .bind(&author_id)
                    .bind(&op.text)
                    .bind(&article_id)
                    .execute(&pool)
                    .await;
                    Ok(match result {
                        Err(e) => {
                            let error = e.as_database_error().unwrap();
                            match error.code().unwrap().as_ref() {
                                "54000" => Ok(()),
                                _ => Err(e),
                            }
                        }
                        Ok(_) => Ok(()),
                    }
                    .inspect_err(|e| pb.suspend(|| error!("1. {:?}", e)))?)
                })
                .await?;
            }
            pb.inc(1);
            Ok::<(), Error>(())
        };
        futs.push(fut);
        if !futs.is_empty() {
            futs.next().await.unwrap().unwrap();
        }
    }
    while (futs.next().await).is_some() {}
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
