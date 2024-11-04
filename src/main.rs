
mod database;
mod model;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer};
use rand::Rng;
use serde::Deserialize;

use crate::model::{
    menu::Menu,
    order::Order,
};

#[derive(Deserialize)]
struct InfoClientTable {
    table_number: String,
    all: Option<bool>,
}

#[derive(Deserialize)]
struct InfoClientTableOrder {
    table_number: String,
}

#[derive(Deserialize)]
struct InfoApiTable {
    all: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct InfoApiOrder {
    // // 1.
    // orders: Option<Vec<(String, u32)>>,
    // // 2.
    // menus: Option<Vec<String>>,
    // amount: Option<Vec<u32>>,

    // TODO: Change to array, as the above array are not working at the moment
    menus_0: String,
    menus_1: String,
    menus_2: String,
    menus_3: String,
    menus_4: String,
    
    amount_0: u32,
    amount_1: u32,
    amount_2: u32,
    amount_3: u32,
    amount_4: u32
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = database::initialize();

    HttpServer::new(|| {
        App::new()
            .service(api_menu_retrieve)
            .service(api_table_order_retrieve)
            .service(api_table_order)
            .service(api_table_order_retrieve_one)
            .service(api_table_order_update)
            .service(api_table_order_delete)
            .service(client)
            .service(client_table_selection)
            .service(client_table)
            .service(client_table_order)
            .service(client_table_order_update)
            .service(client_table_order_delete)
    })
    .workers(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/api/menu")]
async fn api_menu_retrieve() -> HttpResponse {
    let body = Menu::retrieve().unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/api/table/{table_number}/order")]
async fn api_table_order_retrieve(table_number: web::Path<u32>, info: web::Query<InfoApiTable>) -> HttpResponse {
    let is_all: bool = if info.all.is_none() { false } else { true };
    let body = Order::retrieve(table_number.into_inner(), is_all).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[post("/api/table/{table_number}/order")]
async fn api_table_order(table_number: web::Path<u32>, form_data: web::Form<InfoApiOrder>) -> HttpResponse {
    let mut orders: Vec<Order> = Vec::new();
    let mut orders_form: Vec<(String, u32)> = Vec::new();

    orders_form.push((form_data.menus_0.clone(), form_data.amount_0));
    orders_form.push((form_data.menus_1.clone(), form_data.amount_1));
    orders_form.push((form_data.menus_2.clone(), form_data.amount_2));
    orders_form.push((form_data.menus_3.clone(), form_data.amount_3));
    orders_form.push((form_data.menus_4.clone(), form_data.amount_4));

    // TODO: Change to loop/array from form directly, as the form array are not working at the moment due to URL encoding
    for index in 0..orders_form.len() {
        if orders_form[index].1 == 0 {
            continue;
        }

        // Create order number based on the amount
        for _i in 0..orders_form[index].1 {
            orders.push(
                Order {
                    id: 0,
                    menu: orders_form[index].0.clone(),
                    is_served: false,
                    time_estimate: rand::thread_rng().gen_range(5..15),
                }
            );
        }
    }

    let body = Order::create(table_number.into_inner(), orders).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/api/table/{table_number}/order/{order_id}")]
async fn api_table_order_retrieve_one(param: web::Path<(u32, u64)>) -> HttpResponse {
    let (table_number, order_id) = param.into_inner();
    let body = Order::retrieve_one(table_number, order_id).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[put("/api/table/{table_number}/order/{order_id}")]
async fn api_table_order_update(param: web::Path<(u32, u64)>) -> HttpResponse {
    let (table_number, order_id) = param.into_inner();
    let body = Order::update(table_number, order_id).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[delete("/api/table/{table_number}/order/{order_id}")]
async fn api_table_order_delete(param: web::Path<(u32, u64)>) -> HttpResponse {
    let (table_number, order_id) = param.into_inner();
    let body = Order::delete(table_number, order_id).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/client")]
async fn client() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html/index.html"))
}

#[get("/client/table_selection")]
async fn client_table_selection() -> HttpResponse {
    let body_html = include_str!("../html/table_selection.html");
    let mut body_html_table_options: String =  "".to_string();

    for i in 1..=100 {
        body_html_table_options.push_str(&format!("<option value=\"{}\">Table {}</option>", i, i));
    }

    let body = body_html.replace("{table_options}", &body_html_table_options);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[get("/client/table")]
async fn client_table(info: web::Query<InfoClientTable>) -> HttpResponse {
    let body_html = include_str!("../html/table.html");
    let is_all: bool = if info.all.is_none() { false } else { true };
    let remaining_order = Order::retrieve_html(info.table_number.clone(), is_all);
    let body = body_html.replace("{table_number}", &info.table_number)
                                .replace("{remain_items}", remaining_order.unwrap().as_str())
                                .replace("{item_type}", if is_all { "All Items" } else { "Remaining Items" });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[get("/client/table/order")]
async fn client_table_order(info: web::Query<InfoClientTableOrder>) -> HttpResponse {
    let body_html = include_str!("../html/order.html");
    let mut body = body_html.replace("{table_number}", &info.table_number);
    body = body.replace("{order_form}", Menu::retrieve_form().unwrap().as_str());

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

// Just for client/testing
#[get("/client/table/{table_number}/order/{order_id}/serve")]
async fn client_table_order_update(param: web::Path<(u32, u64)>) -> HttpResponse {
    let (table_number, order_id) = param.into_inner();

    Order::update(table_number, order_id).unwrap();

    let body_html = include_str!("../html/table.html");
    let remaining_order = Order::retrieve_html(table_number.to_string(), false);
    let body = body_html.replace("{table_number}", table_number.to_string().as_str()).replace("{remain_items}", remaining_order.unwrap().as_str())
                                .replace("{item_type}", "Remaining Items");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

// Just for client/testing
#[get("/client/table/{table_number}/order/{order_id}/delete")]
async fn client_table_order_delete(param: web::Path<(u32, u64)>) -> HttpResponse {
    let (table_number, order_id) = param.into_inner();
    
    Order::delete(table_number, order_id).unwrap();

    let body_html = include_str!("../html/table.html");
    let remaining_order = Order::retrieve_html(table_number.to_string(), false);
    let body = body_html.replace("{table_number}", table_number.to_string().as_str()).replace("{remain_items}", remaining_order.unwrap().as_str())
                                .replace("{item_type}", "Remaining Items");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
