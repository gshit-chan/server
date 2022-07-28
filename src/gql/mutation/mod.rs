use thread::ThreadMutation;

mod thread;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(ThreadMutation);
