use booru::{
    config::Config,
    api::posts::*,
    http::BooruClient,
};

#[test]
fn test_get_posts() {
    use std::path::Path;
    use std::fs;
    use std::thread::spawn;

    let conf = Config::load().unwrap();
    let client = BooruClient::from_config(&conf).unwrap();
    let query = PostsFetchRequestBuilder::default()
        .limit(100u64)
        .tags(vec!["rating:e".to_string(), "fellatio".to_string()])
        .build()
        .unwrap();
    let posts = client.get_posts(query).unwrap();
    eprintln!("{:#?}", posts);
    
    let path = Path::new("images/");
    fs::create_dir_all(&path).unwrap();
    let mut threads = Vec::new();
    for post in posts {
        threads.push(spawn(move || {
            let _ = post.save(&path);
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
}
