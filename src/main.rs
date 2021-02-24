// Trait tells the Rust compiler about functionality a particular type has and can share with other types.
// Abstract way.
// Trait bounds specify that generics of any type has certain behaviour.

// // Defining a Trait
// // A type's behaviour consists of the methods we can call on that type.
// // Diff types share the same behaviour if we can call the same methods on all of the types.
// // Trait def are a way to group method signatures together to define a set of behaviours necessary to be purposeful.

// pub Trait Summary {
//     fn summarize(&self) -> String;
// }

// // Implementing a Trait on a Type

// pub struct NewsArticle { 
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ", self.username, self.content)
//     }
// }
// // ABove Implementing the Summary trait on the NewsArticle and Tweet types

// // Calling methods

// let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("
//     of course, as you probably already know, people
//     "),
//     reply: false,
//     retweet: false,
// };

// println!("1 new tweet: {}", tweet.summarize());


// Default Implementations
// Defininf a Summary trait with a default implementation of the summarize method

pub trait SUmmary {
    fn summarize(&self) -> String{
        String::from("(Read more ...)")
    }
}

let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship"),
    location: String::from("Pittsburgh, PA, USa"),
    author: String::from("Iceberg"),
    content: String::from("The Pittsburg Penguins once again are the best \ hockey team in the NHL,"),
};

println!("New article available! {}", article.summarize());

// prints New article available! (Read..more).

// // Defn Summary trait hav summarize_author method. impl reqd. defn summarize method hav def impl calls the summarize_author method.

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String{
//         format!("Read more from {}", self.summarize_author())
//     }
// }

// // Only def summarize_author while impl.

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String{
//         format!("@{}", self.username)
//     }
// }

// let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from(
//         "of course, as you probably already know, people"
//     ),
//     reply: false,
//     retweet: false,
// };

// println! ("1 new tweet: {}", tweet.summarize();)
// // prints// 1 new tweet: (Read more from @horse_ebooks...)

// Traits as parameters 

// pub fn notify(item: & impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // Trait Bound Syntax

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking News! {}", item.summarize());
// }

// // 2 parameters implementing Summary

// pub fn notify(item1: &impl Summary, item2: &impl Summary){}

// //2 paramters same type

// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// // Multiple Trait bounds with + syntax

// pub fn notify(item: &(impl Summary + Display))

// // Multiple Trait bounds oni generic typeswith + syntax

// pub fn notify<T: Summary + Display>(item: &T)


// // Clearer Trait Bounds with where clauses

// fn some_function<T:Display + Clone, U: Debug>(t: &T, u: &U) -> i32

// // where clause

// fn some_funciton<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug

// Returning Types that implement Traits

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("ofcourse"),
//         reply: false,
//         retweet: false,
//     }
// }


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![1, 2, 3, 4];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 't', 'u','x']

    let result = largest(char_list);
    println!("The largest char is {}", result);
}


// Using Trait Bounds conditionally impl methods on a generic type depending on trait bounds 

impl<T: Display> ToString for T{}































































































