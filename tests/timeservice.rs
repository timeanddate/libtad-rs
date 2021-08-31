use libtad_rs::service::time::TimeserviceRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_for_numeric_id() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new().set_placeid("179");

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations[0].id == "179");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_for_coordinates() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new().set_placeid("+59.914+10.752");

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations[0].geo.name == "Oslo");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_for_textual_id() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new().set_placeid("norway/oslo");

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations[0].id == "187");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_with_timechanges() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_timechanges(true);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.timechanges.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_without_timechanges() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_timechanges(false);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.timechanges.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_with_coordinates() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_geo(true);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.geo.latitude.is_some() && x.geo.longitude.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_without_coordinates() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_geo(false);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.geo.latitude.is_none() && x.geo.longitude.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_with_sunrise_sunset_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_sun(true);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.astronomy.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_without_sunrise_sunset_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_sun(false);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.astronomy.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_with_tz_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_tz(true);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.time.as_ref().unwrap().timezone.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_current_time_without_tz_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TimeserviceRequest::new()
        .set_placeid("norway/oslo")
        .set_tz(false);

    let response = client.get_current_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.time.as_ref().unwrap().timezone.is_none()));
}
