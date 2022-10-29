impl crate::LinearClient {
    pub async fn sync_bootstrap(
        &self,
        variables: crate::graphql::sync_bootstrap::Variables,
    ) -> Result<crate::graphql::sync_bootstrap::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::SyncBootstrap>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn sync_entity_count(
        &self,
        variables: crate::graphql::sync_entity_count::Variables,
    ) -> Result<crate::graphql::sync_entity_count::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::SyncEntityCount>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn sync_batch(
        &self,
        variables: crate::graphql::sync_batch::Variables,
    ) -> Result<crate::graphql::sync_batch::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::SyncBatch>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn dependent_model_sync(
        &self,
        variables: crate::graphql::dependent_model_sync::Variables,
    ) -> Result<crate::graphql::dependent_model_sync::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::DependentModelSync>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn archived_model_sync(
        &self,
        variables: crate::graphql::archived_model_sync::Variables,
    ) -> Result<crate::graphql::archived_model_sync::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ArchivedModelSync>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn archived_models_sync(
        &self,
        variables: crate::graphql::archived_models_sync::Variables,
    ) -> Result<crate::graphql::archived_models_sync::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ArchivedModelsSync>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn api_keys(
        &self,
        variables: crate::graphql::api_keys::Variables,
    ) -> Result<crate::graphql::api_keys::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ApiKeys>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn application_info(
        &self,
        variables: crate::graphql::application_info::Variables,
    ) -> Result<crate::graphql::application_info::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ApplicationInfo>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn application_info_by_ids(
        &self,
        variables: crate::graphql::application_info_by_ids::Variables,
    ) -> Result<crate::graphql::application_info_by_ids::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ApplicationInfoByIds>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn application_with_authorization(
        &self,
        variables: crate::graphql::application_with_authorization::Variables,
    ) -> Result<crate::graphql::application_with_authorization::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ApplicationWithAuthorization>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn authorized_applications(
        &self,
        variables: crate::graphql::authorized_applications::Variables,
    ) -> Result<crate::graphql::authorized_applications::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AuthorizedApplications>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn workspace_authorized_applications(
        &self,
        variables: crate::graphql::workspace_authorized_applications::Variables,
    ) -> Result<crate::graphql::workspace_authorized_applications::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::WorkspaceAuthorizedApplications>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn attachments(
        &self,
        variables: crate::graphql::attachments::Variables,
    ) -> Result<crate::graphql::attachments::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Attachments>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn attachment(
        &self,
        variables: crate::graphql::attachment::Variables,
    ) -> Result<crate::graphql::attachment::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Attachment>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn attachments_for_url(
        &self,
        variables: crate::graphql::attachments_for_url::Variables,
    ) -> Result<crate::graphql::attachments_for_url::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AttachmentsForUrl>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn attachment_issue(
        &self,
        variables: crate::graphql::attachment_issue::Variables,
    ) -> Result<crate::graphql::attachment_issue::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AttachmentIssue>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn audit_entry_types(
        &self,
        variables: crate::graphql::audit_entry_types::Variables,
    ) -> Result<crate::graphql::audit_entry_types::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AuditEntryTypes>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn audit_entries(
        &self,
        variables: crate::graphql::audit_entries::Variables,
    ) -> Result<crate::graphql::audit_entries::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AuditEntries>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn available_users(
        &self,
        variables: crate::graphql::available_users::Variables,
    ) -> Result<crate::graphql::available_users::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AvailableUsers>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn sso_url_from_email(
        &self,
        variables: crate::graphql::sso_url_from_email::Variables,
    ) -> Result<crate::graphql::sso_url_from_email::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::SsoUrlFromEmail>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn comments(
        &self,
        variables: crate::graphql::comments::Variables,
    ) -> Result<crate::graphql::comments::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Comments>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn comment(
        &self,
        variables: crate::graphql::comment::Variables,
    ) -> Result<crate::graphql::comment::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Comment>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn custom_views(
        &self,
        variables: crate::graphql::custom_views::Variables,
    ) -> Result<crate::graphql::custom_views::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CustomViews>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn custom_view(
        &self,
        variables: crate::graphql::custom_view::Variables,
    ) -> Result<crate::graphql::custom_view::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::CustomView>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn cycles(
        &self,
        variables: crate::graphql::cycles::Variables,
    ) -> Result<crate::graphql::cycles::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Cycles>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn cycle(
        &self,
        variables: crate::graphql::cycle::Variables,
    ) -> Result<crate::graphql::cycle::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Cycle>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn documents(
        &self,
        variables: crate::graphql::documents::Variables,
    ) -> Result<crate::graphql::documents::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Documents>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn document(
        &self,
        variables: crate::graphql::document::Variables,
    ) -> Result<crate::graphql::document::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Document>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn emojis(
        &self,
        variables: crate::graphql::emojis::Variables,
    ) -> Result<crate::graphql::emojis::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Emojis>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn emoji(
        &self,
        variables: crate::graphql::emoji::Variables,
    ) -> Result<crate::graphql::emoji::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Emoji>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn favorites(
        &self,
        variables: crate::graphql::favorites::Variables,
    ) -> Result<crate::graphql::favorites::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Favorites>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn favorite(
        &self,
        variables: crate::graphql::favorite::Variables,
    ) -> Result<crate::graphql::favorite::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Favorite>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn figma_embed_info(
        &self,
        variables: crate::graphql::figma_embed_info::Variables,
    ) -> Result<crate::graphql::figma_embed_info::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::FigmaEmbedInfo>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn initiatives(
        &self,
        variables: crate::graphql::initiatives::Variables,
    ) -> Result<crate::graphql::initiatives::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Initiatives>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn initiative(
        &self,
        variables: crate::graphql::initiative::Variables,
    ) -> Result<crate::graphql::initiative::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Initiative>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integrations(
        &self,
        variables: crate::graphql::integrations::Variables,
    ) -> Result<crate::graphql::integrations::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Integrations>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integration(
        &self,
        variables: crate::graphql::integration::Variables,
    ) -> Result<crate::graphql::integration::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Integration>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integration_resources(
        &self,
        variables: crate::graphql::integration_resources::Variables,
    ) -> Result<crate::graphql::integration_resources::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IntegrationResources>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integration_resource(
        &self,
        variables: crate::graphql::integration_resource::Variables,
    ) -> Result<crate::graphql::integration_resource::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IntegrationResource>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_updates(
        &self,
        variables: crate::graphql::project_updates::Variables,
    ) -> Result<crate::graphql::project_updates::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectUpdates>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integrations_settings(
        &self,
        variables: crate::graphql::integrations_settings::Variables,
    ) -> Result<crate::graphql::integrations_settings::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IntegrationsSettings>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integration_templates(
        &self,
        variables: crate::graphql::integration_templates::Variables,
    ) -> Result<crate::graphql::integration_templates::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IntegrationTemplates>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn integration_template(
        &self,
        variables: crate::graphql::integration_template::Variables,
    ) -> Result<crate::graphql::integration_template::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IntegrationTemplate>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_import_finish_github_oauth(
        &self,
        variables: crate::graphql::issue_import_finish_github_oauth::Variables,
    ) -> Result<crate::graphql::issue_import_finish_github_oauth::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueImportFinishGithubOauth>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_labels(
        &self,
        variables: crate::graphql::issue_labels::Variables,
    ) -> Result<crate::graphql::issue_labels::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueLabels>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_label(
        &self,
        variables: crate::graphql::issue_label::Variables,
    ) -> Result<crate::graphql::issue_label::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueLabel>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_relations(
        &self,
        variables: crate::graphql::issue_relations::Variables,
    ) -> Result<crate::graphql::issue_relations::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueRelations>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_relation(
        &self,
        variables: crate::graphql::issue_relation::Variables,
    ) -> Result<crate::graphql::issue_relation::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueRelation>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issues(
        &self,
        variables: crate::graphql::issues::Variables,
    ) -> Result<crate::graphql::issues::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Issues>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue(
        &self,
        variables: crate::graphql::issue::Variables,
    ) -> Result<crate::graphql::issue::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Issue>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_search(
        &self,
        variables: crate::graphql::issue_search::Variables,
    ) -> Result<crate::graphql::issue_search::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueSearch>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_vcs_branch_search(
        &self,
        variables: crate::graphql::issue_vcs_branch_search::Variables,
    ) -> Result<crate::graphql::issue_vcs_branch_search::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssueVcsBranchSearch>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn issue_priority_values(
        &self,
        variables: crate::graphql::issue_priority_values::Variables,
    ) -> Result<crate::graphql::issue_priority_values::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::IssuePriorityValues>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn milestones(
        &self,
        variables: crate::graphql::milestones::Variables,
    ) -> Result<crate::graphql::milestones::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Milestones>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn milestone(
        &self,
        variables: crate::graphql::milestone::Variables,
    ) -> Result<crate::graphql::milestone::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Milestone>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn notifications(
        &self,
        variables: crate::graphql::notifications::Variables,
    ) -> Result<crate::graphql::notifications::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Notifications>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn notification(
        &self,
        variables: crate::graphql::notification::Variables,
    ) -> Result<crate::graphql::notification::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Notification>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn notification_subscriptions(
        &self,
        variables: crate::graphql::notification_subscriptions::Variables,
    ) -> Result<crate::graphql::notification_subscriptions::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::NotificationSubscriptions>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn notification_subscription(
        &self,
        variables: crate::graphql::notification_subscription::Variables,
    ) -> Result<crate::graphql::notification_subscription::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::NotificationSubscription>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization_domain_claim_request(
        &self,
        variables: crate::graphql::organization_domain_claim_request::Variables,
    ) -> Result<crate::graphql::organization_domain_claim_request::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::OrganizationDomainClaimRequest>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization_invites(
        &self,
        variables: crate::graphql::organization_invites::Variables,
    ) -> Result<crate::graphql::organization_invites::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::OrganizationInvites>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization_invite(
        &self,
        variables: crate::graphql::organization_invite::Variables,
    ) -> Result<crate::graphql::organization_invite::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::OrganizationInvite>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization_invite_details(
        &self,
        variables: crate::graphql::organization_invite_details::Variables,
    ) -> Result<crate::graphql::organization_invite_details::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::OrganizationInviteDetails>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization(
        &self,
        variables: crate::graphql::organization::Variables,
    ) -> Result<crate::graphql::organization::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Organization>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn organization_exists(
        &self,
        variables: crate::graphql::organization_exists::Variables,
    ) -> Result<crate::graphql::organization_exists::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::OrganizationExists>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_links(
        &self,
        variables: crate::graphql::project_links::Variables,
    ) -> Result<crate::graphql::project_links::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectLinks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_link(
        &self,
        variables: crate::graphql::project_link::Variables,
    ) -> Result<crate::graphql::project_link::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectLink>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn projects(
        &self,
        variables: crate::graphql::projects::Variables,
    ) -> Result<crate::graphql::projects::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Projects>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project(
        &self,
        variables: crate::graphql::project::Variables,
    ) -> Result<crate::graphql::project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Project>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_update_interactions(
        &self,
        variables: crate::graphql::project_update_interactions::Variables,
    ) -> Result<crate::graphql::project_update_interactions::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectUpdateInteractions>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_update_interaction(
        &self,
        variables: crate::graphql::project_update_interaction::Variables,
    ) -> Result<crate::graphql::project_update_interaction::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectUpdateInteraction>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn project_update(
        &self,
        variables: crate::graphql::project_update::Variables,
    ) -> Result<crate::graphql::project_update::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::ProjectUpdate>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn push_subscription_test(
        &self,
        variables: crate::graphql::push_subscription_test::Variables,
    ) -> Result<crate::graphql::push_subscription_test::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::PushSubscriptionTest>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn rate_limit_status(
        &self,
        variables: crate::graphql::rate_limit_status::Variables,
    ) -> Result<crate::graphql::rate_limit_status::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::RateLimitStatus>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn roadmaps(
        &self,
        variables: crate::graphql::roadmaps::Variables,
    ) -> Result<crate::graphql::roadmaps::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Roadmaps>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn roadmap(
        &self,
        variables: crate::graphql::roadmap::Variables,
    ) -> Result<crate::graphql::roadmap::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Roadmap>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn roadmap_to_projects(
        &self,
        variables: crate::graphql::roadmap_to_projects::Variables,
    ) -> Result<crate::graphql::roadmap_to_projects::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::RoadmapToProjects>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn roadmap_to_project(
        &self,
        variables: crate::graphql::roadmap_to_project::Variables,
    ) -> Result<crate::graphql::roadmap_to_project::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::RoadmapToProject>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn team_memberships(
        &self,
        variables: crate::graphql::team_memberships::Variables,
    ) -> Result<crate::graphql::team_memberships::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::TeamMemberships>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn team_membership(
        &self,
        variables: crate::graphql::team_membership::Variables,
    ) -> Result<crate::graphql::team_membership::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::TeamMembership>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn teams(
        &self,
        variables: crate::graphql::teams::Variables,
    ) -> Result<crate::graphql::teams::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Teams>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn administrable_teams(
        &self,
        variables: crate::graphql::administrable_teams::Variables,
    ) -> Result<crate::graphql::administrable_teams::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::AdministrableTeams>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn team(
        &self,
        variables: crate::graphql::team::Variables,
    ) -> Result<crate::graphql::team::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Team>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn templates(
        &self,
        variables: crate::graphql::templates::Variables,
    ) -> Result<crate::graphql::templates::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Templates>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn template(
        &self,
        variables: crate::graphql::template::Variables,
    ) -> Result<crate::graphql::template::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Template>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn users(
        &self,
        variables: crate::graphql::users::Variables,
    ) -> Result<crate::graphql::users::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Users>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn user(
        &self,
        variables: crate::graphql::user::Variables,
    ) -> Result<crate::graphql::user::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::User>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn viewer(
        &self,
        variables: crate::graphql::viewer::Variables,
    ) -> Result<crate::graphql::viewer::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Viewer>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn user_settings(
        &self,
        variables: crate::graphql::user_settings::Variables,
    ) -> Result<crate::graphql::user_settings::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::UserSettings>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn webhooks(
        &self,
        variables: crate::graphql::webhooks::Variables,
    ) -> Result<crate::graphql::webhooks::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Webhooks>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn webhook(
        &self,
        variables: crate::graphql::webhook::Variables,
    ) -> Result<crate::graphql::webhook::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::Webhook>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn workflow_states(
        &self,
        variables: crate::graphql::workflow_states::Variables,
    ) -> Result<crate::graphql::workflow_states::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::WorkflowStates>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}

impl crate::LinearClient {
    pub async fn workflow_state(
        &self,
        variables: crate::graphql::workflow_state::Variables,
    ) -> Result<crate::graphql::workflow_state::ResponseData, reqwest::Error> {
        let response_body = self
            .post_graphql::<crate::graphql::WorkflowState>(variables)
            .await?;

        Ok(response_body.data.expect("No data"))
    }
}