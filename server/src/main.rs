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
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl
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
        .route("/rooms", routing::get(list_rooms).post(create_room))
        .route(
            "/rooms/{id}",
            routing::get(get_room)
                .patch(update_room)
                .put(update_room)
                .delete(delete_room)
        )
        .route("/rooms/{room_id}/devices", routing::get(list_devices).post(create_device))
        .route(
            "/rooms/{room_id}/devices/{device_id}",
            routing::get(get_device)
                .patch(update_device)
                .put(update_device)
                .delete(delete_device)
        )
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

    use model::houses::dsl::*;

    let result = houses
        .select(House::as_select())
        .first(&mut conn)
        .await
        .optional();

    match result {
        Ok(Some(result)) => Ok((StatusCode::OK, Json(result.into()))),
        Ok(None) => Err(Error::NotFound),
        Err(err) => Err(Error::from_internal(err))
    }
}

async fn list_rooms(
    State(state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<Vec<shared::Room>>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::rooms::dsl::*;

    rooms
        .select(Room::as_select())
        .load(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| {
            let result: Vec<shared::Room> = result.into_iter().map(Into::into).collect();
            (StatusCode::OK, Json(result))
        })
}

async fn create_room(
    State(state): State<Arc<AppState>>,
    Json(new_room): Json<shared::NewRoom>
) -> Result<(StatusCode, Json<shared::Room>), Error> {
    let mut conn = state.get_db_connection().await?;

    let new_room: NewRoom = new_room.into();

    diesel::insert_into(rooms::table)
        .values(new_room)
        .returning(Room::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| (StatusCode::CREATED, Json(result.into())))
}

async fn get_room(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<uuid::Uuid>
) -> Result<(StatusCode, Json<shared::Room>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::rooms::dsl::*;

    let result = rooms
        .find(room_id)
        .select(Room::as_select())
        .first(&mut conn)
        .await
        .optional();

    match result {
        Ok(Some(room)) => Ok((StatusCode::OK, Json(room.into()))),
        Ok(None) => Err(Error::NotFound),
        Err(err) => Err(Error::from_internal(err))
    }
}

async fn update_room(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<uuid::Uuid>,
    Json(new_room): Json<shared::NewRoom>
) -> Result<(StatusCode, Json<shared::Room>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::rooms::dsl::*;

    let room = rooms.find(room_id);

    room.select(id)
        .first::<uuid::Uuid>(&mut conn)
        .await
        .optional()
        .map_err(Error::from_internal)?
        .ok_or(Error::NotFound)?
    ;

    diesel::update(room)
        .set(name.eq(new_room.name))
        .returning(Room::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| (StatusCode::OK, Json(result.into())))
}

async fn delete_room(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<uuid::Uuid>,
) -> Result<StatusCode, Error> {
    let mut conn = state.get_db_connection().await?;

    use model::rooms::dsl::*;

    let room = rooms.find(room_id);

    room.select(id)
        .first::<uuid::Uuid>(&mut conn)
        .await
        .optional()
        .map_err(Error::from_internal)?
        .ok_or(Error::NotFound)?
    ;

    diesel::delete(room)
        .execute(&mut conn)
        .await
        .map_err(Error::from_internal)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_devices(
    State(state): State<Arc<AppState>>,
    Path(room_id): Path<uuid::Uuid>
) -> Result<(StatusCode, Json<Vec<shared::Device>>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::devices::dsl;

    dsl::devices
        .filter(dsl::room_id.eq(room_id))
        .select(Device::as_select())
        .load(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| {
            let result: Vec<shared::Device> = result.into_iter().map(Into::into).collect();
            (StatusCode::OK, Json(result))
        })
}

async fn create_device(
    Path(room_id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
    Json(new_device): Json<shared::NewDevice>
) -> Result<(StatusCode, Json<shared::Device>), Error> {
    let mut conn = state.get_db_connection().await?;

    let new_device = NewDevice {
        room_id,
        name: new_device.name
    };

    use model::rooms::dsl as rooms_dsl;

    let room = rooms_dsl::rooms.find(room_id);

    room.select(rooms_dsl::id)
        .first::<uuid::Uuid>(&mut conn)
        .await
        .optional()
        .map_err(Error::from_internal)?
        .ok_or(Error::NotFound)?
    ;

    diesel::insert_into(devices::table)
        .values(new_device)
        .returning(Device::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| (StatusCode::CREATED, Json(result.into())))
}

async fn get_device(
    Path((room_id, device_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<shared::Device>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::devices::dsl;

    let result = dsl::devices
        .filter(dsl::room_id.eq(room_id))
        .filter(dsl::id.eq(device_id))
        .select(Device::as_select())
        .first(&mut conn)
        .await
        .optional();

    match result {
        Ok(Some(device)) => Ok((StatusCode::OK, Json(device.into()))),
        Ok(None) => Err(Error::NotFound),
        Err(err) => Err(Error::from_internal(err))
    }
}

async fn update_device(
    Path((room_id, device_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(state): State<Arc<AppState>>,
    Json(new_device): Json<shared::NewDevice>
) -> Result<(StatusCode, Json<shared::Device>), Error> {
    let mut conn = state.get_db_connection().await?;

    use model::devices::dsl;

    let device = dsl::devices
        .filter(dsl::room_id.eq(room_id))
        .filter(dsl::id.eq(device_id))
    ;

    device.select(dsl::id)
        .first::<uuid::Uuid>(&mut conn)
        .await
        .optional()
        .map_err(Error::from_internal)?
        .ok_or(Error::NotFound)?
    ;

    diesel::update(device)
        .set(dsl::name.eq(new_device.name))
        .returning(Device::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Error::from_internal)
        .map(|result| (StatusCode::OK, Json(result.into())))
}

async fn delete_device(
    Path((room_id, device_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, Error> {
    let mut conn = state.get_db_connection().await?;

    use model::devices::dsl;

    let device = dsl::devices
        .filter(dsl::room_id.eq(room_id))
        .filter(dsl::id.eq(device_id));

    device.select(dsl::id)
        .first::<uuid::Uuid>(&mut conn)
        .await
        .optional()
        .map_err(Error::from_internal)?
        .ok_or(Error::NotFound)?
    ;

    diesel::delete(device)
        .execute(&mut conn)
        .await
        .map_err(Error::from_internal)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_report() -> String {
    "get_report".to_string()
}

