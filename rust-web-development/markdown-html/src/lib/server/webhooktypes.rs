use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Config {
    content_type: String,
    insecure_ssl: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct LastResponse {
    code: Option<i32>,
    status: Option<String>,
    message: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Hook {
    active: bool,
    config: Config,
    created_at: String,
    events: Vec<String>,
    id: i32,
    last_response: LastResponse,
    name: String,
    ping_url: String,
    test_url: String,
    #[serde(rename = "type")]
    type_val: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct PushEvent {
    hook: Hook,
    hook_id: i32,
    zen: String,
    #[serde(skip_serializing)]
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub enum GithubResponse {
    PushEvent(PushEvent),
    PullRequest(PullRequest),
}

#[derive(Deserialize, Debug)]
struct PullRequest {
    action: String,
    assignee: Assignee,
    #[serde(skip_serializing)]
    enterprise: Enterprise,
    #[serde(skip_serializing)]
    installation: Installation,
    number: i32,
    #[serde(skip_serializing)]
    organization: Organization,
    pull_request: PullRequestObj,
    repository: Repository,
    sender: Sender,
}

#[derive(Deserialize, Debug)]
struct Assignee {
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
#[derive(Deserialize, Debug)]
struct Enterprise {}
#[derive(Deserialize, Debug)]
struct Installation {}
#[derive(Deserialize, Debug)]
struct Organization {}

// TODO pull_request object for opened
// https://docs.github.com/en/webhooks/webhook-events-and-payloads?actionType=opened#pull_request
#[derive(Deserialize, Debug)]
struct PullRequestObj {
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
#[derive(Deserialize, Debug)]
struct Milestone {
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

#[derive(Deserialize, Debug)]
struct User {
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

#[derive(Deserialize, Debug)]
struct Labels {
    id: i32,
    node_id: String,
    url: String,
    name: String,
    description: Option<String>,
    color: String,
    default: bool,
}
#[derive(Deserialize, Debug)]
struct Links {
    comments: Comments,
}
#[derive(Deserialize, Debug)]
struct Comments {
    href: String,
}
#[derive(Deserialize, Debug)]
struct Repository {}

#[derive(Deserialize, Debug)]
struct AutoMerge {
    enabled_by: User,
    merge_method: String,
    commit_title: String,
    commit_message: String,
}
#[derive(Deserialize, Debug)]
struct RequestedTeams {
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

#[derive(Deserialize, Debug)]
struct Head {
    label: String,
    #[serde(rename = "ref")]
    ref_val: String,
    repo: Option<String>,
    sha: String,
    user: User,
}
#[derive(Deserialize, Debug)]
struct Sender {}
