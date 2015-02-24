use std::num::Float;

use super::{Item, ItemSchedData};
use time::now_utc;

static INITIAL_FF: f64 = 2.5;

pub fn init_item() -> ItemSchedData {
    ItemSchedData {
        last_reviewed: now_utc().to_timespec(),
        ff: INITIAL_FF,
        int_step: 0,
        iri: 0.0,
    }
}

pub fn assess_item(data: &ItemSchedData, fam: u8) -> ItemSchedData {
    let int_step = if fam < 2 {
        1
    } else {
        data.int_step + 1
    };

    let iri = match int_step {
        1 => 1.0,
        2 => 3.0,
        _ => data.iri + data.ff
    };

    // SM-2 algorithm says only to adjust if familiar was above a certain
    // threshold. haven't thought about what effect this has yet.
    // TODO: revisit

    let ff = data.ff + match fam {
        0 => 0.,
        1 => 0.,
        2 => -0.32,
        3 => -0.14,
        4 => 0.,
        5 => 0.1,
        _ => unreachable!("Internal error: managed to reach unreachable code. I'm impressed.")
    };

    ItemSchedData {
        last_reviewed: now_utc().to_timespec(),
        ff: ff,
        int_step: int_step,
        iri: iri,
    }
}


pub fn list_display_item(item: Item) -> String {
    format!("{:3} : {}", item.id, item.desc)
}

pub fn full_display_item(item: Item) -> String {
    let dur = now_utc().to_timespec() - item.data.last_reviewed;
    format!("{:3} : {}\nLast reviewed: {} hours ago\nFF: {}\nint_step: {}\nIRI: {}",
            item.id, item.desc, dur.num_hours(), item.data.ff,
            item.data.int_step, item.data.iri)
}

pub fn filter_review_items(items: Vec<Item>) -> Vec<Item> {
    let curr_time = now_utc().to_timespec();

    fn days_to_seconds(days: f64) -> i64 {
        (days * 86400.0).round() as i64
    }

    items.into_iter()
         .filter(|i| (curr_time - i.data.last_reviewed).num_seconds() > days_to_seconds(i.data.iri))
         .collect()
}


pub fn review_display_item(review_num: usize, item: &Item) -> String {
    format!("\n{:3} - {} (id: {})\n\n0-5 or 's' to skip item or 'q' to quit > ",
            review_num, item.desc, item.id)
}
