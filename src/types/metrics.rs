use crate::types::*;

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        family::Family,
        histogram::{Histogram, exponential_buckets_range},
    },
};

#[derive(Clone, EncodeLabelSet, Hash, PartialEq, Debug)]
pub struct ReqLatencyLabels {
    pub handler_name: &'static str,
}
impl Eq for ReqLatencyLabels {}

pub struct LatencyObserver<'a, L: Clone + core::hash::Hash + Eq> {
    start: Instant,
    labels: L,
    histogram: &'a Family<L, Histogram>,
}
impl<'a, L: Clone + core::hash::Hash + Eq> LatencyObserver<'a, L> {
    fn new(histogram: &'a Family<L, Histogram>, labels: L) -> Self {
        Self {
            start: Instant::now(),
            labels,
            histogram,
        }
    }
}
impl<'a, L: Clone + core::hash::Hash + Eq> Drop for LatencyObserver<'_, L> {
    fn drop(&mut self) {
        let dur = Instant::now() - self.start;
        self.histogram
            .get_or_create(&self.labels)
            .observe(dur.as_secs_f64());
    }
}

#[derive(Clone)]
pub struct Metrics {
    pub req_latency_sec: Family<ReqLatencyLabels, Histogram>,
}
impl Metrics {
    pub fn observe_req_latency(
        &self,
        handler_name: &'static str,
    ) -> LatencyObserver<'_, ReqLatencyLabels> {
        LatencyObserver::new(&self.req_latency_sec, ReqLatencyLabels { handler_name })
    }
}
impl Default for Metrics {
    fn default() -> Self {
        Self {
            req_latency_sec: Family::<ReqLatencyLabels, Histogram>::new_with_constructor(|| {
                Histogram::new(exponential_buckets_range(0.005, 10.0, 11))
            }),
        }
    }
}
