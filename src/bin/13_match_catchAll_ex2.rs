//catch all pattern with the  "_" placeholder in match
// See listing below 6-5 in book for information on use case for below

fn main() {
    let dice_roll = 9;

    // match has to be exhaustive, that is account for all possible values
    match dice_roll {
        // It is syntactically possible to place a function to the right of the => operator
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // "_" used for all other cases where we do not need to use the value
        // passed for further evaluation 
        _ => re_roll()
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn re_roll() {}