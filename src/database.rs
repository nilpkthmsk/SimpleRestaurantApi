
use rusqlite::{Connection, Result};

use crate::model::menu::Menu;

const DATABASE: &str = "restaurant.db3";

pub fn initialize() -> Result<()> {
    // let connection = Connection::open_in_memory()?;
    let connection: Connection = open()?;
    
    // Create tables
    // - menu
    connection.execute(
        "CREATE TABLE menu (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            price REAL NOT NULL
        )",
        (),
    )?;

    // - order
    connection.execute(
        "CREATE TABLE orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            table_number INTEGER NOT NULL,
            menu TEXT NOT NULL,
            is_served BOOL NOT NULL,
            time_estimate INTEGER NOT NULL
        )",
        (),
    )?;

    // Create demo data
    // - menu
    let menus: Vec<Menu> = Menu::generate_demo_data();
    for menu in menus {
        connection.execute("INSERT INTO menu (name, price) VALUES (?1, ?2)", (&menu.name, &menu.price))?;
    }

    Ok(())
}

pub fn open() -> Result<Connection> {
    let path = DATABASE;
    let connection = Connection::open(&path)?;

    Ok(connection)
}
