use crate::tables;
use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Exam {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub subjects: Vec<Subject>,
}

impl From<tables::Exam> for Exam {
    fn from(exam: tables::Exam) -> Self {
        Exam {
            id: exam.id,
            name: exam.name,
            subjects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub exam_id: String,
    pub name: String,
    #[serde(default)]
    pub chapters: Vec<Chapter>,
}

impl From<crate::tables::Subject> for Subject {
    fn from(subject: crate::tables::Subject) -> Self {
        Subject {
            id: subject.id,
            exam_id: subject.exam_id,
            name: subject.name,
            chapters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject_id: String,
    pub name: String,
    #[serde(default)]
    pub questions: Vec<Question>,
}

impl From<crate::tables::Chapter> for Chapter {
    fn from(chapter: crate::tables::Chapter) -> Self {
        Chapter {
            id: chapter.id,
            subject_id: chapter.subject_id,
            name: chapter.name,
            questions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Question {
    SingleMCQ(SingleMCQ),
    Numerical(Numerical),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQ {
    pub _type: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub body: String,
    pub options: Vec<SingleMCQOption>,
    pub answer: String,
}

impl From<crate::tables::SingleMCQ> for SingleMCQ {
    fn from(single_mcq: crate::tables::SingleMCQ) -> Self {
        SingleMCQ {
            _type: "single_mcq".to_string(),
            id: single_mcq.id,
            chapter_id: single_mcq.chapter_id,
            body: single_mcq.body,
            options: Vec::new(),
            answer: single_mcq.answer,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleMCQOption {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub single_mcq_id: String,
    pub key: String,
    pub value: String,
}

impl From<crate::tables::SingleMCQOption> for SingleMCQOption {
    fn from(single_mcq_option: crate::tables::SingleMCQOption) -> Self {
        SingleMCQOption {
            id: single_mcq_option.id,
            single_mcq_id: single_mcq_option.single_mcq_id,
            key: single_mcq_option.key,
            value: single_mcq_option.value,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Numerical {
    pub _type: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub body: String,
    pub answer: String,
}

impl From<crate::tables::Numerical> for Numerical {
    fn from(numerical: crate::tables::Numerical) -> Self {
        Numerical {
            _type: "numerical".to_string(),
            id: numerical.id,
            chapter_id: numerical.chapter_id,
            body: numerical.body,
            answer: numerical.answer,
        }
    }
}
