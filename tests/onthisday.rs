use libtad_rs::models::{onthisday::EventType, time::Time};
use libtad_rs::service::onthisday::OnThisDayRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_all() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new().set_month(5).set_day(24);
    let response = client
        .get_events_on_this_day(&request)
        .await
        .unwrap()
        .unwrap();

    let check_date = |t: &Time| t.datetime.month == 5 && t.datetime.day == 24;

    assert!(response.events.unwrap().iter().all(|x| check_date(&x.date)));

    assert!(response
        .births
        .unwrap()
        .iter()
        .all(|x| check_date(&x.birthdate.as_ref().unwrap())));

    assert!(response
        .deaths
        .unwrap()
        .iter()
        .all(|x| check_date(&x.deathdate.as_ref().unwrap())));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_events_only() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new()
        .set_month(5)
        .set_day(24)
        .with_type(EventType::Events);

    let response = client
        .get_events_on_this_day(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.events.is_some());
    assert!(response.events.unwrap().len() > 0);

    assert!(response.births.is_none());
    assert!(response.deaths.is_none());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_births_only() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new()
        .set_month(5)
        .set_day(24)
        .with_type(EventType::Births);

    let response = client
        .get_events_on_this_day(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.births.is_some());
    assert!(response.births.unwrap().len() > 0);

    assert!(response.events.is_none());
    assert!(response.deaths.is_none());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_deaths_only() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new()
        .set_month(5)
        .set_day(24)
        .with_type(EventType::Deaths);

    let response = client
        .get_events_on_this_day(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.deaths.is_some());
    assert!(response.deaths.unwrap().len() > 0);

    assert!(response.events.is_none());
    assert!(response.births.is_none());
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn requesting_invalid_day() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new().set_month(5).set_day(35);

    let response = client.get_events_on_this_day(&request).await.unwrap();

    assert!(response.is_err());
    assert!(
        response.unwrap_err().to_string() == "API Error: Parameter 'day' must be between 1 and 31."
    );
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn requesting_invalid_month() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = OnThisDayRequest::new().set_month(0).set_day(24);

    let response = client.get_events_on_this_day(&request).await.unwrap();

    assert!(response.is_err());
    assert!(
        response.unwrap_err().to_string()
            == "API Error: Parameter 'month' must be between 1 and 12."
    );
}
