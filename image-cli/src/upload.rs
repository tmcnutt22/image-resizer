pub struct UploadResult {
    pub original: String,
    pub resized: Vec<String>,
}

pub fn upload_image(file_name: String) -> UploadResult {
    let my_vec: Vec<String> = vec!["item1".to_string(), "item2".to_string()];

    UploadResult {
        original: file_name,
        resized: my_vec,
    }
}
