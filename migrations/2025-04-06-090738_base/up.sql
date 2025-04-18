PRAGMA foreign_keys = ON;

CREATE TABLE exams (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE subjects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE
);

CREATE TABLE chapters (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE question_answers (
    id TEXT PRIMARY KEY NOT NULL,
    chapter_id TEXT NOT NULL,
    body TEXT NOT NULL UNIQUE,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);

CREATE TABLE single_mcqs (
    id TEXT PRIMARY KEY NOT NULL,
    chapter_id TEXT NOT NULL,
    body TEXT NOT NULL UNIQUE,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);

CREATE TABLE single_mcq_options (
    id TEXT PRIMARY KEY NOT NULL,
    single_mcq_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (single_mcq_id) REFERENCES single_mcqs(id) ON DELETE CASCADE
);

