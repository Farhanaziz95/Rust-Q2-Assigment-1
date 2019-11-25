#[derive(Debug)]
struct NewsArtical 
    {
        author:String,
        content:String
    }

#[derive(Debug)]
struct Tweet
    {
        username:String,
        content:String
    }

    pub trait Summary{
        fn summarize(&self) -> String;
    }

    impl Summary for NewsArtical{
        fn summarize(&self) -> String{
            let a = String::from("This Summary is For NewsArtical");
            println!("Author Name Is : {} \n Contant Is : {} ",self.author,self.content);
            println!("{}",a );
            a

        }
    }

    impl Summary for Tweet{
        fn summarize(&self) -> String{
            let a = String::from("This Summary is For Tweet");
            println!("User Name Is : {} \n Contant Is : {} ",self.username,self.content);
            println!("{}",a );
            a

        }
    }
fn main() {
    let new_NewsArtical = NewsArtical{
        author:String::from("Farhan Aziz"),
        content:String::from("I Dont Know :-P")
    };

    let new_Tweet = Tweet{
        username:String::from("farhanaziz95"),
        content:String::from("I have Never Tweeted")
    };
    
    new_NewsArtical.summarize();
    new_Tweet.summarize();
}
