use cot::db::migrations::MigrationEngine;
use cot::test::Client;
use cot::{App, Project, StatusCode};
use shrt_backend::{LinkApp, ShrtProject};
use shrt_common::links::{Link, LinkCreateRequest};

#[cot::test]
async fn test_create_and_get_link() {
    let project = ShrtProject;
    let mut client = Client::new(project).await;

    // Run migrations
    let db = client.context().try_database().unwrap();
    let app = LinkApp;
    let engine = MigrationEngine::new(app.migrations()).unwrap();
    engine.run(db).await.unwrap();

    // Create link
    let create_request = LinkCreateRequest {
        slug: Some("test-slug".to_string()),
        url: "https://example.com".to_string(),
    };

    let response = client
        .request(
            cot::http::Request::post("/links")
                .header("Content-Type", "application/json")
                .body(cot::Body::from(
                    serde_json::to_string(&create_request).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .expect("Request failed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .into_bytes()
        .await
        .expect("Failed to read body");
    let link: Link = serde_json::from_slice(&body).expect("Failed to deserialize link");
    assert_eq!(link.slug, "test-slug");
    assert_eq!(link.url, "https://example.com");

    // Get link
    let response = client
        .get("/links/test-slug")
        .await
        .expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .into_bytes()
        .await
        .expect("Failed to read body");
    let link: Link = serde_json::from_slice(&body).expect("Failed to deserialize link");
    assert_eq!(link.slug, "test-slug");
}

#[cot::test]
async fn test_link_exists() {
    let project = ShrtProject;
    let mut client = Client::new(project).await;

    // Run migrations
    let db = client.context().try_database().unwrap();
    let app = LinkApp;
    let engine = MigrationEngine::new(app.migrations()).unwrap();
    engine.run(db).await.unwrap();

    // Initially doesn't exist
    let response = client
        .get("/links/missing/exists")
        .await
        .expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .into_bytes()
        .await
        .expect("Failed to read body");
    let exists: shrt_common::links::LinkExists = serde_json::from_slice(&body).unwrap();
    assert!(!exists.exists);

    // Create link
    let create_request = LinkCreateRequest {
        slug: Some("existing".to_string()),
        url: "https://example.com".to_string(),
    };
    client
        .request(
            cot::http::Request::post("/links")
                .header("Content-Type", "application/json")
                .body(cot::Body::from(
                    serde_json::to_string(&create_request).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now exists
    let response = client
        .get("/links/existing/exists")
        .await
        .expect("Request failed");
    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .into_bytes()
        .await
        .expect("Failed to read body");
    let exists: shrt_common::links::LinkExists = serde_json::from_slice(&body).unwrap();
    assert!(exists.exists);
}
