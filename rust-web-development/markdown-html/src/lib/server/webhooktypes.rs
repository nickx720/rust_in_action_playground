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
    hook: Hook,
    hook_id: i32,
    zen: String,
    #[serde(skip)]
    repository: Repository,
    sender: Assignee,
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
    action: String,
    assignee: Assignee,
    #[serde(skip)]
    enterprise: Enterprise,
    #[serde(skip)]
    installation: Installation,
    number: i32,
    #[serde(skip)]
    organization: Organization,
    pull_request: PullRequestObj,
    #[serde(skip)]
    repository: Repository,
    sender: Sender,
}

#[derive(Deserialize, Default, Debug)]
pub struct Assignee {
    avatar_url: String,
    email: Option<String>,
    events_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    html_url: String,
    id: i32,
    login: String,
    name: String,
    node_id: String,
    gravatar_id: String,
    organizations_url: String,
    received_events_url: String,
    repos_url: String,
    site_admin: bool,
    starred_url: String,
    starred_at: String,
    subscriptions_url: String,
    #[serde(rename = "type")]
    type_val: String,
    url: String,
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
    url: String,
    id: i32,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    number: i32,
    state: String,
    locked: bool,
    title: String,
    user: User,
    body: String,
    labels: Vec<Labels>,
    milestone: Milestone,
    active_lock_reason: Option<String>,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    merge_commit_sha: Option<String>,
    assignees: User,
    requested_reviewers: User,
    requested_teams: RequestedTeams,
    head: Head, // @TODO properties of repo
    base: Head,
    _links: Links,
    author_association: String,
    auto_merge: Option<AutoMerge>,
    draft: bool,
    merged: bool,
    mergeable: Option<bool>,
    rebaseable: Option<bool>,
    mergeable_state: String,
    merged_by: User,
    comments: i32,
    review_comments: i32,
    maintainer_can_modify: bool,
    commits: i32,
    additions: i32,
    deletions: i32,
    changed_files: i32,
    allow_auto_merge: bool,
    allow_update_branch: bool,
    delete_branch_on_merge: bool,
    merge_commit_message: String,
    merge_commit_title: String,
    squash_merge_commit_message: String,
    squash_merge_commit_title: String,
    use_squash_pr_title_as_default: bool,
}
#[derive(Deserialize, Default, Debug)]
pub struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i32,
    node_id: String,
    number: i32,
    state: String,
    title: String,
    description: Option<String>,
    creator: User,
    open_issues: i32,
    closed_issues: i32,
    created_at: String,
    closed_at: Option<String>,
    due_on: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct User {
    name: Option<String>,
    email: Option<String>,
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
    gravatar_id: Option<String>,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_val: String,
    site_admin: bool,
    starred_at: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Labels {
    id: i32,
    node_id: String,
    url: String,
    name: String,
    description: Option<String>,
    color: String,
    default: bool,
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
