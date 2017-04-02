#[derive(Debug)]
pub enum ContentType {
    TextHtml,
    TextCss,
    TextPlain,
    ImageJpg,
    ImagePng,
    ImageGif,
    ApplicationOctetStream,
}

impl ContentType {
    pub fn from_path(x: &str) -> Self {
        let extension = x.split(".").nth(1).unwrap_or("html").to_string();
        ContentType::from_string(extension)
    }

    fn from_string(x: String) -> Self {
        match x.as_str() {
            "html" | "htm" => ContentType::TextHtml,
            "css" => ContentType::TextCss,
            "jpg" | "jpeg" => ContentType::ImageJpg,
            "png" => ContentType::ImagePng,
            "gif" => ContentType::ImageGif,
            "txt" => ContentType::TextPlain,
            _ => ContentType::ApplicationOctetStream,
        }
    }

    pub fn to_string(&self) -> String {
        let content_type = match self {
            &ContentType::TextHtml => "text/html",
            &ContentType::TextCss => "text/css",
            &ContentType::TextPlain => "text/plain",
            &ContentType::ImageJpg => "image/jpeg",
            &ContentType::ImagePng => "image/png",
            &ContentType::ImageGif => "image/gif",
            _ => "application/octet-stream",
        };

        format!("Content-type: {}", content_type)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_extract_content_type() {
        assert_eq!(ContentType::from_path("/test.png").to_string(), "Content-type: image/png");
        assert_eq!(ContentType::from_path("/test.jpg").to_string(), "Content-type: image/jpeg");
        assert_eq!(ContentType::from_path("/").to_string(), "Content-type: text/html");
    }
}