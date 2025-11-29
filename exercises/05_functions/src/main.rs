fn main() {
    print_labeled_measurement(5, 'h');
    // f(0);
    for cond in [true, false] {
        println!("cond = {cond}");
        try_1(cond);
        try_2(cond);
    }
    // just checking ... 
    let x;
    let cond: bool = false;
    if cond { x = 5; } else { x = 6; }
    println!("{x}"); // 6
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value is {value}{unit_label}");
}

// does not compile; does not have type hinting
// fn f(x) {
//     println!("{x}");
// }

fn try_1(cond: bool) {
    let x = if cond { 1 } else { 2 };
    println!("{x}");
}

fn try_2(cond: bool) {
    let x;
    if cond {
        x = 1;
    } else {
        x = 2;
    }
    println!("{x}");
}