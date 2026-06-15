/* 
Add ```input-macro = "0.2"``` under [dependencies] in Cargo.toml
*/

use input_macro::input;

fn results(a: &i32, b: &&'static str, c: &i32, d: &i32) {
    println!("\n{} {} {} Equals: {}.", c, b, d, a,);
}

fn main() {
    println!("Cool calculator\n");
    let first_num: i32 = input!("Enter first number: ").parse().unwrap();
    let math = input!("Enter math calc: ");
    let second_num: i32 = input!("Enter second number: ").parse().unwrap();
    if math == "+" {
        println!("DEBUG: plus");
        let result = first_num + second_num;
        let math_result: &'static str = "plus";
        results(&result, &math_result, &first_num, &second_num);
    }
    else if math == "-" {
        println!("DEBUG: minus");
        let result = first_num - second_num;
        let math_result: &'static str = "minus";
        results(&result, &math_result, &first_num, &second_num);
    }
    else if math == "*" {
        println!("DEBUG: times");
        let result = first_num * second_num;
        let math_result: &'static str = "times";
        results(&result, &math_result, &first_num, &second_num);
    }
    else if math == "/" {
        println!("DEBUG: devide");
        let result = first_num / second_num;
        let math_result: &'static str = "devided by";
        results(&result, &math_result, &first_num, &second_num);
    }
    else {
        println!("error");
    }
}
