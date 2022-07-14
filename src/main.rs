fn middle_value() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
       positive if positive > 0 => Some(sum  / count as f32),
       _ => None
    };

    println!("{:?}", mean);
}

fn median() {
    let mut x = [1, 6, 9, 15, 7];
    x.sort();

    let count = x.len();
    match count {
        positive if positive > 0 => Some(println!("{}", x[count / 2])),
        _ => None
    };
    }
    
fn main() {
    middle_value();
    median();
}
