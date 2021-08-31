use libtad_rs::models::time::DSTEntrySpecialType;
use libtad_rs::service::time::DSTListRequest;
use libtad_rs::ServiceClient;
use maybe_async::maybe_async;

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_year(2016);
    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.len() == 129);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_for_specific_country() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_country("no");

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.len() == 1);
    assert!(response.dstlist[0].region.country.name == "Norway");
    assert!(response.dstlist[0].region.country.id == "no");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_for_specific_country_and_year() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_country("no").set_year(2014);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.len() == 1);
    assert!(response.dstlist[0].region.country.name == "Norway");
    assert!(response.dstlist[0].region.country.id == "no");
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_with_listplaces() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_listplaces(true);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response
        .dstlist
        .iter()
        .all(|x| x.region.locations.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_without_listplaces() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_listplaces(false);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response
        .dstlist
        .iter()
        .all(|x| x.region.locations.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_with_timechanges() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_timechanges(true);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.iter().all(|x| x.timechanges.is_some()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_without_timechanges() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_timechanges(false);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.iter().all(|x| x.timechanges.is_none()));
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_with_onlydst() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_onlydst(true).set_year(2014);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.len() == 132);
}

#[maybe_async]
#[cfg_attr(feature = "async-client", tokio::test)]
#[cfg_attr(feature = "sync-client", test)]
async fn request_dst_entries_without_onlydst() {
    let client = ServiceClient::new(env!("ACCESS_KEY").into(), env!("SECRET_KEY").into());

    let request = DSTListRequest::new().set_onlydst(false).set_year(2014);

    let response = client
        .get_daylight_savings_time(&request)
        .await
        .unwrap()
        .unwrap();

    assert!(response.dstlist.len() == 348);
    assert!(
        response
            .dstlist
            .iter()
            .flat_map(|x| x.special.as_ref())
            .filter(|x| matches!(x.r#type, DSTEntrySpecialType::NoDST))
            .count()
            > 0
    )
}
