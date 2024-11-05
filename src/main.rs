pub mod models;

use std::fs;

fn main() {
    let chapters_dir = "dataset/chapters";
    // let questions_input_dir = "dataset/_questions";
    let questions_output_dir = "dataset/questions";

    for file in fs::read_dir(chapters_dir).unwrap() {
        let mut file_path = file
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        file_path = file_path[0..file_path.len() - 5].to_string();
        fs::create_dir(format!("{}/{}", questions_output_dir, file_path)).unwrap()
        // let chapters: Vec<models::Chapter> =
        //     serde_json::from_str(fs::read_to_string(file.unwrap().path()).unwrap().as_str())
        //         .unwrap();
        // println!("{:#?}", chapters);
    }
}
