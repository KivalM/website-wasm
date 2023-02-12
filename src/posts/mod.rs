pub struct Article {
    pub title: String,
    pub description: String,
    pub content: String,
    pub author: String,
    pub date: String,
    pub tags: Vec<String>,
}

impl Article {
    pub fn new(
        title: String,
        description: String,
        content: String,
        date: String,
        tags: Vec<String>,
    ) -> Self {
        Article {
            title,
            content,
            description,
            author: "Kival Mahadew".to_string(),
            date,
            tags,
        }
    }
}
