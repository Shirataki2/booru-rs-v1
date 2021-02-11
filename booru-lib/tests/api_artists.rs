use booru::{
    config::Config,
    api::artists::*,
    http::BooruClient,
};

#[test]
fn test_get_artists() {
    let conf = Config::load().unwrap();
    let client = BooruClient::from_config(&conf).unwrap();
    let res = client.get_artists().unwrap();
    eprintln!("{:#?}", res);
    assert_ne!(res.len(), 0);
}

#[test]
fn test_get_artist() {
    let conf = Config::load().unwrap();
    let client = BooruClient::from_config(&conf).unwrap();
    let res = client.get_artist(219720).unwrap();
    eprintln!("{:#?}", res);
    assert_eq!(res.name, "nemune_neko".to_owned());
}

#[test]
#[ignore]
fn test_create_artist() {
    let conf = Config::load().unwrap();
    let client = BooruClient::from_config(&conf).unwrap();
    let body = ArtistCreateRequestBuilder::default()
        .name("faeioshfwio".to_owned())
        .group_name("aaa".to_owned())
        .other_names(vec!["bbb".to_owned()])
        .build()
        .unwrap();
    let res = client.create_artist(body).unwrap();
    eprintln!("{:#?}", res);
    panic!();
}