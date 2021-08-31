use libtad_rs::models::{date_calculator::BusinessDaysFilterType, time::DateTime};
use libtad_rs::service::date_calculator::BusinessDateRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_add_days() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_placeid("usa/anchorage")
        .set_addition()
        .with_days(31);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap();

    let period = &response.periods[0];

    assert!(period.startdate.datetime == startdate);
    assert!(period.enddate.datetime == DateTime::from("2018-01-18"));

    assert!(period.includeddays == 31);
    assert!(period.calendardays == 48);
    assert!(period.skippeddays == 17);
    assert!(period.weekdays.mon == 0);
    assert!(period.weekdays.tue == 0);
    assert!(period.weekdays.wed == 0);
    assert!(period.weekdays.thu == 0);
    assert!(period.weekdays.fri == 0);
    assert!(period.weekdays.sat == 7);
    assert!(period.weekdays.sun == 7);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_subtract_days() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2018-02-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_placeid("usa/anchorage")
        .set_subtraction()
        .with_days(31);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap();

    let period = &response.periods[0];

    assert!(period.startdate.datetime == startdate);
    assert!(period.enddate.datetime == DateTime::from("2017-12-15"));

    assert!(period.includeddays == 31);
    assert!(period.calendardays == 48);
    assert!(period.skippeddays == 17);
    assert!(period.weekdays.mon == 0);
    assert!(period.weekdays.tue == 0);
    assert!(period.weekdays.wed == 0);
    assert!(period.weekdays.thu == 0);
    assert!(period.weekdays.fri == 0);
    assert!(period.weekdays.sat == 7);
    assert!(period.weekdays.sun == 7);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_add_days_repeating() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_placeid("usa/anchorage")
        .set_addition()
        .with_days(31)
        .set_repeat(5);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap();

    let period_0 = &response.periods[0];
    let period_1 = &response.periods[1];
    let period_2 = &response.periods[2];
    let period_3 = &response.periods[3];
    let period_4 = &response.periods[4];

    assert!(period_0.startdate.datetime == startdate);
    assert!(period_0.enddate.datetime == DateTime::from("2018-01-18"));

    assert!(period_1.startdate.datetime == DateTime::from("2018-01-18"));
    assert!(period_1.enddate.datetime == DateTime::from("2018-03-05"));

    assert!(period_2.startdate.datetime == DateTime::from("2018-03-05"));
    assert!(period_2.enddate.datetime == DateTime::from("2018-04-18"));

    assert!(period_3.startdate.datetime == DateTime::from("2018-04-18"));
    assert!(period_3.enddate.datetime == DateTime::from("2018-06-01"));

    assert!(period_4.startdate.datetime == DateTime::from("2018-06-01"));
    assert!(period_4.enddate.datetime == DateTime::from("2018-07-17"));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_add_days_with_invalid_repeat() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_placeid("usa/anchorage")
        .set_addition()
        .with_days(31)
        .with_days(41)
        .set_repeat(5);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap_err();

    assert!(response.to_string() == "API Error: Error: The days parameter must have a single value if repeat parameter is enabled.");
}

/*
 * Blocked by API-995
 *
#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_add_days_with_country_and_state() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_country("us")
        .set_state("us-nv")
        .set_addition()
        .with_days(31);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.geo.state.as_deref() == Some("Nevada"));
    assert!(response.geo.country.name == "United States");
}
*/

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_add_days_with_filter() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());
    let startdate = DateTime::from("2017-12-01");

    let request = BusinessDateRequest::new()
        .set_startdt(startdate)
        .set_placeid("usa/anchorage")
        .set_addition()
        .with_days(31)
        .with_days(41)
        .with_filter(BusinessDaysFilterType::Mon)
        .with_filter(BusinessDaysFilterType::Tue);

    let response = client
        .calculate_business_date(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.periods.iter().all(|x| x.weekdays.mon > 0));
    assert!(response.periods.iter().all(|x| x.weekdays.tue > 0));
    assert!(response.periods.iter().all(|x| x.weekdays.wed == 0));
    assert!(response.periods.iter().all(|x| x.weekdays.thu == 0));
    assert!(response.periods.iter().all(|x| x.weekdays.fri == 0));
    assert!(response.periods.iter().all(|x| x.weekdays.sat == 0));
    assert!(response.periods.iter().all(|x| x.weekdays.sun == 0));
}
