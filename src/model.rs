use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Ride {

    pub id:u32,
    pub passenger:String,
    pub pickup:String,
    pub destination:String,
    pub price:f64
}
#[derive(Deserialize)]
pub struct CreateRide{

    pub passenger:String,
    pub pickup:String,
    pub destination:String,
    pub price:f64
}

#[derive(Deserialize)]
pub struct UpdateRide{

    pub passenger:Option<String>,
    pub pickup:Option<String>,
    pub destination:Option<String>,
    pub price:Option<f64>,
}
