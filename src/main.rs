use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatch::primitives::DateTime;
use aws_sdk_cloudwatch::types::{Metric, MetricDataQuery, MetricStat};
use aws_sdk_cloudwatch::Client;
use chrono::{Duration, DurationRound, Utc};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&config);

    let end_time = Utc::now()
        .duration_trunc(Duration::hours(1))
        .unwrap()
        .timestamp() as u64;

    let start_time = end_time - 3600;

    let metric = Metric::builder()
        .namespace("AWS/Lambda".to_string())
        .metric_name("ConcurrentExecutions".to_string())
        .set_dimensions(None)
        .build();

    let metric_stat = MetricStat::builder()
        .metric(metric)
        .period(60)
        .stat("Maximum")
        .build();

    let usage_data = MetricDataQuery::builder()
        .metric_stat(metric_stat)
        .id("e1")
        .return_data(false)
        .build();

    let percentage_usage_data = MetricDataQuery::builder()
        .expression("SERVICE_QUOTA(e1)")
        .id("e2")
        .return_data(true)
        .build();

    let results = client
        .get_metric_data()
        .metric_data_queries(usage_data)
        .metric_data_queries(percentage_usage_data)
        .start_time(DateTime::from_secs(start_time as i64))
        .end_time(DateTime::from_secs(end_time as i64))
        .into_paginator()
        .send()
        .collect::<Result<Vec<_>, _>>()
        .await;

    println!("{:#?}", results);
}
