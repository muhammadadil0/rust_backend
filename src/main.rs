// mod model;
// use axum::{Json, Router, routing::{get, post}, http::StatusCode, extract::Path};
// use model::{Ride, CreateRide};
// use std::sync::Arc;
// use tokio::sync::Mutex;
// async fn hello() -> &'static str {
//     "Ride Booking API is running"
// }

// async fn get_ride() -> Json<Vec<Ride>> {
//     let rides = vec![
//         Ride {
//             id: 1,
//             passenger: "Adil".to_string(),
//             pickup: "Mardan".to_string(),
//             destination: "Peshawar".to_string(),
//             price: 400.0,
//         },
//         Ride {
//             id: 2,
//             passenger: "Ali".to_string(),
//             pickup: "Islamabad".to_string(),
//             destination: "Rawalpindi".to_string(),
//             price: 200.0,
//         },
//     ];
//     Json(rides)
// }

// async fn create_ride(Json(payload):Json<CreateRide>)->(StatusCode, Json<Ride>){
//     let ride = Ride{
//         id: 3,
//         passenger: payload.passenger,
//         pickup: payload.pickup,
//         destination: payload.destination,
//         price: payload.price
//     };
//     (StatusCode::CREATED, Json(ride))
// }
// async fn get_ride_by_id(Path(id):Path<u32>)->Result<Json<Ride>, StatusCode>{
//     let rides = vec![
//         Ride {
//             id: 1,
//             passenger: "Adil".to_string(),
//             pickup: "Mardan".to_string(),
//             destination: "Peshawar".to_string(),
//             price: 400.0,
//         },
//         Ride {
//             id: 2,
//             passenger: "Ali".to_string(),
//             pickup: "Islamabad".to_string(),
//             destination: "Rawalpindi".to_string(),
//             price: 200.0,
//         },
//     ];

//     let ride =  rides.into_iter().find(|ride|ride.id == id  );
//     match ride {
//         Some(ride) => Ok(Json(ride)),
//         None => Err(StatusCode::NOT_FOUND)
    
//     }

// }


// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/welcome", get(hello))
//         .route("/rides", get(get_ride))
//         .route("/rides/{id}", get(get_ride_by_id))
//         .route("/create", get(get_ride).post(create_ride));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3005")
//         .await
//         .unwrap();

//     println!("Server running on http://127.0.0.1:3005");

//     axum::serve(listener, app).await.unwrap();
// }


 mod model;
 use axum::{Json, Router, routing::{get, patch, post, delete}, http::StatusCode, extract::{Path, State}};
 use model::{Ride, CreateRide,UpdateRide};
 use std::sync::Arc;
 use tokio::sync::Mutex;


type AppState = Arc<Mutex<Vec<Ride>>>;

async fn get_ride(State(state): State<AppState>) -> Json<Vec<Ride>> {
    let rides = state.lock().await;
    Json(rides.clone())
}

async fn create_ride(
    State(state): State<AppState>,
    Json(payload): Json<CreateRide>,
) -> (StatusCode, Json<Ride>) {
    let mut rides = state.lock().await;

    let ride = Ride {
        id: rides.len() as u32 + 1,
        passenger: payload.passenger,
        pickup: payload.pickup,
        destination: payload.destination,
        price: payload.price,
    };
    rides.push(ride.clone());
    (StatusCode::CREATED, Json(ride))
}
async fn update_ride(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateRide>,
) -> Result<Json<Ride>, StatusCode> {
    let mut rides = state.lock().await;

    let ride = rides.iter_mut().find(|ride| ride.id == id);

    match ride {
        Some(ride) => {
            if let Some(passenger) = payload.passenger {
                ride.passenger = passenger;
            }

            if let Some(pickup) = payload.pickup {
                ride.pickup = pickup;
            }

            if let Some(destination) = payload.destination {
                ride.destination = destination;
            }

            if let Some(price) = payload.price {
                ride.price = price;
            }

            Ok(Json(ride.clone()))
        }

        None => Err(StatusCode::NOT_FOUND),
    }
}
async fn get_ride_by_id(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Ride>, StatusCode> {
    let rides = state.lock().await;

    let ride = rides.iter().find(|ride| ride.id == id).cloned();

    match ride {
        Some(ride) => Ok(Json(ride)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_ride(Path(id): Path<u32>, State(state): State<AppState>)-> Result<StatusCode, StatusCode>{
    let mut rides = state.lock().await;


    let index = rides.iter().position(|ride| ride.id == id);
    match index {
        Some(index) =>{
            rides.remove(index);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(StatusCode::NOT_FOUND)
    }
}


#[tokio::main]
async fn main() {

    let rides: AppState = Arc::new(Mutex::new(vec![
        Ride {
            id: 1,
            passenger: "Adil".to_string(),
            pickup: "Mardan".to_string(),
            destination: "Peshawar".to_string(),
            price: 1500.0,
        },
        Ride {
            id: 2,
            passenger: "Ali".to_string(),
            pickup: "Abbottabad".to_string(),
            destination: "Islamabad".to_string(),
            price: 2500.0,
        },
    ]));

    let app = Router::new()
        .route("/create", post(create_ride))
        .route("/rides", get(get_ride))
        .route("/rides/{id}", get(get_ride_by_id))
        .route("/rides/{id}/update", patch(update_ride))
        .route("/rides/{id}/delete", delete(delete_ride))
        .with_state(rides);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3006").await.unwrap();

    println!("Server running on http://127.0.0.1:3006");

    axum::serve(listener, app).await.unwrap();
}