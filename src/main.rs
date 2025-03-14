use std::fmt::Alignment;
use comfy_table::{modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS}, presets::*, Cell, ContentArrangement, Table};
use tabled::{builder::Builder, settings::{measurement::Percent, object::Rows, themes::Colorization, Color, Style, Width}};

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
    let header = vec!["Name", "UID", "Uptime", "Message", "Status"];
    let contents = vec![
        vec!["ssh_server", "71", "21.100 seconds", "starting! this is also a really long message but we won't try doing manual line breaks with it on purpose", "Running"],
        vec!["steamcmd", "73", "21.146 seconds", "starting!", "Running"],
        vec!["fax_server", "88", "1.411 seconds", "", "Stopped"],
     ];
    let mut table = Table::new();
    table
        .load_preset(ASCII_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(&header)
        .add_rows(&contents);
    println!("{table}\n\n");
    table.load_preset(UTF8_FULL);
    println!("{table}\n\n");
    table.apply_modifier(UTF8_ROUND_CORNERS).apply_modifier(UTF8_SOLID_INNER_BORDERS);
    println!("{table}\n\n");
    table.load_preset(NOTHING);
    println!("{table}\n\n");

    let mut table2 = Table::new();
    let mut header2 = Vec::<Cell>::new();
    for i in header {
        header2.push(Cell::new(&i).add_attribute(comfy_table::Attribute::Reverse));
    }

    table2.load_preset(NOTHING)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(header2)
        .add_rows(&contents)
        // .add_row(vec![Cell::new("Test!").add_attribute(comfy_table::Attribute::SlowBlink)])
        ;

    let stringed = format!("{table2}\n\n");
    println!("{}", stringed);
}
