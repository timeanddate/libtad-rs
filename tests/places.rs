use libtad_rs::service::places::PlacesRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_places() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = PlacesRequest::new();
    let response = client.get_places(&request).await.unwrap().unwrap();

    let result = &response.places[21];

    assert!(result.id == 22);
    assert!(result.urlid == "new-zealand/auckland");
    assert!(result.geo.name == "Auckland");
    assert!(result.geo.state.as_ref().unwrap() == "Auckland");
    assert!(result.geo.country.id == "nz");
    assert!(result.geo.country.name == "New Zealand");
    assert!(result.geo.latitude == Some(-36.849));
    assert!(result.geo.longitude == Some(174.762));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_places_without_coordinates() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = PlacesRequest::new().set_geo(false);
    let response = client.get_places(&request).await.unwrap().unwrap();

    let result = &response.places[21];

    assert!(result.id == 22);
    assert!(result.urlid == "new-zealand/auckland");
    assert!(result.geo.name == "Auckland");
    assert!(result.geo.state.as_ref().unwrap() == "Auckland");
    assert!(result.geo.country.id == "nz");
    assert!(result.geo.country.name == "New Zealand");
    assert!(result.geo.latitude.is_none());
    assert!(result.geo.longitude.is_none());
}
