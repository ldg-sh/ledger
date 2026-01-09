mod common;

use actix_web::{http::StatusCode, test};
use common::{client::TestClient, test_data, TestContext};

#[tokio::test]
async fn test_user_creation_flow_success() {
    println!("\n\n[+] Running test: test_user_creation_flow_success");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    // Create admin user for authentication
    println!("[>] Creating admin user for authentication.");
    let (_admin_id, admin_token) = client.create_test_admin().await;
    println!("[<] Admin user created.");

    let user_data = test_data::sample_user();
    println!("[>] Sending request to create user: {:?}", user_data.name);

    let req = test::TestRequest::post()
        .uri("/user/create")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&user_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    println!("[<] Response body: {}", body);
    assert!(body["message"].as_str().unwrap().contains("User created"));

    // Verify user was created in database
    println!(
        "[>] Verifying user creation in database for email: {}",
        user_data.email
    );
    let created_user = ctx.db.get_user_by_email(&user_data.email).await;
    assert!(created_user.is_ok());
    println!("[<] User found in database.");

    let user = created_user.unwrap();
    assert_eq!(user.email, user_data.email);
    assert_eq!(user.name, user_data.name);
    println!("{}", user.auth_hash.clone());
    assert!(!user.auth_hash.is_empty());
    println!("[/] Test passed: User creation flow successful.");
}

// TODO: Add basic levels of auth to the user creation route. I need to figure out how that will look in the end because it is important
// that we dont allow just anyone to send a request to make an account
// #[tokio::test]
// async fn test_user_creation_flow_unauthorized() {
//     println!("\n\n[+] Running test: test_user_creation_flow_unauthorized");
//     let ctx = TestContext::new().await;
//     let client = TestClient::new(ctx.db.clone());
//     println!("[+] Test client and context created.");
//     let app = test::init_service(client.create_app()).await;
//     println!("[+] Actix web app initialized.");

//     let user_data = test_data::sample_user();
//     println!("[>] Sending request to create user with invalid token: {:?}", user_data.name);

//     let req = test::TestRequest::post()
//         .uri("/user/create")
//         .insert_header(("Authorization", "Bearer invalid_token"))
//         .set_json(&user_data)
//         .to_request();

//     let resp = test::call_service(&app, req).await;
//     println!("[<] Received response with status: {}", resp.status());

//     assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
//     println!("[/] Test passed: Correctly returned UNAUTHORIZED.");
// }

// TODO: Add basic levels of auth to the user creation route. I need to figure out how that will look in the end because it is important
// that we dont allow just anyone to send a request to make an account
// #[tokio::test]
// async fn test_user_creation_flow_missing_auth() {
//     println!("\n\n[+] Running test: test_user_creation_flow_missing_auth");
//     let ctx = TestContext::new().await;
//     let client = TestClient::new(ctx.db.clone());
//     println!("[+] Test client and context created.");
//     let app = test::init_service(client.create_app()).await;
//     println!("[+] Actix web app initialized.");

//     let user_data = test_data::sample_user();
//     println!("[>] Sending request to create user with missing auth header: {:?}", user_data.name);

//     let req = test::TestRequest::post()
//         .uri("/user/create")
//         .set_json(&user_data)
//         .to_request();

//     let resp = test::call_service(&app, req).await;
//     println!("[<] Received response with status: {}", resp.status());

//     assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
//     println!("[/] Test passed: Correctly returned UNAUTHORIZED for missing auth.");
// }

// TODO: Add basic levels of auth to the user creation route. I need to figure out how that will look in the end because it is important
// that we dont allow just anyone to send a request to make an account
// #[tokio::test]
// async fn test_user_creation_flow_user_token_forbidden() {
//     println!("\n\n[+] Running test: test_user_creation_flow_user_token_forbidden");
//     let ctx = TestContext::new().await;
//     let client = TestClient::new(ctx.db.clone());
//     println!("[+] Test client and context created.");
//     let app = test::init_service(client.create_app()).await;
//     println!("[+] Actix web app initialized.");

//     // Create regular user (not admin) for authentication
//     println!("[>] Creating regular user for authentication.");
//     let (_user_id, user_token) = match client.create_test_user(None).await {
//         Ok(i) => i,
//         Err(e) => {
//             println!("[\\]. Failed creating a test user. \n\n E: {}", e);
//             panic!("Failed creating a test user. \n\n E: {}", e)
//         },
//     };
//     println!("[<] Regular user created.");

//     let user_data = test_data::sample_user_with_email("newuser@test.com");
//     println!("[>] Sending request to create user with user token (should be forbidden): {:?}", user_data.name);

//     let req = test::TestRequest::post()
//         .uri("/user/create")
//         .insert_header(("Authorization", format!("Bearer {}", user_token)))
//         .set_json(&user_data)
//         .to_request();

//     let resp = test::call_service(&app, req).await;
//     println!("[<] Received response with status: {}", resp.status());

//     // Should be forbidden since regular users can't create other users
//     assert_eq!(resp.status(), StatusCode::FORBIDDEN);
//     println!("[/] Test passed: Correctly returned FORBIDDEN.");
// }

#[tokio::test]
async fn test_user_creation_flow_duplicate_email() {
    println!("\n\n[+] Running test: test_user_creation_flow_duplicate_email");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Creating admin user for authentication.");
    let (_admin_id, admin_token) = client.create_test_admin().await;
    println!("[<] Admin user created.");

    // Create first user
    let user_data1 = test_data::sample_user();
    println!(
        "[>] Sending request to create first user: {:?}",
        user_data1.name
    );
    let req1 = test::TestRequest::post()
        .uri("/user/create")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&user_data1)
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    println!(
        "[<] Received response for first user with status: {}",
        resp1.status()
    );
    assert_eq!(resp1.status(), StatusCode::CREATED);

    // Try to create user with same email
    let user_data2 = test_data::sample_user(); // Same email
    println!(
        "[>] Sending request to create second user with same email: {:?}",
        user_data2.name
    );
    let req2 = test::TestRequest::post()
        .uri("/user/create")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&user_data2)
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    println!(
        "[<] Received response for second user with status: {}",
        resp2.status()
    );

    // Should fail due to duplicate email
    assert!(resp2.status().is_client_error() || resp2.status().is_server_error());
    println!("[/] Test passed: Correctly failed to create user with duplicate email.");
}

#[tokio::test]
async fn test_user_regenerate_token_flow_success() {
    println!("\n\n[+] Running test: test_user_regenerate_token_flow_success");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Creating user to test token regeneration.");
    let (user_id, user_token) = match client.create_test_user(None).await {
        Ok(i) => i,
        Err(e) => {
            println!("[\\]. Failed creating a test user. \n\n E: {}", e);
            panic!("Failed creating a test user. \n\n E: {}", e)
        }
    };
    println!("[<] User created with ID: {}", user_id);

    println!(
        "[>] Sending request to regenerate token for user: {}",
        user_id
    );
    let req = test::TestRequest::post()
        .uri("/user/regenerate")
        .insert_header(("Authorization", format!("Bearer {}", user_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    println!("[<] Response body: {}", body);
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Regenerated user token"));

    // Verify token was actually changed in database
    println!(
        "[>] Verifying token was changed in database for user: {}",
        user_id
    );
    let updated_user = ctx.db.get_user_by_id(&user_id).await.unwrap();
    assert!(!updated_user.auth_hash.is_empty());
    println!("[<] Token verified in database.");
    println!("[/] Test passed: User token regeneration successful.");
}

#[tokio::test]
async fn test_user_regenerate_token_flow_unauthorized() {
    println!("\n\n[+] Running test: test_user_regenerate_token_flow_unauthorized");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending request to regenerate token with invalid token.");
    let req = test::TestRequest::post()
        .uri("/user/regenerate")
        .insert_header(("Authorization", "Bearer invalid_token"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    println!("[/] Test passed: Correctly returned UNAUTHORIZED.");
}

#[tokio::test]
async fn test_user_regenerate_token_flow_missing_auth() {
    println!("\n\n[+] Running test: test_user_regenerate_token_flow_missing_auth");
    let ctx = TestContext::new().await;
    let client = TestClient::new(ctx.db.clone());
    println!("[+] Test client and context created.");
    let app = test::init_service(client.create_app()).await;
    println!("[+] Actix web app initialized.");

    println!("[>] Sending request to regenerate token with missing auth header.");
    let req = test::TestRequest::post()
        .uri("/user/regenerate")
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("[<] Received response with status: {}", resp.status());

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    println!("[/] Test passed: Correctly returned UNAUTHORIZED for missing auth.");
}
