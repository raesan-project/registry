// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Text,
        name -> Text,
        subject_id -> Text,
    }
}

diesel::table! {
    exams (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    numericals (id) {
        id -> Text,
        chapter_id -> Text,
        body -> Text,
        answer -> Text,
    }
}

diesel::table! {
    single_mcq_options (id) {
        id -> Text,
        single_mcq_id -> Text,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    single_mcqs (id) {
        id -> Text,
        chapter_id -> Text,
        body -> Text,
        answer -> Text,
    }
}

diesel::table! {
    subjects (id) {
        id -> Text,
        name -> Text,
        exam_id -> Text,
    }
}

diesel::joinable!(chapters -> subjects (subject_id));
diesel::joinable!(numericals -> chapters (chapter_id));
diesel::joinable!(single_mcq_options -> single_mcqs (single_mcq_id));
diesel::joinable!(single_mcqs -> chapters (chapter_id));
diesel::joinable!(subjects -> exams (exam_id));

diesel::allow_tables_to_appear_in_same_query!(
    chapters,
    exams,
    numericals,
    single_mcq_options,
    single_mcqs,
    subjects,
);
