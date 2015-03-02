extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate rusqlite;
extern crate time;

use docopt::Docopt;
pub use time::Timespec;
pub use conductor::{Item, ItemId, ItemSchedData};

use conductor::Conductor;

mod conductor;
mod core;
mod persist;
mod fuzzy;

static USAGE: &'static str = "
Usage:
  hippo review [<N> | --id=<id>]
  hippo add <description>
  hippo edit <id> <description>
  hippo view <id>
  hippo remove <id>
  hippo list [--unreviewed --fuzzy] [<string>]
  hippo (-h | --help)

Options:
  -h, --help    Show this screen.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_description: Option<String>,
    arg_id: Option<String>,
    arg_N: Option<String>,
    arg_string: Option<String>,
    flag_id: Option<String>,
    flag_unreviewed: bool,
    flag_fuzzy: bool,
    cmd_add: bool,
    cmd_edit: bool,
    cmd_view: bool,
    cmd_remove: bool,
    cmd_list: bool,
    cmd_review: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let default_review_num = 20;

    let cond = Conductor::new();

    if args.cmd_add {
        let desc_string = args.arg_description.unwrap();
        let desc = desc_string.as_slice();
        cond.add_item(desc);

    } else if args.cmd_edit {
        let id = args.arg_id.unwrap().as_slice().parse().unwrap();
        let desc_string = args.arg_description.unwrap();
        let desc = desc_string.as_slice();
        cond.edit_item(id, desc);

    } else if args.cmd_view {
        let id = args.arg_id.unwrap().as_slice().parse().unwrap();
        cond.view_item(id);

    } else if args.cmd_remove {
        let id = args.arg_id.unwrap().as_slice().parse().unwrap();
        cond.remove_item(id);

    } else if args.cmd_list {
        cond.list_items(args.arg_string, args.flag_unreviewed, args.flag_fuzzy);

    } else if args.cmd_review {
        match args.arg_N {
            Some(n) => cond.review(n.as_slice().parse().unwrap()),
            None =>
                match args.flag_id {
                    Some(id) => cond.review_item(id.as_slice().parse().unwrap()),
                    None     => cond.review(default_review_num),
                }
        }

    } else {
        println!("No command provided");
    }
}
