use libtad_rs::models::{astronomy::AstronomyObjectType, time::DateTime};
use libtad_rs::service::astronomy::AstroPositionRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_position_for_multiple_intervals() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let intervals: Vec<DateTime> = (1..5_i32)
        .zip(5..9)
        .zip(12..16)
        .map(|((a, b), c)| format!("2020-{:02}-{:02}T05:04:{:02}", a, b, c))
        .map(|x| DateTime::from(x.as_str()))
        .collect();

    let request = AstroPositionRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .set_interval(intervals);

    let response = client.get_astro_position(&request).await.unwrap().unwrap();
    let results = &response.locations[0].astronomy.objects[0].results;

    assert!(results.as_ref().unwrap().len() == 4);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_position_with_lang() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroPositionRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("187")
        .with_interval(DateTime::from("2020-03-01"))
        .set_lang("es");

    let response = client.get_astro_position(&request).await.unwrap().unwrap();

    assert!(response.locations[0].geo.country.name == "Noruega");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn test_geo_info_for_given_placeid() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroPositionRequest::new()
        .with_object(AstronomyObjectType::Sun)
        .with_placeid("3")
        .with_interval(DateTime::from("2020-03-15"));

    let response = client.get_astro_position(&request).await.unwrap().unwrap();
    let geo = &response.locations[0].geo;

    assert!(geo.name == "Acapulco");
    assert!(geo.state.as_deref() == Some("Guerrero"));
    assert!(geo.country.id == "mx");
    assert!(geo.country.name == "Mexico");
    assert!(geo.latitude == Some(16.860));
    assert!(geo.longitude == Some(-99.877));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn compare_with_and_without_localtime() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let mut request = AstroPositionRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .with_interval(DateTime::from("2020-03-05"));

    let response_1 = client.get_astro_position(&request).await.unwrap().unwrap();
    let result_1 = &response_1.locations[0].astronomy.objects[0]
        .results
        .as_ref()
        .unwrap()[0];

    request = request.set_localtime(true);

    let response_2 = client.get_astro_position(&request).await.unwrap().unwrap();
    let result_2 = &response_2.locations[0].astronomy.objects[0]
        .results
        .as_ref()
        .unwrap()[0];

    assert!(result_1.azimuth != result_2.azimuth);
    assert!(result_1.altitude != result_2.altitude);
    assert!(result_1.distance != result_2.distance);
    assert!(result_1.illuminated != result_2.illuminated);
    assert!(result_1.posangle != result_2.posangle);
}
