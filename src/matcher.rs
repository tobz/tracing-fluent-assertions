use tracing::Subscriber;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

#[derive(Eq, Hash, PartialEq)]
enum FieldCriterion {
    Exists(String),
}

#[derive(Default, Eq, Hash, PartialEq)]
pub struct SpanMatcher {
    name: Option<String>,
    target: Option<String>,
    parent_name: Option<String>,
    fields: Vec<FieldCriterion>,
}

impl SpanMatcher {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_parent_name(&mut self, name: String) {
        self.parent_name = Some(name);
    }

    pub fn set_target(&mut self, target: String) {
        self.target = Some(target);
    }

    pub fn add_field_exists(&mut self, field: String) {
        self.fields.push(FieldCriterion::Exists(field));
    }

    pub fn matches<S>(&self, span: &SpanRef<'_, S>) -> bool
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        if let Some(name) = self.name.as_ref() {
            if span.name() != name {
                return false;
            }
        }

        if let Some(target) = self.target.as_ref() {
            if span.metadata().target() != target {
                return false;
            }
        }

        if let Some(name) = self.parent_name.as_ref() {
            let mut has_matching_parent = false;
            let mut parent = span.parent();
            while let Some(span) = parent {
                if span.name() == name {
                    has_matching_parent = true;
                    break;
                }

                parent = span.parent();
            }

            if !has_matching_parent {
                return false;
            }
        }

        if !self.fields.is_empty() {
            let span_fields = span.fields();
            for field in &self.fields {
                match field {
                    FieldCriterion::Exists(expected_field) => {
                        if span_fields.field(expected_field).is_none() {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
