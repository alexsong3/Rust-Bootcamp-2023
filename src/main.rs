fn main() {
    // println!("Hello, Rust Bootcamp by VBI Academy!");

    // assert_eq!(count_char_occurrences("Hello", 'l'), 2);
    // assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);

    // assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12);
    // assert_eq!(sum_even_numbers(&[10, 20, 30, 40, 50]), 150);

    // let numbers = [2.5, 4.8, 6.3, 1.7, 3.9];
    let numbers = [];
    let result = calculate_average(&numbers);
    // println!("result {:?}", result);
    assert_eq!(result, 3.84);
}

// fn count_char_occurrences(string: &str, ch: char) -> usize {
//     // todo!()
//     let res = string.chars().into_iter().filter(|c| c == &ch).count();
//     println!("count res {:?}", res);
//     res
//     // 0
// }

// fn sum_even_numbers(numbers: &[i32]) -> i32 {
//     // todo!()
//     let total: i32 = numbers.iter().filter(|n| (*n % 2) == 0).sum();
//     println!("even {:?}", total);
//     total
// }

fn calculate_average(numbers: &[f64]) -> f64 {
    // todo!()

    // let total: f64 = numbers.iter().sum();
    // println!("total {:?}", total);
    // let res = (total / numbers.len() as f64) as f64;
    // println!("average {:?}", res);
    // res
    // 0.0

    let total = ((numbers.iter().sum::<f64>() as f64) / (numbers.len() as f64)) as f64;
    // println!("total {:?}", total);
    if total > 0.0 {
        total
    } else {
        0.0
    }
}
