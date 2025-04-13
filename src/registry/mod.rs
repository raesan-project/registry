use crate::{cli, database, error, schema, tables};
use bon;
use color_eyre::eyre::{self, WrapErr};
use diesel::{self, prelude::*};
use r2d2;
use serde_json;
use std::{fs, io, path::Path};
use tracing;

#[bon::builder]
pub fn generate_database_records(
    gen_data: cli::GenerateDatabaseRecords,
) -> eyre::Result<(), error::Error> {
    let database = database::Database::builder()
        .database_url(gen_data.database)
        .build()?;
    let mut conn = database.pool.get().map_err(|e| {
        error::Error::DatabaseError(format!(
            "failed to create a connection from the database pool, error: {:#?}",
            e.to_string()
        ))
    })?;

    // classes
    let mut classes_json_string = String::new();
    let classes_json_file = format!("{}/classes.json", gen_data.registry);
    let classes_metadata = fs::metadata(&classes_json_file)?;
    if classes_metadata.is_file() {
        classes_json_string = fs::read_to_string(&classes_json_file)?;
    }
    diesel::insert_into(schema::classes::dsl::classes)
        .values(
            serde_json::from_str::<Vec<tables::Class>>(classes_json_string.as_str())
                .wrap_err("failed to parse string into JSON")?,
        )
        .execute(&mut conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;

    tracing::info!("Successfully created class records from the registry");

    // subjects
    let mut subjects_json_string = String::new();
    let subjects_json_file = format!("{}/subjects.json", gen_data.registry);
    let subjects_metadata = fs::metadata(&subjects_json_file)?;
    if subjects_metadata.is_file() {
        subjects_json_string = fs::read_to_string(&subjects_json_file)?;
    }
    diesel::insert_into(schema::subjects::dsl::subjects)
        .values(
            serde_json::from_str::<Vec<tables::Subject>>(subjects_json_string.as_str())
                .wrap_err("failed to parse string into JSON")?,
        )
        .execute(&mut conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;

    tracing::info!("Successfully created subject records from the registry");

    // chapters
    let chapters_dir = format!("{}/chapters", gen_data.registry);
    let chapters_path = Path::new(&chapters_dir);
    if chapters_path.is_dir() {
        let chapter_entries = fs::read_dir(chapters_path)?;
        for entry in chapter_entries {
            if let Ok(entry) = entry {
                let loop_conn = database.pool.get().map_err(|e| {
                    error::Error::DatabaseError(format!(
                        "failed to create a connection from the database pool, error: {:#?}",
                        e.to_string()
                    ))
                })?;
                insert_chapters(loop_conn, entry.path().to_string_lossy().to_string())?;
            }
        }
    } else {
        return Err(error::Error::InvalidInput(format!(
            "The provided path for generating database records of chapters table is not a directory, input: {}", 
            chapters_dir
        )))?;
    }

    tracing::info!("Successfully created chapter records from the registry");

    // questions
    let questions_dir = format!("{}/questions", gen_data.registry);
    let questions_path = Path::new(&questions_dir);
    if questions_path.is_dir() {
        let subject_entries = fs::read_dir(questions_path)?;
        for subject in subject_entries {
            let subject = subject?;
            let chapter_entries = fs::read_dir(subject.path())?;
            for chapter in chapter_entries {
                if let Ok(chapter) = chapter {
                    let loop_conn = database.pool.get().map_err(|e| {
                        error::Error::DatabaseError(format!(
                            "failed to create a connection from the database pool, error: {:#?}",
                            e.to_string()
                        ))
                    })?;
                    insert_questions(loop_conn, chapter.path().to_string_lossy().to_string())?;
                }
            }
        }
    } else {
        return Err(error::Error::InvalidInput(format!(
            "The provided path for generating database records of question table is not a directory, input: {}", 
            questions_dir
        )))?;
    }
    tracing::info!("Successfully created question records from the registry");

    return Ok(());
}

pub fn insert_chapters(
    mut conn: r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>,
    >,
    chapters_json_file: String,
) -> eyre::Result<(), error::Error> {
    let mut chapters_json_string = String::new();
    let chapters_metadata = fs::metadata(chapters_json_file.clone())?;
    if chapters_metadata.is_file() {
        chapters_json_string = fs::read_to_string(chapters_json_file.clone())?;
    };

    let chapters_json_vec =
        serde_json::from_str::<Vec<tables::Chapter>>(chapters_json_string.as_str())
            .wrap_err("failed to parse string into JSON")?;

    diesel::insert_into(schema::chapters::dsl::chapters)
        .values(chapters_json_vec.clone())
        .execute(&mut conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;

    return Ok(());
}

pub fn insert_questions(
    mut conn: r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>,
    >,
    questions_json_file: String,
) -> eyre::Result<(), error::Error> {
    let mut questions_json_string = String::new();
    let questions_metadata = fs::metadata(questions_json_file.clone())?;
    if questions_metadata.is_file() {
        match fs::read_to_string(questions_json_file.clone()) {
            Ok(safe_contents) => {
                questions_json_string = safe_contents.to_string();
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    return Ok(());
                } else {
                    return Err(eyre::eyre!(e.to_string()))?;
                }
            }
        }
    };

    if questions_json_string.trim().is_empty() {
        return Ok(());
    }

    let questions_json_vec =
        serde_json::from_str::<Vec<tables::Question>>(questions_json_string.as_str())
            .wrap_err("failed to parse string into JSON")?;

    diesel::insert_into(schema::questions::dsl::questions)
        .values(questions_json_vec.clone())
        .execute(&mut conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;

    return Ok(());
}
