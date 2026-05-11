pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}
pub struct News {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by {}", self.author, self.headline)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn aggregate(source: impl Summary) {
    println!("{}", source.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("Angshu"),
        content: String::from("The guy who become the billionaire"),
        reply: true,
        repost: true,
    };

    aggregate(tweet);

    let news = News {
        headline: String::from("Angshu's startup cross billion dollar valuation overnight"),
        location: String::from("Santiago capital of Chille"),
        author: String::from("ThePrime"),
        content: String::from(
            "This is insanly magical that how can a single person do it but never suprise he is Angshuman",
        ),
    };

    aggregate(news);
}
