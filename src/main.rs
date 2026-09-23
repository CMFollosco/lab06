mod report;
mod supplies;

fn main() {
    println!("Oregon Trail Supply Check 1");

    let party: u32 = 4;
    let days: u32 = 30;

    let food = supplies::food_needed(party, days);

    report::print_report(party, days, food);
}
