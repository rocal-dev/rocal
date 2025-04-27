pub struct ContentType {
    file_name: String,
}

impl ContentType {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
        }
    }

    pub fn get_content_type(&self) -> &str {
        if self.file_name.contains(".js") || self.file_name.contains(".mjs") {
            "application/javascript; charset=UTF-8"
        } else if self.file_name.contains(".html") {
            "text/html; charset=UTF-8"
        } else if self.file_name.contains(".wasm") {
            "application/wasm"
        } else if self.file_name.contains(".css") {
            "text/css; charset=UTF-8"
        } else if self.file_name.contains(".jpg") || self.file_name.contains(".jpeg") {
            "image/jpeg"
        } else if self.file_name.contains(".png") {
            "image/png"
        } else if self.file_name.contains(".gif") {
            "image/gif"
        } else if self.file_name.contains(".ico") {
            "image/x-icon"
        } else if self.file_name.contains(".svg") {
            "image/svg+xml"
        } else if self.file_name.contains(".webp") {
            "image/webp"
        } else if self.file_name.contains(".avif") {
            "image/avif"
        } else if self.file_name.contains(".apng") {
            "image/apng"
        } else if self.file_name.contains(".bmp") {
            "image/bmp"
        } else if self.file_name.contains(".heic") {
            "image/heic"
        } else {
            "application/octet-stream"
        }
    }
}
