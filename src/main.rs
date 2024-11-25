pub mod models;

use serde_json;
use std::fs;
// use uuid;

fn main() {
    let chapters_dir = "dataset/chapters";
    // let questions_dir = "dataset/questions";

    // let classes_file_path = "dataset/classes.json";
    // let contents = fs::read_to_string(classes_file_path).unwrap();
    // let class_vec = serde_json::from_str::<Vec<models::Class>>(&contents).unwrap();

    let subject_file_path = "dataset/subjects.json";
    let contents = fs::read_to_string(subject_file_path).unwrap();
    let subjects_vec = serde_json::from_str::<Vec<models::Subject>>(&contents).unwrap();
    // for subject in subjects_vec.iter_mut() {
    //     let curr_class = class_vec
    //         .iter()
    //         .find(|class| class.name == subject.class_name)
    //         .unwrap();
    //     subject.class_id = curr_class.id.clone();
    //     subject.updated_at = 1191105900;
    //     subject.created_at = 1191105900;
    //     subject.id = uuid::Uuid::new_v4().to_string();
    // }
    //
    // fs::write(
    //     subject_file_path,
    //     serde_json::to_string(&subjects_vec).unwrap(),
    // )
    // .unwrap();

    // let mut glob_chapters_vec: Vec<models::Chapter> = vec![];
    for chapter_file in fs::read_dir(chapters_dir).unwrap() {
        let chapter_file_path = chapter_file.unwrap().path();
        let mut curr_chapters_vec = serde_json::from_str::<Vec<models::Chapter>>(
            &fs::read_to_string(chapter_file_path.clone()).unwrap(),
        )
        .unwrap();
        for chapter in curr_chapters_vec.iter_mut() {
            let curr_subject = subjects_vec
                .iter()
                .find(|subject| {
                    subject.name == chapter.subject_name && subject.class_name == chapter.class_name
                })
                .unwrap();
            chapter.subject_id = curr_subject.id.clone();
        }
        fs::write(
            chapter_file_path.clone(),
            serde_json::to_string(&curr_chapters_vec).unwrap(),
        )
        .unwrap();
        // glob_chapters_vec.append(&mut curr_chapters_vec.clone());
        // let file_path = file.unwrap().path();
        // let mut file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        // file_name = file_name[0..file_name.len() - 5].to_string();

        // println!("{:#?}", new_folder_path);
        // fs::create_dir(new_folder_path).unwrap();

        // let contents = fs::read_to_string(file_path.clone()).unwrap();
        // let chapters: Vec<models::Chapter> = serde_json::from_str(&contents).unwrap();
        //
        // for chapter in chapters.iter() {
        //     let new_path = format!(
        //         "{}/{}/{}.json",
        //         questions_output_dir, file_name, chapter.name
        //     );
        //     println!("{:#?}", new_path);
        //     // match fs::File::create(new_path.clone()) {
        //     //     Ok(_) => {}
        //     //     Err(e) => {
        //     //         eprintln!("{:#?},{:#?}", new_path, e.to_string())
        //     //     }
        //     // };
        // }
    }
    //
    // for question_subject_folder in fs::read_dir(questions_dir).unwrap() {
    //     for question_file in fs::read_dir(question_subject_folder.unwrap().path()).unwrap() {
    //         let q_path = question_file.unwrap().path();
    //         let contents = fs::read_to_string(q_path.clone()).unwrap();
    //         if contents.trim().len() != 0 {
    //             let mut questions_vec =
    //                 serde_json::from_str::<Vec<models::Question>>(&contents).unwrap();
    //             for question in questions_vec.iter_mut() {
    //                 let curr_chapter = glob_chapters_vec
    //                     .iter()
    //                     .find(|chapter| {
    //                         chapter.name == question.chapter_name
    //                             && chapter.subject_name == question.subject_name
    //                             && chapter.class_name == question.class_name
    //                     })
    //                     .unwrap();
    //                 question.chapter_id = curr_chapter.id.clone();
    //                 question.id = uuid::Uuid::new_v4().to_string();
    //                 question.updated_at = 1191105900;
    //                 question.created_at = 1191105900;
    //             }
    //             fs::write(
    //                 q_path.clone(),
    //                 serde_json::to_string(&questions_vec).unwrap(),
    //             )
    //             .unwrap();
    //         } else {
    //             fs::write(q_path.clone(), "[]").unwrap();
    //         }
    //     }
    // }
}
