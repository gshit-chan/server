mod thread;

use thread::ThreadQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(ThreadQuery);
