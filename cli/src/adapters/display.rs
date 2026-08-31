use crate::domain::ports::DisplayPort;
use colored::*;

pub struct RealDisplay;

impl DisplayPort for RealDisplay {
    fn print_info(&self, msg: &str) {
        println!("{}", msg.cyan());
    }

    fn print_success(&self, msg: &str) {
        println!("{}", msg.green());
    }

    fn print_warning(&self, msg: &str) {
        println!("{}", msg.yellow());
    }

    fn print_error(&self, msg: &str) {
        eprintln!("{}", msg.red());
    }

    fn print_dry_run(&self, msg: &str) {
        println!("{}", msg.yellow());
    }

    fn print_table(&self, headers: &[&str], rows: Vec<Vec<String>>) {
        let mut table = comfy_table::Table::new();
        table.set_header(headers.iter().map(|h| h.cyan().to_string()).collect::<Vec<_>>());
        for row in rows {
            table.add_row(row);
        }
        println!("{table}");
    }
}
