#![feature(test)]

#[macro_use]
extern crate tokio_trace;
extern crate test;
use test::Bencher;

use std::sync::Mutex;
use tokio_trace::{field, span, Id, Metadata};

/// A subscriber that is enabled but otherwise does nothing.
struct EnabledSubscriber;

impl tokio_trace::Subscriber for EnabledSubscriber {
    fn new_span(&self, span: &Metadata) -> Id {
        let _ = span;
        Id::from_u64(0)
    }

    fn record_debug(&self, span: &Id, field: &field::Field, value: &::std::fmt::Debug) {
        let _ = (span, field, value);
    }

    fn add_follows_from(&self, span: &Id, follows: Id) {
        let _ = (span, follows);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        let _ = metadata;
        true
    }

    fn enter(&self, span: &Id) {
        let _ = span;
    }

    fn exit(&self, span: &Id) {
        let _ = span;
    }
}

/// Simulates a subscriber that records span data.
struct Record(Mutex<Option<String>>);

impl tokio_trace::Subscriber for Record {
    fn new_span(&self, span: &Metadata) -> Id {
        *self.0.lock().unwrap() = Some(span.name().to_string());
        Id::from_u64(0)
    }

    fn record_debug(&self, _span: &Id, _field: &field::Field, value: &::std::fmt::Debug) {
       *self.0.lock().unwrap() = Some(format!("{:?}", value));
    }

    fn add_follows_from(&self, span: &Id, follows: Id) {
        let _ = (span, follows);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        let _ = metadata;
        true
    }

    fn enter(&self, span: &Id) {
        let _ = span;
    }

    fn exit(&self, span: &Id) {
        let _ = span;
    }
}

const N_SPANS: usize = 100;

#[bench]
fn span_no_fields(b: &mut Bencher) {
    tokio_trace::subscriber::with_default(EnabledSubscriber, || {
        b.iter(|| span!("span"))
    });
}

#[bench]
fn span_repeatedly(b: &mut Bencher) {
    #[inline]
    fn mk_span(i: u64) -> tokio_trace::Span<'static> {
        span!("span", i = i)
    }

    let n = test::black_box(N_SPANS);
    tokio_trace::subscriber::with_default(EnabledSubscriber, || {
        b.iter(|| (0..n).fold(mk_span(0), |_, i| mk_span(i as u64)))
    });
}

#[bench]
fn span_with_fields(b: &mut Bencher) {
    tokio_trace::subscriber::with_default(EnabledSubscriber, || {
        b.iter(|| {
            span!(
                "span",
                foo = "foo",
                bar = "bar",
                baz = 3,
                quuux = tokio_trace::field::debug(0.99)
            )
        })
    });
}

#[bench]
fn span_with_fields_record(b: &mut Bencher) {
    tokio_trace::subscriber::with_default(Record(Mutex::new(None)), || {
        b.iter(|| {
            span!(
                "span",
                foo = "foo",
                bar = "bar",
                baz = 3,
                quuux = tokio_trace::field::debug(0.99)
            )
        })
    });
}
