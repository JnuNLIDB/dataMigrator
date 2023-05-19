use backoff::future::retry;
use backoff::ExponentialBackoff;
use chrono::SecondsFormat::Millis;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Error, Row};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use dotenv::dotenv;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "People")]
    pub people: People,
    #[serde(rename = "Index")]
    pub index: Option<String>,
    #[serde(rename = "Headline")]
    pub headline: Option<String>,
    #[serde(rename = "Source")]
    pub source: Vec<Source>,
    #[serde(rename = "Time")]
    pub time: String,
    pub topic: Option<String>,
    #[serde(rename = "Body")]
    pub body: Option<String>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Peo_State")]
    pub peo_state: Option<String>,
    #[serde(rename = "Org_State")]
    pub org_state: Option<String>,
    #[serde(rename = "Ori_State")]
    pub ori_state: Option<String>,
    #[serde(rename = "By_State")]
    pub by_state: Option<String>,
    #[serde(rename = "Peo_Expression_State")]
    pub peo_expression_state: Option<String>,
    #[serde(rename = "Headline_Classification_State")]
    pub headline_classification_state: Option<String>,
    #[serde(rename = "Keywords_State")]
    pub keywords_state: Option<String>,
    #[serde(rename = "Abstract_State")]
    pub abstract_state: Option<String>,
    #[serde(rename = "Topic_Sentence_State")]
    pub topic_sentence_state: Option<String>,
    #[serde(rename = "Topic_Sentence_Classification_State")]
    pub topic_sentence_classification_state: Option<String>,
    #[serde(rename = "Expression_Classification_State")]
    pub expression_classification_state: Option<String>,
    #[serde(rename = "Update_Time")]
    pub update_time: Option<String>,
    #[serde(rename = "Expresstion")]
    pub expresstion: Vec<Expresstion>,
    #[serde(rename = "Media_Id")]
    pub media_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People {
    #[serde(rename = "Geography")]
    pub geography: Value,
    #[serde(rename = "From_Tibet")]
    pub from_tibet: Value,
    #[serde(rename = "Identity_Entertainment")]
    pub identity_entertainment: Value,
    #[serde(rename = "Identity_Refugee")]
    pub identity_refugee: Value,
    #[serde(rename = "From_Uyghur")]
    pub from_uyghur: Value,
    #[serde(rename = "Twitter")]
    pub twitter: Value,
    #[serde(rename = "Youtube_Url")]
    pub youtube_url: Value,
    #[serde(rename = "Information")]
    pub information: Value,
    #[serde(rename = "Identity_Crime")]
    pub identity_crime: Value,
    #[serde(rename = "Fb_Shot")]
    pub fb_shot: Value,
    #[serde(rename = "From_Congressman_State")]
    pub from_congressman_state: Value,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "From_Media")]
    pub from_media: Value,
    #[serde(rename = "Twitter_Acc")]
    pub twitter_acc: Value,
    #[serde(rename = "From_Congressman_District")]
    pub from_congressman_district: Value,
    #[serde(rename = "Name_Clean")]
    pub name_clean: Option<String>,
    #[serde(rename = "From_Expert")]
    pub from_expert: Value,
    #[serde(rename = "From_Congressman")]
    pub from_congressman: Value,
    #[serde(rename = "From_Congressman_Period")]
    pub from_congressman_period: Option<String>,
    #[serde(rename = "Identity")]
    pub identity: Value,
    #[serde(rename = "Identity_Military")]
    pub identity_military: Value,
    #[serde(rename = "Fb")]
    pub fb: Value,
    #[serde(rename = "Orob_Region")]
    pub orob_region: Value,
    #[serde(rename = "Identity_Business")]
    pub identity_business: Value,
    #[serde(rename = "Identity_Expert")]
    pub identity_expert: Value,
    #[serde(rename = "Geopolitics")]
    pub geopolitics: Value,
    #[serde(rename = "Identity_Media")]
    pub identity_media: Value,
    #[serde(rename = "Identity_Religion")]
    pub identity_religion: Value,
    #[serde(rename = "Title")]
    pub title: Value,
    #[serde(rename = "From_Congressman_Party")]
    pub from_congressman_party: Value,
    #[serde(rename = "Identity_Activist")]
    pub identity_activist: Value,
    #[serde(rename = "From_Topic")]
    pub from_topic: Value,
    #[serde(rename = "Orob")]
    pub orob: Value,
    #[serde(rename = "Identity_Politician")]
    pub identity_politician: Value,
    #[serde(rename = "Expression")]
    pub expression: Value,
    #[serde(rename = "Identity_Judge")]
    pub identity_judge: Value,
    #[serde(rename = "Identity_Student")]
    pub identity_student: Value,
    #[serde(rename = "Note")]
    pub note: Value,
    #[serde(rename = "Country")]
    pub country: Value,
    #[serde(rename = "Wiki")]
    pub wiki: Value,
    #[serde(rename = "Id")]
    pub id: Value,
    #[serde(rename = "Identity_Terrorist")]
    pub identity_terrorist: Value,
    #[serde(rename = "Identity_Sports")]
    pub identity_sports: Value,
    #[serde(rename = "Identity_Lawyer")]
    pub identity_lawyer: Value,
    #[serde(rename = "Opinion")]
    pub opinion: Opinion,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opinion {
    pub score: f64,
    pub start: i64,
    pub end: i64,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "Geography")]
    pub geography: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Name_Clean")]
    pub name_clean: Option<String>,
    #[serde(rename = "Orob_Region")]
    pub orob_region: Option<String>,
    #[serde(rename = "Geopolitics")]
    pub geopolitics: Option<String>,
    #[serde(rename = "Orob")]
    pub orob: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalAgency {
    #[serde(rename = "Official Site")]
    pub official_site: Value,
    #[serde(rename = "Name(Clean)")]
    pub name_clean: Value,
    #[serde(rename = "Note")]
    pub note: Value,
    #[serde(rename = "Country")]
    pub country: Value,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expresstion {
    #[serde(rename = "Entity")]
    pub entity: Option<String>,
    #[serde(rename = "Score")]
    pub score: f64,
    #[serde(rename = "Opinion")]
    pub opinion: Vec<Opinion2>,
    #[serde(rename = "Start")]
    pub start: i64,
    #[serde(rename = "Entity_Type")]
    pub entity_type: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "End")]
    pub end: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opinion2 {
    pub score: f64,
    pub start: i64,
    pub end: i64,
    pub text: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS authors (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        name TEXT UNIQUE NOT NULL,
        title TEXT
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
        country TEXT
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS article (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        title TEXT UNIQUE NOT NULL,
        update_time INT NOT NULL,
        source_id INT NOT NULL,
        CONSTRAINT fk_source
            FOREIGN KEY (source_id)
            REFERENCES source(id)
            ON DELETE CASCADE
    );",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS opinion (
        id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        author_id INT NOT NULL,
        score REAL NOT NULL,
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

    let path = Path::new("./report_2020.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let roots = serde_json::from_reader::<BufReader<File>, Vec<Root>>(reader).unwrap();

    let mut futs = FuturesUnordered::new();

    for x in roots {
        let pool = pool.clone();
        let fut = async move {
            let source = x.source.iter().next().unwrap();
            if source.name.is_none() {
                return Ok::<(), Error>(())
            }
            let source_id = match sqlx::query(
                        "INSERT INTO source (name, geography, country, geopolitical, orob, orob_region) \
                        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (name) DO NOTHING RETURNING id",
                    )
                        .bind(&source.name)
                        .bind(&source.geography)
                        .bind(&source.country)
                        .bind(&source.geography)
                        .bind(&source.orob)
                        .bind(&source.orob_region)
                        .fetch_one(&pool)
                        .await {
                        Ok(row) => {
                            row.get::<i32, _>("id")
                        }
                        Err(e) => {
                            sqlx::query(
                                "SELECT id FROM source WHERE name = $1",
                            )
                                .bind(&source.name)
                                .fetch_one(&pool)
                                .await?
                                .get::<i32, _>("id")
                        }
                    };

            let update_time = parse_time(&x.time).unwrap_or(0);

            let article_id = match sqlx::query(
                        "INSERT INTO article (title, source_id, update_time) VALUES ($1, $2, $3) ON CONFLICT (title) DO NOTHING RETURNING id",
                    )
                        .bind(&x.headline)
                        .bind(source_id)
                        .bind(update_time)
                        .fetch_one(&pool)
                        .await {
                        Ok(row) => {row.get::<i32, _>("id")}
                        Err(e) => {
                            sqlx::query(
                                "SELECT id FROM article WHERE title = $1",
                            )
                                .bind(&x.headline)
                                .fetch_one(&pool)
                                .await?
                                .get::<i32, _>("id")
                        }
                    };

            for x in x.expresstion {
                let author_id =
                        match sqlx::query(
                            "INSERT INTO authors (name, title) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING RETURNING id",
                        )
                            .bind(&x.entity)
                            .bind(&x.title)
                            .fetch_one(&pool)
                            .await
                        {
                            Ok(row) => {row.get::<i32, _>("id")}
                            Err(e) => {
                                sqlx::query(
                                    "SELECT id FROM authors WHERE name = $1",
                                )
                                    .bind(&x.entity)
                                    .fetch_one(&pool)
                                    .await?
                                    .get::<i32, _>("id")
                            }
                        };

                for op in x.opinion {
                    let _ = sqlx::query(
                                "INSERT INTO opinion (author_id, score, start, \"end\", text, article_id) VALUES ($1, $2, $3, $4, $5, $6)",
                            )
                                .bind(&author_id)
                                .bind(&op.score)
                                .bind(&op.start)
                                .bind(&op.end)
                                .bind(&op.text)
                                .bind(&article_id)
                                .execute(&pool)
                                .await;
                }
            }

            Ok::<(), Error>(())
        };
        futs.push(fut);
        if futs.len() >= 16 {
            futs.next().await;
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
