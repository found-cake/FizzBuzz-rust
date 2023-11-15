fn main() {
    // 1 ~ 100
    for i in 1..101 {
        // 3의 배수 이면서 5의 배수
        if i % 3 == 0 && i % 5 == 0 {
            println!("PizzBuzz");
        // 3의 배수
        } else if i % 3 == 0 {
            println!("Pizz");
        // 5의 배수
        } else if i % 5 == 0 {
            println!("Buzz");
        // 숫자 출력
        } else {
            println!("{}", i);
        }
    }
}
