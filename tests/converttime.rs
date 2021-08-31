use libtad_rs::models::time::DateTime;
use libtad_rs::service::time::ConvertTimeRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_without_toid() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response.locations[0].geo.name == "Oslo");
    assert!(response.locations[0].geo.country.name == "Norway");
    assert!(response.locations[0].time.as_ref().unwrap().datetime == datetime);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_with_toid() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_datetime(datetime)
        .with_toid("usa/anchorage");

    let response = client.convert_time(&request).await.unwrap().unwrap();
    let oslo = &response.locations[0];
    let anchorage = &response.locations[1];

    assert!(anchorage.geo.state == Some("Alaska".into()));
    assert!(anchorage.geo.name == "Anchorage");
    assert!(oslo.geo.name == "Oslo");
    assert!(oslo.time.as_ref().unwrap().datetime == datetime);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_with_timechange_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_timechanges(true)
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.timechanges.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_without_timechange_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_timechanges(false)
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response.locations.iter().all(|x| x.timechanges.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_with_timezone_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_tz(true)
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.time.as_ref().unwrap().timezone.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_without_timezone_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("norway/oslo")
        .set_tz(false)
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response
        .locations
        .iter()
        .all(|x| x.time.as_ref().unwrap().timezone.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn convert_time_with_radius() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let datetime = DateTime::from("2021-07-27T12:00:00");

    let request = ConvertTimeRequest::new()
        .set_fromid("+59.914+10.752")
        .set_radius(50)
        .set_datetime(datetime);

    let response = client.convert_time(&request).await.unwrap().unwrap();

    assert!(response.locations[0].geo.name == "Oslo");
    assert!(response.locations[0].geo.country.name == "Norway");
}
