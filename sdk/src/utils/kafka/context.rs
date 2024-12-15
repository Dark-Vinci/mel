use {
    log::info,
    rdkafka::{
        consumer::{BaseConsumer, ConsumerContext, Rebalance},
        error::KafkaResult,
        ClientContext, TopicPartitionList,
    },
};

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre re balance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post re balance {:?}", rebalance);
    }

    fn commit_callback(
        &self,
        result: KafkaResult<()>,
        _offsets: &TopicPartitionList,
    ) {
        info!("Committing offsets: {:?}", result);
    }
}
