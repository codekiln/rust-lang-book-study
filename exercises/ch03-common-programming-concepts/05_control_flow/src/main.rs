fn conditional_branching() {
    println!("Conditional Branching:");
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn returning_values_from_loops() {
    println!("returning_values_from_loops:");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // semicolon necessary because this is a statement

    println!("The result is {result}"); // 20
}

fn looping_with_while() {
    println!("Looping with while:");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_with_for() {
    println!("Looping with for:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_labels_to_disambiguate_multiple_loops() {
    println!("Loop labels to disambiguate multiple loops:");
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn looping_with_for_range() {
    println!("Looping with for and range:");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn main() {
    let testing_funcs = [
        conditional_branching,
        returning_values_from_loops,
        looping_with_while,
        looping_with_for,
        loop_labels_to_disambiguate_multiple_loops,
        looping_with_for_range
    ];
    for func in testing_funcs {
        func();
        println!("----------------------------");
    }
}
