use libtad_rs::service::holidays::HolidaysRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn test_authentication_error() {
    let client = ServiceClient::new("".into(), "".into());

    let request = HolidaysRequest::new().with_country("no").set_year(2021);

    let http_response = client.get_holidays(&request).await;

    assert!(http_response.is_ok());

    let api_response = http_response.unwrap();

    assert!(api_response.is_err());
    assert!(api_response.unwrap_err().errors[0] == "Authorization failed.");
}
