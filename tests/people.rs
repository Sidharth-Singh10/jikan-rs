use common::rate_limited_test;
use jikan_rs::JikanClient;
mod common;

#[tokio::test]
async fn get_person_from_id() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_by_id(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_full() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_by_id_full(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_anime() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_anime(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_manga() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_manga(2619).await; // JOJO REFERENCEEEEEAAAAAAAA (Araki, Hirohiko)
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_voice() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_voice(195).await; // Junichi Suwabe(Seikyuu: Ryomen Sukuna)
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_pictures() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_person_pictures(1).await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client.get_people().await;
        assert!(result.is_ok());
    })
    .await;
}

#[tokio::test]
async fn get_person_search() {
    rate_limited_test(|| async {
        let client = JikanClient::new();
        let result = client
            .get_people_search(
                None,
                None,
                Some(String::from("Junichi Suwabe")),
                None,
                None,
                None,
            )
            .await;
        assert!(result.is_ok());
    })
    .await;
}
