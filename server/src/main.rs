mod error;
mod model;
mod schema;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    Router,
    routing,
    extract::{State, Path},
    http::StatusCode,
    response::Json,
    serve
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tokio::net;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

use error::Error;

use crate::model::*;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
type DbConnection<'a> = bb8::PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

struct AppState {
    pool: Pool
}

impl AppState {
    async fn get_db_connection(&self) -> Result<DbConnection<'_>, Error> {
        self.pool.get().await.map_err(Error::from_internal)
    }
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    env_logger::init();

    let pool = establish_db_connection().await;
    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .route("/", routing::get(get_house))
        .route("/rooms", routing::get(list_rooms).post(add_room))
        .route("/rooms/{id}", routing::get(get_room).patch(update_room).put(update_room))
        .route("/rooms/{room_id}/devices", routing::get(list_devices).post(add_device))
        .route("/rooms/{room_id}/devices/{id}", routing::get(get_device).patch(update_device).put(update_device))
        .route("/report", routing::get(get_report))
        .with_state(app_state)
    ;

    let addr: SocketAddr = env::var("API_HOST").unwrap().parse().unwrap();
    log::info!("Starting server in {addr}");

    let listener = net::TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}

async fn establish_db_connection() -> Pool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    bb8::Pool::builder().build(config).await.unwrap()
}

async fn get_house(
    State(state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<shared::House>), Error> {
    let mut conn = state.get_db_connection().await?;

    model::houses::table
        .select(House::as_select())
        .limit(1)
        .load(&mut conn)
        .await
        .map_err(Error::from_internal)?
        .into_iter()
        .next()
        .ok_or(Error::NotFound)
        .map(|result| {
            (
                StatusCode::OK,
                Json(result.into())
            )
        })
}

async fn list_rooms(
    State(state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<Vec<shared::Room>>), Error> {
    let mut conn = state.get_db_connection().await?;

    let rooms: Vec<shared::Room> = rooms::table
        .select(Room::as_select())
        .load(&mut conn)
        .await
        .map_err(Error::from_internal)?
        .into_iter()
        .map(Into::into)
        .collect()
    ;

    Ok((StatusCode::OK, Json(rooms)))
}

async fn add_room(
    State(state): State<Arc<AppState>>,
    Json(new_room): Json<shared::NewRoom>
) -> Result<(StatusCode, Json<shared::Room>), Error> {
    let mut conn = state.get_db_connection().await?;

    let new_room: NewRoom = new_room.into();

    let result = diesel::insert_into(rooms::table)
        .values(new_room)
        .returning(Room::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Error::from_internal)?;
    
    Ok((StatusCode::CREATED, Json(result.into())))
}

async fn get_room(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>
) -> Result<(StatusCode, Json<shared::Room>), Error> {
    let mut conn = state.get_db_connection().await?;

    model::rooms::table
        .find(id)
        .select(Room::as_select())
        .load(&mut conn)
        .await
        .map_err(Error::from_internal)?
        .into_iter()
        .next()
        .ok_or(Error::NotFound)
        .map(|result| {
            (
                StatusCode::OK,
                Json(result.into())
            )
        })
}

async fn update_room(Path(id): Path<String>) -> String {
    format!("update_room({id})")
}

async fn list_devices(Path(room_id): Path<String>) -> String {
    format!("list_devices({room_id})")
}

async fn add_device(Path(room_id): Path<String>) -> String {
    format!("add_device({room_id})")
}

async fn get_device(Path((room_id, id)): Path<(String, String)>) -> String {
    format!("get_device({room_id}, {id})")
}

async fn update_device(Path((room_id, id)): Path<(String, String)>) -> String {
    format!("update_device({room_id}, {id})")
}

async fn get_report() -> String {
    "get_report".to_string()
}

