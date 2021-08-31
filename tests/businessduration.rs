use libtad_rs::models::{date_calculator::BusinessDaysFilterType, time::DateTime};
use libtad_rs::service::date_calculator::BusinessDurationRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_duration() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");
    let enddate = DateTime::from("2018-01-31");

    let request = BusinessDurationRequest::new()
        .set_startdt(startdate)
        .set_enddt(enddate)
        .set_placeid("usa/anchorage");

    let response = client
        .calculate_business_duration(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.geo.name == "Anchorage");

    assert!(response.period.includeddays == 40);
    assert!(response.period.calendardays == 61);
    assert!(response.period.skippeddays == 21);

    assert!(response.period.weekdays.sat == 9);
    assert!(response.period.weekdays.sun == 9);

    assert!(response.period.holidays.count == Some(3));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_duration_with_include() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");
    let enddate = DateTime::from("2018-01-31");

    let request = BusinessDurationRequest::new()
        .set_startdt(startdate)
        .set_enddt(enddate)
        .set_include(true)
        .set_placeid("usa/anchorage");

    let response = client
        .calculate_business_duration(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.geo.name == "Anchorage");

    assert!(response.period.includeddays == 21);
    assert!(response.period.calendardays == 61);
    assert!(response.period.skippeddays == 40);

    assert!(response.period.weekdays.sat == 9);
    assert!(response.period.weekdays.sun == 9);

    assert!(response.period.holidays.count == Some(3));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_duration_with_includelastdate() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");
    let enddate = DateTime::from("2018-01-31");

    let request = BusinessDurationRequest::new()
        .set_startdt(startdate)
        .set_enddt(enddate)
        .set_includelastdate(true)
        .set_placeid("usa/anchorage");

    let response = client
        .calculate_business_duration(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.geo.name == "Anchorage");

    assert!(response.period.includeddays == 41);
    assert!(response.period.calendardays == 62);
    assert!(response.period.skippeddays == 21);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_duration_with_filter() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");
    let enddate = DateTime::from("2018-01-31");

    let request = BusinessDurationRequest::new()
        .set_startdt(startdate)
        .set_enddt(enddate)
        .with_filter(BusinessDaysFilterType::Mon)
        .with_filter(BusinessDaysFilterType::Tue)
        .set_placeid("usa/anchorage");

    let response = client
        .calculate_business_duration(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.geo.name == "Anchorage");

    assert!(response.period.includeddays == 43);
    assert!(response.period.calendardays == 61);
    assert!(response.period.skippeddays == 18);

    assert!(response.period.weekdays.mon == 9);
    assert!(response.period.weekdays.tue == 9);
    assert!(response.period.weekdays.wed == 0);
    assert!(response.period.weekdays.thu == 0);
    assert!(response.period.weekdays.fri == 0);
    assert!(response.period.weekdays.sat == 0);
    assert!(response.period.weekdays.sun == 0);
}
