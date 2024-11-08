pub mod models;

use serde_json;
use std::fs;

fn main() {
    let chapters_dir = "dataset/chapters";
    // let questions_input_dir = "dataset/_questions";
    let questions_output_dir = "dataset/questions";

    // for file in fs::read_dir(chapters_dir).unwrap() {
    //     let mut file_path = file
    //         .unwrap()
    //         .path()
    //         .file_name()
    //         .unwrap()
    //         .to_string_lossy()
    //         .to_string();
    //     file_path = file_path[0..file_path.len() - 5].to_string();
    //     fs::create_dir(format!("{}/{}", questions_output_dir, file_path)).unwrap()
    // }
    for file in fs::read_dir(chapters_dir).unwrap() {
        let file_path = file.unwrap().path();
        let mut file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        file_name = file_name[0..file_name.len() - 5].to_string();

        let new_folder_path = format!("{}/{}", questions_output_dir, file_name);
        // println!("{:#?}", new_folder_path);
        fs::create_dir(new_folder_path).unwrap();

        let contents = fs::read_to_string(file_path.clone()).unwrap();
        let chapters: Vec<models::Chapter> = serde_json::from_str(&contents).unwrap();

        for chapter in chapters.iter() {
            let new_path = format!("{}/{}/{}.json", questions_output_dir, file_name, chapter.name);
            // println!("{:#?}", new_path);
            match fs::File::create(new_path.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{:#?},{:#?}", new_path, e.to_string())
                }
            };
        }
    }
}
