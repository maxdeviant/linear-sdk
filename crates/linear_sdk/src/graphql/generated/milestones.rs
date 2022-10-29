#![allow(clippy::all, warnings)]
pub struct Milestones;
pub mod milestones {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Milestones";
    pub const QUERY : & str = "query Milestones($filter: MilestoneFilter, $before: String, $after: String, $first: Int, $last: Int, $include_archived: Boolean, $order_by: PaginationOrderBy) {\n    milestones(filter: $filter, before: $before, after: $after, first: $first, last: $last, includeArchived: $include_archived, orderBy: $order_by) {\n        ...MilestoneConnection\n    }\n}\n\nfragment MilestoneConnection on MilestoneConnection {\n    __typename\n    \n}" ;
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
    type TimelessDate = crate::graphql::custom_scalars::TimelessDate;
    type DateTime = crate::graphql::custom_scalars::DateTime;
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
    #[derive(Serialize)]
    pub struct BooleanComparator {
        pub eq: Option<Boolean>,
        pub neq: Option<Boolean>,
    }
    #[derive(Serialize)]
    pub struct DateComparator {
        pub eq: Option<DateTime>,
        pub neq: Option<DateTime>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<DateTime>>,
        pub nin: Option<Vec<DateTime>>,
        pub lt: Option<DateTime>,
        pub lte: Option<DateTime>,
        pub gt: Option<DateTime>,
        pub gte: Option<DateTime>,
    }
    #[derive(Serialize)]
    pub struct NullableDateComparator {
        pub eq: Option<DateTime>,
        pub neq: Option<DateTime>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<DateTime>>,
        pub nin: Option<Vec<DateTime>>,
        pub null: Option<Boolean>,
        pub lt: Option<DateTime>,
        pub lte: Option<DateTime>,
        pub gt: Option<DateTime>,
        pub gte: Option<DateTime>,
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
    pub struct IssueLabelFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        pub creator: Box<Option<NullableUserFilter>>,
        pub team: Box<Option<TeamFilter>>,
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
        pub team: Box<Option<TeamFilter>>,
        pub parent: Box<Option<IssueLabelFilter>>,
        pub and: Box<Option<Vec<IssueLabelCollectionFilter>>>,
        pub or: Box<Option<Vec<IssueLabelCollectionFilter>>>,
        pub some: Box<Option<IssueLabelFilter>>,
        pub every: Box<Option<IssueLabelFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct NullableTimelessDateComparator {
        pub eq: Option<TimelessDate>,
        pub neq: Option<TimelessDate>,
        #[serde(rename = "in")]
        pub in_: Option<Vec<TimelessDate>>,
        pub nin: Option<Vec<TimelessDate>>,
        pub null: Option<Boolean>,
        pub lt: Option<TimelessDate>,
        pub lte: Option<TimelessDate>,
        pub gt: Option<TimelessDate>,
        pub gte: Option<TimelessDate>,
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
        #[serde(rename = "isNext")]
        pub is_next: Option<BooleanComparator>,
        #[serde(rename = "isPrevious")]
        pub is_previous: Option<BooleanComparator>,
        pub team: Box<Option<TeamFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableCycleFilter>>>,
        pub or: Box<Option<Vec<NullableCycleFilter>>>,
    }
    #[derive(Serialize)]
    pub struct MilestoneFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Option<NumberComparator>,
        pub projects: Box<Option<ProjectCollectionFilter>>,
        pub and: Option<Vec<MilestoneFilter>>,
        pub or: Option<Vec<MilestoneFilter>>,
    }
    #[derive(Serialize)]
    pub struct NullableMilestoneFilter {
        pub id: Option<IDComparator>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<DateComparator>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<DateComparator>,
        pub name: Option<StringComparator>,
        #[serde(rename = "sortOrder")]
        pub sort_order: Option<NumberComparator>,
        pub projects: Box<Option<ProjectCollectionFilter>>,
        pub null: Option<Boolean>,
        pub and: Box<Option<Vec<NullableMilestoneFilter>>>,
        pub or: Box<Option<Vec<NullableMilestoneFilter>>>,
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
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserFilter>>,
        pub milestone: Box<Option<NullableMilestoneFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
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
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserFilter>>,
        pub milestone: Box<Option<NullableMilestoneFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
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
        #[serde(rename = "startDate")]
        pub start_date: Option<NullableDateComparator>,
        #[serde(rename = "targetDate")]
        pub target_date: Option<NullableDateComparator>,
        pub creator: Box<Option<UserFilter>>,
        pub lead: Box<Option<NullableUserFilter>>,
        pub members: Box<Option<UserFilter>>,
        pub milestone: Box<Option<NullableMilestoneFilter>>,
        pub issues: Box<Option<IssueCollectionFilter>>,
        pub roadmaps: Box<Option<RoadmapCollectionFilter>>,
        pub and: Box<Option<Vec<ProjectCollectionFilter>>>,
        pub or: Box<Option<Vec<ProjectCollectionFilter>>>,
        pub some: Box<Option<ProjectFilter>>,
        pub every: Box<Option<ProjectFilter>>,
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
    pub struct NestedStringComparator {
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
        pub source_type: Option<NestedStringComparator>,
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
        pub source_type: Option<NestedStringComparator>,
        pub and: Box<Option<Vec<AttachmentCollectionFilter>>>,
        pub or: Box<Option<Vec<AttachmentCollectionFilter>>>,
        pub some: Box<Option<AttachmentFilter>>,
        pub every: Box<Option<AttachmentFilter>>,
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
        pub issue: Box<Option<IssueFilter>>,
        pub and: Box<Option<Vec<CommentFilter>>>,
        pub or: Box<Option<Vec<CommentFilter>>>,
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
        pub issue: Box<Option<IssueFilter>>,
        pub and: Box<Option<Vec<CommentCollectionFilter>>>,
        pub or: Box<Option<Vec<CommentCollectionFilter>>>,
        pub some: Box<Option<CommentFilter>>,
        pub every: Box<Option<CommentFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct ContentComparator {
        pub contains: Option<String>,
        #[serde(rename = "notContains")]
        pub not_contains: Option<String>,
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
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
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
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
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
        pub creator: Box<Option<NullableUserFilter>>,
        pub parent: Box<Option<NullableIssueFilter>>,
        #[serde(rename = "snoozedBy")]
        pub snoozed_by: Box<Option<NullableUserFilter>>,
        pub labels: Box<Option<IssueLabelCollectionFilter>>,
        pub subscribers: Box<Option<UserCollectionFilter>>,
        pub team: Box<Option<TeamFilter>>,
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
        pub and: Box<Option<Vec<IssueCollectionFilter>>>,
        pub or: Box<Option<Vec<IssueCollectionFilter>>>,
        pub some: Box<Option<IssueFilter>>,
        pub every: Box<Option<IssueFilter>>,
        pub length: Option<NumberComparator>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<MilestoneFilter>,
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
    pub enum MilestoneConnection {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[deprecated(note = "Milestones will be removed. Use roadmaps instead.")]
        pub milestones: MilestonesMilestones,
    }
    pub type MilestonesMilestones = MilestoneConnection;
}
impl graphql_client::GraphQLQuery for Milestones {
    type Variables = milestones::Variables;
    type ResponseData = milestones::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: milestones::QUERY,
            operation_name: milestones::OPERATION_NAME,
        }
    }
}
