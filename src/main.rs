// use std::fs;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, PartialEq, Eq)]
struct MigrationInfo<'a> {
    order: usize,
    direction: &'a Direction,
    name: &'a String,
}


/// main doc
fn main() {
    // migration +1
    // migration -1
    // migration up
    // migration down
    // migration 0004
    
    // list migration files
    // make migration order
    //

    // let migrations = load_migrations();
    // let current_migration = load_current_migration();
    // let destination_migration = parse_args();

    // destination_migration
}

/// load_migration_info doc
// fn load_migrations() -> HashMap<usize, HashMap<Direction, MigrationInfo>> {
//     let mut migrations: HashMap<usize, HashMap<Direction, MigrationInfo>> = HashMap::new();

//     let filenames = fs::read_dir("migrations").unwrap();
    
//     for entry_res in filenames {
//         let entry = entry_res.unwrap();
//         let filename_buf = entry.file_name();
//         let info = parse_migration_filename(filename_buf.to_str().unwrap());
        
//         let mut m = HashMap::new();
//         m.insert(info.direction, info);
//         migrations.insert(info.order, m);
//     }

//     migrations
// }


fn load_migrations<'a>(filenames: Vec<String>) -> HashMap<usize, HashMap<Direction, &'a MigrationInfo>> {
    let mut migrations: HashMap<usize, HashMap<Direction, &MigrationInfo>> = HashMap::new();

    for filename in filenames {
        let info = parse_migration_filename(&filename.to_string());
        let mut m = HashMap::new();
        m.insert(info.direction, &info);
        migrations.insert(info.order, m);
    }

    migrations
}


fn parse_migration_filename(filename: &str) -> MigrationInfo {
    let (order_str, postfix) = filename.split_at(5);

    let error_format_message = "Migration filename should be format xxxx_[forward|backward]_xxxx.sql";

    let index = postfix.find('_').expect(&error_format_message);
    let (direction_str, fname) = postfix.split_at(index);

    let index = fname.rfind('.').expect(&error_format_message);
    let (name, _) = fname.split_at(index);

    let order = order_str[0..4].parse::<usize>().ok().expect(&error_format_message);
    let direction = match direction_str {
        "forward" => Direction::Forward,
        "backward" => Direction::Backward,
        _ => panic!(""),
    };

    let (_, name) = name.split_at(1);
    MigrationInfo {order: order , direction: direction, name: name.to_string() }
}


#[test]
fn test_parse_migration_filename() {
    let actual = parse_migration_filename("0001_forward_app_datas__add_ymd.sql");
    let info = MigrationInfo { order: 1, direction: Direction::Forward, name: "app_datas__add_ymd".to_string() };
    assert_eq!(info, actual);
}

// fn list_migration_dir() -> Vec<String> {
    
// }

// fn load_current_migration() {

// }

// fn parse_args() {

// }

