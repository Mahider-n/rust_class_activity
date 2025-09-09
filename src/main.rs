// pure function 
fn sum(a:i32,b:i32)->i32{
    a+b
}
// impure function
fn print_name(name:&str){
    println!("Hello, {name} wanna grab a coffee? :) ")
}
// rust like oop style
struct Calculator{
    value:i32,
}
impl Calculator{
    fn new()->Calculator{
        Calculator {value: 0}
    }
    fn add(&mut self, num:i32){
        self.value += num
    }
    fn double(&mut self, num:i32){
        self.value= num * 2
    }
}
// struct
struct Human{
    name: String,
    age: u32,
    is_student: bool,
}
// higher-order functions
fn apply_twice(f: fn(i32) -> i32, x:i32)-> i32{
    f(f(x))
}
fn add_two(n:i32) -> i32{
    n + 2
}
// fucntion composition 
// sample example
// f(x) = 2x + 1
// g(x) = x * x
// f(g(x))= 2 (x *x ) +1 
// f(g(2)) = 2(2x2) +1
// f(g(2)) = 9
// g(2) = 4
// f(4) = 9
// implementation of the above sample example 
fn fog(f: fn(i32)->i32, g: fn(i32)->i32, x: i32)-> i32{
    f(g(x))
}
fn first_function(x:i32) -> i32{
    (2* x)+ 1
}
fn second_function(x:i32)-> i32{
    x*x
}
// partial application 

fn multiply(x:i32,y:i32) -> i32{
    x * y
}
fn double(y:i32)-> i32{
    multiply(2,y)
}
// factorial function 
fn factorial(n: u64)->u64 {
    if n == 0 {1} else {n * factorial(n-1) }
}
// implementing map function
fn map_impl<T, U, F>(arr: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    arr.iter().map(f).collect()
}
//  compose function 
// compose(f, g)(x) = g(f(x))..computes g of f(x)

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// square function
fn square(x: i32) -> i32 {
    x * x
}
// builder pattern 
struct User {
    name: String,
    age: u32,
    email: String,
}

struct UserBuilder {
    name: String,
    age: u32,
    email: String,
}

impl UserBuilder {
    fn new() -> Self {
        Self { name: "".to_string(), age: 0, email: "".to_string() }
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }

    fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    fn build(self) -> User {
        User { name: self.name, age: self.age, email: self.email }
    }
}
// curring 
fn addition(a:i32)->impl Fn(i32) -> i32{
    // |argument| and a+b is the main body
    move |b| a + b
}
// pattern matching using enum
enum Color{
    Red,
    Green,
    Blue,
}
// partial composition example
fn partial_compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B + Copy,
    G: Fn(B) -> C + Copy,
{
    move |x| g(f(x))
}

// pattern matching with structs
fn match_struct(human: &Human) {
    match human {
        Human { age: 0..=17, .. } => println!("{} is a minor", human.name),
        Human { age: 18..=64, .. } => println!("{} is an adult", human.name),
        Human { age: 65.., .. } => println!("{} is a senior", human.name),
    }
}
// pattern matching in +ve/-ve nums
fn classify_number(n: i32) {
    match n {
        x if x < 0 => println!("{} is negative number", x),
        x if x == 0 => println!("zero"),
        x if x % 2 == 0 => println!("{} is even number", x),
        _ => println!("{} is odd number", n),
    }
}
// lazy evaluation with iterators 
fn lazy_evaluation() {
    let numbers = 1..; 
    let even_squares = numbers
        .map(|x| {
            println!("Processing: {}", x);
            x * x
        })
        .filter(|&x| x % 2 == 0)
        .take(5); // takes the first 5
    
    println!("Created lazy iterator");
    for n in even_squares {
        println!("Even square: {}", n);
    }
}
// main function 
fn main() {
    // using factorial 
    let ans = factorial(10);
    println!("the ans is {}",ans);
    // using map_implementation 
    let array = [1, 2, 3, 4, 5];
    let squared = map_impl(&array, |x| x * x);
    println!("original: {:?}", array);
    println!("squared:  {:?}", squared);
    // Compose: square(add_one(x))... 
    let square_after_add_one = compose(add_one, square);

    println!("square result after adding one is: {}", square_after_add_one(3)); 
    println!("square result after adding one is: {}", square_after_add_one(5)); 
// trail
    println!("pure function computation result: {}", sum(9,10));
    print_name("abebe");
    let mut calc = Calculator::new();
    calc.add(5);
    calc.add(10);
    calc.double(10);
    // calc.value prints the recent computation 20
    println!("Result: {}",calc.value);
// creating instance of human
    let person = Human {
        name: String::from("Mahider"),
        age: 21,
        is_student: true,
    };
    println!("{} is {} years old. And is she student ? {}", person.name,person.age,person.is_student);
    let result = apply_twice(add_two,5);
    println!("higher order function result {}",result);
    let sec_computation = fog(first_function,second_function,2);
    println!("function composition result {}",sec_computation);
    let result2 = double(10);
    println!("partial application result {}",result2);
    let user1 = UserBuilder::new()
        .name("beletu")
        .age(30)
        .email("belu@gmail.com")
        .build();
    println!("{} - {} - {}", user1.name,user1.age,user1.email); 
    let add_five = addition(5);
    println!("currying example result {}",add_five(10));
    // closure
    let x = 10;
    let add_x = |y| y + x;
    println!("closure example result {}",add_x(10));
    // lazy with iterators
    let nums = [1,2,3,4];
    // LIST COMPRSSION EQUIVALENT IN RUST USIGN ITERETOR,MAP AND FILTER
    let squares: Vec<i32> = nums.iter()
    // if nums is vec<i32> then nums.iter() gives items of type &i32, so 
    // Here |&x| is pattern matching on the reference.
                                .filter(|&x| x % 2 == 0)
                                // Even though x is a &i32, Rust allows autoderef in numeric operations.
                                //x * x automatically dereferences &i32 to i32 under the hood. 
                                .map(|x| x * x)
                                .collect();
    println!("{:?}",squares); 
    // pattern matching
    let n = "bird";
    match n {
        "bird" => println!("living-org"),
        "table" => println!("non-living org"),
        _ => println!("unknown"),
    }
    // pattern matching in enum
    let color = Color::Green;
    let color2 = Color::Red;
    let color3 = Color::Blue;

    
    match color {
        Color::Red => println!("Red color"),
        Color::Green => println!("Green color"),
        Color::Blue => println!("Blue color"),
    }
    match color2 {
        Color::Red => println!("Red color"),
        Color::Green => println!("Green color"),
        Color::Blue => println!("Blue color"),
    }
    match color3 {
        Color::Red => println!("Red color"),
        Color::Green => println!("Green color"),
        Color::Blue => println!("Blue color"),
    }
    // partial composition test
    let example2 = partial_compose(add_one, square);
    println!("partial composition result: {}", example2(4));
    // lazy evaluation
    lazy_evaluation();
    // pattern matching
    match_struct(&person);
    // pattern matching in classifing nums
    classify_number(-5);
    classify_number(0);
    classify_number(4);
    classify_number(7);

    // println!("impure function computation result{:?}",print_name("abebe"));
}

//---------------------------//-----------------------------

// struct Human {
//     name: String,
// }
// fn get_human(whoman: Option<String>)-> Option<Human>{
//     match whoman{
//         Some(name) => Some(Human {name}),
//         None => None

//     }
// //    Some(human {name: "sura".to_string()})
// }
// pub enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }
// fn create_human(whoman: String) ->Result<Human,String>{
//     if (whoman.is_empty()){
//         Result::Err("lls".to_string())
        
//     } else{
//         Result::Ok(Human {name:whoman})
        
//     }

// }
// fn use_human(){
//     let human = create_human("".to_string());

//     match human {
//         Result::Ok(H) => println!("{}",H.name),
//         Result::Err(e) => println!("{}",e)
//     }
// }

// partial function
// sum 
//  number -> num -> numb
// 
// User()
//     .name("name")
//     .

 