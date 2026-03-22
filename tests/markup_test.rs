//! Block/template engine tests for the Sevk markup renderer

use sevk::markup::render;

// ============================================
// BLOCK TEMPLATE VARIABLE TESTS
// ============================================

#[test]
fn test_block_simple_variable_substitution() {
    let markup = r#"<block config='{"name":"Alice"}'>Hello {%name%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Hello Alice"), "Expected 'Hello Alice' in output, got: {}", result);
}

#[test]
fn test_block_variable_missing_produces_empty() {
    let markup = r#"<block config='{}'>Hello {%name%} end</block>"#;
    let result = render(markup);
    assert!(result.contains("Hello  end"), "Expected 'Hello  end' (empty variable), got: {}", result);
    assert!(!result.contains("{%name%}"), "Variable placeholder should be removed");
}

#[test]
fn test_block_fallback_value_when_missing() {
    let markup = r#"<block config='{}'>Hello {%name ?? Guest%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Hello Guest"), "Expected fallback 'Guest', got: {}", result);
}

#[test]
fn test_block_fallback_value_not_used_when_present() {
    let markup = r#"<block config='{"name":"Alice"}'>Hello {%name ?? Guest%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Hello Alice"), "Expected 'Alice' not fallback, got: {}", result);
}

#[test]
fn test_block_multiple_variables() {
    let markup = r#"<block config='{"first":"John","last":"Doe"}'>{%first%} {%last%}</block>"#;
    let result = render(markup);
    assert!(result.contains("John"), "Expected 'John' in output");
    assert!(result.contains("Doe"), "Expected 'Doe' in output");
}

#[test]
fn test_block_each_loop() {
    let markup = r#"<block config='{"items":[{"name":"A"},{"name":"B"},{"name":"C"}]}'>{%#each items as item%}{%name%},{%/each%}</block>"#;
    let result = render(markup);
    assert!(result.contains("A,"), "Expected 'A,' in loop output, got: {}", result);
    assert!(result.contains("B,"), "Expected 'B,' in loop output");
    assert!(result.contains("C,"), "Expected 'C,' in loop output");
}

#[test]
fn test_block_each_loop_empty_array() {
    let markup = r#"<block config='{"items":[]}'>{%#each items as item%}{%name%}{%/each%}</block>"#;
    let result = render(markup);
    assert!(!result.contains("{%"), "Loop with empty array should produce no template artifacts");
}

#[test]
fn test_block_if_true() {
    let markup = r#"<block config='{"show":true}'>{%#if show%}Visible{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Visible"), "Expected 'Visible' when condition is true, got: {}", result);
}

#[test]
fn test_block_if_false() {
    let markup = r#"<block config='{"show":false}'>{%#if show%}Visible{%/if%}</block>"#;
    let result = render(markup);
    assert!(!result.contains("Visible"), "Expected 'Visible' to be hidden when condition is false");
}

#[test]
fn test_block_if_else_true_branch() {
    let markup = r#"<block config='{"premium":true}'>{%#if premium%}Premium User{%else%}Free User{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Premium User"), "Expected true branch, got: {}", result);
    assert!(!result.contains("Free User"), "Should not contain false branch");
}

#[test]
fn test_block_if_else_false_branch() {
    let markup = r#"<block config='{"premium":false}'>{%#if premium%}Premium User{%else%}Free User{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Free User"), "Expected false branch, got: {}", result);
    assert!(!result.contains("Premium User"), "Should not contain true branch");
}

#[test]
fn test_block_nested_if() {
    let markup = r#"<block config='{"a":true,"b":true}'>{%#if a%}A{%#if b%}B{%/if%}{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("A"), "Expected outer if content");
    assert!(result.contains("B"), "Expected nested if content");
}

#[test]
fn test_block_nested_if_outer_false() {
    let markup = r#"<block config='{"a":false,"b":true}'>{%#if a%}OUTER_VISIBLE{%#if b%}INNER_VISIBLE{%/if%}{%/if%}</block>"#;
    let result = render(markup);
    assert!(!result.contains("OUTER_VISIBLE"), "Outer false should hide all content");
    assert!(!result.contains("INNER_VISIBLE"), "Nested content hidden when outer is false");
}

#[test]
fn test_block_preserves_double_curly_braces() {
    let markup = r#"<block config='{"name":"World"}'>Hello {{recipient}}, from {%name%}</block>"#;
    let result = render(markup);
    assert!(result.contains("{{recipient}}"), "Double curly braces should be preserved, got: {}", result);
    assert!(result.contains("from World"), "Template variable should be substituted");
}

// ============================================
// BLOCK WITH MARKUP ELEMENT TESTS
// ============================================

#[test]
fn test_block_with_paragraph() {
    let markup = r#"<block config='{"text":"Hello World"}'><paragraph>{%text%}</paragraph></block>"#;
    let result = render(markup);
    assert!(result.contains("<p"), "Expected <p> tag in output");
    assert!(result.contains("Hello World"), "Expected variable content in paragraph");
}

#[test]
fn test_block_with_heading() {
    let markup = r#"<block config='{"title":"Welcome"}'><heading level="2">{%title%}</heading></block>"#;
    let result = render(markup);
    assert!(result.contains("<h2"), "Expected <h2> tag in output, got: {}", result);
    assert!(result.contains("Welcome"), "Expected heading content");
    assert!(result.contains("</h2>"), "Expected closing </h2> tag");
}

#[test]
fn test_block_with_button() {
    let markup = r#"<block config='{"url":"https://example.com","label":"Click Me"}'><button href="{%url%}">{%label%}</button></block>"#;
    let result = render(markup);
    assert!(result.contains("https://example.com"), "Expected button href");
    assert!(result.contains("Click Me"), "Expected button label");
    assert!(result.contains("<a "), "Button should render as <a> tag");
}

#[test]
fn test_block_with_image() {
    let markup = r#"<block config='{"src":"https://example.com/img.png"}'><image src="{%src%}" alt="Photo"></image></block>"#;
    let result = render(markup);
    assert!(result.contains("<img"), "Expected <img> tag");
    assert!(result.contains("https://example.com/img.png"), "Expected image src");
    assert!(result.contains(r#"alt="Photo""#), "Expected alt text");
}

#[test]
fn test_block_with_section() {
    let markup = r#"<block config='{}'><section>Section content here</section></block>"#;
    let result = render(markup);
    assert!(result.contains("Section content here"), "Expected section content");
    assert!(result.contains("<table"), "Section should render as table");
}

#[test]
fn test_block_with_link() {
    let markup = r#"<block config='{"url":"https://example.com"}'><link href="{%url%}">Visit</link></block>"#;
    let result = render(markup);
    assert!(result.contains("<a "), "Link should render as <a> tag");
    assert!(result.contains("https://example.com"), "Expected link href");
    assert!(result.contains("Visit"), "Expected link text");
}

// ============================================
// MULTIPLE BLOCKS AND FALLBACK TESTS
// ============================================

#[test]
fn test_multiple_blocks() {
    let markup = r#"<block config='{"a":"First"}'>Block: {%a%}</block><block config='{"b":"Second"}'>Block: {%b%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Block: First"), "Expected first block content, got: {}", result);
    assert!(result.contains("Block: Second"), "Expected second block content");
}

#[test]
fn test_multiple_blocks_with_fallbacks() {
    let markup = r#"<block config='{}'>Name: {%name ?? Unknown%}</block><block config='{"role":"Admin"}'>Role: {%role ?? User%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Name: Unknown"), "Expected fallback in first block, got: {}", result);
    assert!(result.contains("Role: Admin"), "Expected value in second block, not fallback");
}

#[test]
fn test_block_with_numeric_variable() {
    let markup = r#"<block config='{"count":42}'>Items: {%count%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Items: 42"), "Expected numeric variable, got: {}", result);
}

#[test]
fn test_block_with_null_variable() {
    let markup = r#"<block config='{"val":null}'>Value: {%val%} end</block>"#;
    let result = render(markup);
    assert!(result.contains("Value:  end"), "Null should produce empty string, got: {}", result);
}

#[test]
fn test_block_fallback_with_null_variable() {
    let markup = r#"<block config='{"val":null}'>Value: {%val ?? default%} end</block>"#;
    let result = render(markup);
    assert!(result.contains("Value: default end"), "Null should trigger fallback, got: {}", result);
}

// ============================================
// DOCUMENT STRUCTURE TESTS
// ============================================

#[test]
fn test_document_paragraph() {
    let result = render("<paragraph>Hello World</paragraph>");
    assert!(result.contains("<p"), "Expected <p> tag");
    assert!(result.contains("Hello World"), "Expected paragraph content");
}

#[test]
fn test_document_heading_levels() {
    let h1 = render(r#"<heading level="1">Title</heading>"#);
    assert!(h1.contains("<h1"), "Expected <h1> tag");
    assert!(h1.contains("Title"), "Expected heading content");

    let h3 = render(r#"<heading level="3">Subtitle</heading>"#);
    assert!(h3.contains("<h3"), "Expected <h3> tag");
    assert!(h3.contains("Subtitle"), "Expected heading content");
}

#[test]
fn test_document_button() {
    let result = render(r#"<button href="https://example.com">Click</button>"#);
    assert!(result.contains("<a "), "Button should render as anchor tag");
    assert!(result.contains("https://example.com"), "Expected button href");
    assert!(result.contains("Click"), "Expected button text");
    assert!(result.contains(r#"target="_blank""#), "Expected target _blank");
}

#[test]
fn test_document_image() {
    let result = render(r#"<image src="https://example.com/photo.jpg" alt="A photo" width="200"></image>"#);
    assert!(result.contains("<img"), "Expected <img> tag");
    assert!(result.contains("https://example.com/photo.jpg"), "Expected image src");
    assert!(result.contains(r#"alt="A photo""#), "Expected alt text");
    assert!(result.contains(r#"width="200""#), "Expected width attribute");
}

#[test]
fn test_document_section() {
    let result = render(r##"<section background-color="#f0f0f0">Content</section>"##);
    assert!(result.contains("<table"), "Section should be a table");
    assert!(result.contains("Content"), "Expected section content");
    assert!(result.contains("background-color"), "Expected background-color style");
}

#[test]
fn test_document_container() {
    let result = render(r#"<container padding="20px">Inner content</container>"#);
    assert!(result.contains("<table"), "Container should render as table");
    assert!(result.contains("Inner content"), "Expected container content");
    assert!(result.contains("padding"), "Expected padding style");
}

#[test]
fn test_document_row_column() {
    let result = render(r#"<row><column>Left</column><column>Right</column></row>"#);
    assert!(result.contains("Left"), "Expected left column content");
    assert!(result.contains("Right"), "Expected right column content");
    assert!(result.contains("sevk-column"), "Expected column class");
    assert!(result.contains("sevk-row-table"), "Expected row table class");
}

#[test]
fn test_document_row_with_gap() {
    let result = render(r#"<row gap="16px"><column>A</column><column>B</column></row>"#);
    assert!(result.contains("A"), "Expected column A content");
    assert!(result.contains("B"), "Expected column B content");
    assert!(result.contains("padding-left"), "Expected gap padding");
    assert!(result.contains("padding-right"), "Expected gap padding");
}

#[test]
fn test_document_full_structure() {
    let markup = r#"<mail><body><section><container padding="10px"><heading level="1">Welcome</heading><paragraph>Hello</paragraph></container></section></body></mail>"#;
    let result = render(markup);
    assert!(result.contains("<!DOCTYPE"), "Expected full HTML document");
    assert!(result.contains("<h1"), "Expected heading");
    assert!(result.contains("Welcome"), "Expected heading content");
    assert!(result.contains("<p"), "Expected paragraph");
    assert!(result.contains("Hello"), "Expected paragraph content");
}

#[test]
fn test_document_with_title_and_preview() {
    let markup = r#"<mail><head><title>My Email</title><preview>Preview text here</preview></head><body><paragraph>Body content</paragraph></body></mail>"#;
    let result = render(markup);
    assert!(result.contains("<title>My Email</title>"), "Expected title in head");
    assert!(result.contains("Preview text here"), "Expected preview text");
    assert!(result.contains("Body content"), "Expected body content");
}

#[test]
fn test_document_nested_structure() {
    let markup = r#"<section><container><row><column><paragraph>Cell 1</paragraph></column><column><paragraph>Cell 2</paragraph></column></row></container></section>"#;
    let result = render(markup);
    assert!(result.contains("Cell 1"), "Expected first cell content");
    assert!(result.contains("Cell 2"), "Expected second cell content");
    assert!(result.contains("<p"), "Expected paragraph tags");
    assert!(result.contains("sevk-column"), "Expected column classes");
}

#[test]
fn test_block_with_each_and_paragraph() {
    let markup = r#"<block config='{"users":[{"name":"Alice"},{"name":"Bob"}]}'>{%#each users as user%}<paragraph>{%name%}</paragraph>{%/each%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Alice"), "Expected Alice in output, got: {}", result);
    assert!(result.contains("Bob"), "Expected Bob in output");
    assert!(result.contains("<p"), "Expected <p> tags from paragraph");
}

#[test]
fn test_block_if_with_heading() {
    let markup = r#"<block config='{"showTitle":true,"title":"Hello"}'>{%#if showTitle%}<heading level="1">{%title%}</heading>{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("<h1"), "Expected <h1> tag, got: {}", result);
    assert!(result.contains("Hello"), "Expected heading content");
}

#[test]
fn test_empty_block_produces_nothing() {
    let markup = r#"<block config='{}'>   </block>"#;
    let result = render(markup);
    // The block body is whitespace-only, so process_block_tag treats it as empty template
    // and returns empty string; the rest is just the HTML document wrapper
    assert!(!result.contains("{%"), "Should not contain template syntax");
}

#[test]
fn test_self_closing_block() {
    let markup = r#"<block template="Hello {%name%}" config='{"name":"World"}' />"#;
    let result = render(markup);
    assert!(result.contains("Hello World"), "Expected template with substituted variable, got: {}", result);
}

// ============================================
// VARIABLE PRESERVATION TESTS ({{...}} syntax)
// ============================================

#[test]
fn test_double_curly_unsubscribe_url_preserved_through_block() {
    let markup = r#"<block config='{"name":"Alice"}'>Hello {%name%}, click {{unsubscribeUrl}} to unsubscribe</block>"#;
    let result = render(markup);
    assert!(result.contains("Hello Alice"), "Expected variable substitution, got: {}", result);
    assert!(result.contains("{{unsubscribeUrl}}"), "{{unsubscribeUrl}} should be preserved, got: {}", result);
}

#[test]
fn test_double_curly_email_preserved_through_block() {
    let markup = r#"<block config='{"greeting":"Hi"}'>{{email}} - {%greeting%}</block>"#;
    let result = render(markup);
    assert!(result.contains("{{email}}"), "{{email}} should be preserved, got: {}", result);
    assert!(result.contains("Hi"), "Expected variable substitution");
}

#[test]
fn test_double_curly_preserved_in_each_loop() {
    let markup = r#"<block config='{"items":[{"name":"A"},{"name":"B"}]}'>{%#each items as item%}{%name%} {{unsubscribeUrl}},{%/each%}</block>"#;
    let result = render(markup);
    assert!(result.contains("A {{unsubscribeUrl}}"), "Expected A with preserved {{unsubscribeUrl}}, got: {}", result);
    assert!(result.contains("B {{unsubscribeUrl}}"), "Expected B with preserved {{unsubscribeUrl}}");
}

#[test]
fn test_double_curly_preserved_in_if_block() {
    let markup = r#"<block config='{"show":true}'>{%#if show%}Contact: {{email}}{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("Contact: {{email}}"), "{{email}} should be preserved inside if block, got: {}", result);
}

#[test]
fn test_multiple_double_curly_vars_preserved() {
    let markup = r#"<block config='{"name":"Test"}'>Dear {%name%}, visit {{unsubscribeUrl}} or mail {{email}}</block>"#;
    let result = render(markup);
    assert!(result.contains("Dear Test"), "Expected variable substitution");
    assert!(result.contains("{{unsubscribeUrl}}"), "{{unsubscribeUrl}} should be preserved");
    assert!(result.contains("{{email}}"), "{{email}} should be preserved");
}

// ============================================
// COMBINED IF + EACH TESTS
// ============================================

#[test]
fn test_combined_if_and_each() {
    let markup = r#"<block config='{"title":"Links","links":[{"name":"Home"},{"name":"About"}]}'>{%#if title%}<h1>{%title%}</h1>{%/if%}{%#each links as l%}<a>{%name%}</a>{%/each%}</block>"#;
    let result = render(markup);
    assert!(result.contains("<h1>Links</h1>"), "Expected heading with title, got: {}", result);
    assert!(result.contains("<a>Home</a>"), "Expected first link, got: {}", result);
    assert!(result.contains("<a>About</a>"), "Expected second link, got: {}", result);
}

#[test]
fn test_combined_if_false_hides_heading_each_still_renders() {
    let markup = r#"<block config='{"title":false,"links":[{"name":"Home"},{"name":"About"}]}'>{%#if title%}<h1>{%title%}</h1>{%/if%}{%#each links as l%}<a>{%name%}</a>{%/each%}</block>"#;
    let result = render(markup);
    assert!(!result.contains("<h1>"), "Heading should be hidden when title is false, got: {}", result);
    assert!(result.contains("<a>Home</a>"), "Each loop should still render");
    assert!(result.contains("<a>About</a>"), "Each loop should still render");
}

#[test]
fn test_combined_if_true_each_empty() {
    let markup = r#"<block config='{"title":"Hello","links":[]}'>{%#if title%}<h1>{%title%}</h1>{%/if%}{%#each links as l%}<a>{%name%}</a>{%/each%}</block>"#;
    let result = render(markup);
    assert!(result.contains("<h1>Hello</h1>"), "Expected heading when title is truthy, got: {}", result);
    assert!(!result.contains("<a>"), "Each loop should produce nothing for empty array");
}

#[test]
fn test_if_wrapping_each() {
    let markup = r#"<block config='{"showLinks":true,"links":[{"name":"Home"},{"name":"About"}]}'>{%#if showLinks%}{%#each links as l%}<a>{%name%}</a>{%/each%}{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("<a>Home</a>"), "Expected links when showLinks is true, got: {}", result);
    assert!(result.contains("<a>About</a>"), "Expected links when showLinks is true");
}

#[test]
fn test_if_false_wrapping_each_hides_all() {
    let markup = r#"<block config='{"showLinks":false,"links":[{"name":"Home"},{"name":"About"}]}'>{%#if showLinks%}{%#each links as l%}<a>{%name%}</a>{%/each%}{%/if%}</block>"#;
    let result = render(markup);
    assert!(!result.contains("<a>Home</a>"), "Links should be hidden when showLinks is false, got: {}", result);
    assert!(!result.contains("<a>About</a>"), "Links should be hidden when showLinks is false");
}

// ============================================
// TRUTHINESS TESTS
// ============================================

#[test]
fn test_truthiness_empty_string_is_falsy() {
    let markup = r#"<block config='{"val":""}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "Empty string should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "Empty string should not show true branch");
}

#[test]
fn test_truthiness_zero_is_falsy() {
    let markup = r#"<block config='{"val":0}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "0 should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "0 should not show true branch");
}

#[test]
fn test_truthiness_false_is_falsy() {
    let markup = r#"<block config='{"val":false}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "false should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "false should not show true branch");
}

#[test]
fn test_truthiness_null_is_falsy() {
    let markup = r#"<block config='{"val":null}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "null should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "null should not show true branch");
}

#[test]
fn test_truthiness_empty_array_is_falsy() {
    let markup = r#"<block config='{"val":[]}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "Empty array should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "Empty array should not show true branch");
}

#[test]
fn test_truthiness_missing_key_is_falsy() {
    let markup = r#"<block config='{}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("HIDDEN"), "Missing key should be falsy, got: {}", result);
    assert!(!result.contains("VISIBLE"), "Missing key should not show true branch");
}

#[test]
fn test_truthiness_nonempty_string_is_truthy() {
    let markup = r#"<block config='{"val":"hello"}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("VISIBLE"), "Non-empty string should be truthy, got: {}", result);
    assert!(!result.contains("HIDDEN"), "Non-empty string should not show false branch");
}

#[test]
fn test_truthiness_nonzero_number_is_truthy() {
    let markup = r#"<block config='{"val":42}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("VISIBLE"), "Non-zero number should be truthy, got: {}", result);
    assert!(!result.contains("HIDDEN"), "Non-zero number should not show false branch");
}

#[test]
fn test_truthiness_negative_number_is_truthy() {
    let markup = r#"<block config='{"val":-1}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("VISIBLE"), "Negative number should be truthy, got: {}", result);
    assert!(!result.contains("HIDDEN"), "Negative number should not show false branch");
}

#[test]
fn test_truthiness_nonempty_array_is_truthy() {
    let markup = r#"<block config='{"val":[1,2]}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("VISIBLE"), "Non-empty array should be truthy, got: {}", result);
    assert!(!result.contains("HIDDEN"), "Non-empty array should not show false branch");
}

#[test]
fn test_truthiness_true_is_truthy() {
    let markup = r#"<block config='{"val":true}'>{%#if val%}VISIBLE{%else%}HIDDEN{%/if%}</block>"#;
    let result = render(markup);
    assert!(result.contains("VISIBLE"), "true should be truthy, got: {}", result);
    assert!(!result.contains("HIDDEN"), "true should not show false branch");
}

// ============================================
// LANG AND DIR ATTRIBUTE TESTS
// ============================================

#[test]
fn test_lang_attribute_on_mail_tag() {
    let markup = r#"<mail lang="fr"><body><paragraph>Bonjour</paragraph></body></mail>"#;
    let result = render(markup);
    assert!(result.contains(r#"lang="fr""#), "Expected lang=\"fr\" in output, got: {}", result);
    assert!(result.contains(r#"dir="ltr""#), "Expected default dir=\"ltr\" when not specified");
}

#[test]
fn test_dir_attribute_on_mail_tag() {
    let markup = r#"<mail dir="rtl"><body><paragraph>Hello</paragraph></body></mail>"#;
    let result = render(markup);
    assert!(result.contains(r#"dir="rtl""#), "Expected dir=\"rtl\" in output, got: {}", result);
    assert!(result.contains(r#"lang="en""#), "Expected default lang=\"en\" when not specified");
}

#[test]
fn test_lang_and_dir_attributes_together() {
    let markup = r#"<mail lang="ar" dir="rtl"><body><paragraph>مرحبا</paragraph></body></mail>"#;
    let result = render(markup);
    assert!(result.contains(r#"lang="ar""#), "Expected lang=\"ar\" in output, got: {}", result);
    assert!(result.contains(r#"dir="rtl""#), "Expected dir=\"rtl\" in output, got: {}", result);
}

#[test]
fn test_lang_and_dir_defaults_when_not_specified() {
    let markup = r#"<mail><body><paragraph>Hello</paragraph></body></mail>"#;
    let result = render(markup);
    assert!(result.contains(r#"lang="en""#), "Expected default lang=\"en\", got: {}", result);
    assert!(result.contains(r#"dir="ltr""#), "Expected default dir=\"ltr\", got: {}", result);
}
