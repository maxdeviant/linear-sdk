#![allow(clippy::all, warnings)]
pub struct Issues;
pub mod issues {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Issues";
    pub const QUERY : & str = "query Issues($filter: IssueFilter, $before: String, $after: String, $first: Int, $last: Int, $include_archived: Boolean, $order_by: PaginationOrderBy) {\n    issues(filter: $filter, before: $before, after: $after, first: $first, last: $last, includeArchived: $include_archived, orderBy: $order_by) {\n        ...IssueConnection\n    }\n}\n\nfragment IssueConnection on IssueConnection {\n    __typename\n    \n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTimeOrDuration = crate::graphql::custom_scalars::DateTimeOrDuration;
    type TimelessDateOrDuration = crate::graphql::custom_scalars::TimelessDateOrDuration;
    #[derive(Debug)]
    pub enum PaginationOrderBy {
        createdAt,
        updatedAt,
        Other(String),
    }
    impl ::serde::Serialize for PaginationOrderBy {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaginationOrderBy::createdAt => "createdAt",
                PaginationOrderBy::updatedAt => "updatedAt",
                PaginationOrderBy::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaginationOrderBy {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "createdAt" => Ok(PaginationOrderBy::createdAt),
                "updatedAt" => Ok(PaginationOrderBy::updatedAt),
                _ => Ok(PaginationOrderBy::Other(s)),
            }
        }
    }
    #[derive(Debug)]
    pub enum SlaStatus {
        Breached,
        HighRisk,
        MediumRisk,
        LowRisk,
        Completed,
        Failed,
        Other(String),
    }
    impl ::serde::Serialize for SlaStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                SlaStatus::Breached => "Breached",
                SlaStatus::HighRisk => "HighRisk",
                SlaStatus::MediumRisk => "MediumRisk",
                SlaStatus::LowRisk => "LowRisk",
                SlaStatus::Completed => "Completed",
                SlaStatus::Failed => "Failed",
                SlaStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SlaStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "Breached" => Ok(SlaStatus::Breached),
                "HighRisk" => Ok(SlaStatus::HighRisk),
                "MediumRisk" => Ok(SlaStatus::MediumRisk),
                "LowRisk" => Ok(SlaStatus::LowRisk),
                "Completed" => Ok(SlaStatus::Completed),
                "Failed" => Ok(SlaStatus::Failed),
                _ => Ok(SlaStatus::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct BooleanComparator {
        pub eq: Option<Boolean>,
        pub neq: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct ContentComparator {
        pub contains: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
    }
    #[derive(Serialize)]
    pub struct DateComparator {
        pub eq: Option<DateTimeOrDuration>,
        pub neq: Option<DateTimeOrDuration>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<DateTimeOrDuration>>,
        pub nin: Option<Vec<DateTimeOrDuration>>,
        pub lt: Option<DateTimeOrDuration>,
        pub lte: Option<DateTimeOrDuration>,
        pub gt: Option<DateTimeOrDuration>,
        pub gte: Option<DateTimeOrDuration>,
    }
    #[derive(Serialize)]
    pub struct NullableDateComparator {
        pub eq: Option<DateTimeOrDuration>,
        pub neq: Option<DateTimeOrDuration>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<DateTimeOrDuration>>,
        pub nin: Option<Vec<DateTimeOrDuration>>,
        pub null: Option<Boolean>,
        pub lt: Option<DateTimeOrDuration>,
        pub lte: Option<DateTimeOrDuration>,
        pub gt: Option<DateTimeOrDuration>,
        pub gte: Option<DateTimeOrDuration>,
    }
    #[derive(Serialize)]
    pub struct EstimateComparator {
        pub eq: Option<Float>,
        pub neq: Option<Float>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<Float>>,
        pub nin: Option<Vec<Float>>,
        pub null: Option<Boolean>,
        pub lt: Option<Float>,
        pub lte: Option<Float>,
        pub gt: Option<Float>,
        pub gte: Option<Float>,
        pub or: Option<Vec<NullableNumberComparator>>,
        pub and: Option<Vec<NullableNumberComparator>>,
    }
    #[derive(Serialize)]
    pub struct RelationExistsComparator {
        pub eq: Option<Boolean>,
        pub neq: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct IDComparator {
        pub eq: Option<ID>,
        pub neq: Option<ID>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<ID>>,
        pub nin: Option<Vec<ID>>,
    }
    #[derive(Serialize)]
    pub struct NumberComparator {
        pub eq: Option<Float>,
        pub neq: Option<Float>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<Float>>,
        pub nin: Option<Vec<Float>>,
        pub lt: Option<Float>,
        pub lte: Option<Float>,
        pub gt: Option<Float>,
        pub gte: Option<Float>,
    }
    #[derive(Serialize)]
    pub struct NullableNumberComparator {
        pub eq: Option<Float>,
        pub neq: Option<Float>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<Float>>,
        pub nin: Option<Vec<Float>>,
        pub null: Option<Boolean>,
        pub lt: Option<Float>,
        pub lte: Option<Float>,
        pub gt: Option<Float>,
        pub gte: Option<Float>,
    }
    #[derive(Serialize)]
    pub struct StringComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        #[serde(rename = "eqIgnoreCase")]
        pub eq_ignore_case: Option<String>,
        #[serde(rename = "neqIgnoreCase")]
        pub neq_ignore_case: Option<String>,
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
        #[serde(rename = "startsWithIgnoreCase")]
        pub starts_with_ignore_case: Option<String>,
        #[serde(rename = "notStartsWith")]
        pub not_starts_with: Option<String>,
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(rename = "notEndsWith")]
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        #[serde(rename = "containsIgnoreCase")]
        pub contains_ignore_case: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
        #[serde(rename = "notContainsIgnoreCase")]
        pub not_contains_ignore_case: Option<String>,
    }
    #[derive(Serialize)]
    pub struct NullableStringComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub null: Option<Boolean>,
        #[serde(rename = "eqIgnoreCase")]
        pub eq_ignore_case: Option<String>,
        #[serde(rename = "neqIgnoreCase")]
        pub neq_ignore_case: Option<String>,
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
        #[serde(rename = "startsWithIgnoreCase")]
        pub starts_with_ignore_case: Option<String>,
        #[serde(rename = "notStartsWith")]
        pub not_starts_with: Option<String>,
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(rename = "notEndsWith")]
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        #[serde(rename = "containsIgnoreCase")]
        pub contains_ignore_case: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
        #[serde(rename = "notContainsIgnoreCase")]
        pub not_contains_ignore_case: Option<String>,
    }
    #[derive(Serialize)]
    pub struct SourceTypeComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        #[serde(rename = "eqIgnoreCase")]
        pub eq_ignore_case: Option<String>,
        #[serde(rename = "neqIgnoreCase")]
        pub neq_ignore_case: Option<String>,
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
        #[serde(rename = "startsWithIgnoreCase")]
        pub starts_with_ignore_case: Option<String>,
        #[serde(rename = "notStartsWith")]
        pub not_starts_with: Option<String>,
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(rename = "notEndsWith")]
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        #[serde(rename = "containsIgnoreCase")]
        pub contains_ignore_case: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
        #[serde(rename = "notContainsIgnoreCase")]
        pub not_contains_ignore_case: Option<String>,
    }
    #[derive(Serialize)]
    pub struct NullableTimelessDateComparator {
        pub eq: Option<TimelessDateOrDuration>,
        pub neq: Option<TimelessDateOrDuration>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<TimelessDateOrDuration>>,
        pub nin: Option<Vec<TimelessDateOrDuration>>,
        pub null: Option<Boolean>,
        pub lt: Option<TimelessDateOrDuration>,
        pub lte: Option<TimelessDateOrDuration>,
        pub gt: Option<TimelessDateOrDuration>,
        pub gte: Option<TimelessDateOrDuration>,
    }
    #[derive(Serialize)]
    pub struct SubTypeComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub null: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct SourceMetadataComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        pub null: Option<Boolean>,
        #[serde(rename = "subType")]
        pub sub_type: Option<SubTypeComparator>,
    }
    #[derive(Serialize)]
    pub struct AttachmentFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        pub subtitle: Option<NullableStringComparator>,
        pub url: Option<StringComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        #[serde(rename = "sourceType")]
        pub source_type: Option<SourceTypeComparator>,
        pub and: Box<Option<Vec<AttachmentFilter>>>,
        pub or: Box<Option<Vec<AttachmentFilter>>>,
    }
    #[derive(Serialize)]
    pub struct AttachmentCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        pub subtitle: Option<NullableStringComparator>,
        pub url: Option<StringComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        #[serde(rename = "sourceType")]
        pub source_type: Option<SourceTypeComparator>,
        pub and: Box<Option<Vec<AttachmentCollectionFilter>>>,
        pub or: Box<Option<Vec<AttachmentCollectionFilter>>>,
        pub some: Box<Option<AttachmentFilter>>,
        pub every: Box<Option<AttachmentFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct StringItemComparator {
        pub eq: Option<String>,
        pub neq: Option<String>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        pub nin: Option<Vec<String>>,
        #[serde(rename = "eqIgnoreCase")]
        pub eq_ignore_case: Option<String>,
        #[serde(rename = "neqIgnoreCase")]
        pub neq_ignore_case: Option<String>,
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
        #[serde(rename = "startsWithIgnoreCase")]
        pub starts_with_ignore_case: Option<String>,
        #[serde(rename = "notStartsWith")]
        pub not_starts_with: Option<String>,
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(rename = "notEndsWith")]
        pub not_ends_with: Option<String>,
        pub contains: Option<String>,
        #[serde(rename = "containsIgnoreCase")]
        pub contains_ignore_case: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
        #[serde(rename = "notContainsIgnoreCase")]
        pub not_contains_ignore_case: Option<String>,
    }
    #[derive(Serialize)]
    pub struct StringArrayComparator {
        pub length: Option<NumberComparator>,
        pub every: Option<Vec<StringItemComparator>>,
        pub some: Option<Vec<StringItemComparator>>,
    }
    #[derive(Serialize)]
    pub struct CustomerFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slackChannelId")]
        pub slack_channel_id: Option<StringComparator>,
        pub domains: Option<StringArrayComparator>,
        #[serde(rename = "externalIds")]
        pub external_ids: Option<StringArrayComparator>,
        pub owner: Box<Option<UserFilter>>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<CustomerFilter>>>,
        pub or: Box<Option<Vec<CustomerFilter>>>,
    }
    #[derive(Serialize)]
    pub struct CustomerNeedFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub priority: Option<NumberComparator>,
        pub project: Box<Option<NullableProjectFilter>>,
        pub issue: Box<Option<NullableIssueFilter>>,
        pub comment: Box<Option<NullableCommentFilter>>,
        pub customer: Box<Option<CustomerFilter>>,
        pub and: Box<Option<Vec<CustomerNeedFilter>>>,
        pub or: Box<Option<Vec<CustomerNeedFilter>>>,
    }
    #[derive(Serialize)]
    pub struct CustomerNeedCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub priority: Option<NumberComparator>,
        pub project: Box<Option<NullableProjectFilter>>,
        pub issue: Box<Option<NullableIssueFilter>>,
        pub comment: Box<Option<NullableCommentFilter>>,
        pub customer: Box<Option<CustomerFilter>>,
        pub and: Box<Option<Vec<CustomerNeedCollectionFilter>>>,
        pub or: Box<Option<Vec<CustomerNeedCollectionFilter>>>,
        pub some: Box<Option<CustomerNeedFilter>>,
        pub every: Box<Option<CustomerNeedFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct CommentFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Box<Option<UserFilter>>,
        pub issue: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "projectUpdate")]
        pub project_update: Box<Option<ProjectUpdateFilter>>,
        pub parent: Box<Option<NullableCommentFilter>>,
        #[serde(rename = "documentContent")]
        pub document_content: Box<Option<DocumentContentFilter>>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<CommentFilter>>>,
        pub or: Box<Option<Vec<CommentFilter>>>,
    }
    #[derive(Serialize)]
    pub struct NullableCommentFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Box<Option<UserFilter>>,
        pub issue: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "projectUpdate")]
        pub project_update: Box<Option<ProjectUpdateFilter>>,
        pub parent: Box<Option<NullableCommentFilter>>,
        #[serde(rename = "documentContent")]
        pub document_content: Box<Option<DocumentContentFilter>>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableCommentFilter>>>,
        pub or: Box<Option<Vec<NullableCommentFilter>>>,
    }
    #[derive(Serialize)]
    pub struct CommentCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub body: Option<StringComparator>,
        pub user: Box<Option<UserFilter>>,
        pub issue: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "projectUpdate")]
        pub project_update: Box<Option<ProjectUpdateFilter>>,
        pub parent: Box<Option<NullableCommentFilter>>,
        #[serde(rename = "documentContent")]
        pub document_content: Box<Option<DocumentContentFilter>>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<CommentCollectionFilter>>>,
        pub or: Box<Option<Vec<CommentCollectionFilter>>>,
        pub some: Box<Option<CommentFilter>>,
        pub every: Box<Option<CommentFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct NullableCycleFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub number: Option<NumberComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "startsAt")]
        pub starts_at: Option<DateComparator>,
        #[serde(rename = "endsAt")]
        pub ends_at: Option<DateComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateComparator>,
        #[serde(rename = "isActive")]
        pub is_active: Option<BooleanComparator>,
        #[serde(rename = "isInCooldown")]
        pub is_in_cooldown: Option<BooleanComparator>,
        #[serde(rename = "isNext")]
        pub is_next: Option<BooleanComparator>,
        #[serde(rename = "isPrevious")]
        pub is_previous: Option<BooleanComparator>,
        #[serde(rename = "isFuture")]
        pub is_future: Option<BooleanComparator>,
        #[serde(rename = "isPast")]
        pub is_past: Option<BooleanComparator>,
        pub team: Box<Option<TeamFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableCycleFilter>>>,
        pub or: Box<Option<Vec<NullableCycleFilter>>>,
    }
    #[derive(Serialize)]
    pub struct ProjectStatusFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub description: Option<StringComparator>,
        pub position: Option<NumberComparator>,
        #[serde(rename = "type")]
        pub type_: Option<StringComparator>,
        pub projects: Box<Option<ProjectCollectionFilter>>,
        pub and: Box<Option<Vec<ProjectStatusFilter>>>,
        pub or: Box<Option<Vec<ProjectStatusFilter>>>,
    }
    #[derive(Serialize)]
    pub struct SlaStatusComparator {
        pub eq: Option<SlaStatus>,
        pub neq: Option<SlaStatus>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<SlaStatus>>,
        pub nin: Option<Vec<SlaStatus>>,
        pub null: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct NullableIssueFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub number: Option<NumberComparator>,
        pub title: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub priority: Option<NullableNumberComparator>,
        pub estimate: Option<EstimateComparator>,
        #[serde(rename = "startedAt")]
        pub started_at: Option<NullableDateComparator>,
        #[serde(rename = "triagedAt")]
        pub triaged_at: Option<NullableDateComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<NullableDateComparator>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<NullableDateComparator>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<NullableDateComparator>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<NullableTimelessDateComparator>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<NullableDateComparator>,
        pub assignee: Box<Option<NullableUserFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        #[serde(rename = "sourceMetadata")]
        pub source_metadata: Option<SourceMetadataComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
        #[serde(rename = "projectMilestone")]
        pub project_milestone: Option<NullableProjectMilestoneFilter>,
        pub comments: Box<Option<CommentCollectionFilter>>,
        pub cycle: Box<Option<NullableCycleFilter>>,
        pub project: Box<Option<NullableProjectFilter>>,
        pub state: Box<Option<WorkflowStateFilter>>,
        pub children: Box<Option<IssueCollectionFilter>>,
        pub attachments: Box<Option<AttachmentCollectionFilter>>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDuplicateRelations")]
        pub has_duplicate_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "slaStatus")]
        pub sla_status: Option<SlaStatusComparator>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableIssueFilter>>>,
        pub or: Box<Option<Vec<NullableIssueFilter>>>,
    }
    #[derive(Serialize)]
    pub struct IssueFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub number: Option<NumberComparator>,
        pub title: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub priority: Option<NullableNumberComparator>,
        pub estimate: Option<EstimateComparator>,
        #[serde(rename = "startedAt")]
        pub started_at: Option<NullableDateComparator>,
        #[serde(rename = "triagedAt")]
        pub triaged_at: Option<NullableDateComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<NullableDateComparator>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<NullableDateComparator>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<NullableDateComparator>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<NullableTimelessDateComparator>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<NullableDateComparator>,
        pub assignee: Box<Option<NullableUserFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        #[serde(rename = "sourceMetadata")]
        pub source_metadata: Option<SourceMetadataComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
        #[serde(rename = "projectMilestone")]
        pub project_milestone: Option<NullableProjectMilestoneFilter>,
        pub comments: Box<Option<CommentCollectionFilter>>,
        pub cycle: Box<Option<NullableCycleFilter>>,
        pub project: Box<Option<NullableProjectFilter>>,
        pub state: Box<Option<WorkflowStateFilter>>,
        pub children: Box<Option<IssueCollectionFilter>>,
        pub attachments: Box<Option<AttachmentCollectionFilter>>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDuplicateRelations")]
        pub has_duplicate_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "slaStatus")]
        pub sla_status: Option<SlaStatusComparator>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<IssueFilter>>>,
        pub or: Box<Option<Vec<IssueFilter>>>,
    }
    #[derive(Serialize)]
    pub struct IssueCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub number: Option<NumberComparator>,
        pub title: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub priority: Option<NullableNumberComparator>,
        pub estimate: Option<EstimateComparator>,
        #[serde(rename = "startedAt")]
        pub started_at: Option<NullableDateComparator>,
        #[serde(rename = "triagedAt")]
        pub triaged_at: Option<NullableDateComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "canceledAt")]
        pub canceled_at: Option<NullableDateComparator>,
        #[serde(rename = "autoClosedAt")]
        pub auto_closed_at: Option<NullableDateComparator>,
        #[serde(rename = "autoArchivedAt")]
        pub auto_archived_at: Option<NullableDateComparator>,
        #[serde(rename = "dueDate")]
        pub due_date: Option<NullableTimelessDateComparator>,
        #[serde(rename = "snoozedUntilAt")]
        pub snoozed_until_at: Option<NullableDateComparator>,
        pub assignee: Box<Option<NullableUserFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        #[serde(rename = "sourceMetadata")]
        pub source_metadata: Option<SourceMetadataComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
        #[serde(rename = "projectMilestone")]
        pub project_milestone: Option<NullableProjectMilestoneFilter>,
        pub comments: Box<Option<CommentCollectionFilter>>,
        pub cycle: Box<Option<NullableCycleFilter>>,
        pub project: Box<Option<NullableProjectFilter>>,
        pub state: Box<Option<WorkflowStateFilter>>,
        pub children: Box<Option<IssueCollectionFilter>>,
        pub attachments: Box<Option<AttachmentCollectionFilter>>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDuplicateRelations")]
        pub has_duplicate_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "slaStatus")]
        pub sla_status: Option<SlaStatusComparator>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<IssueCollectionFilter>>>,
        pub or: Box<Option<Vec<IssueCollectionFilter>>>,
        pub some: Box<Option<IssueFilter>>,
        pub every: Box<Option<IssueFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct NullableTemplateFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub null: Option<Boolean>,
        pub and: Option<Vec<NullableTemplateFilter>>,
        pub or: Option<Vec<NullableTemplateFilter>>,
    }
    #[derive(Serialize)]
    pub struct UserFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "displayName")]
        pub display_name: Option<StringComparator>,
        pub email: Option<StringComparator>,
        pub active: Option<BooleanComparator>,
        #[serde(rename = "assignedIssues")]
        pub assigned_issues: Box<Option<IssueCollectionFilter>>,
        pub admin: Option<BooleanComparator>,
        #[serde(rename = "isMe")]
        pub is_me: Option<BooleanComparator>,
        pub and: Box<Option<Vec<UserFilter>>>,
        pub or: Box<Option<Vec<UserFilter>>>,
    }
    #[derive(Serialize)]
    pub struct NullableUserFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "displayName")]
        pub display_name: Option<StringComparator>,
        pub email: Option<StringComparator>,
        pub active: Option<BooleanComparator>,
        #[serde(rename = "assignedIssues")]
        pub assigned_issues: Box<Option<IssueCollectionFilter>>,
        pub admin: Option<BooleanComparator>,
        #[serde(rename = "isMe")]
        pub is_me: Option<BooleanComparator>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableUserFilter>>>,
        pub or: Box<Option<Vec<NullableUserFilter>>>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "displayName")]
        pub display_name: Option<StringComparator>,
        pub email: Option<StringComparator>,
        pub active: Option<BooleanComparator>,
        #[serde(rename = "assignedIssues")]
        pub assigned_issues: Box<Option<IssueCollectionFilter>>,
        pub admin: Option<BooleanComparator>,
        #[serde(rename = "isMe")]
        pub is_me: Option<BooleanComparator>,
        pub and: Box<Option<Vec<UserCollectionFilter>>>,
        pub or: Box<Option<Vec<UserCollectionFilter>>>,
        pub some: Box<Option<UserFilter>>,
        pub every: Box<Option<UserFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct InitiativeFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub status: Option<StringComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub and: Box<Option<Vec<InitiativeFilter>>>,
        pub or: Box<Option<Vec<InitiativeFilter>>>,
    }
    #[derive(Serialize)]
    pub struct InitiativeCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub status: Option<StringComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub and: Box<Option<Vec<InitiativeCollectionFilter>>>,
        pub or: Box<Option<Vec<InitiativeCollectionFilter>>>,
        pub some: Box<Option<InitiativeFilter>>,
        pub every: Box<Option<InitiativeFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct ProjectUpdatesFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub health: Option<StringComparator>,
        pub and: Option<Vec<ProjectUpdatesFilter>>,
        pub or: Option<Vec<ProjectUpdatesFilter>>,
    }
    #[derive(Serialize)]
    pub struct ProjectUpdatesCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub health: Option<StringComparator>,
        pub and: Option<Vec<ProjectUpdatesCollectionFilter>>,
        pub or: Option<Vec<ProjectUpdatesCollectionFilter>>,
        pub some: Option<ProjectUpdatesFilter>,
        pub every: Option<ProjectUpdatesFilter>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct ProjectFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Box<Option<ProjectStatusFilter>>,
        pub priority: Option<NullableNumberComparator>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        #[serde(rename = "healthWithAge")]
        pub health_with_age: Option<StringComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependedOnByRelations")]
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependsOnRelations")]
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasViolatedRelations")]
        pub has_violated_relations: Option<RelationExistsComparator>,
        #[serde(rename = "projectUpdates")]
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserCollectionFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
        pub initiatives: Box<Option<InitiativeCollectionFilter>>,
        #[serde(rename = "projectMilestones")]
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "completedProjectMilestones")]
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "nextProjectMilestone")]
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        #[serde(rename = "accessibleTeams")]
        pub accessible_teams: Box<Option<TeamCollectionFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<ProjectFilter>>>,
        pub or: Box<Option<Vec<ProjectFilter>>>,
    }
    #[derive(Serialize)]
    pub struct NullableProjectFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Box<Option<ProjectStatusFilter>>,
        pub priority: Option<NullableNumberComparator>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        #[serde(rename = "healthWithAge")]
        pub health_with_age: Option<StringComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependedOnByRelations")]
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependsOnRelations")]
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasViolatedRelations")]
        pub has_violated_relations: Option<RelationExistsComparator>,
        #[serde(rename = "projectUpdates")]
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserCollectionFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
        pub initiatives: Box<Option<InitiativeCollectionFilter>>,
        #[serde(rename = "projectMilestones")]
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "completedProjectMilestones")]
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "nextProjectMilestone")]
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        #[serde(rename = "accessibleTeams")]
        pub accessible_teams: Box<Option<TeamCollectionFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableProjectFilter>>>,
        pub or: Box<Option<Vec<NullableProjectFilter>>>,
    }
    #[derive(Serialize)]
    pub struct ProjectCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub state: Option<StringComparator>,
        pub status: Box<Option<ProjectStatusFilter>>,
        pub priority: Option<NullableNumberComparator>,
        #[serde(rename = "searchableContent")]
        pub searchable_content: Option<ContentComparator>,
        #[serde(rename = "completedAt")]
        pub completed_at: Option<NullableDateComparator>,
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub health: Option<StringComparator>,
        #[serde(rename = "healthWithAge")]
        pub health_with_age: Option<StringComparator>,
        #[serde(rename = "hasRelatedRelations")]
        pub has_related_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependedOnByRelations")]
        pub has_depended_on_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasDependsOnRelations")]
        pub has_depends_on_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockedByRelations")]
        pub has_blocked_by_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasBlockingRelations")]
        pub has_blocking_relations: Option<RelationExistsComparator>,
        #[serde(rename = "hasViolatedRelations")]
        pub has_violated_relations: Option<RelationExistsComparator>,
        #[serde(rename = "projectUpdates")]
        pub project_updates: Option<ProjectUpdatesCollectionFilter>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserCollectionFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
        pub initiatives: Box<Option<InitiativeCollectionFilter>>,
        #[serde(rename = "projectMilestones")]
        pub project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "completedProjectMilestones")]
        pub completed_project_milestones: Option<ProjectMilestoneCollectionFilter>,
        #[serde(rename = "nextProjectMilestone")]
        pub next_project_milestone: Option<ProjectMilestoneFilter>,
        #[serde(rename = "accessibleTeams")]
        pub accessible_teams: Box<Option<TeamCollectionFilter>>,
        #[serde(rename = "lastAppliedTemplate")]
        pub last_applied_template: Option<NullableTemplateFilter>,
        pub needs: Box<Option<CustomerNeedCollectionFilter>>,
        pub and: Box<Option<Vec<ProjectCollectionFilter>>>,
        pub or: Box<Option<Vec<ProjectCollectionFilter>>>,
        pub some: Box<Option<ProjectFilter>>,
        pub every: Box<Option<ProjectFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct ProjectMilestoneFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub and: Option<Vec<ProjectMilestoneFilter>>,
        pub or: Option<Vec<ProjectMilestoneFilter>>,
    }
    #[derive(Serialize)]
    pub struct NullableProjectMilestoneFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub null: Option<Boolean>,
        pub and: Option<Vec<NullableProjectMilestoneFilter>>,
        pub or: Option<Vec<NullableProjectMilestoneFilter>>,
    }
    #[derive(Serialize)]
    pub struct ProjectMilestoneCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub and: Option<Vec<ProjectMilestoneCollectionFilter>>,
        pub or: Option<Vec<ProjectMilestoneCollectionFilter>>,
        pub some: Option<ProjectMilestoneFilter>,
        pub every: Option<ProjectMilestoneFilter>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct RoadmapFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub and: Box<Option<Vec<RoadmapFilter>>>,
        pub or: Box<Option<Vec<RoadmapFilter>>>,
    }
    #[derive(Serialize)]
    pub struct RoadmapCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub and: Box<Option<Vec<RoadmapCollectionFilter>>>,
        pub or: Box<Option<Vec<RoadmapCollectionFilter>>>,
        pub some: Box<Option<RoadmapFilter>>,
        pub every: Box<Option<RoadmapFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct TeamFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub key: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub and: Box<Option<Vec<TeamFilter>>>,
        pub or: Box<Option<Vec<TeamFilter>>>,
    }
    #[derive(Serialize)]
    pub struct NullableTeamFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub key: Option<StringComparator>,
        pub description: Option<NullableStringComparator>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableTeamFilter>>>,
        pub or: Box<Option<Vec<NullableTeamFilter>>>,
    }
    #[derive(Serialize)]
    pub struct TeamCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub and: Box<Option<Vec<TeamCollectionFilter>>>,
        pub or: Box<Option<Vec<TeamCollectionFilter>>>,
        pub some: Box<Option<TeamFilter>>,
        pub every: Box<Option<TeamFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct WorkflowStateFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub description: Option<StringComparator>,
        pub position: Option<NumberComparator>,
        #[serde(rename = "type")]
        pub type_: Option<StringComparator>,
        pub team: Box<Option<TeamFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub and: Box<Option<Vec<WorkflowStateFilter>>>,
        pub or: Box<Option<Vec<WorkflowStateFilter>>>,
    }
    #[derive(Serialize)]
    pub struct DocumentContentFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub project: Box<Option<ProjectFilter>>,
        pub document: Box<Option<DocumentFilter>>,
    }
    #[derive(Serialize)]
    pub struct IssueLabelFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub team: Box<Option<NullableTeamFilter>>,
        pub parent: Box<Option<IssueLabelFilter>>,
        pub and: Box<Option<Vec<IssueLabelFilter>>>,
        pub or: Box<Option<Vec<IssueLabelFilter>>>,
    }
    #[derive(Serialize)]
    pub struct IssueLabelCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub team: Box<Option<NullableTeamFilter>>,
        pub parent: Box<Option<IssueLabelFilter>>,
        pub and: Box<Option<Vec<IssueLabelCollectionFilter>>>,
        pub or: Box<Option<Vec<IssueLabelCollectionFilter>>>,
        pub some: Box<Option<IssueLabelFilter>>,
        pub every: Box<Option<IssueLabelFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct ProjectUpdateFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub user: Box<Option<UserFilter>>,
        pub project: Box<Option<ProjectFilter>>,
        pub reactions: Option<ReactionCollectionFilter>,
        pub and: Box<Option<Vec<ProjectUpdateFilter>>>,
        pub or: Box<Option<Vec<ProjectUpdateFilter>>>,
    }
    #[derive(Serialize)]
    pub struct DocumentFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub title: Option<StringComparator>,
        #[serde(rename = "slugId")]
        pub slug_id: Option<StringComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub project: Box<Option<ProjectFilter>>,
        pub initiative: Box<Option<InitiativeFilter>>,
        pub and: Box<Option<Vec<DocumentFilter>>>,
        pub or: Box<Option<Vec<DocumentFilter>>>,
    }
    #[derive(Serialize)]
    pub struct ReactionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub emoji: Option<StringComparator>,
        #[serde(rename = "customEmojiId")]
        pub custom_emoji_id: Option<IDComparator>,
        pub and: Option<Vec<ReactionFilter>>,
        pub or: Option<Vec<ReactionFilter>>,
    }
    #[derive(Serialize)]
    pub struct ReactionCollectionFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub emoji: Option<StringComparator>,
        #[serde(rename = "customEmojiId")]
        pub custom_emoji_id: Option<IDComparator>,
        pub and: Option<Vec<ReactionCollectionFilter>>,
        pub or: Option<Vec<ReactionCollectionFilter>>,
        pub some: Option<ReactionFilter>,
        pub every: Option<ReactionFilter>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<IssueFilter>,
        pub before: Option<String>,
        pub after: Option<String>,
        pub first: Option<Int>,
        pub last: Option<Int>,
        pub include_archived: Option<Boolean>,
        pub order_by: Option<PaginationOrderBy>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum IssueConnection {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub issues: IssuesIssues,
    }
    pub type IssuesIssues = IssueConnection;
}
impl graphql_client::GraphQLQuery for Issues {
    type Variables = issues::Variables;
    type ResponseData = issues::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issues::QUERY,
            operation_name: issues::OPERATION_NAME,
        }
    }
}
