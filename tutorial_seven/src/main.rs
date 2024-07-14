fn main() {
    // let cond = 2 < 3;
    let cond = (2 as f32) <= 2.2;

    println!("{}", cond);

    // Compound Conditions
    // && 'and'
    // || 'or'
    // ! 'not'

    // let cond2 = true && cond;
    // let cond2 = false && cond;
    // let cond2 = true || cond;
    // let cond2 = !(true || cond);
    let cond2 = true || !cond;

    // Order to apply the conditions in:
    // ! 'first' 
    // && 'second'
    // || 'last'

    println!("{}", cond2);

    // Control Flow
    // let food = "cookie";
    let food = "fruit";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else {
        println!("Oh, that's too bad!");
    }
}
