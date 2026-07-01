use cot::test::TestServerBuilder;
use shrt_backend::ShrtProject;
use shrt_common::links::{Link, LinkCreateRequest, LinkExists};

#[cot::e2e_test]
async fn test_create_and_get_link() -> cot::Result<()> {
    let server = TestServerBuilder::new(ShrtProject).start().await;
    let url = server.url();
    let client = reqwest::Client::new();

    // Create link
    let create_request = LinkCreateRequest {
        slug: Some("test-slug".to_string()),
        url: "https://example.com".to_string(),
    };

    let response = client
        .post(format!("{url}/links"))
        .json(&create_request)
        .send()
        .await
        .expect("Request failed");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let link: Link = response.json().await.expect("Failed to deserialize link");
    assert_eq!(link.slug, "test-slug");
    assert_eq!(link.url, "https://example.com");

    // Get link
    let response = client
        .get(format!("{url}/links/test-slug"))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let link: Link = response.json().await.expect("Failed to deserialize link");
    assert_eq!(link.slug, "test-slug");

    server.close().await;
    Ok(())
}

#[cot::e2e_test]
async fn test_link_exists() -> cot::Result<()> {
    let server = TestServerBuilder::new(ShrtProject).start().await;
    let url = server.url();
    let client = reqwest::Client::new();

    // Initially doesn't exist
    let response = client
        .get(format!("{url}/links/missing/exists"))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let exists: LinkExists = response.json().await.unwrap();
    assert!(!exists.exists);

    // Create link
    let create_request = LinkCreateRequest {
        slug: Some("existing".to_string()),
        url: "https://example.com".to_string(),
    };
    client
        .post(format!("{url}/links"))
        .json(&create_request)
        .send()
        .await
        .unwrap();

    // Now exists
    let response = client
        .get(format!("{url}/links/existing/exists"))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let exists: LinkExists = response.json().await.unwrap();
    assert!(exists.exists);

    server.close().await;
    Ok(())
}
