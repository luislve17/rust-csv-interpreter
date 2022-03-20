use regex::Regex;

fn resolve_quoted_items (items_row: &mut Vec<String>) -> &mut Vec<String> {
    let quote_search = items_row.iter().position(|r| r.contains("\""));
    let mut quote_left_limit = 0 as usize;
    let mut quote_right_limit = 0 as usize;
    return match quote_search {
        Some(_) => {
            quote_left_limit = quote_search.unwrap();
            quote_right_limit = items_row[(quote_left_limit + 1)..].iter().position(|r| r.contains("\"")).unwrap() + quote_left_limit + 1;

            let merged_string = items_row[quote_left_limit..(quote_right_limit + 1)].join("");
            items_row[quote_left_limit] = merged_string;
            items_row.drain((quote_left_limit + 1)..(quote_right_limit + 1));

            let row_reminder = &mut items_row[(quote_left_limit + 1)..].to_vec();
                let resolved_reminder = resolve_quoted_items(row_reminder);
                items_row.drain((quote_left_limit + 1)..);
                items_row.append(resolved_reminder);
                items_row
            }
        None => { items_row }
    }
}

pub fn read_csv(raw_data: String){
    let line_regex: &str = "(?P<data>.*?)\\n";
    let line_re = Regex::new(line_regex).unwrap();

    for line in line_re.captures_iter(&raw_data) {
        let mut items = line["data"].split(',').map(str::to_string).collect::<Vec<String>>();
        let processed_items = resolve_quoted_items(&mut items);
        println!("{:?}", processed_items);
    }
}
