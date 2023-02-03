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



}
