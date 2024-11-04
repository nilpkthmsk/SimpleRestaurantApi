
use rusqlite::{Result, Statement};

use crate::database;

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub menu: String,
    pub is_served: bool,
    pub time_estimate: u32,
}

impl Order {
    /**
     * Create order to table number
     */
    pub fn create(table_number: u32, orders: Vec<Order>) -> Result<String> {
        let connection = database::open()?;
        let json: String = String::from("{'status': 'success'}");

        // Loop all menu and insert into order table
        for order in orders {
            connection.execute("INSERT INTO orders (table_number, menu, is_served, time_estimate) VALUES (?1, ?2, ?3, ?4)", (table_number, order.menu, order.is_served, order.time_estimate))?;
        }

        Ok(json)
    }

    /**
     * Retrieve orders from specific table number
     * is_all: true -> retrieve all orders
     */
    pub fn retrieve(table_number: u32, is_all: bool) -> Result<String> {
        let connection = database::open()?;
        let mut json: String = String::from("");
        let mut statement: Statement<'_>;

        json.push_str(&format!("{{'table': {}, 'order': [", table_number));

        if is_all {
            statement = connection.prepare("SELECT id, menu, is_served, time_estimate FROM orders WHERE table_number=:table_number")?;
        } else {
            statement = connection.prepare("SELECT id, menu, is_served, time_estimate FROM orders WHERE table_number=:table_number AND is_served = false")?;
        }

        let orders = statement.query_map(&[(":table_number", table_number.to_string().as_str())], |row| {
            Ok(Order {
                id: row.get(0)?,
                menu: row.get(1)?,
                is_served: row.get(2)?,
                time_estimate: row.get(3)?
            })
        }).unwrap();

        for order_wrapped in orders {
            let order = order_wrapped.unwrap();
            // json.push_str(&format!("<Menu id='{}' name='{}' price='{}' />", menu.id, menu.name, menu.price));
            json.push_str(&format!("{{'id':{}, 'name':'{}'', 'is_served':'{}', 'time_estimate':'{}'",
                    order.id, order.menu, order.is_served, order.time_estimate));
        }

        // json.push_str("</Menus>");
        json.push_str("]}");

        Ok(json)
    }

    /**
     * Retrieve orders from specific table number
     * is_all: true -> retrieve all orders
     */
    pub fn retrieve_one(table_number: u32, order_id: u64) -> Result<String> {
        let connection = database::open()?;
        let mut json: String = String::from("");
        let mut statement: Statement<'_> = connection.prepare("SELECT id, menu, is_served, time_estimate FROM orders WHERE table_number=:table_number AND id=:id")?;

        json.push_str(&format!("{{'table': {}, 'order': {{", table_number));

        let orders = statement.query_map(&[
            (":table_number", table_number.to_string().as_str()),
            (":id", order_id.to_string().as_str())], |row| {
                Ok(Order {
                    id: row.get(0)?,
                    menu: row.get(1)?,
                    is_served: row.get(2)?,
                    time_estimate: row.get(3)?
                })
            }
        ).unwrap();

        for order_wrapped in orders {
            let order = order_wrapped.unwrap();
            // json.push_str(&format!("<Menu id='{}' name='{}' price='{}' />", menu.id, menu.name, menu.price));
            json.push_str(&format!("{{'id':{}, 'name':'{}'', 'is_served':'{}', 'time_estimate':'{}'",
                    order.id, order.menu, order.is_served, order.time_estimate));
        }

        // json.push_str("</Menus>");
        json.push_str("}}");

        Ok(json)
    }

    /**
     * Update the specific order
     * only available for updating is_served for now.
     */
    pub fn update(table_number: u32, order_id: u64) -> Result<String> {
        let connection = database::open()?;
        let json: String = String::from("{'status': 'success'}");

        // Update, set is_served to true
        connection.execute("UPDATE orders SET is_served=?1 WHERE table_number=?2 AND id=?3", (true, table_number, order_id))?;

        Ok(json)
    }

    /**
     * Delete the specific order
     */
    pub fn delete(table_number: u32, order_id: u64) -> Result<String> {
        let connection = database::open()?;
        let json: String = String::from("{'status': 'success'}");

        // Delete from orders table
        // PS. Passing only order_id cause compilation error.
        connection.execute("DELETE FROM orders WHERE table_number=?1 AND id=?2", (table_number, order_id))?;

        Ok(json)
    }

    // For Client

    /**
     * Retrieve orders from specific table number
     * is_all: true -> retrieve all orders
     */
    pub fn retrieve_html(table_number: String, is_all: bool) -> Result<String> {
        let connection = database::open()?;
        let mut tag: String = String::from("
            <table border='1'>
                <theader>
                    <tr>
                        <th>Menu</th>
                        <th>Time Estimate</th>
                        <th>Action</th>
                    </tr>
                </theader>
                <tbody>");
        let mut statement: Statement<'_>;

        if is_all {
            statement = connection.prepare("SELECT id, menu, is_served, time_estimate FROM orders WHERE table_number=:table_number")?;
        } else {
            statement = connection.prepare("SELECT id, menu, is_served, time_estimate FROM orders WHERE table_number=:table_number AND is_served = false")?;
        }
        
        let orders = statement.query_map(&[(":table_number", table_number.to_string().as_str())], |row| {
            Ok(Order {
                id: row.get(0)?,
                menu: row.get(1)?,
                is_served: row.get(2)?,
                time_estimate: row.get(3)?
            })
        }).unwrap();

        for order_wrapped in orders {
            let order = order_wrapped.unwrap();
            tag.push_str(&format!("
                <tr>
                    <td>{0}</td>
                    <td>{1}min</td>
                    <td>", order.menu, order.time_estimate));

            if !order.is_served {
                tag.push_str(&format!("
                        <span>
                            <form action='/client/table/{}/order/{}/serve' method='GET'>
                                <input type='submit' value='Serve'/>
                            </form>
                        </span>", table_number, order.id));
            }

            tag.push_str(&format!("
                        <span>
                            <form action='/client/table/{}/order/{}/delete' method='GET'>
                                <input type='submit' value='Delete'/>
                            </form>
                        </span>
                    </td>
                </tr>", table_number, order.id));
        }

        tag.push_str(&format!("
                </tbody>
            </table>"));   

        Ok(tag)
    }
}
