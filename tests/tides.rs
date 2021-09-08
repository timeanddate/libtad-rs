use libtad_rs::models::tides::TidalPhase;
use libtad_rs::models::time::DateTime;
use libtad_rs::service::tides::TidesRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_tidal_data() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new()
        .with_placeid("norway/stavanger")
        .set_startdt(DateTime::from("2021-09-08T00:00:00"))
        .set_enddt(DateTime::from("2021-09-08T23:59:59"));

    let response = client.get_tidal_data(&request).await.unwrap().unwrap();

    assert!(response.stations.len() == 1);
    assert!(response.stations[0].source.name == "Stavanger");
    assert!(response.stations[0].source.r#type == "Reference Station");
    assert!(response.stations[0].matchparam == "norway/stavanger");
    assert!(response.stations[0]
        .result
        .iter()
        .all(|x| x.time.datetime.month == 9 && x.time.datetime.day == 8));
    assert!(response.stations[0]
        .result
        .iter()
        .all(|x| matches!(x.phase, TidalPhase::High) || matches!(x.phase, TidalPhase::Low)));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_tidal_data_without_onlyhighlow() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new()
        .with_placeid("norway/stavanger")
        .set_startdt(DateTime::from("2021-09-08T00:00:00"))
        .set_enddt(DateTime::from("2021-09-08T23:59:59"))
        .set_onlyhighlow(false);

    let response = client.get_tidal_data(&request).await.unwrap().unwrap();

    assert!(!response.stations[0]
        .result
        .iter()
        .all(|x| matches!(x.phase, TidalPhase::High) || matches!(x.phase, TidalPhase::Low)));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_invalid_end_date() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new()
        .with_placeid("norway/stavanger")
        .set_startdt(DateTime::from("2021-09-08T00:00:00"))
        .set_enddt(DateTime::from("2021-09-07T23:59:59"));

    let response = client.get_tidal_data(&request).await.unwrap();

    assert!(response.is_err());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_subordinate_station() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new()
        .with_placeid("norway/sola")
        .set_subordinate(true);

    let response = client.get_tidal_data(&request).await.unwrap().unwrap();

    assert!(response.stations[0].source.r#type == "Subordinate Station");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_without_radius() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new().with_placeid("4");

    let response = client.get_tidal_data(&request).await.unwrap();

    assert!(response.is_err());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_with_radius() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = TidesRequest::new().with_placeid("4").set_radius(186);

    let response = client.get_tidal_data(&request).await.unwrap();

    assert!(response.is_ok());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_intervals() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let mut request = TidesRequest::new()
        .with_placeid("norway/stavanger")
        .set_onlyhighlow(false)
        .set_interval(60);

    let response_60 = client.get_tidal_data(&request).await.unwrap().unwrap();
    assert!(response_60.stations[0].result.len() == 24);

    request = request.set_interval(30);

    let response_30 = client.get_tidal_data(&request).await.unwrap().unwrap();
    assert!(response_30.stations[0].result.len() == 48);

    request = request.set_interval(15);

    let response_15 = client.get_tidal_data(&request).await.unwrap().unwrap();
    assert!(response_15.stations[0].result.len() == 96);

    request = request.set_interval(5);

    let response_5 = client.get_tidal_data(&request).await.unwrap().unwrap();
    assert!(response_5.stations[0].result.len() == 288);
}
