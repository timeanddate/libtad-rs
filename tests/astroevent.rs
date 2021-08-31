use libtad_rs::models::{
    astronomy::{AstronomyEventClass, AstronomyObjectType},
    time::DateTime,
};
use libtad_rs::service::astronomy::AstroEventRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_events_for_multiple_days() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroEventRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .set_startdt(DateTime::from("2020-03-01"))
        .set_enddt(DateTime::from("2020-03-20"));

    let response = client.get_astro_events(&request).await.unwrap().unwrap();
    let days = &response.locations[0].astronomy.objects[0].days;

    assert!(days.as_ref().unwrap().len() == 20);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_events_with_lang() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroEventRequest::new()
        .with_object(AstronomyObjectType::Sun)
        .with_placeid("187")
        .set_startdt(DateTime::from("2020-03-01"))
        .set_lang("es");

    let response = client.get_astro_events(&request).await.unwrap().unwrap();

    assert!(response.locations[0].geo.country.name == "Noruega");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn test_geo_info_for_given_placeid() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroEventRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .set_startdt(DateTime::from("2020-03-15"));

    let response = client.get_astro_events(&request).await.unwrap().unwrap();
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
async fn test_event_info_for_given_placeid() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroEventRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .set_startdt(DateTime::from("2020-03-05"));

    let response = client.get_astro_events(&request).await.unwrap().unwrap();
    let day = &response.locations[0].astronomy.objects[0]
        .days
        .as_ref()
        .unwrap()[0];

    assert!(day.events[0].r#type == "set");
    assert!(day.events[1].r#type == "rise");

    assert!(day.events[0].azimuth == Some(294.4));
    assert!(day.events[1].azimuth == Some(66.0));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn requesting_current_event() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = AstroEventRequest::new()
        .with_object(AstronomyObjectType::Moon)
        .with_placeid("3")
        .set_startdt(DateTime::from("2020-03-05"))
        .set_utctime(true)
        .set_isotime(true)
        .with_type(AstronomyEventClass::Current);

    let response = client.get_astro_events(&request).await.unwrap().unwrap();
    let current = &response.locations[0].astronomy.objects[0]
        .current
        .as_ref()
        .unwrap();

    assert!(current.isotime.is_some());
    assert!(current.utctime.is_some());

    assert!(current.illuminated.is_some());
    assert!(current.posangle.is_some());
    assert!(current.moonphase.is_some());
}
