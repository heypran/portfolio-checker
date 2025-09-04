use prettytable::{Table, Row, Cell};
use std::collections::HashMap;

pub fn print_balances(token_balance_map:HashMap<String,u128>){
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Token"),
        Cell::new("Balance"),
    ]));

    // Add data rows
    for (token, balance) in &token_balance_map {
        table.add_row(Row::new(vec![
            Cell::new(token),
            Cell::new(&format!("{:.1}", balance)),  // Format balance
        ]));
    }

    // Print to terminal
    table.printstd();
}
