
# Simple Restaurant API

## System

OS: Windows 11
Rust Version: rustc 1.82.0 (f6e511eec 2024-10-15)

## Libraries

- Rusqlite: Sqlite Database

```Shell
$ cargo add rusqlite
```

- Actix: HTTP Request

```Shell
$ cargo add actix-web
$ cargo add actix-files
```

- Serde: Serialize

```Shell
$ cargo add serde
```

## Checklist

### Platform Check

[/] Windows<br />
[-] MAC

### Requirement Check

[/] The server API MUST fully follow REST API principles and present a set of HTTP endpoints to connect to.<br />
[/] The client (the restaurant staff “devices” making the requests) MUST be able to: add one or more items with a table number, remove an item for a table, and query the items still remaining for a table.<br />
[/] The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.<br />
[/] The application MUST, upon deletion request, remove a specified item for a specified table number.<br />
[/] The application MUST, upon query request, show all items for a specified table number.<br />
[/] The application MUST, upon query request, show a specified item for a specified table number.<br />
[/] The application MUST accept at least 10 simultaneous incoming add/remove/query requests.<br />
[/] The client MAY limit the number of specific tables in its requests to a finite set (at least 100).<br />
[/] The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.<br />
[/] The application MAY keep the length of time for the item to prepare static (in other words, the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion).<br />

### TODO

[ ] CORs<br />
[ ] CSRF<br />
[ ] URL Encoded - Actix form array<br />
[ ] Client

#### URI

##### Client

http://127.0.0.1:8080/client

##### API

http://127.0.0.1:8080/api/menu<br />
[/] GET -> retrieve menu<br />
<br />
http://127.0.0.1:8080/api/table/{table_number}/order<br />
[/] GET -> retrieve remaining order for table number + ?all to retrieve all orders for table number<br />
[/] POST -> create order for table number<br />
<br />
http://127.0.0.1:8080/api/table/{table_number}/order?all=true<br />
[/] GET -> retrieve all orders for table number<br />
<br />
http://127.0.0.1:8080/api/table/{table_number}/order/{order_id}<br />
[/] GET -> retrieve specific order for table number<br />
[/] UPDATE -> update order status<br />
[/] DELETE -> cancel order<br />
