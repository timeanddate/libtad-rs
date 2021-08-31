use libtad_models::holidays::HolidayType;
use libtad_rs::service::holidays::HolidaysRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_holidays() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = HolidaysRequest::new().set_year(2014).with_country("us");

    let response = client.get_holidays(&request).await.unwrap();

    let first = &response.unwrap().holidays[0];

    assert!(first.name[0].text == "New Year's Day");
    assert!(first.uid == "0007d600000007de");
    assert!(first.url.as_str() == "https://www.timeanddate.com/holidays/us/new-year-day");
    assert!(first.id == 2006);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_holidays_filtered_by_type() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = HolidaysRequest::new()
        .with_country("us")
        .set_year(2014)
        .with_type(HolidayType::Christian)
        .with_type(HolidayType::Buddhism);

    let response = client.get_holidays(&request).await.unwrap().unwrap();

    // Returns 25 holidays
    assert!(response.holidays.len() == 25);

    // Returns a list of holidays with the requested types
    assert!(response.holidays.iter().all(|x| {
        x.types
            .as_ref()
            .unwrap()
            .iter()
            .any(|y| y == "Christian" || y == "Buddhism")
    }));

    // Every holiday belongs to the requested country
    assert!(response
        .holidays
        .iter()
        .all(|x| x.country.as_ref().unwrap().id == "us"));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_holidays_without_time_zone_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = HolidaysRequest::new()
        .with_country("us")
        .set_year(2021)
        .with_type(HolidayType::Seasons)
        .set_tz(false);

    let response = client.get_holidays(&request).await.unwrap().unwrap();

    // Holidays do not contain timezone information
    assert!(response.holidays.iter().all(|x| x.date.timezone.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_holidays_with_time_zone_information() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = HolidaysRequest::new()
        .with_country("us")
        .set_year(2021)
        .with_type(HolidayType::Seasons)
        .set_tz(true);

    let response = client.get_holidays(&request).await.unwrap().unwrap();

    // Holidays contain timezone information
    assert!(response.holidays.iter().all(|x| x.date.timezone.is_some()));
}
