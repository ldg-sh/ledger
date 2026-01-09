mod common;

use actix_web::{http::StatusCode, test};
use common::{client::TestClient, TestContext};

#[tokio::test]
async fn test_health_check_flow_success() {
    println!("\n\n[+] Running test: test_health_check_flow_success");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");

    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending GET request to /health");
    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::OK);
    println!("[/] Test passed: Health check successful.");
}

#[tokio::test]
async fn test_health_check_flow_wrong_http_method() {
    println!("\n\n[+] Running test: test_health_check_flow_wrong_http_method");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");

    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    let (_user_id, user_token) = match client.create_test_user(None).await {
        Ok(i) => i,
        Err(e) => {
            println!("[\\]. Failed creating a test user. \n\n E: {}", e);
            panic!("Failed creating a test user. \n\n E: {}", e)
        }
    };
    println!("[+] Created test user for auth.");

    // Health endpoint expects GET, try POST
    println!("[>] Sending POST request to /health (expecting failure)");
    let req = test::TestRequest::post()
        .uri("/health")
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    // Should return not found.
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    println!("[/] Test passed: Correctly returned NOT_FOUND for wrong HTTP method.");
}
