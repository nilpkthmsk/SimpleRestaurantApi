
use rusqlite::Result;

use crate::database;

#[derive(Debug)]
pub struct Menu {
    pub id: u32,
    pub name: String,
    pub price: f32,
}

impl Menu {
    /**
     * Generate demo data
     */
    pub fn generate_demo_data() -> Vec<Menu> {
        let menus: Vec<Menu> = vec![
            Menu {
                id: 0,
                name: "Papaya salad".to_string(),
                price: 800.0,
            },
            Menu {
                id: 0,
                name: "Red curry rice".to_string(),
                price: 1000.0,
            },
            Menu {
                id: 0,
                name: "Padthai".to_string(),
                price: 800.0,
            },
            Menu {
                id: 0,
                name: "Tom yum goong".to_string(),
                price: 1200.0,
            },
            Menu {
                id: 0,
                name: "Grilled chicken".to_string(),
                price: 1000.0,
            },
        ];
    
        return menus;
    }

    /**
     * Retrieve all menus from database to json format
     */
    pub fn retrieve() -> Result<String> {
        let connection = database::open()?;
        let mut json: String = String::from("{'items': [");
        let mut statement = connection.prepare("SELECT id, name, price FROM menu")?;

        // Query
        let menus = statement.query_map([], |row| {
            Ok(Menu {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
            })
        }).unwrap();

        for menu_wrapped in menus {
            let menu = menu_wrapped.unwrap();
            json.push_str(&format!("{{'id':{}, 'name':'{}', 'price':'{}'}},", menu.id, menu.name, menu.price));
        }

        json.push_str("]}");

        Ok(json)
    }

    /**
     * Retrieve all menus from database to form
     */
    pub fn retrieve_form() -> Result<String> {
        let connection = database::open()?;
        let mut form: String = String::from("");
        let mut statement = connection.prepare("SELECT id, name, price FROM menu")?;
        let mut index = 0;

        // Query
        let menus = statement.query_map([], |row| {
            Ok(Menu {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
            })
        }).unwrap();

        for menu_wrapped in menus {
            let menu = menu_wrapped.unwrap();
            form.push_str(&format!("
                <div>
                    <span>{}({} yen)</span>
                    <input id='menus[]' name='menus_{}' type='hidden' value='{}'>
                    <input id='amount[]' name='amount_{}' type='number' value='0' max='10' min='0'>
                </div>", menu.name, menu.price, index, menu.name, index));

            // Prefer... but not working
            // <input name='orders[][menu]' type='hidden' value='{}'>
            // <input name='orders[][amount]' type='number' value='0' max='10' min='0'>

            index += 1;
        }


        Ok(form)
    }
}
