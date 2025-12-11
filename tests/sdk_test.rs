//! Integration tests for the Sevk SDK

use sevk::{Sevk, SevkOptions};
use sevk::types::*;

const BASE_URL: &str = "http://localhost:4000";

fn unique_id() -> String {
    format!("{}{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis(), rand::random::<u32>() % 10000)
}

async fn setup_test_environment() -> Sevk {
    let http_client = reqwest::Client::new();
    let unique = unique_id();

    // 1. Register a new test user
    let test_email = format!("sdk-test-{}@test.example.com", unique);
    let test_password = "TestPassword123!";

    let register_res = http_client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "email": test_email,
            "password": test_password
        }))
        .send()
        .await
        .expect("Failed to register");

    let register_body: serde_json::Value = register_res.json().await.expect("Failed to parse register response");
    let token = register_body["token"].as_str().expect("No token in response");

    // 2. Create Project
    let project_res = http_client
        .post(format!("{}/projects", BASE_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": "Test Project",
            "slug": format!("test-project-{}", unique),
            "supportEmail": "support@test.com"
        }))
        .send()
        .await
        .expect("Failed to create project");

    let project_body: serde_json::Value = project_res.json().await.expect("Failed to parse project response");
    let project_id = project_body["project"]["id"].as_str().expect("No project id");

    // 3. Create API Key
    let api_key_res = http_client
        .post(format!("{}/projects/{}/api-keys", BASE_URL, project_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "title": "Test Key",
            "fullAccess": true
        }))
        .send()
        .await
        .expect("Failed to create API key");

    let api_key_body: serde_json::Value = api_key_res.json().await.expect("Failed to parse API key response");
    let api_key = api_key_body["apiKey"]["key"].as_str().expect("No API key");

    let options = SevkOptions {
        base_url: BASE_URL.to_string(),
        timeout: 30000,
    };

    Sevk::with_options(api_key, options)
}

// ============================================
// AUTHENTICATION TESTS
// ============================================

#[tokio::test]
async fn test_auth_invalid_api_key() {
    let options = SevkOptions {
        base_url: BASE_URL.to_string(),
        timeout: 30000,
    };
    let sevk = Sevk::with_options("sevk_invalid_api_key_12345", options);

    let result = sevk.contacts().list(None).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("401") || error_str.to_lowercase().contains("invalid"));
    }
}

#[tokio::test]
async fn test_auth_empty_api_key() {
    let options = SevkOptions {
        base_url: BASE_URL.to_string(),
        timeout: 30000,
    };
    let sevk = Sevk::with_options("", options);

    let result = sevk.contacts().list(None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auth_malformed_api_key() {
    let options = SevkOptions {
        base_url: BASE_URL.to_string(),
        timeout: 30000,
    };
    let sevk = Sevk::with_options("invalid_key_format", options);

    let result = sevk.contacts().list(None).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("401"));
    }
}

// ============================================
// CONTACTS TESTS
// ============================================

#[tokio::test]
async fn test_contacts_list_structure() {
    let sevk = setup_test_environment().await;

    let result = sevk.contacts().list(None).await;
    assert!(result.is_ok());

    let contacts = result.unwrap();
    assert!(contacts.total >= 0);
    assert!(contacts.page >= 1);
    assert!(contacts.total_pages >= 0);
}

#[tokio::test]
async fn test_contacts_list_pagination() {
    let sevk = setup_test_environment().await;

    let params = ListParams {
        page: Some(1),
        limit: Some(5),
        search: None,
    };
    let result = sevk.contacts().list(Some(params)).await;
    assert!(result.is_ok());

    let contacts = result.unwrap();
    assert_eq!(contacts.page, 1);
}

#[tokio::test]
async fn test_contacts_create_required() {
    let sevk = setup_test_environment().await;

    let email = format!("test-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };

    let result = sevk.contacts().create(&email, Some(create_request)).await;
    assert!(result.is_ok());

    let contact = result.unwrap();
    assert!(!contact.id.is_empty());
    assert_eq!(contact.email, email);
    assert!(contact.subscribed); // Default is true? Or check if it's boolean
}

#[tokio::test]
async fn test_contacts_get_by_id() {
    let sevk = setup_test_environment().await;

    let email = format!("get-test-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_request)).await.unwrap();

    let result = sevk.contacts().get(&contact.id).await;
    assert!(result.is_ok());

    let fetched = result.unwrap();
    assert_eq!(fetched.id, contact.id);
    assert_eq!(fetched.email, email);
}

#[tokio::test]
async fn test_contacts_update() {
    let sevk = setup_test_environment().await;

    let email = format!("update-test-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        subscribed: Some(true),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_request)).await.unwrap();

    let update_request = UpdateContactRequest {
        subscribed: Some(false),
        ..Default::default()
    };

    let result = sevk.contacts().update(&contact.id, update_request).await;
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.id, contact.id);
    assert_eq!(updated.subscribed, false);
}

#[tokio::test]
async fn test_contacts_error_non_existent() {
    let sevk = setup_test_environment().await;

    let result = sevk.contacts().get("non-existent-id").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

#[tokio::test]
async fn test_contacts_delete() {
    let sevk = setup_test_environment().await;

    let email = format!("delete-test-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_request)).await.unwrap();

    let result = sevk.contacts().delete(&contact.id).await;
    assert!(result.is_ok());

    let get_result = sevk.contacts().get(&contact.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// AUDIENCES TESTS
// ============================================

#[tokio::test]
async fn test_audiences_list_structure() {
    let sevk = setup_test_environment().await;

    let result = sevk.audiences().list(None).await;
    assert!(result.is_ok());

    let audiences = result.unwrap();
    assert!(audiences.total >= 0);
    assert!(audiences.page >= 1);
}

#[tokio::test]
async fn test_audiences_create_required() {
    let sevk = setup_test_environment().await;

    let name = format!("Test Audience {}", unique_id());
    let create_request = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };

    let result = sevk.audiences().create(create_request).await;
    assert!(result.is_ok());

    let audience = result.unwrap();
    assert!(!audience.id.is_empty());
    assert_eq!(audience.name, name);
}

#[tokio::test]
async fn test_audiences_create_all_fields() {
    let sevk = setup_test_environment().await;

    let name = format!("Full Audience {}", unique_id());
    let create_request = CreateAudienceRequest {
        name: name.clone(),
        description: Some("Test description".to_string()),
        users_can_see: Some("PUBLIC".to_string()),
    };

    let result = sevk.audiences().create(create_request).await;
    assert!(result.is_ok());

    let audience = result.unwrap();
    assert_eq!(audience.name, name);
    assert_eq!(audience.description, Some("Test description".to_string()));
    assert_eq!(audience.users_can_see, Some("PUBLIC".to_string()));
}

#[tokio::test]
async fn test_audiences_get_by_id() {
    let sevk = setup_test_environment().await;

    let name = format!("Get Audience {}", unique_id());
    let create_request = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_request).await.unwrap();

    let result = sevk.audiences().get(&audience.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, audience.id);
}

#[tokio::test]
async fn test_audiences_update() {
    let sevk = setup_test_environment().await;

    let name = format!("Update Audience {}", unique_id());
    let create_request = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_request).await.unwrap();

    let new_name = format!("Updated Audience {}", unique_id());
    let update_request = UpdateAudienceRequest {
        name: Some(new_name.clone()),
        ..Default::default()
    };

    let result = sevk.audiences().update(&audience.id, update_request).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, new_name);
}

#[tokio::test]
async fn test_audiences_add_contacts() {
    let sevk = setup_test_environment().await;

    // Create audience
    let name = format!("Add Contacts Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create contact
    let email = format!("add-contact-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();

    // Add contact to audience
    let contact_ids = vec![contact.id.clone()];
    let result = sevk.audiences().add_contacts(&audience.id, contact_ids).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_audiences_delete() {
    let sevk = setup_test_environment().await;

    let name = format!("Delete Audience {}", unique_id());
    let create_request = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_request).await.unwrap();

    let result = sevk.audiences().delete(&audience.id).await;
    assert!(result.is_ok());

    let get_result = sevk.audiences().get(&audience.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// TEMPLATES TESTS
// ============================================

#[tokio::test]
async fn test_templates_list_structure() {
    let sevk = setup_test_environment().await;

    let result = sevk.templates().list(None).await;
    assert!(result.is_ok());

    let templates = result.unwrap();
    assert!(templates.total >= 0);
    assert!(templates.page >= 1);
}

#[tokio::test]
async fn test_templates_create_required() {
    let sevk = setup_test_environment().await;

    let title = format!("Test Template {}", unique_id());
    let create_request = CreateTemplateRequest {
        title: title.clone(),
        content: "<p>Hello {{name}}</p>".to_string(),
    };

    let result = sevk.templates().create(create_request).await;
    assert!(result.is_ok());

    let template = result.unwrap();
    assert!(!template.id.is_empty());
    assert_eq!(template.title, title);
    assert_eq!(template.content, "<p>Hello {{name}}</p>");
}

#[tokio::test]
async fn test_templates_get_by_id() {
    let sevk = setup_test_environment().await;

    let title = format!("Get Template {}", unique_id());
    let create_request = CreateTemplateRequest {
        title: title.clone(),
        content: "<p>Test</p>".to_string(),
    };
    let template = sevk.templates().create(create_request).await.unwrap();

    let result = sevk.templates().get(&template.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, template.id);
}

#[tokio::test]
async fn test_templates_update() {
    let sevk = setup_test_environment().await;

    let title = format!("Update Template {}", unique_id());
    let create_request = CreateTemplateRequest {
        title: title.clone(),
        content: "<p>Test</p>".to_string(),
    };
    let template = sevk.templates().create(create_request).await.unwrap();

    let new_title = format!("Updated Template {}", unique_id());
    let update_request = UpdateTemplateRequest {
        title: Some(new_title.clone()),
        ..Default::default()
    };

    let result = sevk.templates().update(&template.id, update_request).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().title, new_title);
}

#[tokio::test]
async fn test_templates_duplicate() {
    let sevk = setup_test_environment().await;

    let title = format!("Duplicate Template {}", unique_id());
    let create_request = CreateTemplateRequest {
        title: title.clone(),
        content: "<p>Test</p>".to_string(),
    };
    let template = sevk.templates().create(create_request).await.unwrap();

    let result = sevk.templates().duplicate(&template.id).await;
    assert!(result.is_ok());

    let duplicated = result.unwrap();
    assert!(!duplicated.id.is_empty());
    assert_ne!(duplicated.id, template.id);
}

#[tokio::test]
async fn test_templates_delete() {
    let sevk = setup_test_environment().await;

    let title = format!("Delete Template {}", unique_id());
    let create_request = CreateTemplateRequest {
        title: title.clone(),
        content: "<p>Test</p>".to_string(),
    };
    let template = sevk.templates().create(create_request).await.unwrap();

    let result = sevk.templates().delete(&template.id).await;
    assert!(result.is_ok());

    let get_result = sevk.templates().get(&template.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// BROADCASTS TESTS
// ============================================

#[tokio::test]
async fn test_broadcasts_list_structure() {
    let sevk = setup_test_environment().await;

    let result = sevk.broadcasts().list(None).await;
    assert!(result.is_ok());

    let broadcasts = result.unwrap();
    assert!(broadcasts.total >= 0);
    assert!(broadcasts.page >= 1);
}

#[tokio::test]
async fn test_broadcasts_list_pagination() {
    let sevk = setup_test_environment().await;

    let params = ListParams {
        page: Some(1),
        limit: Some(10),
        search: None,
    };
    let result = sevk.broadcasts().list(Some(params)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().page, 1);
}

#[tokio::test]
async fn test_broadcasts_list_search() {
    let sevk = setup_test_environment().await;

    let params = ListParams {
        search: Some("test".to_string()),
        ..Default::default()
    };
    let result = sevk.broadcasts().list(Some(params)).await;
    assert!(result.is_ok());
}

// ============================================
// DOMAINS TESTS
// ============================================

#[tokio::test]
async fn test_domains_list_structure() {
    let sevk = setup_test_environment().await;

    let result = sevk.domains().list(None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_domains_list_verified() {
    let sevk = setup_test_environment().await;

    let params = DomainListParams {
        verified: Some(true),
    };
    let result = sevk.domains().list(Some(params)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    for domain in response.domains {
        assert!(domain.verified);
    }
}

// ============================================
// TOPICS TESTS
// ============================================

#[tokio::test]
async fn test_topics_list() {
    let sevk = setup_test_environment().await;

    // Create audience first
    let name = format!("Topics Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let result = sevk.topics().list(&audience.id, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().total >= 0);
}

#[tokio::test]
async fn test_topics_create() {
    let sevk = setup_test_environment().await;

    let name = format!("Topics Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let topic_name = format!("Test Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };

    let result = sevk.topics().create(&audience.id, create_topic_req).await;
    assert!(result.is_ok());

    let topic = result.unwrap();
    assert!(!topic.id.is_empty());
    assert_eq!(topic.name, topic_name);
    assert_eq!(topic.audience_id, audience.id);
}

#[tokio::test]
async fn test_topics_get_by_id() {
    let sevk = setup_test_environment().await;

    let name = format!("Topics Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let topic_name = format!("Get Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    let result = sevk.topics().get(&audience.id, &topic.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, topic.id);
}

#[tokio::test]
async fn test_topics_update() {
    let sevk = setup_test_environment().await;

    let name = format!("Topics Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let topic_name = format!("Update Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    let new_name = format!("Updated Topic {}", unique_id());
    let update_req = UpdateTopicRequest {
        name: Some(new_name.clone()),
        ..Default::default()
    };

    let result = sevk.topics().update(&audience.id, &topic.id, update_req).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, new_name);
}

#[tokio::test]
async fn test_topics_delete() {
    let sevk = setup_test_environment().await;

    let name = format!("Topics Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let topic_name = format!("Delete Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    let result = sevk.topics().delete(&audience.id, &topic.id).await;
    assert!(result.is_ok());

    let get_result = sevk.topics().get(&audience.id, &topic.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// SEGMENTS TESTS
// ============================================

#[tokio::test]
async fn test_segments_list() {
    let sevk = setup_test_environment().await;

    let name = format!("Segments Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let result = sevk.segments().list(&audience.id, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().total >= 0);
}

#[tokio::test]
async fn test_segments_create() {
    let sevk = setup_test_environment().await;

    let name = format!("Segments Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let segment_name = format!("Test Segment {}", unique_id());
    let create_segment_req = CreateSegmentRequest {
        name: segment_name.clone(),
        rules: vec![SegmentRule {
            field: "email".to_string(),
            operator: "contains".to_string(),
            value: "@example.com".to_string(),
        }],
        operator: "AND".to_string(),
    };

    let result = sevk.segments().create(&audience.id, create_segment_req).await;
    assert!(result.is_ok());

    let segment = result.unwrap();
    assert!(!segment.id.is_empty());
    assert_eq!(segment.name, segment_name);
    assert_eq!(segment.audience_id, audience.id);
    assert_eq!(segment.operator, "AND");
}

#[tokio::test]
async fn test_segments_get_by_id() {
    let sevk = setup_test_environment().await;

    let name = format!("Segments Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let segment_name = format!("Get Segment {}", unique_id());
    let create_segment_req = CreateSegmentRequest {
        name: segment_name.clone(),
        rules: vec![],
        operator: "AND".to_string(),
    };
    let segment = sevk.segments().create(&audience.id, create_segment_req).await.unwrap();

    let result = sevk.segments().get(&audience.id, &segment.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, segment.id);
}

#[tokio::test]
async fn test_segments_update() {
    let sevk = setup_test_environment().await;

    let name = format!("Segments Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let segment_name = format!("Update Segment {}", unique_id());
    let create_segment_req = CreateSegmentRequest {
        name: segment_name.clone(),
        rules: vec![],
        operator: "AND".to_string(),
    };
    let segment = sevk.segments().create(&audience.id, create_segment_req).await.unwrap();

    let new_name = format!("Updated Segment {}", unique_id());
    let update_req = UpdateSegmentRequest {
        name: Some(new_name.clone()),
        ..Default::default()
    };

    let result = sevk.segments().update(&audience.id, &segment.id, update_req).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, new_name);
}

#[tokio::test]
async fn test_segments_delete() {
    let sevk = setup_test_environment().await;

    let name = format!("Segments Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let segment_name = format!("Delete Segment {}", unique_id());
    let create_segment_req = CreateSegmentRequest {
        name: segment_name.clone(),
        rules: vec![],
        operator: "AND".to_string(),
    };
    let segment = sevk.segments().create(&audience.id, create_segment_req).await.unwrap();

    let result = sevk.segments().delete(&audience.id, &segment.id).await;
    assert!(result.is_ok());

    let get_result = sevk.segments().get(&audience.id, &segment.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// SUBSCRIPTIONS TESTS
// ============================================

#[tokio::test]
async fn test_subscriptions_subscribe() {
    let sevk = setup_test_environment().await;

    let name = format!("Subs Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    let email = format!("subscribe-test-{}@example.com", unique_id());
    let subscribe_req = SubscribeRequest {
        email: email.clone(),
        audience_id: audience.id.clone(),
        topic_ids: None,
    };

    let result = sevk.subscriptions().subscribe(subscribe_req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subscriptions_unsubscribe() {
    let sevk = setup_test_environment().await;

    // Create contact first
    let email = format!("unsubscribe-test-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        subscribed: Some(true),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();

    let unsubscribe_req = UnsubscribeRequest {
        email: email.clone(),
        audience_id: None,
    };

    let result = sevk.subscriptions().unsubscribe(unsubscribe_req).await;
    assert!(result.is_ok());

    // Verify unsubscription
    let updated_contact = sevk.contacts().get(&contact.id).await.unwrap();
    assert_eq!(updated_contact.subscribed, false);
}

// ============================================
// EMAILS TESTS
// ============================================

#[tokio::test]
async fn test_emails_reject_unverified_domain() {
    let sevk = setup_test_environment().await;

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "no-reply@unverified-domain.com".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
    };

    let result = sevk.emails().send(send_request).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        // Check for 403 or DomainNotVerified error
        assert!(error_str.contains("403") || error_str.contains("DomainNotVerified"));
        assert!(error_str.to_lowercase().contains("domain"));
    }
}

#[tokio::test]
async fn test_emails_reject_domain_not_owned() {
    let sevk = setup_test_environment().await;

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "no-reply@not-my-domain.io".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
    };

    let result = sevk.emails().send(send_request).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("403") || error_str.contains("DomainNotVerified"));
        assert!(error_str.to_lowercase().contains("domain"));
    }
}

#[tokio::test]
async fn test_emails_reject_invalid_from_address() {
    let sevk = setup_test_environment().await;

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "invalid-email-without-domain".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
    };

    let result = sevk.emails().send(send_request).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("400") || error_str.contains("Validation"));
    }
}

#[tokio::test]
async fn test_emails_error_message_domain_verification() {
    let sevk = setup_test_environment().await;

    let send_request = SendEmailRequest {
        to: "recipient@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "sender@random-unverified-domain.xyz".to_string(),
        html: Some("<p>Hello World</p>".to_string()),
        text: None,
        reply_to: None,
    };

    let result = sevk.emails().send(send_request).await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        let lower_message = error_str.to_lowercase();
        assert!(
            lower_message.contains("domain") ||
            lower_message.contains("verified") ||
            lower_message.contains("forbidden")
        );
    }
}

// ============================================
// ERROR HANDLING TESTS
// ============================================

#[tokio::test]
async fn test_error_404() {
    let sevk = setup_test_environment().await;

    let result = sevk.contacts().get("non-existent-id-12345").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

#[tokio::test]
async fn test_error_validation() {
    let sevk = setup_test_environment().await;

    let result = sevk.contacts().create("invalid-email", None).await;
    assert!(result.is_err());
}

// ============================================
// MARKUP RENDERER TESTS
// ============================================

#[test]
fn test_markup_html_document_structure() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("<!DOCTYPE html"));
    assert!(html.contains("<html"));
    assert!(html.contains("<head>"));
    assert!(html.contains("<body"));
    assert!(html.contains("</html>"));
}

#[test]
fn test_markup_meta_tags() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("charset=UTF-8"));
    assert!(html.contains("viewport"));
}

#[test]
fn test_markup_title_when_provided() {
    let markup = "<email><head><title>Test Email</title></head><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("<title>Test Email</title>"));
}

#[test]
fn test_markup_preview_text_when_provided() {
    let markup = "<email><head><preview>Preview text here</preview></head><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("Preview text here"));
    assert!(html.contains("display:none"));
}

#[test]
fn test_markup_custom_styles_when_provided() {
    let markup = "<email><head><style>.custom { color: red; }</style></head><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains(".custom { color: red; }"));
}

#[test]
fn test_markup_empty_with_document_structure() {
    let html = sevk::generate_email_from_markup("", None);
    assert!(html.contains("<!DOCTYPE html"));
    assert!(html.contains("<body"));
}

#[test]
fn test_markup_default_body_styles() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("margin:0"));
    assert!(html.contains("padding:0"));
    assert!(html.contains("font-family"));
}

#[test]
fn test_markup_html_lang_attribute() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("lang=\"en\""));
}

#[test]
fn test_markup_html_dir_attribute() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("dir=\"ltr\""));
}

#[test]
fn test_markup_content_type_meta_tag() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("Content-Type"));
    assert!(html.contains("text/html"));
}

#[test]
fn test_markup_xhtml_doctype() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("XHTML 1.0 Transitional"));
}

#[test]
fn test_markup_background_color_in_body_styles() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("background-color"));
}

#[test]
fn test_markup_mail_tag_same_as_email_tag() {
    let markup = "<mail><body></body></mail>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("<!DOCTYPE html"));
    assert!(html.contains("<body"));
}

#[test]
fn test_markup_complex_structure() {
    let markup = "<email>
        <head>
            <title>Complex Email</title>
            <preview>This is a preview</preview>
            <style>.test { color: blue; }</style>
        </head>
        <body></body>
    </email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("Complex Email"));
    assert!(html.contains("This is a preview"));
    assert!(html.contains(".test { color: blue; }"));
}

#[test]
fn test_markup_whitespace_handling() {
    let markup = "   <email>   <body>   </body>   </email>   ";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(html.contains("<!DOCTYPE html"));
    assert!(html.contains("<body"));
}

#[test]
fn test_markup_returns_string() {
    let markup = "<email><body></body></email>";
    let html = sevk::generate_email_from_markup(markup, None);

    assert!(!html.is_empty());
}
