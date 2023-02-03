extern crate core;

use core::num;

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
    fn multiplay(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
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

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiplay("10", "2");
    print(twenty);
    let tt = multiplay("t", "2");
    print(tt);
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
    fn multiplying(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number + second_number)
        })
    }

    fn print_v1(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiplying("10", "2"));
    print(multiplying("r", "2"));
    //endregion

    //region 18.3.3.提前返回
    println!("\n\n=====18.3.3.提前返回=====");
    // 在上一个例子中，显式地使用组合算子处理了错误。
    // 另一种处理错误的方式是使用 match 语句和提前返回(early return)的结合。
    // 也就是说，如果发生错误，可以停止函数的执行然后返回错误。





}
