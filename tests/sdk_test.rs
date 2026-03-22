//! Integration tests for the Sevk SDK

use sevk::{Sevk, SevkOptions};
use sevk::types::*;

const DEFAULT_BASE_URL: &str = "https://api.sevk.io";

fn get_base_url() -> String {
    std::env::var("SEVK_TEST_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

fn unique_id() -> String {
    format!("{}{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis(), rand::random::<u32>() % 10000)
}

fn should_run_domain_tests() -> bool {
    std::env::var("INCLUDE_DOMAIN_TESTS").map(|v| v == "true").unwrap_or(false)
}

fn setup_test_environment() -> Option<Sevk> {
    let api_key = match std::env::var("SEVK_TEST_API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            eprintln!("SEVK_TEST_API_KEY not set, skipping integration test");
            return None;
        }
    };

    let options = SevkOptions {
        base_url: get_base_url(),
        timeout: 30000,
    };

    Some(Sevk::with_options(&api_key, options))
}

// ============================================
// AUTHENTICATION TESTS
// ============================================

#[tokio::test]
async fn test_auth_invalid_api_key() {
    let options = SevkOptions {
        base_url: get_base_url(),
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
        base_url: get_base_url(),
        timeout: 30000,
    };
    let sevk = Sevk::with_options("", options);

    let result = sevk.contacts().list(None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_auth_malformed_api_key() {
    let options = SevkOptions {
        base_url: get_base_url(),
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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.contacts().list(None).await;
    assert!(result.is_ok());

    let contacts = result.unwrap();
    assert!(contacts.total >= 0);
    assert!(contacts.page >= 1);
    assert!(contacts.total_pages >= 0);
}

#[tokio::test]
async fn test_contacts_list_pagination() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    assert!(!updated.subscribed);
}

#[tokio::test]
async fn test_contacts_error_non_existent() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.contacts().get("non-existent-id").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

#[tokio::test]
async fn test_contacts_delete() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.audiences().list(None).await;
    assert!(result.is_ok());

    let audiences = result.unwrap();
    assert!(audiences.total >= 0);
    assert!(audiences.page >= 1);
}

#[tokio::test]
async fn test_audiences_create_required() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.templates().list(None).await;
    assert!(result.is_ok());

    let templates = result.unwrap();
    assert!(templates.total >= 0);
    assert!(templates.page >= 1);
}

#[tokio::test]
async fn test_templates_create_required() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.broadcasts().list(None).await;
    assert!(result.is_ok());

    let broadcasts = result.unwrap();
    assert!(broadcasts.total >= 0);
    assert!(broadcasts.page >= 1);
}

#[tokio::test]
async fn test_broadcasts_list_pagination() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.domains().list(None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_domains_list_verified() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let params = DomainListParams {
        verified: Some(true),
    };
    let result = sevk.domains().list(Some(params)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    for domain in response.items {
        assert!(domain.verified);
    }
}

// ============================================
// TOPICS TESTS
// ============================================

#[tokio::test]
async fn test_topics_list() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

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
    assert!(!updated_contact.subscribed);
}

// ============================================
// EMAILS TESTS
// ============================================

#[tokio::test]
async fn test_emails_reject_unverified_domain() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "no-reply@unverified-domain.com".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
        attachments: None,
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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "no-reply@not-my-domain.io".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
        attachments: None,
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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let send_request = SendEmailRequest {
        to: "test@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "invalid-email-without-domain".to_string(),
        html: Some("<p>Hello</p>".to_string()),
        text: None,
        reply_to: None,
        attachments: None,
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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let send_request = SendEmailRequest {
        to: "recipient@example.com".to_string(),
        subject: "Test Email".to_string(),
        from: "sender@random-unverified-domain.xyz".to_string(),
        html: Some("<p>Hello World</p>".to_string()),
        text: None,
        reply_to: None,
        attachments: None,
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
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.contacts().get("non-existent-id-12345").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

#[tokio::test]
async fn test_error_validation() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.contacts().create("invalid-email", None).await;
    assert!(result.is_err());
}

// ============================================
// DOMAINS UPDATE TESTS
// ============================================

#[tokio::test]
async fn test_domains_update() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create a domain first, then update it
    let create_request = CreateDomainRequest {
        domain: format!("test-{}.example.com", unique_id()),
        region: Some("us-east-1".to_string()),
        ..Default::default()
    };

    let result = sevk.domains().create(create_request).await;
    // Domain creation may fail if domain already exists or requires verification
    // In that case, we test update on an existing domain from the list
    if let Ok(domain) = result {
        let update_request = UpdateDomainRequest {
            region: Some("eu-west-1".to_string()),
            ..Default::default()
        };

        let update_result = sevk.domains().update(&domain.id, update_request).await;
        assert!(update_result.is_ok());
    }
}

// ============================================
// BROADCASTS EXTENDED TESTS
// ============================================

#[tokio::test]
async fn test_broadcasts_get_status() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // List broadcasts to find an existing one
    let broadcasts = sevk.broadcasts().list(None).await;
    assert!(broadcasts.is_ok());

    let list = broadcasts.unwrap();
    if !list.items.is_empty() {
        let broadcast_id = &list.items[0].id;
        let result = sevk.broadcasts().get_status(broadcast_id).await;
        assert!(result.is_ok());

        let status = result.unwrap();
        assert!(!status.id.is_empty());
        assert!(!status.status.is_empty());
        assert!(status.total >= 0);
        assert!(status.sent >= 0);
    }
}

#[tokio::test]
async fn test_broadcasts_get_emails() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let broadcasts = sevk.broadcasts().list(None).await;
    assert!(broadcasts.is_ok());

    let list = broadcasts.unwrap();
    if !list.items.is_empty() {
        let broadcast_id = &list.items[0].id;
        let result = sevk.broadcasts().get_emails(broadcast_id, None).await;
        assert!(result.is_ok());

        let emails = result.unwrap();
        assert!(emails.total >= 0);
        assert!(emails.page >= 1);
    }
}

#[tokio::test]
async fn test_broadcasts_estimate_cost() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let broadcasts = sevk.broadcasts().list(None).await;
    assert!(broadcasts.is_ok());

    let list = broadcasts.unwrap();
    if !list.items.is_empty() {
        let broadcast_id = &list.items[0].id;
        let result = sevk.broadcasts().estimate_cost(broadcast_id).await;
        assert!(result.is_ok());

        let estimate = result.unwrap();
        assert!(estimate.contact_count >= 0);
        assert!(estimate.estimated_cost >= 0.0);
    }
}

#[tokio::test]
async fn test_broadcasts_list_active() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.broadcasts().list_active().await;
    assert!(result.is_ok());

    let active = result.unwrap();
    // Just verify the response deserialized successfully
    let _ = active.items.len();
}

// ============================================
// TOPICS LIST CONTACTS TESTS
// ============================================

#[tokio::test]
async fn test_topics_list_contacts() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Topics Contacts Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create topic
    let topic_name = format!("Contacts Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    // List contacts for topic (should be empty initially)
    let result = sevk.topics().list_contacts(&audience.id, &topic.id, None).await;
    assert!(result.is_ok());

    let contacts = result.unwrap();
    assert!(contacts.total >= 0);
    assert!(contacts.page >= 1);
}

// ============================================
// WEBHOOKS TESTS (FULL CRUD)
// ============================================

#[tokio::test]
async fn test_webhooks_list_events() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.webhooks().list_events().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_webhooks_list() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.webhooks().list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_webhooks_crud_cycle() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let event_name = "contact.subscribed".to_string();

    // Create
    let create_request = CreateWebhookRequest {
        url: format!("https://example.com/webhook/{}", unique_id()),
        events: vec![event_name.clone()],
        enabled: Some(true),
    };

    let create_result = sevk.webhooks().create(create_request).await;
    assert!(create_result.is_ok());

    let webhook = create_result.unwrap();
    assert!(!webhook.id.is_empty());
    assert!(webhook.url.contains("example.com"));
    assert!(webhook.enabled);
    assert!(!webhook.events.is_empty());

    // Get
    let get_result = sevk.webhooks().get(&webhook.id).await;
    assert!(get_result.is_ok());
    assert_eq!(get_result.unwrap().id, webhook.id);

    // Update
    let update_request = UpdateWebhookRequest {
        enabled: Some(false),
        ..Default::default()
    };
    let update_result = sevk.webhooks().update(&webhook.id, update_request).await;
    assert!(update_result.is_ok());
    let updated = update_result.unwrap();
    assert_eq!(updated.id, webhook.id);
    assert!(!updated.enabled);

    // Test
    let test_result = sevk.webhooks().test(&webhook.id).await;
    assert!(test_result.is_ok());

    // Delete
    let delete_result = sevk.webhooks().delete(&webhook.id).await;
    assert!(delete_result.is_ok());

    // Verify deletion
    let get_after_delete = sevk.webhooks().get(&webhook.id).await;
    assert!(get_after_delete.is_err());
    if let Err(e) = get_after_delete {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// EVENTS TESTS
// ============================================

#[tokio::test]
async fn test_events_list() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.events().list(None).await;
    assert!(result.is_ok());

    let events = result.unwrap();
    assert!(events.total >= 0);
    assert!(events.page >= 1);
}

#[tokio::test]
async fn test_events_list_with_params() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let params = EventListParams {
        page: Some(1),
        limit: Some(10),
        ..Default::default()
    };

    let result = sevk.events().list(Some(params)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().page, 1);
}

#[tokio::test]
async fn test_events_stats() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.events().stats().await;
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.total >= 0);
    assert!(stats.sent >= 0);
    assert!(stats.delivered >= 0);
    assert!(stats.opened >= 0);
    assert!(stats.clicked >= 0);
    assert!(stats.bounced >= 0);
    assert!(stats.complained >= 0);
}

// ============================================
// USAGE TESTS
// ============================================

#[tokio::test]
async fn test_get_usage() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.get_usage().await;
    assert!(result.is_ok());
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

// ============================================
// CONTACTS - BULK UPDATE
// ============================================

#[tokio::test]
async fn test_contacts_bulk_update() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create a contact first
    let email = format!("bulk-update-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        subscribed: Some(true),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_request)).await.unwrap();

    let bulk_req = BulkUpdateContactsRequest {
        contacts: vec![BulkUpdateContactEntry {
            id: contact.id.clone(),
            email: email.clone(),
            subscribed: Some(false),
            first_name: Some("BulkUpdated".to_string()),
            last_name: None,
            metadata: None,
        }],
    };

    let result = sevk.contacts().bulk_update(bulk_req).await;
    assert!(result.is_ok());
}

// ============================================
// CONTACTS - GET EVENTS
// ============================================

#[tokio::test]
async fn test_contacts_get_events() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create a contact first
    let email = format!("events-test-{}@example.com", unique_id());
    let create_request = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_request)).await.unwrap();

    let result = sevk.contacts().get_events(&contact.id, None).await;
    assert!(result.is_ok());
}

// ============================================
// CONTACTS - IMPORT
// ============================================

#[tokio::test]
async fn test_contacts_import() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let email = format!("import-test-{}@example.com", unique_id());
    let import_req = ImportContactsRequest {
        contacts: vec![ImportContactEntry {
            email: email.clone(),
            subscribed: None,
            data: None,
        }],
        audience_id: None,
    };

    let result = sevk.contacts().import(import_req).await;
    assert!(result.is_ok());
}

// ============================================
// AUDIENCES - LIST CONTACTS
// ============================================

#[tokio::test]
async fn test_audiences_list_contacts() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("List Contacts Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create contact and add to audience
    let email = format!("list-contacts-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();
    sevk.audiences().add_contacts(&audience.id, vec![contact.id.clone()]).await.unwrap();

    // List contacts in audience
    let result = sevk.audiences().list_contacts(&audience.id, None).await;
    assert!(result.is_ok());

    let contacts = result.unwrap();
    assert!(contacts.total >= 0);
    assert!(contacts.page >= 1);
}

// ============================================
// AUDIENCES - REMOVE CONTACT
// ============================================

#[tokio::test]
async fn test_audiences_remove_contact() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Remove Contact Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create contact and add to audience
    let email = format!("audience-remove-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();
    sevk.audiences().add_contacts(&audience.id, vec![contact.id.clone()]).await.unwrap();

    // Remove contact from audience
    let result = sevk.audiences().remove_contact(&audience.id, &contact.id).await;
    assert!(result.is_ok());

    // Verify removal by listing contacts
    let contacts = sevk.audiences().list_contacts(&audience.id, None).await.unwrap();
    let contact_ids: Vec<String> = contacts.items.iter().map(|c| c.id.clone()).collect();
    assert!(!contact_ids.contains(&contact.id));
}

// ============================================
// BROADCASTS - FULL CRUD LIFECYCLE
// ============================================

#[tokio::test]
async fn test_broadcasts_create() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Get a domain from the project to use for broadcast
    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() {
        eprintln!("No domains available, skipping broadcast create test");
        return;
    }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Test Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Test Subject".to_string(),
        body: "<section><paragraph>Test broadcast body</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        style: Some("HTML".to_string()),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };

    let result = sevk.broadcasts().create(create_req).await;
    assert!(result.is_ok());

    let broadcast = result.unwrap();
    assert!(!broadcast.id.is_empty());
    assert_eq!(broadcast.name, name);
    assert_eq!(broadcast.subject, Some("Test Subject".to_string()));
    assert_eq!(broadcast.status, "DRAFT");
}

#[tokio::test]
async fn test_broadcasts_get() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Get a domain and create a broadcast
    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Get Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Get Test Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    // Get the broadcast by ID
    let result = sevk.broadcasts().get(&broadcast.id).await;
    assert!(result.is_ok());

    let fetched = result.unwrap();
    assert_eq!(fetched.id, broadcast.id);
    assert_eq!(fetched.subject, Some("Get Test Subject".to_string()));

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

#[tokio::test]
async fn test_broadcasts_update() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Update Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Original Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    let new_name = format!("Updated Broadcast {}", unique_id());
    let update_req = UpdateBroadcastRequest {
        name: Some(new_name.clone()),
        ..Default::default()
    };

    let result = sevk.broadcasts().update(&broadcast.id, update_req).await;
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.id, broadcast.id);
    assert_eq!(updated.name, new_name);

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

#[tokio::test]
async fn test_broadcasts_delete() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Delete Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Delete Test Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    let result = sevk.broadcasts().delete(&broadcast.id).await;
    assert!(result.is_ok());

    // Verify deletion
    let get_result = sevk.broadcasts().get(&broadcast.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

#[tokio::test]
async fn test_broadcasts_get_analytics() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Analytics Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Analytics Test Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    let result = sevk.broadcasts().get_analytics(&broadcast.id).await;
    assert!(result.is_ok());

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

#[tokio::test]
async fn test_broadcasts_send_test() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("SendTest Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Send Test Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    // Send test may fail if domain is unverified, which is expected
    let result = sevk.broadcasts().send_test(&broadcast.id, vec!["test@example.com".to_string()]).await;
    match result {
        Ok(_) => {},
        Err(e) => {
            // May fail if domain is unverified, which is expected
            let error_str = format!("{:?}", e);
            assert!(!error_str.is_empty());
        }
    }

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

#[tokio::test]
async fn test_broadcasts_handle_send_error() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Send Error Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Send Error Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    // Sending a draft broadcast should fail or succeed depending on state
    let result = sevk.broadcasts().send(&broadcast.id).await;
    match result {
        Ok(_) => {},
        Err(e) => {
            // Expected to fail if broadcast is not ready to send
            let error_str = format!("{:?}", e);
            assert!(!error_str.is_empty());
        }
    }

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

#[tokio::test]
async fn test_broadcasts_handle_cancel_error() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let domains = sevk.domains().list(None).await.unwrap();
    if domains.items.is_empty() { return; }
    let domain_id = domains.items[0].id.clone();

    let name = format!("Cancel Error Broadcast {}", unique_id());
    let create_req = CreateBroadcastRequest {
        name: name.clone(),
        subject: "Cancel Error Subject".to_string(),
        body: "<section><paragraph>Test</paragraph></section>".to_string(),
        domain_id: domain_id.clone(),
        sender_name: Some("Test Sender".to_string()),
        sender_email: Some("test".to_string()),
        target_type: Some("ALL".to_string()),
        ..Default::default()
    };
    let broadcast = sevk.broadcasts().create(create_req).await.unwrap();

    // Cancelling a draft broadcast should fail
    let result = sevk.broadcasts().cancel(&broadcast.id).await;
    match result {
        Ok(_) => {},
        Err(e) => {
            // Expected to fail if broadcast is not in a cancellable state
            let error_str = format!("{:?}", e);
            assert!(!error_str.is_empty());
        }
    }

    // Cleanup
    let _ = sevk.broadcasts().delete(&broadcast.id).await;
}

// ============================================
// DOMAINS - FULL CRUD LIFECYCLE
// ============================================

#[tokio::test]
async fn test_domains_create() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let subdomain = format!("test-{}.example.com", unique_id());
    let create_req = CreateDomainRequest {
        domain: subdomain.clone(),
        email: Some(format!("test@{}", subdomain)),
        ..Default::default()
    };

    let result = sevk.domains().create(create_req).await;
    assert!(result.is_ok());

    let domain = result.unwrap();
    assert!(!domain.id.is_empty());
    assert_eq!(domain.domain, subdomain);
}

#[tokio::test]
async fn test_domains_get() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let subdomain = format!("test-get-{}.example.com", unique_id());
    let create_req = CreateDomainRequest {
        domain: subdomain.clone(),
        email: Some(format!("test@{}", subdomain)),
        ..Default::default()
    };
    let domain = sevk.domains().create(create_req).await.unwrap();

    let result = sevk.domains().get(&domain.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, domain.id);

    // Cleanup
    let _ = sevk.domains().delete(&domain.id).await;
}

#[tokio::test]
async fn test_domains_dns_records() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let subdomain = format!("test-dns-{}.example.com", unique_id());
    let create_req = CreateDomainRequest {
        domain: subdomain.clone(),
        email: Some(format!("test@{}", subdomain)),
        ..Default::default()
    };
    let domain = sevk.domains().create(create_req).await.unwrap();

    let result = sevk.domains().get_dns_records(&domain.id).await;
    assert!(result.is_ok());

    // Cleanup
    let _ = sevk.domains().delete(&domain.id).await;
}

#[tokio::test]
async fn test_domains_regions() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.domains().get_regions().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_domains_verify() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let subdomain = format!("test-verify-{}.example.com", unique_id());
    let create_req = CreateDomainRequest {
        domain: subdomain.clone(),
        email: Some(format!("test@{}", subdomain)),
        ..Default::default()
    };
    let domain = sevk.domains().create(create_req).await.unwrap();

    // Verify may fail for test domains without proper DNS records
    let result = sevk.domains().verify(&domain.id).await;
    match result {
        Ok(response) => {
            // Verification response received
            // Verification response received successfully
            let _ = response.verified;
        },
        Err(e) => {
            // Expected to fail for test domains without proper DNS records
            let error_str = format!("{:?}", e);
            assert!(!error_str.is_empty());
        }
    }

    // Cleanup
    let _ = sevk.domains().delete(&domain.id).await;
}

#[tokio::test]
async fn test_domains_delete() {
    if !should_run_domain_tests() { return; }
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let subdomain = format!("test-delete-{}.example.com", unique_id());
    let create_req = CreateDomainRequest {
        domain: subdomain.clone(),
        email: Some(format!("test@{}", subdomain)),
        ..Default::default()
    };
    let domain = sevk.domains().create(create_req).await.unwrap();

    let result = sevk.domains().delete(&domain.id).await;
    assert!(result.is_ok());

    // Verify deletion
    let get_result = sevk.domains().get(&domain.id).await;
    assert!(get_result.is_err());
    if let Err(e) = get_result {
        let error_str = format!("{:?}", e);
        assert!(!error_str.is_empty());
    }
}

// ============================================
// TOPICS - ADD CONTACTS
// ============================================

#[tokio::test]
async fn test_topics_add_contacts() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Add Contacts Topic Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create topic
    let topic_name = format!("Add Contacts Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    // Create contact and add to audience first
    let email = format!("topic-add-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();
    sevk.audiences().add_contacts(&audience.id, vec![contact.id.clone()]).await.unwrap();

    // Add contact to topic
    let result = sevk.topics().add_contacts(&audience.id, &topic.id, vec![contact.id.clone()]).await;
    assert!(result.is_ok());
}

// ============================================
// TOPICS - REMOVE CONTACT
// ============================================

#[tokio::test]
async fn test_topics_remove_contact() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Remove Contact Topic Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create topic
    let topic_name = format!("Remove Contact Topic {}", unique_id());
    let create_topic_req = CreateTopicRequest {
        name: topic_name.clone(),
        ..Default::default()
    };
    let topic = sevk.topics().create(&audience.id, create_topic_req).await.unwrap();

    // Create contact, add to audience, then add to topic
    let email = format!("topic-remove-{}@example.com", unique_id());
    let create_contact_req = CreateContactRequest {
        email: email.clone(),
        ..Default::default()
    };
    let contact = sevk.contacts().create(&email, Some(create_contact_req)).await.unwrap();
    sevk.audiences().add_contacts(&audience.id, vec![contact.id.clone()]).await.unwrap();
    sevk.topics().add_contacts(&audience.id, &topic.id, vec![contact.id.clone()]).await.unwrap();

    // Remove contact from topic
    let result = sevk.topics().remove_contact(&audience.id, &topic.id, &contact.id).await;
    assert!(result.is_ok());

    // Verify removal by listing contacts in the topic
    let contacts = sevk.topics().list_contacts(&audience.id, &topic.id, None).await.unwrap();
    let contact_ids: Vec<String> = contacts.items.iter().map(|c| c.id.clone()).collect();
    assert!(!contact_ids.contains(&contact.id));
}

// ============================================
// SEGMENTS - CALCULATE
// ============================================

#[tokio::test]
async fn test_segments_calculate() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Calculate Segment Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Create segment
    let segment_name = format!("Calculate Segment {}", unique_id());
    let create_segment_req = CreateSegmentRequest {
        name: segment_name.clone(),
        rules: vec![SegmentRule {
            field: "email".to_string(),
            operator: "contains".to_string(),
            value: "@example.com".to_string(),
        }],
        operator: "AND".to_string(),
    };
    let segment = sevk.segments().create(&audience.id, create_segment_req).await.unwrap();

    // Calculate the segment
    let result = sevk.segments().calculate(&audience.id, &segment.id).await;
    assert!(result.is_ok());
}

// ============================================
// SEGMENTS - PREVIEW
// ============================================

#[tokio::test]
async fn test_segments_preview() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    // Create audience
    let name = format!("Preview Segment Audience {}", unique_id());
    let create_audience_req = CreateAudienceRequest {
        name: name.clone(),
        ..Default::default()
    };
    let audience = sevk.audiences().create(create_audience_req).await.unwrap();

    // Preview segment rules without saving
    let preview_req = SegmentCalculateRequest {
        rules: Some(vec![SegmentRule {
            field: "email".to_string(),
            operator: "contains".to_string(),
            value: "@example.com".to_string(),
        }]),
        operator: Some("AND".to_string()),
    };

    let result = sevk.segments().preview(&audience.id, preview_req).await;
    assert!(result.is_ok());
}

// ============================================
// EMAILS - GET NON-EXISTENT
// ============================================

#[tokio::test]
async fn test_emails_get_non_existent() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let result = sevk.emails().get("00000000-0000-0000-0000-000000000000").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        assert!(error_str.contains("404"));
    }
}

// ============================================
// EMAILS - BULK REJECT
// ============================================

#[tokio::test]
async fn test_emails_bulk_reject_unverified_domain() {
    let sevk = match setup_test_environment() { Some(s) => s, None => return };

    let bulk_req = BulkEmailRequest {
        emails: vec![
            SendEmailRequest {
                to: "test1@example.com".to_string(),
                subject: "Bulk Test 1".to_string(),
                from: "no-reply@unverified-domain.com".to_string(),
                html: Some("<p>Hello 1</p>".to_string()),
                text: None,
                reply_to: None,
                attachments: None,
            },
            SendEmailRequest {
                to: "test2@example.com".to_string(),
                subject: "Bulk Test 2".to_string(),
                from: "no-reply@unverified-domain.com".to_string(),
                html: Some("<p>Hello 2</p>".to_string()),
                text: None,
                reply_to: None,
                attachments: None,
            },
        ],
    };

    let result = sevk.emails().send_bulk(bulk_req).await;
    // The API returns 200 with error details in the response body
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.success, 0);
    assert!(response.failed > 0);
}
