# Time and Date Rust API
libtad-rs is a Rust library for accessing Time and Date API services. 

An access key and a secret key is required to use the API. For more information, see our [API Services page](https://services.timeanddate.com).

### Cargo features
- "sync-client": Enabled by default.
- "async-client": Disabled by default.

"async-client" can be enabled by disabling default features and adding "async-client" as a feature.

## Astronomy API
### Astro Event Service
Get astronomical events for multiple places and objects:

```rust ignore
let client = ServiceClient::new("access_key".into(), "secret_key".into());
let request = AstroEventRequest::new()
    .with_object(AstronomyObjectType::Sun)
    .with_object(AstronomyObjectType::Saturn)
    .with_placeid("3")
    .with_placeid("4")
    .set_startdt(DateTime::from("2021-08-18"));

let response = client.get_astro_events(&request);
```

### Astro Position Service
Get astronomical data for multiple places and objects:

```rust ignore
let request = AstroPositionRequest::new()
    .with_object(AstronomyObjectType::Sun)
    .with_placeid("3")
    .with_placeid("norway/oslo")
    .with_interval(DateTime::from("2021-08-18"))
    .with_interval(DateTime::from("2021-08-25"));

let response = client.get_astro_position(&request);
```

## Date Calculator API
### Business Date Service
Calculate a business date by adding days to a given date:

```rust ignore
let request = BusinessDateRequest::new()
	.set_placeid("norway/stavanger")
	.set_startdt(DateTime::from("2021-04-04"))
	.with_days(4);

let response = client.calculate_business_date(&request);
```

Calculate a business date by subtracting days to a given date:

```rust ignore
let request = BusinessDateRequest::new()
	.set_placeid("norway/stavanger")
	.set_startdt(DateTime::from("2021-04-04"))
	.with_days(4)
	.set_subtraction();

let response = client.calculate_business_date(&request);
```

### Business Duration Service
Calculate duration between two timestamps:

```rust ignore
let request = BusinessDurationRequest::new()
	.set_placeid("norway/stavanger")
	.set_startdt(DateTime::from("2021-04-04"))
	.set_enddt(DateTime::from("2021-04-21"));

let response = client.calculate_business_duration(&request);
```

## Holidays API
### Holidays Service
Get all holidays for a year and one or multiple countries:

```rust ignore
let request = HolidaysRequest::new()
	.with_country("no")
	.with_country("us")
	.set_year(2022);

let response = client.get_holidays(&request);
```

## On This Day API
### On This Day Service
Get events on this day:

```rust ignore
let request = OnThisDayRequest::new();

let response = client.get_events_on_this_day(&request);
```

Get events for a given month and day:

```rust ignore
let request = OnThisDayRequest::new()
	.set_month(4)
	.set_day(24);

let response = client.get_events_on_this_day(&request);
```

Filter events by event type:

```rust ignore
let request = OnThisDayRequest::new()
	.with_type(EventType::Events)
	.with_type(EventType::Births);

let response = client.get_events_on_this_day(&request);
```

## Places API
### Places Service
Get all places:

```rust ignore
let request = PlacesRequest::new();

let response = client.get_places(&request);
```

Query for a place:

```rust ignore
let request = PlacesRequest::new()
	.set_query("new york")
	.set_qlimit(10);

let response = client.get_places(&request);
```

## Time API
### ConvertTime Service
Convert time from a location to multiple locations:

```rust ignore
let request = ConvertTimeRequest::new()
    .set_fromid("norway/oslo")
    .with_toid("usa/chicago")
    .with_toid("179")
    .set_datetime(DateTime::from("2021-04-05T16:45:02"));

let response = client.convert_time(&request).unwrap();
```

### DSTList Service
Get all daylight saving times:

```rust ignore
let request = DSTListRequest::new();

let response = client.get_daylight_savings_time(&request).unwrap();
```

Get daylight saving times for a specific year:

```rust ignore
let request = DSTListRequest::new().set_year(2021);

let response = client.get_daylight_savings_time(&request).unwrap();
```


### Timeservice Service
Get current time for a place:

```rust ignore
let request = TimeserviceRequest::new()
	.set_placeid("norway/oslo");

let response = client.get_current_time(&request);
```
