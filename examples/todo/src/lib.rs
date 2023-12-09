use anyhow::{Context, Result};
use serde::Serialize;
use spin_sdk::{
    http::{Params, Request, Response, Router, IntoResponse},
    http_component,
    sqlite::{self, Connection},
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_todo(req: Request) -> Result<impl IntoResponse> {
    let mut router = Router::new();
    router.get("/api/todos", get_todos);
    router.post("/api/todos/create", create_todo);
    router.patch("/api/todos/:id", update_todo);
    router.delete("/api/todos/:id", delete_todo);
    router.any("/*", route_not_found);
    Ok(router.handle(req))
}

/*
This is all assumes the following table has been created in the sqlite database already:
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    due_date DATE,
    starred BOOLEAN DEFAULT 0,
    is_completed BOOLEAN DEFAULT 0
);
 */

const DATE_FORMAT: &str = &"[year]-[month]-[day]";

#[derive(serde::Deserialize)]
struct GetParams {
    #[serde(default)]
    due: Option<bool>,
    #[serde(default)]
    complete: Option<bool>,
}

pub fn get_todos(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let params: GetParams = serde_qs::from_str(req.query())?;
    let due_date = params.due.map(|due| {
        let format = time::format_description::parse(DATE_FORMAT).unwrap();
        let today = time::OffsetDateTime::now_utc()
            .date()
            .format(&format)
            .unwrap();
        if due {
            format!("due_date <= '{today}'")
        } else {
            format!("(due_date > '{today}' OR due_date is NULL)")
        }
    });

    let incomplete = params.complete.map(|complete| {
        if complete {
            "is_completed == TRUE"
        } else {
            "is_completed == FALSE"
        }
    });

    let w = match (due_date, incomplete) {
        (Some(due_date), Some(incomplete)) => format!("WHERE {due_date} AND {incomplete}"),
        (Some(due_date), None) => format!("WHERE {due_date}"),
        (None, Some(incomplete)) => format!("WHERE {incomplete}"),
        (None, None) => String::new(),
    };

    let conn = Connection::open_default()?;
    let todos = conn
        .execute(&format!("SELECT * FROM todos {w};"), &[])?
        .rows()
        .map(|r| -> anyhow::Result<Todo> { r.try_into() })
        .collect::<anyhow::Result<Vec<Todo>>>()?;

    Ok(Response::new(200, serde_json::to_vec(&todos)?))
}

#[derive(serde::Deserialize)]
struct CreateParams {
    description: String,
    due_date: Option<time::Date>,
}

pub fn create_todo(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let create: CreateParams = serde_json::from_slice(req.body())?;

    let format = time::format_description::parse(DATE_FORMAT)?;
    let format = create.due_date.map(|d| d.format(&format).unwrap());
    let params = [
        sqlite::Value::Text(create.description.clone()),
        format
            .map(|s| sqlite::Value::Text(s))
            .unwrap_or(sqlite::Value::Null),
    ];

    let conn = Connection::open_default()?;
    let response = &conn
        .execute(
            "INSERT INTO todos (description, due_date) VALUES(?, ?) RETURNING id;",
            params.as_slice(),
        )?
        .rows;
    let Some(id) = response.get(0) else { anyhow::bail!("Expected number got {response:?}")};
    let todo = Todo {
        id: id.get(0).unwrap(),
        description: create.description,
        due_date: create.due_date,
        starred: false,
        is_completed: false,
    };

    Ok(Response::new(200, serde_json::to_vec(&todo)?))
}

#[derive(serde::Deserialize)]
struct UpdateParams {
    is_completed: bool,
}

pub fn update_todo(req: Request, params: Params) -> anyhow::Result<Response> {
    let id = params.get("id").unwrap();
    let update: UpdateParams = serde_json::from_slice(req.body())?;

    let params = [
        sqlite::Value::Integer(update.is_completed as i64),
        sqlite::Value::Integer(id.parse().unwrap()),
    ];
    let conn = Connection::open_default()?;
    conn.execute(
        "UPDATE todos SET is_completed = (?) WHERE ID = (?);",
        params.as_slice(),
    )?;
    Ok(Response::new(204, ""))
}

pub fn delete_todo(_req: Request, params: Params) -> anyhow::Result<Response> {
    let id = params.get("id").unwrap();
    let params = [sqlite::Value::Integer(id.parse().unwrap())];
    let conn = Connection::open_default()?;
    conn.execute("DELETE FROM todos WHERE ID = (?);", params.as_slice())?;
    Ok(Response::new(204, ""))
}

#[derive(Serialize)]
struct Todo {
    id: u32,
    description: String,
    due_date: Option<time::Date>,
    starred: bool,
    is_completed: bool,
}

impl<'a> TryFrom<sqlite::Row<'a>> for Todo {
    type Error = anyhow::Error;
    fn try_from(row: sqlite::Row<'a>) -> std::result::Result<Self, Self::Error> {
        let id = row.get("id").context("row has no id")?;
        let description: &str = row.get("description").context("row has no description")?;
        let due_date = row.get::<&str>("due_date");
        let format = time::format_description::parse(DATE_FORMAT)?;
        let due_date = due_date
            .map(|dd| time::Date::parse(dd, &format))
            .transpose()
            .context("due_date is in wrong format")?;
        let starred = row.get("starred").context("row has no starred")?;
        let is_completed = row.get("is_completed").context("row has no is_completed")?;
        Ok(Self {
            id,
            description: description.to_owned(),
            due_date,
            starred,
            is_completed,
        })
    }
}

fn route_not_found(req: Request, _params: Params) -> Result<impl IntoResponse> {
    println!("No handler for {} {}", req.uri(), req.method());
    Ok(Response::new(404, serde_json::json!({"error":"not_found"}).to_string()))
}