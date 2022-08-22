use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let post = NewPostBuilder::new()
        .with_title("Что такое Lorem Ipsum?")
        .with_text("Lorem Ipsum - это текст-'рыба', часто используемый в печати и вэб-дизайне. Lorem Ipsum является стандартной 'рыбой' для текстов на латинице с начала XVI века. В то время некий безымянный печатник создал большую коллекцию размеров и форм шрифтов, используя Lorem Ipsum для распечатки образцов. Lorem Ipsum не только успешно пережил без заметных изменений пять веков, но и перешагнул в электронный дизайн. Его популяризации в новое время послужили публикация листов Letraset с образцами Lorem Ipsum в 60-х годах и, в более недавнее время, программы электронной вёрстки типа Aldus PageMaker, в шаблонах которых используется Lorem Ipsum.")
        .build()?;
    post.review().approve().publish().print_post();
    Ok(())
}

#[derive(Debug)]
struct NewPost {
    title: String,
    text: String,
}

struct ReviewedPost {
    title: String,
    text: String,
}

struct ApprovedPost {
    title: String,
    text: String,
}

struct PublishedPost {
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

impl NewPost {
    pub fn review(self) -> ReviewedPost {
        ReviewedPost {
            title: self.title,
            text: self.text,
        }
    }
}

impl ReviewedPost {
    pub fn approve(self) -> ApprovedPost {
        ApprovedPost {
            title: self.title,
            text: self.text,
        }
    }
}

impl ApprovedPost {
    pub fn publish(self) -> PublishedPost {
        PublishedPost {
            title: self.title,
            text: self.text,
        }
    }
}

impl PublishedPost {
    pub fn print_post(&self) {
        println!("Post with title: '{}'\n{}", self.title, self.text)
    }
}
