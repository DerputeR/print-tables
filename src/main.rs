use std::fmt::Alignment;
use comfy_table::{modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS}, presets::*, ContentArrangement, Table};
use tabled::{builder::Builder, settings::{measurement::Percent, themes::Colorization, Color, Style, Width}};

pub struct TableFormatOptions {
    padding: u8,
    use_first_row_as_header: bool,
    draw_border: bool,
    align: Alignment
}

pub const DEFAULT_TABLE_FORMAT: TableFormatOptions = TableFormatOptions{
    padding: 2,
    use_first_row_as_header: true,
    draw_border: false,
    align: Alignment::Left
};

// TODO
pub fn stringify_table<T>(table: &[T], row_size: u32, format_options: &TableFormatOptions) -> String {
    String::from("")
}

// fn get_max_char_len<T>(table: &[T], row_size: u32) -> u32 {
//     for r in 0..row_size {

//     }
// }


#[cfg(test)]
mod tests {
    


}

fn main() {
    let header = vec!["Name", "PID", "Uptime", "Message", "Status"];
    let contents = vec![
        vec!["gtrand2", "71", "21.100 seconds", "starting! this is also a really long message but we won't try doing manual line breaks with it on purpose", "Running"],
        vec!["gtrand", "73", "21.146 seconds", "starting!", "Running"]
     ];
    let mut table = Table::new();
    table
        .load_preset(NOTHING)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(&header)
        .add_rows(&contents);

    println!("{table}\n\n");
    table.load_preset(ASCII_FULL_CONDENSED);
    println!("{table}\n\n");
    table.load_preset(UTF8_FULL);
    println!("{table}\n\n");
    table.apply_modifier(UTF8_ROUND_CORNERS).apply_modifier(UTF8_SOLID_INNER_BORDERS);
    println!("{table}\n\n");

    let mut table_builder = Builder::new();
    table_builder.push_record(header.clone());
    table_builder.push_record(contents[0].clone());
    let mut table2 = table_builder.build();
    table2.with(Style::sharp());
    table2.with(Width::wrap(Percent(100)));
    println!("{table2}");
    table2.with(Style::empty());
    table2.with(Colorization::by_row([Color::BG_WHITE]));
    println!("{table2}");
}
