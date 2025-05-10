use crate::{cli, database, error, schema, tables};
use bon;
use color_eyre::eyre::{self, WrapErr};
use diesel::{self, prelude::*};
use serde_json;
use std::{fs, path};
use uuid;

pub mod reg_models;

#[bon::builder]
pub fn generate_database_records(
    gen_data: cli::GenerateDatabaseRecords,
) -> eyre::Result<(), error::Error> {
    let database = database::Database::builder()
        .database_url(gen_data.database)
        .build()
        .wrap_err("failed to create database object")?;
    let mut conn = database
        .pool
        .get()
        .wrap_err("failed to create a connection from the database pool")?;

    let registry = map_registry()
        .registry_path_string(gen_data.registry)
        .map_questions(true)
        .call()?;

    conn.immediate_transaction::<_, error::Error, _>(|conn| {
        for exam in registry {
            insert_exam(conn, exam).wrap_err("failed to insert exam records into the database")?;
        }
        Ok(())
    })
    .wrap_err("failed to commit an immediate transaction into the database")?;

    return Ok(());
}

// the "map_questions" here is used to differentiate between context of generating database records
// or getting metadata for serving questions in the registry server
#[bon::builder]
pub fn map_registry(
    registry_path_string: String,
    map_questions: bool,
) -> eyre::Result<Vec<reg_models::Exam>, error::Error> {
    let main_path = path::Path::new(&registry_path_string);
    if !main_path.is_dir() {
        return Err(error::Error::InvalidInput(format!(
            "the provided registry path is not a valid directory, path: {}",
            registry_path_string
        )))?;
    }
    let exam_entries = main_path.read_dir()?;

    let mut registry: Vec<reg_models::Exam> = Vec::new();

    // exam loop
    for exam_entry in exam_entries {
        let exam_entry = match exam_entry {
            Ok(entry) => entry,
            Err(e) => return Err(e)?,
        };

        let curr_exam = map_exam()
            .exam_entry(exam_entry)
            .map_questions(map_questions)
            .call()
            .wrap_err("failed to map exam data from the registry")?;
        registry.push(curr_exam);
    }
    return Ok(registry);
}

#[bon::builder]
pub fn map_exam(
    exam_entry: fs::DirEntry,
    map_questions: bool,
) -> eyre::Result<reg_models::Exam, error::Error> {
    let exam_entry_path = exam_entry.path();
    let exam_entry_path_string = exam_entry_path.to_string_lossy().to_string();
    if !exam_entry_path.is_dir() {
        return Err(error::Error::InvalidInput(format!(
            "the provided exam entry path is not a valid directory, path: {}",
            exam_entry_path_string
        )))?;
    }

    let curr_exam_contents = fs::read_to_string(format!("{}/_index.json", exam_entry_path_string))?;
    let curr_exam_json: serde_json::Value = serde_json::from_str(&curr_exam_contents)?;
    let mut curr_exam: reg_models::Exam = serde_json::from_value(
        curr_exam_json
            .get("_index")
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                    "failed to get _index field from the given JSON value, {:#?}",
                    curr_exam_json
                ))
            })?
            .clone(),
    )?;
    // at this point we have to add the ID fields manually because the registry contains basic structures
    curr_exam.id = uuid::Uuid::new_v4().to_string();

    // subject loop
    let subject_entries = curr_exam_json
        .get("_children")
        .ok_or_else(|| {
            error::Error::NotFound(format!(
                "failed to get _children field from the given JSON value, {:#?}",
                curr_exam_json
            ))
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::NotFound(format!(
                "failed to convert _children field into an array from the given JSON value, {:#?}",
                curr_exam_json
            ))
        })?;
    for subject_entry in subject_entries {
        let mut curr_subject = map_subject()
            .subject_entry(subject_entry)
            .exam_entry_path_string(exam_entry_path_string.clone())
            .map_questions(map_questions)
            .call()
            .wrap_err("failed to map subject data from the registry")?;

        // the exam_id is only accessible right here so we have to add it to the struct right here
        curr_subject.exam_id = curr_exam.id.clone();
        curr_exam.subjects.push(curr_subject);
    }

    return Ok(curr_exam);
}

#[bon::builder]
pub fn map_subject(
    subject_entry: &serde_json::Value,
    exam_entry_path_string: String,
    map_questions: bool,
) -> eyre::Result<reg_models::Subject, error::Error> {
    let subject_entry = match subject_entry {
        serde_json::Value::String(subject_string) => subject_string,
        _ => {
            return Err(error::Error::InvalidInput(format!(
                "the provided subject entry is not a valid path string, entry: {:#?}",
                subject_entry
            )))?;
        }
    };
    let subject_entry_path_string = format!("{}/{}", exam_entry_path_string, subject_entry.clone());
    let subject_entry_path = path::Path::new(&subject_entry_path_string);
    if !subject_entry_path.is_dir() {
        return Err(error::Error::InvalidInput(format!(
            "the provided subject entry path is not a valid directory, path: {}",
            subject_entry_path_string
        )))?;
    }
    let curr_subject_contents =
        fs::read_to_string(format!("{}/_index.json", subject_entry_path_string))?;
    let curr_subject_json: serde_json::Value = serde_json::from_str(&curr_subject_contents)?;
    let mut curr_subject: reg_models::Subject = serde_json::from_value(
        curr_subject_json
            .get("_index")
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                    "failed to get _children field from the given JSON value, {:#?}",
                    curr_subject_json
                ))
            })?
            .clone(),
    )?;
    // at this point we have to add the ID fields manually because the registry contains basic structures
    curr_subject.id = uuid::Uuid::new_v4().to_string();

    // subject loop
    let chapter_entries = curr_subject_json
        .get("_children")
        .ok_or_else(|| {
            error::Error::NotFound(format!(
                "failed to get _children field from the given JSON value, {:#?}",
                curr_subject_json
            ))
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::NotFound(format!(
                "failed to convert _children field into an array from the given JSON value, {:#?}",
                curr_subject_json
            ))
        })?;
    for chapter_entry in chapter_entries {
        let mut curr_chapter = map_chapter()
            .chapter_entry(chapter_entry)
            .subject_entry_path_string(subject_entry_path_string.clone())
            .map_questions(map_questions)
            .call()
            .wrap_err("failed to map chapter data from the registry")?;

        // the subject_id is only accessible right here so we have to add it to the struct right here
        curr_chapter.subject_id = curr_subject.id.clone();
        curr_subject.chapters.push(curr_chapter);
    }

    return Ok(curr_subject);
}

#[bon::builder]
pub fn map_chapter(
    chapter_entry: &serde_json::Value,
    subject_entry_path_string: String,
    map_questions: bool,
) -> eyre::Result<reg_models::Chapter, error::Error> {
    let chapter_entry = match chapter_entry {
        serde_json::Value::String(chapter_string) => chapter_string,
        _ => {
            return Err(error::Error::InvalidInput(format!(
                "the provided chapter entry is not a valid path string, entry: {:#?}",
                chapter_entry
            )))?;
        }
    };
    let chapter_entry_path_string =
        format!("{}/{}", subject_entry_path_string, chapter_entry.clone());
    let chapter_entry_path = path::Path::new(&chapter_entry_path_string);
    if !chapter_entry_path.is_file() {
        return Err(error::Error::InvalidInput(format!(
            "the provided chapter entry path is not a valid file, path: {}",
            chapter_entry_path_string
        )))?;
    }
    let curr_chapter_contents = fs::read_to_string(format!("{}", chapter_entry_path_string))?;
    let curr_chapter_json: serde_json::Value = serde_json::from_str(&curr_chapter_contents)?;
    let mut curr_chapter: reg_models::Chapter = serde_json::from_value(
        curr_chapter_json
            .get("_index")
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                    "failed to get _children field from the given JSON value, {:#?}",
                    curr_chapter_json
                ))
            })?
            .clone(),
    )?;
    // at this point we have to add the ID fields manually because the registry contains basic structures
    curr_chapter.id = uuid::Uuid::new_v4().to_string();

    // chapter loop
    if map_questions == true {
        let question_entries = curr_chapter_json
            .get("_children")
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                    "failed to get _children field from the given JSON value, {:#?}",
                    curr_chapter_json
                ))
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                "failed to convert _children field into an array from the given JSON value, {:#?}",
                curr_chapter_json
            ))
            })?;
        for question_entry in question_entries {
            let mut curr_question = map_question()
                .question_entry(question_entry)
                .call()
                .wrap_err("failed to map question data from the registry")?;

            // the chapter_id is only accessible right here so we have to add it to the struct right here
            match &mut curr_question {
                reg_models::Question::Numerical(curr_question) => {
                    curr_question.chapter_id = curr_chapter.id.clone();
                }
                reg_models::Question::SingleMCQ(curr_question) => {
                    curr_question.chapter_id = curr_chapter.id.clone();
                }
            };

            // curr_question. = .id.clone();
            curr_chapter.questions.push(curr_question);
        }
    }

    return Ok(curr_chapter);
}

#[bon::builder]
pub fn map_question(question_entry: &serde_json::Value) -> eyre::Result<reg_models::Question> {
    match question_entry {
        serde_json::Value::Object(_) => {}
        _ => {
            return Err(error::Error::InvalidInput(format!(
                "the provided question entry is not a valid JSON object, entry: {:#?}",
                question_entry
            )))?;
        }
    };

    // at this point we have to add the ID fields manually because the registry contains basic structures
    let question_id = uuid::Uuid::new_v4().to_string();
    let question_entry_type: String = serde_json::from_value(
        question_entry
            .clone()
            .get("_type")
            .ok_or_else(|| {
                error::Error::NotFound(format!(
                    "failed to get _type field from the given JSON value, {:#?}",
                    question_entry
                ))
            })?
            .clone(),
    )?;
    match question_entry_type.as_str() {
        "single_mcq" => {
            let mut curr_question: reg_models::SingleMCQ =
                serde_json::from_value(question_entry.clone())?;
            curr_question.id = question_id.clone();
            for mcq_option in &mut curr_question.options {
                // at this point we have to add the ID fields manually because the registry contains basic structures
                mcq_option.id = uuid::Uuid::new_v4().to_string();
                mcq_option.single_mcq_id = curr_question.id.clone();
            }
            return Ok(reg_models::Question::SingleMCQ(curr_question));
        }
        "numerical" => {
            let mut curr_question: reg_models::Numerical =
                serde_json::from_value(question_entry.clone())?;
            curr_question.id = question_id.clone();
            return Ok(reg_models::Question::Numerical(curr_question));
        }
        other => {
            return Err(error::Error::InvalidInput(format!(
                "invalid question type: {}",
                other
            )))?;
        }
    }
}

pub fn insert_exam(
    conn: &mut diesel::SqliteConnection,
    exam: reg_models::Exam,
) -> eyre::Result<(), error::Error> {
    let db_exam: tables::Exam = exam.clone().into();
    diesel::insert_into(schema::exams::table)
        .values(&db_exam)
        .execute(conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;
    for subject in exam.subjects {
        insert_subject(conn, subject)
            .wrap_err("failed to insert subject records into the database")?;
    }
    Ok(())
}

pub fn insert_subject(
    conn: &mut SqliteConnection,
    subject: reg_models::Subject,
) -> eyre::Result<(), error::Error> {
    let db_subject: tables::Subject = subject.clone().into();
    diesel::insert_into(schema::subjects::table)
        .values(&db_subject)
        .execute(conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;
    for chapter in subject.chapters {
        insert_chapter(conn, chapter)
            .wrap_err("failed to insert chapter records into the database")?;
    }
    Ok(())
}

pub fn insert_chapter(
    conn: &mut SqliteConnection,
    chapter: reg_models::Chapter,
) -> eyre::Result<(), error::Error> {
    let db_chapter: tables::Chapter = chapter.clone().into();
    diesel::insert_into(schema::chapters::table)
        .values(&db_chapter)
        .execute(conn)
        .map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to execute database query, error: {:#?}",
                e.to_string()
            ))
        })?;
    for question in chapter.questions {
        insert_question(conn, question)
            .wrap_err("failed to insert question records into the database")?;
    }
    Ok(())
}

pub fn insert_question(
    conn: &mut SqliteConnection,
    question: reg_models::Question,
) -> eyre::Result<(), error::Error> {
    match question {
        reg_models::Question::Numerical(numerical) => {
            let db_numerical: tables::Numerical = numerical.into();
            diesel::insert_into(schema::numericals::table)
                .values(&db_numerical)
                .execute(conn)
                .map_err(|e| {
                    error::Error::DatabaseError(format!(
                        "failed to execute database query, error: {:#?}",
                        e.to_string()
                    ))
                })?;
        }
        reg_models::Question::SingleMCQ(mcq) => {
            let db_mcq: tables::SingleMCQ = mcq.clone().into();
            diesel::insert_into(schema::single_mcqs::table)
                .values(&db_mcq)
                .execute(conn)
                .map_err(|e| {
                    error::Error::DatabaseError(format!(
                        "failed to execute database query, error: {:#?}",
                        e.to_string()
                    ))
                })?;

            let db_opts: Vec<tables::SingleMCQOption> =
                mcq.options.into_iter().map(Into::into).collect();
            diesel::insert_into(schema::single_mcq_options::table)
                .values(&db_opts)
                .execute(conn)
                .map_err(|e| {
                    error::Error::DatabaseError(format!(
                        "failed to execute database query, error: {:#?}",
                        e.to_string()
                    ))
                })?;
        }
    }
    Ok(())
}
