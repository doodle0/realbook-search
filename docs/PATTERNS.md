# Code Patterns & Conventions

Standard patterns for writing consistent code in the Real Book Search project.

---

## Backend Patterns

### Adding Routes

**File:** `api/src/controller.rs`

```rust
#[get("/endpoint?<param>")]
pub fn handler_name(
    data: &State<Arc<Vec<RealBookEntry>>>,
    param: Option<String>
) -> Json<ResponseType> {
    // Implementation
    Json(response)
}
```

**Register in `api/src/main.rs`:**
```rust
.mount("/api", routes![index, handler_name])
```

### Data Models

**File:** `api/src/models.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyModel {
    pub field: String,
}
```

---

## Frontend Patterns

### Yew Function Components (Preferred)

```rust
#[component]
fn MyComponent() -> Html {
    let state = use_state(|| initial_value);
    
    html! {
        <div>{ "content" }</div>
    }
}
```

### Yew Struct Components (For Complex State)

```rust
pub struct MyComponent;

pub enum Msg {
    Event,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { /* ... */ }
    }
}
```

### API Calls

**File:** `ui/src/api.rs`

```rust
pub async fn fetch_data() -> Result<DataType, ApiError> {
    let url = format!("{}/endpoint", API_BASE_URL);
    let response = reqwest::get(&url).await?;
    Ok(response.json().await?)
}
```

---

## Conventions

- **File naming:** snake_case (main.rs, controller.rs)
- **Type naming:** PascalCase (RealBookEntry)
- **Function naming:** snake_case (load_data)
- **Constants:** SCREAMING_SNAKE_CASE (API_BASE_URL)

