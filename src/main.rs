struct NewsArticle {
   author:String,
   content:String,
}

struct Tweet {
   username:String,
   content:String,
}

pub trait Summary {
   fn summarize(&self)->String;
}

impl Summary for NewsArticle {
   fn summarize(&self) -> String {
        format!("{} by {}",self.author,self.content)
   }
}

impl Summary for Tweet {
   fn summarize(&self) -> String {
        format!("{} Tweeted {} ",self.username,self.content)
   }
}

fn main() {
    let tweet_1 = Tweet {
        username:String::from("Zeeshan"),
        content:String::from("The hills and the eyes"),
    };

    let newsarticle_1 = NewsArticle {
        author:String::from("Asif"),
        content:String::from("Abu bin adam"),
    };

println!("{}",tweet_1.summarize());
println!("{}",newsarticle_1.summarize());

}