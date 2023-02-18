# pakbon-parser

Little parsing thing for Albert Heijn's "hier is uw pakbon" emails.

Collects all the prices from exported raw emails and generates a nice little CSV report with each product's price from each order.

Contains probably the worst Rust code you've seen in your life.

## Usage

1. Export your "pakbon" emails. In Apple Mail, you can search for them, then select all, and go File -> Save As. Use the "raw message" format. Save wherever
2. `cargo run -- my-raw-pakbon-emails > report.csv`
