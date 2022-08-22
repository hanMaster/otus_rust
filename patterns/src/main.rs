use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let post = NewPostBuilder::new()
        .with_title("Title")
        .with_text("Post text")
        .build()?;
    println!("{:#?}", post);
    Ok(())
}

#[derive(Debug)]
struct NewPost {
    title: String,
    text: String,
}

struct NewPostBuilder {
    title: Option<String>,
    text: Option<String>,
}

impl NewPostBuilder {
    pub fn new() -> Self {
        NewPostBuilder {
            title: None,
            text: None,
        }
    }
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn build(self) -> Result<NewPost, Box<dyn Error>> {
        let title = self.title.ok_or("Title is not defined")?;
        let text = self.text.ok_or("Text is not defined")?;
        Ok(NewPost { title, text })
    }
}
