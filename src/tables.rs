use crate::schema;
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
