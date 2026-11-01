# Ride Booking Backend API

A high-performance, asynchronous REST API for a ride-booking service built with **Rust**, **Axum**, and **Tokio**.

---

## 🚀 Features

- **Asynchronous Architecture**: Powered by Tokio runtime and Axum web framework.
- **Thread-Safe In-Memory State**: State synchronized safely across async tasks using `Arc<tokio::sync::Mutex<Vec<Ride>>>`.
- **JSON Serialization**: Powered by `serde` and `serde_json`.
- **Full CRUD Support**:
  - Fetch all rides
  - Fetch a single ride by ID
  - Book a new ride
  - Update ride details
  - Cancel/delete a ride

---

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) (v0.8)
- **Async Runtime**: [Tokio](https://tokio.rs/) (v1)
- **Serialization**: [Serde](https://serde.rs/) & [Serde JSON](https://github.com/serde-rs/json)

---

## 📦 Project Structure

```text
ride_booking/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Locked dependency tree
├── .gitignore          # Git ignore rules for Rust projects
├── README.md           # Project documentation
└── src/
    ├── main.rs         # Application entry point, router, and route handlers
    └── model.rs        # Data structures (Ride, CreateRide, UpdateRide)
```

---

## 🚦 API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/rides` | Retrieve all booked rides |
| `GET` | `/rides/{id}` | Retrieve details for a specific ride |
| `POST` | `/create` | Book / create a new ride |
| `PATCH` | `/rides/{id}/update` | Update fields of an existing ride |
| `DELETE` | `/rides/{id}/delete` | Delete a ride by ID |

---

## 📋 Request & Response Examples

### 1. Get All Rides
**Request:**
```bash
curl -X GET http://127.0.0.1:3006/rides
```

**Response (200 OK):**
```json
[
  {
    "id": 1,
    "passenger": "Adil",
    "pickup": "Mardan",
    "destination": "Peshawar",
    "price": 1500.0
  },
  {
    "id": 2,
    "passenger": "Ali",
    "pickup": "Abbottabad",
    "destination": "Islamabad",
    "price": 2500.0
  }
]
```

---

### 2. Get Ride by ID
**Request:**
```bash
curl -X GET http://127.0.0.1:3006/rides/1
```

**Response (200 OK):**
```json
{
  "id": 1,
  "passenger": "Adil",
  "pickup": "Mardan",
  "destination": "Peshawar",
  "price": 1500.0
}
```

---

### 3. Create a New Ride
**Request:**
```bash
curl -X POST http://127.0.0.1:3006/create \
  -H "Content-Type: application/json" \
  -d '{
    "passenger": "Hamza",
    "pickup": "Lahore",
    "destination": "Islamabad",
    "price": 3000.0
  }'
```

**Response (201 Created):**
```json
{
  "id": 3,
  "passenger": "Hamza",
  "pickup": "Lahore",
  "destination": "Islamabad",
  "price": 3000.0
}
```

---

### 4. Update a Ride
**Request:**
```bash
curl -X PATCH http://127.0.0.1:3006/rides/1/update \
  -H "Content-Type: application/json" \
  -d '{
    "price": 1800.0
  }'
```

**Response (200 OK):**
```json
{
  "id": 1,
  "passenger": "Adil",
  "pickup": "Mardan",
  "destination": "Peshawar",
  "price": 1800.0
}
```

---

### 5. Delete a Ride
**Request:**
```bash
curl -X DELETE http://127.0.0.1:3006/rides/1/delete -i
```

**Response (204 No Content):**
```text
HTTP/1.1 204 No Content
```

---

## 🏃 Running Locally

### Prerequisites
Make sure you have [Rust and Cargo installed](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Start Server
Inside the `ride_booking` directory:

```bash
cargo run
```

The server will start listening at:
```text
http://127.0.0.1:3006
```

---

## 📄 License
This project is open-source and available under the [MIT License](LICENSE).
