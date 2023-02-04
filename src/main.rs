extern crate core;

use core::num;
use std::fmt::Formatter;
use std::string::ParseError;

//region 18.错误处理
// 错误处理(error handling)是处理可能发生的失败情况的过程。
// 在 Rust 中有多种处理方式，总的来说：
//  显式的panic 主要用于测试，以及处理不可恢复的错误
//  Option 类型是为了值是可选的，或者缺少值并不错误的情况准备的
//      比如寻找父目录时，像 /、C：这样的目录就没有父目标不，就当然不是错误
//      当处理 Option 时，unwrap 可用一于原型开发，也可以用于能够确定 Option
//      中一定有值的情形，然而 expect 更有用
//  当错误有可能发生，且应当由调用者处理时，使用 Result。
//      也可以用 unwrap，然后使用 expect
//endregion
fn main() {
    // 在标准库当中有个叫 Option<T> 的枚举类型，用于有“不存在”的可能性的情况。
    // 它表现为以下两个“option”中的一个：
    //      Some(T) : 找到一个属于 T 类型的元素
    //      None：找不到相应的元素
    // 这些选项可以通过 match 显式处理，或使用 unwrap 隐式处理。
    // 隐式处理要么返回 Some 内部的元素，要么就 panic。
    //region 18.1.panic
    println!("\n\n=====18.1.panic=====");
    // 它会打印一个错误信息，开始回退 (unwind) 任务，且通常会退出程序
    // fn give_princess(gift: &str) {
    //     if gift == "snake" {
    //         panic!("AAAAaaaa!!!!");
    //     }
    //     println!("I love {}s.", gift);
    // }
    // give_princess("teddy bear");
    // give_princess("snake");
    //endregion

    //region 18.2.Option&unwrap
    println!("\n\n=====18.2.Option&unwrap=====");
    // 平民（commoner）们见多识广，收到什么礼物都能应付。
    // 所有礼物都显式地使用 `match` 来处理。
    fn give_commoner(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
            Some(inner) =>  println!("{}? How nice.", inner),
            None => println!("No gift? Oh well"),
        }
    }
    // 养在深闺人未识的公主见到蛇就会 `panic` 。
    // 这里所有的礼物都使用 unwrap 进行隐式处理。
    fn give_princess(gift: Option<&str>) {
        // unwrap 在接收到 `None` 时将返回 `panic`。
        let inside = gift.unwrap();
        if inside == "snake" {
            panic!("AAAaaa!!!");
        }
        println!("I love {}s!!", inside);
    }
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing = None;
    give_princess(bird);
    give_commoner(bird);
    // give_princess(snake);
    // give_princess(nothing);
    //endregion

    //region 18.2.1.使用？解开Option
    println!("\n\n=====18.2.1.使用？解开Option=====");
    // 你可以使用 match 语句来解开 Option，但使用 ？运算符通常会更易。
    // 如果 x 是 Option，那么若 x 是 Some，对 x? 表达式求值将返回底层值，
    // 否则无论函数是否正在执行都将终止且返回 None。
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // 如果 `current_age` 是 `None`，将返回 `None`
        // 如果 `current_age` 是 `Some`，内部的 `u8` 将赋值给 `next_age`
        let next_age: u8 = current_age?;
        Some(format!("Next year I will be {}", next_age))
    }
    let my_str = next_birthday(Some(32));
    println!("The result is: {:?}", my_str);
    // 可以将多个 ？ 链接在一起，以例代码更具可读性
    struct Person {
        job: Option < Job >,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }
    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            // 没有 ？ 运算符的话，这将需要很多的嵌套的 `match` 语句
            self.job?.phone_number?.area_code
        }
    }
    let p = Person {
        job: Some( Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 43922222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));
    println!("the word phone area code is: {:?}", p.job.unwrap().phone_number.unwrap().area_code);
    //endregion

    //region 18.2.2.组合算子：map
    println!("\n\n=====18.2.2.组合算子：map=====");
    // match是处理 Option 的一个可用的方法，但繁琐，特别是当操作只对一种输入是有效时
    // 这时可以使用 组合算子(combinator)，以模块化的风格来管理控制流。
    // Option有一个内置方法 map()，多个不同的 map() 调用可以串起来
    // #![allow(dead_code)]
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }
    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // 削皮，如果没有食物，就返回 `None`，否则返回削好皮的食物
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }
    // 切食物，如果没有食物，返回 `None`，否则返回切好的食物
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }
    // 烹饪食物，这里用 `map()` 来替代 `match` 以处理各种情况
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }
    // 这个函数将削皮、切块、烹饪一条龙
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
        // 代表的意思就是食物先去皮、然后对去皮的进行切块(chop)、对切块的进行烹饪
        // 用这种样式来模拟一个烹饪的流程，很好。 2023-02-03 15:43:02
    }
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm, I love {:?}", food),
            None => println!("Oh no!It wasn't edible."),
        }
    }
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);
    eat(cooked_potato);
    eat(cooked_apple);
    eat(cooked_carrot);
    //endregion

    //region 18.2.3.组合算子：and_then
    println!("\n\n=====18.2.3.组合算子：and_then=====");
    // map() 以链接调用的方式来简化 match 语句，如果返回类型是 Option<T> 的函数作为 map()
    // 的参数，会导致出现嵌套形式 Option<Option<T>>。这样多层串联调用就会变得混乱。
    // 所以有必要引进 and_them()，在某些语言中它叫做 flatmap
    // and_them() 使用被 Option 包裹的值 来调用其输入函数并返回结果。如果 Option 是 None，
    // 那么它返回 None。
    // 在下面的例子中，cookable-v2() 会产生一个 Option<Food>，如果在这里使用 map() 而不是
    // and_then() 将会得到 Option<Option<Food>> ，这对 eat() 来说是一个无效类型
    // #![allow(dead_code)]
    #[derive(Debug)] enum FoodV2 { CordonBleu, Steak, Sushi }
    #[derive(Debug)] enum Day {Monday, Tuesday, Wednesday}
    // 我们没有制作寿司所需的原材料(ingredient)
    fn have_ingredients(food: FoodV2) -> Option<FoodV2> {
        match food {
            FoodV2::Sushi => None,
            _ => Some(food),
        }
    }
    // 我们拥有全部食物的食谱，除了法国的蓝带猪排(Cordon Bleu)
    fn have_recipe(food: FoodV2) -> Option<FoodV2> {
        match food {
            FoodV2::CordonBleu => None,
            _ => Some(food),
        }
    }
    // 要做一份好菜，我们需要原材料和食谱
    // 我们可以借助一系列的 match 来表达这个逻辑：
    fn cookable_v1(food: FoodV2) -> Option<FoodV2> {
        match have_ingredients(food) {
            None => None,
            Some(food) => match have_recipe(food) {
                None => None,
                Some(food) => Some(food),
            }
        }
    }

    // 也可以用 and_then() 把上面的逻辑改写得更紧凑
    fn cookable_v2(food: FoodV2) -> Option<FoodV2> {
        have_ingredients(food).and_then(have_recipe)
    }
    fn eat_v2(food: FoodV2, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
            None => println!("Oh no. We don't get to eat on {:?}?", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (FoodV2::CordonBleu, FoodV2::Steak, FoodV2::Sushi);
    eat_v2(cordon_bleu, Day::Monday);
    eat_v2(steak, Day::Tuesday);
    eat_v2(sushi, Day::Wednesday);
    //endregion

    //region 18.3.结果Result
    println!("\n\n=====18.3.结果Resul=====");
    // Result 是 Option 类型的更丰富的版本，描述的是可能的错误而不是可能的不存在。
    // 也就是说，Result<T, E> 可以有两个结果的其中一个：
    //      Ok<T>   ：找到 T 元素
    //      Err<E>  ：找到 E 元素，E 表示错误的类型
    // 按照约定，预期结果是 "OK"，而意外结果就是 “Err”。
    // Result 有很多类似于 Option 的方法。例如： unwrap() ，它要么举出元素 T，要么
    // 就 panic。
    // 而对于事件的处理，Result 和 Option 有很多相同的组合算子
    // parse() 方法返回 Result 类型，并不总是能把字符串解析成指定的类型，也有可能失败
    fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);
    // 这里如果第一个参数为字母的话，只能是引发 panic，退出程序。不友好，看下面的例子就好多了。
    // 2023年2月3日21时19分24秒
    let tt = multiply("9","2");
    println!("double is {}", tt);
    // 失败的情况下，parse()产生一个错误，留给 unwrap() 来解包并产生 panic。
    // 另外，panic 会退出程序，并提供一个让人很不爽的错误信息
    //endregion

    //region 18.3.1.Result的map
    println!("\n\n=====18.3.1.Result的map=====");
    // 上一节的 multiply 函数的设计不是健壮的 (robust)，一般地，我们希望把错误返回给调用者
    // 这样它可以决定回应错误的正确方式
    // 首先，我们需要了解要处理的错误类型是什么。为了确定 Err 的类型，我们可以用 parse() 来
    // 实验。Rust 已经为 i32 类型使用 FromStr trait 实现了 parse()。结果表明，
    // 这里的 Err 类型被指定为 ParseIntError。
    use std::num::ParseIntError;
    fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => {
                match second_number_str.parse::<i32>() {
                    Ok(second_number) => {
                        Ok(first_number * second_number)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn print_v1(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiply_v1("10", "2");
    print_v1(twenty);
    let tt = multiply_v1("t", "2");
    print_v1(tt);
    //endregion

    //region 18.3.2.给Result取别名
    println!("\n\n=====18.3.2.给Result取别名=====");
    // 当我们要重用某个 Result 类型时，可以创建别名。
    // 若某个 Result 有可能被重用，可以方便地给它取一个别名。
    // 在模块层面上创建别名特别有用。同一模块中的错误常常会有相同的 Err 类型，所以单个别名就能简便定义所有相关
    // 的 Result 。这太有用了，以至于标准库也提供了一个别名： io::Result
    // use std::num::ParseIntError;  前面已经定义过了！！！ 2023年2月3日23时57分49秒
    // 为带有错误类型的 'ParseIntError` 的 'Result` 定义一个泛型别名。
    type AliasedResult<T> = Result<T, ParseIntError>;
    // 使用上面定义过的别名来表示上一节中的 `Result<i32, ParseIntError>` 类型
    fn multiply_v2(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    fn print_v2(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print_v2(multiply_v2("10", "2"));
    print_v2(multiply_v2("r", "2"));
    //endregion

    //region 18.3.3.提前返回
    println!("\n\n=====18.3.3.提前返回=====");
    // 在上一个例子中，显式地使用组合算子处理了错误。
    // 另一种处理错误的方式是使用 match 语句和提前返回(early return)的结合。
    // 也就是说，如果发生错误，可以停止函数的执行然后返回错误。
    fn multiply_v3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(first_number) => first_number,
            Err(e) => return Err(e),
        };
        let second_number = match second_number_str.parse::<i32>() {
            Ok(second_number) => second_number,
            Err(e) => return Err(e),
        };
        Ok(first_number * second_number)
    }
    fn print_v3(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    print_v3(multiply_v3("10", "2"));
    print_v3(multiply_v3("z", "2"));
    // 处理错误的一般原则是，要避免 panic 情况的出现，但显式处理所有错误确实是显得过于繁琐。
    //endregion

    //region 18.3.4.引入？
    println!("\n\n=====18.3.4.引入？=====");
    // 有时我们只是想 unwrap 且产生 panic。到现在为止，对 unwrap 的错误处理都在强迫我们一层层地嵌套，然而我们
    // 只是想把的变量拿出来，而 ？ 正是为这种情况准备的。
    // 当找到一个 Err 时，可以采取两种行动：
    //      1、panic!，不过我们已经决定要尽可能避免 panic
    //      2、返回它，因为 Err 就意味着它已经不能被处理了
    // ？ 几乎就等于一个会返回 Err 而不是 panic 的 unwrap！
    fn multiply_v4(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;
        Ok(first_number * second_number)
    }
    fn print_v4(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    print_v4(multiply_v4("10", "2"));
    print_v4(multiply_v4("z", "2"));
    //endregion

    //region 18.4.处理多种错误类型
    println!("\n\n=====18.4.处理多种错误类型=====");
    // 前面出现的例子都是很方便的情况，都是 Result 和其他 Result 交互，还有 Optiont 和其他 Option 交互
    // 有时 Option 需要和 Result 进行交互，或者 Result<T, Error1> 和 Result<T, Error2> 进行交互。
    // 在下面的代码中，unwrap 的两个实例生成了不同的错误类型。
    //      Vec::first 返回一个 Option
    //      parse::<i32> 返回一个 Result<i32, ParseIntError>
    fn double_first(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap();
        2 * first.parse::<i32>().unwrap()
    }
    let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {}", double_first(numbers));
    // println!("The first doubled is {}", double_first(empty));
    // println!("The first doubled is {}", double_first(strings));
    //endregion

    //region 18.4.1.从Option中取出Result
    println!("\n\n=====18.4.1.从Option中取出Result=====");
    // 处理混合错误类型的最基本的手段就是让它们互相包含。
    fn double_first_v1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        })
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("the first doubled is {:?}", double_first_v1(numbers));
    println!("the first doubled is {:?}", double_first_v1(empty));
    println!("the first doubled is {:?}", double_first_v1(strings));

    // 有时候我们不想再处理错误（比如使用 ？ 的时候），但如果 Option 是 None 则 继续处理错误
    // 一些组合算子可以让我们轻松地交换 Result 和 Option。
    println!("\n使用组合可以让我们轻松地交换 Result 和 Option。");
    fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        });
        opt.map_or(Ok(None), |r| r.map(Some))
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("the first doubled is {:?}", double_first_v2(numbers));
    println!("the first doubled is {:?}", double_first_v2(empty));
    println!("the first doubled is {:?}", double_first_v2(strings));
    //endregion

    //region 18.4.2.定义一个错误类型
    println!("\n\n=====18.4.2.定义一个错误类型=====");
    // 有时候把所有不同的错误都视为一种错误类型会简化代码。
    // 我们将用一个自定义错误类型来演示这一点。
    // Rust 允许我们定义自己的错误类型，一般来说，一个“好的”错误类型应当：
    //      用一个类型代表了多种错误
    //      向用户提供了清楚的错误信息
    //      能够容易地与其它类型比较
    //          好的例子：Err(EmptyVec)
    //          坏的例子：Err("Please use a vector with at least one element".to_owned())
    //      能够容纳错误的具体信息
    //          好的例子：Err(BadChar(c, position))
    //          坏的例子：Err("+ cannot be used here".to_owned())
    //      能够与其他错误很好地整合
    use std::error;
    use std::fmt;
    // 定义我们的错误类型，这种类型可以根据错误处理的实际情况定制。
    // 我们可以完全自定义错误类型，也可以在类型中完全采用底层的错误实现，也可以介于二者之间。
    #[derive(Debug)]
    struct DoubleError;
    type MyResult<T> = std::result::Result<T, DoubleError>;

    // 错误的生成与它如何显示是完全没关系的，没有必要担心的逻辑会导致混乱的显示
    // 注意：我们没有储存关于错误的任何额外信息，也就是说，如果不修改我们的错误类型定义的话，就
    // 无法指明是哪个字符串解析失败了。
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double.")
        }
    }
     
    // 为 `DoubleError` 实现 `Error` trait，这样其他错误可以包裹这个错误类型。
    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            // 泛型错误，没有记录其内部原因
            None
        }
    }

    fn double_first_v3(vec: Vec<&str>) -> MyResult<i32> {
        vec.first()
            .ok_or(DoubleError)
            .and_then(|s| {
                s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
            })
    }
    fn print_double_v3(result: MyResult<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_double_v3(double_first_v3(numbers));
    print_double_v3(double_first_v3(empty));
    print_double_v3(double_first_v3(strings));
    //endregion

    //region 18.4.3.把错误“装箱”
    println!("\n\n=====18.4.3.把错误“装箱”=====");
    // 如果想写简单代码，又想保存原始错误信息，一个方法就是把它们装箱(box)
    // 唯一的坏处就是，被包装的错误类型只能在运行了解，而不能被 静态地判别。
    // 对任何实现了 Error trait 的类型，标准库的 Box 通过 From 为它们提供了到 Box<Error> 的转换。

    // 为 `Box<error::Error>` 取别名。
    type OtherResult<T> = std::result::Result<T, Box<dyn error::Error>>;
    #[derive(Debug, Clone)]
    struct EmptyVec;
    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double.")
        }
    }
    impl error::Error for EmptyVec {
        fn description(&self) -> &str {
            "invalid first item to double"
        }
        fn cause(&self) -> Option<&dyn error::Error> {
            // 泛型错误。没有记录其内部原因。
            None
        }
    }
    fn double_first_v4(vec: Vec<&str>) -> OtherResult<i32> {
        // 我靠，这个太复杂了，顺序怎么办？！2023年2月4日13时17分31秒
        vec.first()
            .ok_or_else(|| EmptyVec.into())
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| e.into())
                    .map(|i| i * 2)
            })
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("the first doubled is {:?}", double_first_v4(numbers));
    println!("the first doubled is {:?}", double_first_v4(empty));
    println!("the first doubled is {:?}", double_first_v4(strings));
    //endregion

    //region 18.4.4.？的其他用法
    println!("\n\n=====18.4.4.？的其他用法=====");
    // 注意在上一个例子中，我们调用 parse 后总是立即将错误从标准库的错误 map（映射）到装箱错误中去：
    //      .and_then(|s| s.parse::<i32>()
    //      .map_err(|e| e.into())
    // 因为这个操作很简单常见，如果有省略写法就好了。
    // 遗憾的是 and_then 不够灵活，所以实现不了这样的写法。不过，我们可以用 ？ 来代替它。
    // ？之前被解释为要么 unwrap，要么 return Err(err)，这只是在大多数情况下是正确的。
    // ？实际上是指 unwrap 或 return Err(From::from(err))。
    // 由于 From::from 是不同类型之间的转换工具，也就是说，如果在错误可转换成返回类型
    // 的地方使用 ？，它将自动转换成返回类型。
    // 我们在这里重写之前的例子，用？，重写后，只要为我们的错误类型实现 From::from ，
    // 就可以不同去使用 map_err。
    type AgainResult<T> = std::result::Result<T, Box<dyn error::Error>>;
    #[derive(Debug)]
    struct AgainEmptyVec;

    impl fmt::Display for AgainEmptyVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }
    impl error::Error for AgainEmptyVec {}
    // 这里的结构和之前的一样，但是这次没有把所有的 `Result` 和 `Option` 串起来，
    // 而是使用 ？ 立即得到内部值。
    fn double_first_v5(vec: Vec<&str>) -> AgainResult<i32> {
        let first = vec.first().ok_or(AgainEmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }
    fn print_double_v5(result: AgainResult<i32>) {
       match result {
           Ok(n) => println!("The first doubled is {}", n),
           Err(e) => println!("Error: {}", e),
       }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_double_v5(double_first_v5(numbers));
    print_double_v5(double_first_v5(empty));
    print_double_v5(double_first_v5(strings));
    // 此段代码已经相当清楚，与原来的 panic 相比，除了返回类型是 Result 之外，它就像是把所有的
    // unwrap 调用都换成了 ？ 一样，因此必须在顶层解构它们。
    //endregion

    //region 18.4.5.包裹错误
    println!("\n\n=====18.4.5.包裹错误=====");
    // 把错误装箱这种做法也可以改成把它包裹到你自己的错误类型中去。
    type ResultV6<T> = std::result::Result<T, DoubleErrorV6>;
    #[derive(Debug)]
    enum DoubleErrorV6 {
        EmptyVec,
        // 在这个错误类型中，采用 `parse` 的错误类型中 `Err` 部分的实现
        Parse(ParseIntError),
    }
    impl fmt::Display for DoubleErrorV6 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleErrorV6::EmptyVec => write!(f, "Please use a vector with at least one element"),
                DoubleErrorV6::Parse(ref e) => e.fmt(f),
            }
        }
    }
    impl error::Error for DoubleErrorV6 {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleErrorV6::EmptyVec => None,
                DoubleErrorV6::Parse(ref e) => Some(e),
            }
        }
    }
    // 实现从 `ParseIntError` 到 `DoubleError` 的转换
    // 在使用 ？ 时，或者一个 'ParseIntError' 需要转换成 'DoubleErrorV6' 时，它会被自动调用。
    impl From<ParseIntError> for DoubleErrorV6 {
        fn from(err: ParseIntError) -> DoubleErrorV6 {
            DoubleErrorV6::Parse(err)
        }
    }
    fn double_first_v6(vec: Vec<&str>) -> ResultV6<i32> {
        let first = vec.first().ok_or(DoubleErrorV6::EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }
    fn print_v6(result: ResultV6<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(err) => println!("Error: {}", err),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_v6(double_first_v6(numbers));
    print_v6(double_first_v6(empty));
    print_v6(double_first_v6(strings));
    //endregion

    //region 18.5.遍历Result
    println!("\n\n=====18.5.遍历Result=====");
    // 有可能 map 操作失败，可以用 filter_map 过滤掉为 None 的所有结果
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("遍历的结果是：{:?}", numbers);
    //endregion
}
