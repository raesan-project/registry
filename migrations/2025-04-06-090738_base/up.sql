PRAGMA foreign_keys = ON;

CREATE TABLE classes (
	id TEXT PRIMARY KEY CONSTRAINT primary_key_classes NOT NULL,
	name INTEGER NOT NULL CONSTRAINT unique_classes_name UNIQUE,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL
);

CREATE TABLE subjects (
	id TEXT PRIMARY KEY CONSTRAINT primary_key_subjects NOT NULL,
	name TEXT NOT NULL,
	class_id TEXT NOT NULL,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	CONSTRAINT foreign_key_subjects_class FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
);

CREATE TABLE chapters (
	id TEXT PRIMARY KEY CONSTRAINT primary_key_chapters NOT NULL,
	name TEXT NOT NULL,
	subject_id TEXT NOT NULL,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	CONSTRAINT foreign_key_chapters_subject FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE questions (
	id TEXT PRIMARY KEY CONSTRAINT primary_key_questions NOT NULL,
	body TEXT NOT NULL CONSTRAINT unique_questions_body UNIQUE,
	chapter_id TEXT NOT NULL,
	created_at BIGINT NOT NULL,
	updated_at BIGINT NOT NULL,
	CONSTRAINT foreign_key_questions_chapter FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);

