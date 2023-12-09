use exports::fermyon::spin::sqlite::{
    Error,
    Value,
    RowResult,
    QueryResult,
    OwnConnection, 
    GuestConnection
};

pub struct ProxyConnection(spin_sdk::sqlite::Connection);

impl GuestConnection for ProxyConnection {
    fn open(database: String) -> Result<OwnConnection, Error> {
        println!("IN WRAPPER");
        let conn = spin_sdk::sqlite::Connection::open(&database).map(ProxyConnection)?;
        Ok(spin_sdk::wit_bindgen::rt::Resource::new(conn))
    }

    fn execute(&self, statement: String, parameters: Vec<Value>) -> Result<QueryResult, Error> {
        println!("IN EXECUTE");

        let parameters = parameters
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        Ok(self
            .0
            .execute(&statement, &parameters)
            .map(From::from)?)
    }
}

impl From<spin_sdk::sqlite::QueryResult> for QueryResult {
    fn from(qr: spin_sdk::sqlite::QueryResult) -> Self {
        QueryResult {
            columns: qr.columns,
            rows: qr
                .rows
                .into_iter()
                .map(From::from)
                .collect(),
        }
    }
}

impl From<spin_sdk::sqlite::RowResult> for RowResult {
    fn from(row: spin_sdk::sqlite::RowResult) -> RowResult {
        RowResult {
            values: row
                .values
                .into_iter()
                .map(From::from)
                .collect(),
        }
    }
}

impl Into<spin_sdk::sqlite::Value> for Value {
    fn into(self) -> spin_sdk::sqlite::Value {
        match self {
            Value::Integer(v) => spin_sdk::sqlite::Value::Integer(v),
            Value::Real(v) => spin_sdk::sqlite::Value::Real(v),
            Value::Text(v) => spin_sdk::sqlite::Value::Text(v),
            Value::Blob(v) => spin_sdk::sqlite::Value::Blob(v),
            Value::Null => spin_sdk::sqlite::Value::Null,
        }
    }
}

impl From<spin_sdk::sqlite::Value> for Value {
    fn from(val: spin_sdk::sqlite::Value) -> Self {
        match val {
            spin_sdk::sqlite::Value::Integer(v) => Value::Integer(v),
            spin_sdk::sqlite::Value::Real(v) => Value::Real(v),
            spin_sdk::sqlite::Value::Text(v) => Value::Text(v),
            spin_sdk::sqlite::Value::Blob(v) => Value::Blob(v),
            spin_sdk::sqlite::Value::Null => Value::Null,
        }
    }
}


impl From<spin_sdk::sqlite::Error> for Error {
    fn from(e: spin_sdk::sqlite::Error) -> Self {
        match e {
            spin_sdk::sqlite::Error::AccessDenied => Error::AccessDenied,
            spin_sdk::sqlite::Error::NoSuchDatabase => Error::NoSuchDatabase,
            spin_sdk::sqlite::Error::InvalidConnection => Error::InvalidConnection,
            spin_sdk::sqlite::Error::DatabaseFull => Error::DatabaseFull,
            spin_sdk::sqlite::Error::Io(e) => Error::Io(e),
        }
    }
}

wit_bindgen::generate!({
    runtime_path: "::spin_sdk::wit_bindgen::rt",
    world: "proxy",
    path: "wit",
    exports: {
        world: ProxyConnection,
        "fermyon:spin/sqlite/connection": ProxyConnection,
    }
});
