use agql::{InputObject, Object};
use async_graphql as agql;
use sqlx::PgPool;

use crate::{db::PageRecord, model::Page};

#[derive(InputObject)]
pub struct CreatePageInput {
    title: String,
    source: String,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_page(
        &self,
        ctx: &agql::Context<'_>,
        input: CreatePageInput,
    ) -> Result<Page, agql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql = "
        insert into pages (
            title, source, create_time, update_time
        )
        values (
            $1, $2, current_timestamp, current_timestamp
        )
        returning
            id, title, source, create_time, update_time
        ;
    ";
        let page_record: PageRecord = sqlx::query_as(sql)
            .bind(&input.title)
            .bind(&input.source)
            .fetch_one(&mut tx)
            .await?;
        let sql = "
            insert into page_revisions (
                page_id, source, author, create_time
            )
            values (
                $1, $2, $3, $4
            );
        ";
        sqlx::query(sql)
            .bind(page_record.id)
            .bind(&input.source)
            .bind("example@example.com") // Todo: FIXME
            .bind(page_record.update_time)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        let gql_page = page_record.into();
        Ok(gql_page)
    }
}
