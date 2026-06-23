
// fn main() {
//     // println!("Hello, world!");
//     let a = 1;
//     println!("test! {a}");
// }


// //赋值
// fn main() {
//     let a: u32 = 1u32;
//     println!("test! {a}");
// }

// fn main() {
//     let a: u32 = 1;
//     println!("test! {a}");
// }


// fn main() {
//     let a: u32 = 10_000_000_000;  // u32范围区间 0..=4294967295 此处改成u64合适
//     println!("test! {a}");
// }




//浮点数   f64 f32
// fn main() {
//     let a: f64 = 4515.25556;
//     println!("test!{a}");
// }

// fn main() {
//     let a: f64 = 4514621f64;
//     println!("test!{a}");
// }

// fn main() {
//     let a: f64 = 4514621.0f64;
//     println!("test!{a}");
// }

//字符  char
// fn main() {
//     let c = "z";
//     let z: char ='§';
//     let emage = "📤";
//     println!("c = {c}, z = {z}, emage = {emage}");
// }


//添加下划线前缀（表示故意不使用）
// fn main() {
//     let _c = "z";
//     let _z: char ='§';
//     let _emage = "📤";
// }

//  fn main(){
//     let s = "dfwsfs";
//     println!("cs,{s}");
//  }


//   fn main(){
//     let s = "dfws\"fs";
//     println!("cs,{s}");
//  }


//字节串
// fn main() {
//     let s: &[u8; 21] =b"this is a byte string";
//     // 等同于 let a =b'a';
//     println!("test,{s:?}")
// }


//定长数组
// fn main() {
//     let a: [i32; 5] = [1,2,3,4,5];
//     let a = [1,2,3,4,5];
//     let months = ["January","February","March","April","May","June","July","August","September","October","November","December"];
// }

//下标索引索引数组元素
// fn main() {
//     let a: [i32; 5] = [1,2,3,4,5];
//     let b = a[3];
//     println!("{b}");
// }

//动态数组
// fn main() {
//     let v: Vec<u32> = Vec::new();
//     let v =vec![1,2,3];
//     let mut v = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("{v:?}")
// }


// fn main() {
//     let a: [i32; 5] = [1,2,3,4,5];
//     let b = a[3];
//     println!("{b}");

//     let mut v: Vec<u32> = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("{v:?}")
// }
//同下：
// fn main() {
//     let a: [i32; 5] = [1,2,3,4,5];
//     let b = a[3];
//     println!("{}",b);

//     let mut v: Vec<u32> = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("{:?}",v)
// }

// fn main() {
//     let a: [i32; 5] = [1,2,3,4,5];
//     let b = a[3];
//     println!("{}",b);

//     let mut v: Vec<u32> = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("{:?}",v);
//     let v: Vec<u32> = vec![1,2,3,4,5];
//     println!("{:?}",v);

//     let t = v[1];
//     println!("{:?}",t)
// }



//哈希表
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"),10);
//     scores.insert(String::from("Yellow"),50);
//     println!("{:?}",scores);
// }



//元组
// fn main() {
//     let tup: (i32,f64,u8,char) = (500,6.4,1,'a');
//     println!("{:?}",tup);
//     println!("{:?}",tup.0);
//     println!("{:?}",tup.1);
//     println!("{:?}",tup.2);
//     println!("{:?}",tup.3);    
// }

// fn main() {
//     let x: (i32,f64,u8) = (500,6.4,1);
//     let _five_hundred = x.0;
//     let _six_point_four = x.1;
//     let _one = x.2;
//     println!("{}",x.0);
//     println!("{}",x.1);
//     println!("{}",x.2);
// }

//结构体
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username:String,
//     email:String,
//     age:u64,
// }

// fn main() {
//     let u = User {
//         active: true,
//         username: String::from("charles"),
//         email:String::from("charles@iieao.com"),
//         age:23,
//     };
//     println!("{:?}",u)


//枚举   也是一种复合类型
// enum IpAddrKind {
//     V4,
//     V6,
// }
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;


//分支语句 if else
// fn main() {
// let num=6;
//     if num %4 == 0 {
//         println!("num is divisible by 4");
//     }else if num %3 == 0 {
//         println!("num is divisible by 3");
//     }else if num %2 == 0 {
//         println!("num is divisible by 2");
//     } else {
//         println!("num is not divisible by 4,3,2");
//     }
// }

// if else 其实可以返回数值
// fn main() {
//     let x = 1;
//     let y = if x == 0 {
//         100
//     } else {
//         101
//     };
//     println!("y  is {}",y);
// }


//循环语句 loop(无条件循环) while for
// fn main() {
//     let mut counter = 0;
//     let result =loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("the result is {result}");
// }

// //while循环
// fn main() {
//     let mut num = 3;//声明一个可变的变量 num，初始值是 3
//     while num != 0 {
//         println!("{num}!");
//         num -= 1;
//     }
//     println!("LIFTOFF")
// }// while true {} 是不是等于 loop {}?
// while 循环：只要 num 不等于 0，就反复执行大括号里的代码。!= 是“不等于”的意思。
// 回答：不完全相等，它们在 Rust 中有重要区别。
// 相同点
// 两者都会无限循环，除非内部使用 break 跳出。
// 执行效果上，如果循环体内没有 break，它们都会永远跑下去

////for循环
// fn main() {
//     let a = [10,20,30,40,50];
//     for element in a {
//         println!("the value is:{element}");
//     }
// }


//range..
// fn main() {
//     for num in 1..4 { //1..4 左闭右开区间
//         println!("{num}");
//     }
//     for num in 1..=4 {    // 1..=4 包含一到四的所有数字 即左闭右闭区间
//         println!("{num}");
//     }
//     for num in (1..4).rev() {   //rev 是 reverse（反转）的缩写，把队伍的顺序完全倒过来
//         println!("{num}");
//     }
// }

//rang a..z
// fn main() {
//     for ch in 'a'..= 'z' {   //'a'..='z' 是一个字符的闭区间，和之前的 1..=4 完全类似。
//         println!("{ch}");
//     }
//     for zh in ('a'..='z').rev() {
//         println!("{zh}");
//     }
// }

//函数 fn
// fn print_a_b(value:i32,unit_lable:char) {
//     println!("the value of a b is: {value} {unit_lable}");
//     println!("the value of a b is: {} {}",value,unit_lable);
// }
// fn main() {
//     print_a_b(5,'h');
// }

 //模块
//  backyard
//     Cargo.lock
//     Cargo.toml
//     src
//       garden
//          vegetables.rs
//       garden.rs
//       main.rs

//模块的另一种组织形式
// backyard
//    Cargo.lock
//    Cargo.toml
//    src
//      garden
//          mod.rs
//          vegetables.rs
//      main.rs


// fn main() {
//     for ch in 'a'..= 'z' {   
//         println!("{ch}");
//     }
//     let m = "#######分隔符#######";
//     println!("{m}");

//     for zh in ('A'..='Z') {
//         println!("{zh}");
//     }
// }



//所有权


// fn main() {
//     let a = 10u32;
//     let b = a;
//     println!("{a}");
//     println!("{b}");
// }


// fn main() {
//     let s1 = String::from("this is");
//     let s2 = s1;
//     println!("{s1}");
//     println!("{s2}");
// } //打印出来是 this is 吗？其它语言都是这样的

// fn main() {
//     let s1 = String::from("this is");
//     let s2 = s1.clone();
//     println!("{s1}");
//     println!("{s2}");
// }    
//解析：rust的字符串为何会有如此奇怪的行为？【let s2 = s1】不行，而【let s2 = s1.clone()】可以
//【  let a = 10u32;】这种数字类型是直接可以打出来的
// u32的类型是 占用内存字节数固定的类型
// 而String这种类型。是占用字节数不固定的（动态的）类型
//一般来说，对于固定字节数的类型，会默认放在栈上（栈帧中）
//而不固定字节数的类型，会默认创建在堆上 （成为堆上面的一个资源），然后在栈上用一个局部变量来指向它
// 例如 let s1 = String::from("this is") 中的 s1 就是在栈上 ，右边的字符串是放在堆里面的


// fn main() {
//     let s1 = String::from("this is");
//     let s2 = s1;
//     // println!("{s1}");  //将此处注释，编译成功
//     println!("{s2}");
// } // 也就是说，s1把内容“复制”给s2之后，s2可用，但s1是不能用了
//当然，也可以说，s1  把 值（资源）”移动“ 给了 s2，既然是“移动”，那么原本的变量中就没有这个数值了
// Rust 把字符串的引用由 s1 拷贝到了 s2，但是只保留了最新的 s2 到字符串的指向，同时却把 s1 到字符串的指向给“抹去”
// java不同，java默认做了引用的拷贝，并且新旧两个变量同时指向那个对象
// rust 这样设计的用处是？
//解答：这正是Rust从头开始梳理整个软件体系的地方，剑指一个目标：内存安全
//那为什么要这样设计？（长久以来，计算机领域都在探索如何写出更安全的程序，而rust带来了全新的思路）
//Rust 明确了所有权的概念，值 也可以叫 资源。
//所有权就是对 资源拥有的权利。
//Rust 基于所有权定义。推导了整个世界。
//所有权的基础是三条定义：
    // Rust 中，每一个值（资源）都有一个所有者；
    // 任何一个时刻，一个值只有一个所有者；
    // 当所有者所在作用域结束的时候，值 就会被释放掉。
// 三个规则，涉及两个概念，所有者，作用域
//所谓 所有者 ：在代码中，就表示为变量。也即是 所有者 会用变量名来表示
//变量的作用域 ： 就是变量有效（valid）的那个区间。【在 rust 中，简单来说就是在一对花括号括起来的里面的部分中，从变量创建时开始，到花括号结束的地方】


// fn main() {
//     let s = String::from("open");
//     //do stuff with s     这个地方是 s 的“有效期”，随便你用它
// }
// //s 的有效范围是从创建它的那行开始，一直到 main 函数的大尾巴 } 结束。
// // 在整个程序运行期间，它都活着，直到程序结束自动释放
// fn main() {
//     let a = lu32;
//     {
//         let s = String::from("open");
//     }
//     // other stuff    这个地方已经过了 s 的“有效期”，别再惦记 s 了
// }

// fn main() {
//     let s = String::from("open");
//     // do stuff with s
//     println!("{}", s);  // 这行注释实际上是在提醒你：在这里可以操作 s
//     s.push_str("score");  // 比如修改它  此处直接运行会  编译器报错！ 正确代码见下文
// }  // 到这里，s 才死掉


// fn main() {
//     let mut s = String::from("open"); //在 Rust 里，变量默认是“死的”（不可变）。必须明确写上 mut 告诉编译器：“我要动它了，别拦我！”
//     // do stuff with s
//     println!("{}", s);   
//     s.push_str("score");   
// }   


//程式变式
// fn main() {
//     let mut s = String::from("open");
//     println!("修改前：{}", s);  // 第一次打印 -> open

//     s.push_str("score");      // 偷偷把 open 改成了 openscore

//     println!("修改后：{}", s);  // 第二次打印 -> openscore
// }  // 到这里，s 才死掉
//你不说“打印”，它绝不主动显示；你让它什么时候打印，它就只打印那一瞬间变量的样子。
//push_str 是“闷声干大事”：它只负责偷偷改内存里的值，改完也不会敲锣打鼓告诉你。
// 想知道改没改成功，必须在push_str它后面再写一个 println! 去“验收”。



// fn main() {
//     let a = 32;          // a 活了
//     {                    // 进入小房间
//         let s = String::from("open");  // s 活了
//     }                    // <--- 小房间关门，s 被炸毁（释放）了！
    
//     // other stuff
//     // 现在这里能写什么？只能写关于 a 的事情，因为 a 还活着
//     println!("{}", a);   // 没问题，a 还在
//     // println!("{}", s); //  如果把这行取消注释，编译会报错！因为 s 已经被释放了
// }  // 到这里，a 也死了


//变式
// fn main() {
//     let a = 32;           
//     { 
//         println!("{}",a);                    
//         let mut s = String::from("open");  
//         println!("修改前：{}",s);
//         s.push_str("score");
//         println!("修改后{}",s)
//     }                     
//     println!("最终：{}", a);   
//     // println!("s最终{}",s);  //s应该是打印不出来，会报错，因为s在作用域结束就被销毁了
// }   
//以后看到花括号 {}，记住口诀：
//“外层是大王，内层是小弟。小弟随时可以找大王帮忙（访问外层变量），
// 但大王绝不关心小弟的死活（无法访问内层变量）。”


//上面的代码排版整理：
// fn main() {
//     let a = 32;           
//     { 
//         println!("{}",a);  // 内部可以看见外部的 a，打印 32                  
//         let mut s = String::from("open");  
//         println!("修改前：{}",s);  // 打印 open
//         s.push_str("score");
//         println!("修改后：{}",s)   // 打印 openscore（注意这里少了个冒号，但不影响运行）
//     }                     // <--- 到这里，s 的内存被释放，s 彻底消失！                     
//     println!("最终：{}", a);   // 外部依然拥有 a，打印 32   
//     // println!("s最终{}",s);  // 把这行注释掉，程序就能完美运行了！
// }


// fn main() {
//     let a = 32;           
//     { 
//         println!("{}",a);                    
//         let mut a = String::from("open");  
//         println!("修改前：{}",a);  
//         a.push_str("score");
//         println!("修改后：{}",a)    
//     }                                      
//     println!("最终：{}", a);    
// }
//  Rust 里一个极其重要且高级的概念——变量遮蔽
//同时存在两个完全独立的变量，只是恰巧都叫 a
//是空间上的并存。编译器在栈上开辟了两个不同的变量空间
// 只要给它们起别名或通过引用，理论上还能找到前一个。
//外面的 a：类型是整数（32），住在函数的最外层。
//里面的 a：类型是字符串（"open"），住在里面的小花括号里。
//当小花括号里写 let mut a = String::from("open"); 时，这里并没有“修改”外面那个整数 a，
// 而是新创建了一个同名的变量，把外面那个 a 暂时“遮蔽（Shadow）”住了。
//###“内层可以临时‘顶替’外层的同名变量，但内层变量一死，外层原变量立刻‘复活’。”


                             // 所有权 规则的理论解析

                    // fn main() {
                    //     let a = 10u32;   // a 具有对 值 10u32 的所有
                    //     let b = a;       // 将值 10u32 复制了一份，b 具有了这个新的 10u32的所有权
                    //     println!("{}",a);
                    //     println!("{}",b);
                    // } // 当 main 函数结束后
                    // a b 两个变量离开了作用域
                    //其对应的两个 10u32 就被回收了
                    //这里是 栈帧 直接结束，栈帧 内存被回收
                    //局部变量所占用的内存就一起被回收
                        //字符串 例子
                        // fn main() {
                        //     let s1 = String::from("open");  // 变量 s1 具有对这个字符串的所有权
                        //     println!("{}",s1);
                        // } 
                        // s1 的作用域从定义到开始，直到花括号结束
                        // s1 （栈帧上的局部变量）离开作用域时，变量 s1 上绑定的内存资源（字符串），就被回收掉了
                        // 这里发生的事情是，栈帧中的局部变量开始离开作用域了，顺带要求 堆 内存中的字符串资源被回收了
                        //能够做到这一点，是因为这个 堆 中的字符串资源被栈帧中的局部变量指向了的
                        // 当然，从 Rust 语法来看：s1 拥有字符串的所有权，当s1离开作用域的时候，资源就自动被回收了。
// RAII
// 这种堆内存资源随着关联的栈上局部变量一起被回收的内存管理特性，叫做RAII（Resource Acquisition Is Initialization）
//它实际上并不是Rust的原创，是由C++创造的
//C语言里面必须由程序员手动在后面的代码中使用free()来释放堆内存资源，而RAII不需要手动写free()

 

// 使用所有权书写函数 （基于所有权规则，写函数）
// fn foo(s: String) {
//     println!("{s}");
// }

// fn main() {
//     let s1 = String::from("demo");
//     foo(s1);
// }
//  fn	      “我要定义一个名叫……”（这是 Rust 的关键字，表示“函数”）
//  foo	      “……叫做 foo 的函数”（函数的名字，可以随便起，就像变量名叫 a 或 s1 一样）
//  ( ... )	  “括号里是我要收的东西”（函数的“收件入口”）
//  s	      “我给它起了个临时名字叫 s”（这是函数内部使用的变量名，只在函数体里有效）
//  :	      “它的类型是……”（冒号后面跟类型，就像 let a: u32 一样）
//  String	  “它是一个 String 类型的字符串”（也就是那种储存在堆上的、可变长的文本）

//即可以把所有权移动进函数里，当然也可以把所有权转移出来
// fn foo(s:String) -> String {      // // 声明：我有个“出货口”，吐出来的是 String //函数签名里的 -> String，表示“这个函数承诺，等它结束时，会从门缝里塞出一张 String 类型的便利贴”。
//     println!("{}",s);
//     s   // 把 s 从“出货口”塞出去  // 注意：这里没有分号，表示“把这个值作为结果返回”
// }
// fn main() {
//     let s1 = String:: from("demo");
//     let s1 = foo(s1);   //// 左边的新 s1 接住了从门缝里塞出来的便利贴
//     println!("{}",s1);
// }
//-> String 不是用来“让字符串出现在屏幕上”的（那是 println! 的活）。
//-> String 是用来“把字符串活着送出函数大门，还给调用者”的。
//如果函数不写 -> String，就等于默认告诉编译器：
// “这门只进不出，进来的东西，关门时我直接砸烂销毁。”


                 
                 // 复制 还是 移动
        // 对于变量的绑定来说，
        // 哪些类型默认是做移动（所有权）操作
        // 哪些基本类型默认是做复制（而产生新的资源及所有权）操作
        // 赋值的时候，默认做复制操作的有：
                           // 所有的整数类型，比如u32
                           // 布尔类型，bool 
                           // 浮点数类型：f32，f64
                           // 字符类型 char
                           // 由以上类型组成的元组类型 Tuple ，如 i32 char
        // 其他类型，默认都是做值的移动操作


                 // 借用 与 引用
        // 借用和引用是一体两面，对同一个事情的两个面的描述
        // 你把东西借给别人，也就是别人持有了对你这个东西的引用
        // 借用：站在资源拥有者的角度
        // 引用：站在想借这个资源的变量角度

 
// 复制操作
// let b = &a;    //&a 不是壳子，是地址值。  //let b = &a; 不是“变身”，是“把地址值存入 b”。
// let c = b;   //let c = b; 不是“再套壳”，是“复印地址值给 c”。  //因为地址值太小（8字节），复印它合法，且原件（b）不受影响。
//内存地址 0x100 (a 的家): 存放着数字 10
//内存地址 0x200 (b 的家): 存放着数字 0x100 (这就是 &a)
//内存地址 0x300 (c 的家): 存放着数字 0x100 (复制 b 得到的)
//当你执行 let c = b; 时，电脑做的事情极其简单：
//去 b 的家（0x200）看一眼，里面写着 0x100。
//把这个数字 0x100 抄了一份，放到 c 的家（0x300）里去

//在 rust中，引用具体就在变量前用 & 符号 来表示，比如&x。
//其实，引用也是一种值，并且是固定长度的值
//既然是值，当然可以赋给另一个变量
//既然是固定长度的值。那其实做的是引用的复制操作
//在 Rust 底层，引用就是一个内存地址（指针），它的大小是固定的（通常是 8 个字节)
// fn main() {
//     let a = 10u32;  //
//     let b = &a;     //把 &b 这个新产生的值，赋给变量 d
//     let c = &&&&&a;
//     let d = &b;
//     let e = b;   // 把 b 的值（&a），赋给变量 e
    //如果 b 是一个 String（大块头），let e = b; 会把 b 作废（移动）
    //但 b 是一个 &a（引用，8字节的小不点），let e = b; 只是把门牌号抄了一份给 e。
    //结果：b 和 e 同时存在，都指向 a，谁也没作废。这就是“复制操作”！
//     println!("{}",a);
//     println!("{}",b);
//     println!("{}",c);   // 虽然套了5层，但解引用后地址还是 a 的地址
//     println!("{}",d);
//     println!("{}",e);
// }
//引用（&a）是啥？ 是一个 8 字节的门牌号（值）。
//门牌号能复制吗？ 能，因为它太小了，复印毫无成本（let e = b; 就是复印）。
//套多层壳（&&&&a）咋办？ 依然是门牌号，只是指向路径长了点，底层还是 8 字节。
//打印时为啥看到 10？ 因为 println! 自动帮你顺着门牌号找过去，并剥掉所有壳。

// fn main() {
//     let a = 10u32;
//     let b = &a;
//     let c = &&&&&a;
//     let d = &b;
//     let e = b;

//     // 打印这些变量本身存放的数值（也就是内存地址）
//     println!("b 里存的地址: {:p}", b);   // 打印 0x...（a 的地址）
//     println!("e 里存的地址: {:p}", e);   // 打印 0x...（和 b 完全一样！）
//     println!("d 里存的地址: {:p}", d);   // 打印 0x...（指向 b 的地址，但最终也通向 a）
//     println!("c 里存的地址: {:p}", c);   // 打印 0x...（虽然套了5层，底层还是 a 的地址）
// }
//                                                                    //解析 【从解释类型（层数）上 100% 正确】
                                                                   //     【从内存值（Value）来看，没有 壳子 这个概念，只是一张纸条】

                                //变量	    代码	           类型（“套娃层数”）	                   解释
                                //a	    let a = 10u32;	        u32（实心数字）	               就是那个光溜溜的木雕小人，数值是 10。
                                //b	    let b = &a;	            &u32（1 层壳）	               给小人套了 1 层包装盒。
                                //c	    let c = &&&&&a;	        &&&&&u32（5 层壳）	           给小人套了 5 层包装盒。即 a 前面有 5 个 &。
                                //d	    let d = &b;	    	    &&u32（2 层壳）                因为 b 本身是 1 层壳，再套 1 层，总共 2 层。
                                //e	    let e = b;	            &u32（1 层壳）	               因为 b 就是 1 层壳，直接复制了一份，所以还是 1 层。

// 字符串
// fn main() {
//     let s1 = String::from("demo");
//     let s2 = &s1;
//     let s3 = &&&&&s1;
//     let s4 = &s2;
//     let s5 = s2;
//     println!("{}",s1);
//     println!("{}",s2);
//     println!("{}",s3);
//     println!("{}",s4);
//     println!("{}",s5);
// }
//这些引用，都没有导致堆中的字符串资源被复制一份或多份
//字符串的所有权仍然在s1那里，其它s2 s3 s4 s5都是对这个所有权的引用
//在Rust中，一个所有权型变量（s1）带有值 和类型的信息，
//一个引用型变量（s2 s3 s4 s5）也带有值和类型的信息，不然它没法正确回溯到最终的值



// 不可变引用、可变引用
       //引用包含不可变引用和可变引用
       //&x 对变量的x的不可变引用
       //&mut x 对变量x的可变引用
//引用的scope特性
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{}",b);
}

fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{}",b);
    println!("{}",a);
}

fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{}",a);
    println!("{}",b);
}
 

fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a; 
}
 

fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a; 
    println!("{}",b);
} // 此代码无法编译，请勿修改，做样式示范
 
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a; 
    println!("{}",c);
} // 此代码可以正常编译


fn main() {
    let mut a = 10u32;
    let c = &a; 
    let b = &mut a;
    *b = 20;
     
    println!("{}",c);
} //此代码也无法编译，请勿修改，做样式示范

fn main() {
    let mut a = 10u32;
    let c = &a; 
    let b = &mut a;
    *b = 20;
     
    println!("{}",b);
} // 此代码可以正常编译

// 一个资源的可变引用与不可变引用的作用域不能交叠（overlap），也就是说不能同时存在
//引用的作用域是从它定义到它最后一次使用时结束，如果它定义了，却并没有被使用，那么它的作用域就只有它定义的那一行，即，出生即死亡

//再看一个例子 多个可变引用
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let d = &mut a;
    println!("{}",b);
} //此代码也无法编译，请勿修改，做样式示范

fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let d = &mut a;
    println!("{}",d);
}// 此代码可以正常编译

                         // 示例总结 关于引用（借用）的规则
                    //引用（不可变引用和可变引用都是）变量的作用域不会长于所有权变量的作用域，
                                //不然就会出现悬锤引用，导致出现典型的内存安全问题。Rust中的引用必定是有效的
                    //一个资源的可变引用与不可变引用的作用域不能交叠（overlap），也就是说不能同时存在
                    //某个时刻对某个资源只能存在一个可变引用，不能有超过一个可变引用同时存在
                    //一个资源的不可变引用，可以同时存在多个

// 可变引用的排它性
fn main() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    println!("{}",r1);
}

fn main() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    println!("{}",r2);
}
// 不可变引用可以被复制
// 可变引用不能被复制，只能被move


//使用 & 和 &mut 来改进函数的定义
  // 第一个例子：将字符串的不可变引用传进函数参数
  fn foo(s: &String) {
    println!("in fn foo:{s}");
  }
  fn main() {
    let s1 = String::from("demo");
    foo (&s1);
    println!("{s1}");
  }
   // 第二个例子 试试将字符串的可变引用传进函数，并修改字符串的内容
fn foo (s: &mut String) {
    s.push_str("demo");
}
fn main() {
    let mut s1 =String::from("demo test");
    println!("{s1}");
    foo(&mut s1);
    println!("{s1}");
}
 
// 函数的形参就是接受实参的过程，就是变量绑定值的过程
// 和以前那些普通的变量绑定是一个道理，不过函数的参数是这个新函数的局部变量
// 上述Rust代码非常清晰。如果一个函数参数接受的是可变引用，或所有权参数，
// 那么它里面的逻辑一般都会对引用的资源进行修改
//如果一个函数参数只接受不可变引用，那么它里面的逻辑，
//就一定不会修改被引用的资源 
