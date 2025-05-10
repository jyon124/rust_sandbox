/// This is comment for the function
fn main() {
    scope();
    data_types();
}

fn scope() {
    let x = 5;
    let x = x + 1;
    print!("{}\n", x);
    {
        let x = x * 2;
        print!("{}\n", x);
        {
            print!("{}\n", x);
            // x + 100;
            print!("{}\n", x);
            let x = x * 200;
            {
                print!("{}\n", x);
            }
        }
    }
    print!("{}\n", x);

    let sn: u32 = "42".parse().expect("NaN");
    print!("{}\n", sn);

    let arbitrary_num: u32 = 1_000_000;
    print!("{}\n", arbitrary_num);
}

fn data_types() {
    let sum = 5 + 10;
    println!("sum: {}", sum);
    let diff = 95.5 - 4.3;
    println!("diff: {}", diff);
    let prod = 4*30;
    println!("prod: {}", prod);
    let quotient = 56.7/32.2;
    println!("quotient: {}", quotient);
    let trunc = -5/3;
    println!("trunc: {}", trunc);
    let remainder = 43%5;
    println!("remainder: {}", remainder);

    let a: [i32; 5] = [1,2,3,4,5];
    println!("arr: {:?}", a); // for array to print, you need ':?' inside bracket
}