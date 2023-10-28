use agql::{EmptySubscription, Object, Schema};
use async_graphql as agql;
use chrono::NaiveDateTime;
use pulldown_cmark::{html, Options, Parser};

use crate::{db::PageRecord, model::query::QueryRoot};

use self::mutation::Mutation;

pub mod mutation;
pub mod query;

pub type RewekeySchema = Schema<QueryRoot, Mutation, EmptySubscription>;

#[derive(Debug)]
struct Page {
    id: i32,
    title: String,
    source: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl From<PageRecord> for Page {
    fn from(
        PageRecord {
            id,
            title,
            source,
            create_time,
            update_time,
        }: PageRecord,
    ) -> Self {
        Self {
            id,
            title,
            source,
            create_time,
            update_time,
        }
    }
}
#[Object]
impl Page {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn body_html(&self) -> Result<String, agql::Error> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&self.source, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}
