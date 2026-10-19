use super::BroadcastLifecycleStatus;

pub(super) fn pagination_query(
    page: Option<i64>,
    limit: Option<i64>,
    after: Option<&str>,
    before: Option<&str>,
) -> Vec<(&'static str, String)> {
    let mut params = Vec::new();
    if let Some(page) = page {
        params.push(("page", page.to_string()));
    }
    if let Some(limit) = limit {
        params.push(("limit", limit.to_string()));
    }
    if let Some(after) = after {
        params.push(("after", after.to_owned()));
    }
    if let Some(before) = before {
        params.push(("before", before.to_owned()));
    }
    params
}

/// Generic list parameters (used by audiences, topics, segments, templates,
/// broadcasts, etc.).
///
/// Pass either `page` (offset mode) or `after` / `before` (cursor mode), not both.
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Forward cursor: id of the last item from the previous page. Mutually exclusive with `page`.
    pub after: Option<String>,
    /// Backward cursor: id of the first item from the next page. Walks results in reverse.
    /// Mutually exclusive with `page`.
    pub before: Option<String>,
    pub search: Option<String>,
}

impl ListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl PaginationParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        )
    }
}

/// Topic contacts list parameters.
///
/// Pass either `page` (offset mode) or `after` / `before` (cursor mode), not both.
#[derive(Debug, Clone, Default)]
pub struct ListTopicContactsParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub search: Option<String>,
    pub subscribed: Option<bool>,
}

impl ListTopicContactsParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        if let Some(subscribed) = self.subscribed {
            params.push(("subscribed", subscribed.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListBroadcastEmailsParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl ListBroadcastEmailsParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListContactActivityParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl ListContactActivityParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListBroadcastsParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub search: Option<String>,
    pub status: Option<BroadcastLifecycleStatus>,
}

impl ListBroadcastsParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        if let Some(ref status) = self.status {
            params.push(("status", status.as_str().to_string()));
        }
        params
    }
}

/// Audience contacts list parameters.
///
/// Pass either `page` (offset mode) or `after` / `before` (cursor mode), not both.
#[derive(Debug, Clone, Default)]
pub struct ListAudienceContactsParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Forward cursor: id of the last item from the previous page. Mutually exclusive with `page`.
    pub after: Option<String>,
    /// Backward cursor: id of the first item from the next page. Walks results in reverse.
    /// Mutually exclusive with `page`.
    pub before: Option<String>,
    pub search: Option<String>,
    pub subscribed: Option<bool>,
    pub exclude_topic_id: Option<String>,
}

impl ListAudienceContactsParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        if let Some(subscribed) = self.subscribed {
            params.push(("subscribed", subscribed.to_string()));
        }
        if let Some(ref topic_id) = self.exclude_topic_id {
            params.push(("excludeTopicId", topic_id.clone()));
        }
        params
    }
}

/// Contacts list parameters.
///
/// Pass either `page` (offset mode) or `after` / `before` (cursor mode), not both.
#[derive(Debug, Clone, Default)]
pub struct ListContactsParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Forward cursor: id of the last item from the previous page. Mutually exclusive with `page`.
    pub after: Option<String>,
    /// Backward cursor: id of the first item from the next page. Walks results in reverse.
    /// Mutually exclusive with `page`.
    pub before: Option<String>,
    pub search: Option<String>,
    pub exclude_audience: Option<String>,
    pub subscribed: Option<bool>,
}

impl ListContactsParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        if let Some(ref exclude_audience) = self.exclude_audience {
            params.push(("excludeAudience", exclude_audience.clone()));
        }
        if let Some(subscribed) = self.subscribed {
            params.push(("subscribed", subscribed.to_string()));
        }
        params
    }
}

/// Domain list parameters
#[derive(Debug, Clone, Default)]
pub struct DomainListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub verified: Option<bool>,
}

impl DomainListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(verified) = self.verified {
            params.push(("verified", verified.to_string()));
        }
        params
    }
}
