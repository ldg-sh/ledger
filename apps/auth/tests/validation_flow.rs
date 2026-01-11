mod common;

use actix_web::{http::StatusCode, test};
use common::{client::TestClient, TestContext};
use ledger_auth::grpc::pb::authentication_server::Authentication;
use tonic::Request;

// HTTP validation tests
#[tokio::test]
async fn test_http_token_validation_flow_success() {
    println!("\n\n[+] Running test: test_http_token_validation_flow_success");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Creating user for token validation.");
    let (_user_id, user_token) = match client.create_test_user(None).await {
        Ok(i) => i,
        Err(e) => {
            println!("[\\]. Failed creating a test user. \n\n E: {}", e);
            panic!("Failed creating a test user. \n\n E: {}", e)
        }
    };
    println!("[<] User created.");

    println!("[>] Sending request to /validate with valid token.");
    let req = test::TestRequest::post()
        .uri("/validate")
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::OK);
    println!("[/] Test passed: HTTP token validation successful.");
}

#[tokio::test]
async fn test_http_token_validation_flow_invalid_token() {
    println!("\n\n[+] Running test: test_http_token_validation_flow_invalid_token");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending request to /validate with invalid token.");
    let req = test::TestRequest::post()
        .uri("/validate")
        .insert_header(("Authorization", "Bearer invalid_token_here"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    println!("[/] Test passed: Correctly returned UNAUTHORIZED for invalid token.");
}

#[tokio::test]
async fn test_http_token_validation_flow_missing_auth() {
    println!("\n\n[+] Running test: test_http_token_validation_flow_missing_auth");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending request to /validate with missing auth header.");
    let req = test::TestRequest::post().uri("/validate").to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    println!("[/] Test passed: Correctly returned UNAUTHORIZED for missing auth.");
}

#[tokio::test]
async fn test_http_token_validation_flow_malformed_auth() {
    println!("\n\n[+] Running test: test_http_token_validation_flow_malformed_auth");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending request to /validate with malformed auth header.");
    let req = test::TestRequest::post()
        .uri("/validate")
        .insert_header(("Authorization", "NotBearer invalid_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    println!("[/] Test passed: Correctly returned UNAUTHORIZED for malformed auth.");
}

// gRPC validation tests
#[tokio::test]
async fn test_grpc_token_validation_flow_success() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_success");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");

    // Create test user and get token
    println!("[>] Creating user for gRPC token validation.");
    let (_user_id, user_token) = match client.create_test_user(None).await {
        Ok(i) => i,
        Err(e) => {
            println!("[\\]. Failed creating a test user. \n\n E: {}", e);
            panic!("Failed creating a test user. \n\n E: {}", e)
        }
    };
    println!("[<] User created.");
    println!("[<] User token {}", user_token);

    // Create gRPC service
    println!("[+] Creating gRPC authentication service.");
    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());

    // Create request with metadata
    println!("[>] Creating gRPC request with valid token.");
    let mut request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: user_token.clone(),
    });

    let grpc_auth_key = ledger_auth::config::config().grpc.auth_key.clone();
    request
        .metadata_mut()
        .insert("authorization", grpc_auth_key.parse().unwrap());

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    assert!(validation_response.is_valid);
    assert_eq!(validation_response.message, "ok");
    println!("[/] Test passed: gRPC token validation successful.");
}

#[tokio::test]
async fn test_grpc_token_validation_flow_invalid_token() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_invalid_token");
    let ctx = TestContext::new().await;
    println!("[+] Test context created.");

    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());
    println!("[+] gRPC authentication service created.");

    println!("[>] Creating gRPC request with invalid token.");
    let mut request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: "invalid_token".to_string(),
    });

    request
        .metadata_mut()
        .insert("authorization", "Bearer invalid_token".parse().unwrap());

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    assert!(!validation_response.is_valid);
    println!("[/] Test passed: Correctly identified invalid gRPC token.");
}

#[tokio::test]
async fn test_grpc_token_validation_flow_missing_auth_header() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_missing_auth_header");
    let ctx = TestContext::new().await;
    println!("[+] Test context created.");

    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());
    println!("[+] gRPC authentication service created.");

    // Request without authorization header
    println!("[>] Creating gRPC request with missing auth header.");
    let request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: "some_token".to_string(),
    });

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    assert!(!validation_response.is_valid);
    println!("[/] Test passed: Correctly identified missing gRPC auth header.");
}

#[tokio::test]
async fn test_grpc_token_validation_flow_malformed_auth_header() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_malformed_auth_header");
    let ctx = TestContext::new().await;
    println!("[+] Test context created.");

    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());
    println!("[+] gRPC authentication service created.");

    println!("[>] Creating gRPC request with malformed auth header.");
    let mut request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: "some_token".to_string(),
    });

    // Malformed auth header (not "Bearer token" format)
    request
        .metadata_mut()
        .insert("authorization", "NotBearer some_token".parse().unwrap());

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    assert!(!validation_response.is_valid);
    println!("[/] Test passed: Correctly identified malformed gRPC auth header.");
}

#[tokio::test]
async fn test_grpc_token_validation_flow_admin_token() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_admin_token");
    let ctx = TestContext::new().await;
    println!("[+] Test context created.");

    // Create test admin and get token
    println!("[>] Creating admin user for gRPC token validation.");
    let (_admin_id, admin_token) = {
        let client = TestClient::new(ctx.db.clone());
        client.create_test_admin().await
    };
    println!("[<] Admin user created.");

    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());
    println!("[+] gRPC authentication service created.");

    println!("[>] Creating gRPC request with admin token.");
    let mut request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: admin_token.clone(),
    });

    let grpc_auth_key = ledger_auth::config::config().grpc.auth_key.clone();
    request
        .metadata_mut()
        .insert("authorization", grpc_auth_key.parse().unwrap());

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    assert!(validation_response.is_valid);
    assert_eq!(validation_response.message, "ok");
    println!("[/] Test passed: gRPC admin token validation successful.");
}

#[tokio::test]
async fn test_grpc_token_validation_flow_token_mismatch() {
    println!("\n\n[+] Running test: test_grpc_token_validation_flow_token_mismatch");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");

    // Create test user and get token
    println!("[>] Creating user for gRPC token validation.");
    let (_user_id, user_token) = match client.create_test_user(None).await {
        Ok(i) => i,
        Err(e) => {
            println!("[\\]. Failed creating a test user. \n\n E: {}", e);
            panic!("Failed creating a test user. \n\n E: {}", e)
        }
    };
    println!("[<] User created.");

    let auth_svc = ledger_auth::grpc::authentication::AuthenticationSvc::new(ctx.db.clone());
    println!("[+] gRPC authentication service created.");

    println!("[>] Creating gRPC request with token mismatch.");
    let mut request = Request::new(ledger_auth::grpc::pb::ValidationRequest {
        token: "different_token".to_string(),
    });

    request.metadata_mut().insert(
        "authorization",
        format!("Bearer {}", user_token).parse().unwrap(),
    );

    println!("[>] Sending gRPC request to validate_authentication.");
    let response = auth_svc.validate_authentication(request).await;
    println!("[<] Received gRPC response.");

    assert!(response.is_ok());
    let resp = response.unwrap();
    let validation_response = resp.into_inner();
    println!("[<] gRPC response body: {:?}", validation_response);

    // Should be invalid because the token in the request body doesn't match the auth header
    assert!(!validation_response.is_valid);
    println!("[/] Test passed: Correctly identified gRPC token mismatch.");
}
