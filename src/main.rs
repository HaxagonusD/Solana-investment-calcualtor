fn main() {
    let mut interest_rate = 0.08;
    let final_interest_rate = 0.015;
    let yearly_solana_increase = 20.0;
    let monthly_solana_increase = yearly_solana_increase / 12.0;
    let interest_rate_decrease = (interest_rate - final_interest_rate) / 10.0;
    let mut invesment_amount = 0.0;
    let length_of_time = 10.0;
    let invesment_monthly_dollars = 7000.0;
    let mut current_price_of_solana = 200.0;
    let mut invesment_monthly_solana = invesment_monthly_dollars / current_price_of_solana;
    let mut if_five = 0.0;

    for year in 1..=(length_of_time * 12.0) as i32 {
        invesment_amount += invesment_monthly_solana;
        invesment_amount *= 1.0 + interest_rate / 12.0;

        current_price_of_solana *= 1.0 + (monthly_solana_increase / 100.0) as f32;
        invesment_monthly_solana = invesment_monthly_dollars / current_price_of_solana;
        // println!("invesment_monthly_solana: {}", invesment_monthly_solana);
        if_five += current_price_of_solana * 5.0;

        println!(
            "How much money to buy 5 solana every month: {}, in year {}",
            current_price_of_solana * 5.0,
            year / 12
        );

        // invesment_amount += 12.0 * invesment_monthly_dollars;
        // invesment_amount *= 1.0 + interest_rate;
        if interest_rate > final_interest_rate {
            interest_rate -= interest_rate_decrease
        }
    }

    println!("if_five: {}", if_five);

    let total_return = invesment_amount * current_price_of_solana as f32;
    let total_invested = invesment_monthly_dollars * length_of_time * 12.0;
    let profit = total_return - total_invested;
    let its_like = profit / length_of_time;

    println!("invesment_amount: {}", invesment_amount);

    println!("current_price_of_solana: {}", current_price_of_solana);

    println!("Total return: {} us dollars", total_return);
    println!("Total invested: {} us dollars", total_invested);
    println!(
        "Difference {} us Dollars over {} years",
        profit, length_of_time
    );

    println!(
        "How much you would have made every year: {} us dollars",
        its_like
    );
}
