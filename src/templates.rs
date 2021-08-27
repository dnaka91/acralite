use askama::Template;
use hyper::StatusCode;

pub mod apps {
    use anyhow::Result;
    use askama::Template;

    use crate::db::models::{App, Version};

    #[derive(Template)]
    #[template(path = "apps/index.html")]
    pub struct Index {
        pub apps: Vec<App>,
    }

    #[derive(Template)]
    #[template(path = "apps/create.html")]
    pub struct Create {}

    #[derive(Template)]
    #[template(path = "apps/create_result.html")]
    pub struct CreateResult {
        pub result: Result<App>,
    }

    #[derive(Template)]
    #[template(path = "apps/details.html")]
    pub struct Details {
        pub app: App,
        pub versions: Vec<Version>,
    }
}

pub mod users {
    use askama::Template;

    use crate::db::models::User;

    #[derive(Template)]
    #[template(path = "users/list.html")]
    pub struct List {
        pub users: Vec<User>,
    }

    #[derive(Template)]
    #[template(path = "users/create.html")]
    pub struct Create {}
}

#[derive(Template)]
#[template(path = "error_page.html")]
pub struct ErrorPage {
    pub status: StatusCode,
    pub message: String,
}
