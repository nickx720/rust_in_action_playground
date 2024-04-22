use serde::Deserialize;
#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub content_type: String,
    pub insecure_ssl: String,
    pub url: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LastResponse {
    pub code: Option<i32>,
    pub status: Option<String>,
    pub message: Option<String>,
}
#[derive(Deserialize, Default, Debug)]
pub struct Hook {
    pub active: bool,
    pub config: Config,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: i32,
    pub last_response: LastResponse,
    pub name: String,
    pub ping_url: String,
    pub test_url: String,
    #[serde(rename = "type")]
    pub type_val: String,
    pub url: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct PushEvent {
    pub hook: Hook,
    pub hook_id: i32,
    pub zen: String,
    #[serde(skip)]
    pub repository: Repository,
    pub sender: Assignee,
}

// https://serde.rs/enum-representations.html#untagged

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GithubResponse {
    PushEvent(PushEvent),
    PullRequest(PullRequest),
}

#[derive(Deserialize, Default, Debug)]
pub struct PullRequest {
    pub action: String,
    pub assignee: Assignee,
    #[serde(skip)]
    pub enterprise: Enterprise,
    #[serde(skip)]
    pub installation: Installation,
    pub number: i32,
    #[serde(skip)]
    pub organization: Organization,
    pub pull_request: PullRequestObj,
    #[serde(skip)]
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Deserialize, Default, Debug)]
pub struct Assignee {
    pub avatar_url: String,
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub html_url: String,
    pub id: i32,
    pub login: String,
    pub name: String,
    pub node_id: String,
    pub gravatar_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub starred_at: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_val: String,
    pub url: String,
}
#[derive(Deserialize, Default, Debug)]
pub struct Enterprise {}
#[derive(Deserialize, Default, Debug)]
pub struct Installation {}
#[derive(Deserialize, Default, Debug)]
pub struct Organization {}

// TODO pull_request object for opened
// https://docs.github.com/en/webhooks/webhook-events-and-payloads?actionType=opened#pull_request
#[derive(Deserialize, Default, Debug)]
pub struct PullRequestObj {
    pub url: String,
    pub id: i32,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: i32,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: String,
    pub labels: Vec<Labels>,
    pub milestone: Milestone,
    pub active_lock_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignees: User,
    pub requested_reviewers: User,
    pub requested_teams: RequestedTeams,
    pub head: Head, // @TODO properties of repo
    pub base: Head,
    pub _links: Links,
    pub author_association: String,
    pub auto_merge: Option<AutoMerge>,
    pub draft: bool,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: User,
    pub comments: i32,
    pub review_comments: i32,
    pub maintainer_can_modify: bool,
    pub commits: i32,
    pub additions: i32,
    pub deletions: i32,
    pub changed_files: i32,
    pub allow_auto_merge: bool,
    pub allow_update_branch: bool,
    pub delete_branch_on_merge: bool,
    pub merge_commit_message: String,
    pub merge_commit_title: String,
    pub squash_merge_commit_message: String,
    pub squash_merge_commit_title: String,
    pub use_squash_pr_title_as_default: bool,
}
#[derive(Deserialize, Default, Debug)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i32,
    pub node_id: String,
    pub number: i32,
    pub state: String,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    pub open_issues: i32,
    pub closed_issues: i32,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub due_on: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_val: String,
    pub site_admin: bool,
    pub starred_at: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Labels {
    pub id: i32,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}
#[derive(Deserialize, Default, Debug)]
pub struct Links {
    comments: Comments,
}
#[derive(Deserialize, Default, Debug)]
pub struct Comments {
    href: String,
}
#[derive(Deserialize, Default, Debug)]
pub struct Repository {}

#[derive(Deserialize, Default, Debug)]
pub struct AutoMerge {
    enabled_by: User,
    merge_method: String,
    commit_title: String,
    commit_message: String,
}
#[derive(Deserialize, Default, Debug)]
pub struct RequestedTeams {
    id: i32,
    node_id: String,
    url: String,
    members_url: String,
    name: String,
    description: Option<String>,
    permission: String,
    privacy: String,
    notification_setting: String,
    html_url: String,
    repositories_url: String,
    slug: String,
    ldap_dn: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Head {
    label: String,
    #[serde(rename = "ref")]
    ref_val: String,
    repo: Option<String>,
    sha: String,
    user: User,
}
#[derive(Deserialize, Default, Debug)]
pub struct Sender {}
