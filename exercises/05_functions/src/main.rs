fn main() {
    // print_labeled_measurement(5, 'h');
    f(0);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value is {value}{unit_label}");
}

// does not compile; does not have type hinting
// fn f(x) {
//     println!("{x}");
// }

