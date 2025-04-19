use crate::{registry::reg_models, schema};
use diesel;
use serde;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=schema::exams)]
pub struct Exam {
    pub id: String,
    pub name: String,
}

impl From<reg_models::Exam> for Exam {
    fn from(exam: reg_models::Exam) -> Self {
        Exam {
            id: exam.id,
            name: exam.name,
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Exam))]
#[diesel(table_name=schema::subjects)]
pub struct Subject {
    pub id: String,
    pub exam_id: String,
    pub name: String,
}

impl From<reg_models::Subject> for Subject {
    fn from(subject: reg_models::Subject) -> Self {
        Subject {
            id: subject.id,
            exam_id: subject.exam_id,
            name: subject.name,
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Subject))]
#[diesel(table_name=schema::chapters)]
pub struct Chapter {
    pub id: String,
    pub subject_id: String,
    pub name: String,
}

impl From<reg_models::Chapter> for Chapter {
    fn from(chapter: reg_models::Chapter) -> Self {
        Chapter {
            id: chapter.id,
            subject_id: chapter.subject_id,
            name: chapter.name,
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Chapter))]
#[diesel(table_name=schema::question_answers)]
pub struct QuestionAnswer {
    pub id: String,
    pub chapter_id: String,
    pub body: String,
}

impl From<reg_models::QuestionAnswer> for QuestionAnswer {
    fn from(question_answer: reg_models::QuestionAnswer) -> Self {
        QuestionAnswer {
            id: question_answer.id,
            chapter_id: question_answer.chapter_id,
            body: question_answer.body,
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Chapter))]
#[diesel(table_name=schema::single_mcqs)]
pub struct SingleMCQ {
    pub id: String,
    pub chapter_id: String,
    pub body: String,
}

impl From<reg_models::SingleMCQ> for SingleMCQ {
    fn from(single_mcq: reg_models::SingleMCQ) -> Self {
        SingleMCQ {
            id: single_mcq.id,
            chapter_id: single_mcq.chapter_id,
            body: single_mcq.body,
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(SingleMCQ, foreign_key = single_mcq_id ))]
#[diesel(table_name=schema::single_mcq_options)]
pub struct SingleMCQOption {
    pub id: String,
    pub single_mcq_id: String,
    pub key: String,
    pub value: String,
}

impl From<reg_models::SingleMCQOption> for SingleMCQOption {
    fn from(single_mcq_option: reg_models::SingleMCQOption) -> Self {
        SingleMCQOption {
            id: single_mcq_option.id,
            single_mcq_id: single_mcq_option.single_mcq_id,
            key: single_mcq_option.key,
            value: single_mcq_option.value,
        }
    }
}
