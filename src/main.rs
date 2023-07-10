fn main() {
    exercise3()
}

fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<_> = vec![0];

    println!("{:?}", values_number);

    // println!("add {:?}", &additions.len());
    let len_add = additions.len() as i32;
    println!("add {:?}", len_add);

    while len_add > 0 {
        // println!("add {:?}", additions.len());
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for element_index in additions.iter() {
            // println!("index {:?}", element_index);
            let addition_aux: f64 = values[*element_index];
            println!("add res {:?}", addition_aux);
            addition = addition_aux + addition;
        }
    }
}
