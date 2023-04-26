use std::collections::HashMap;

use log::log_enabled;

use crate::gen::mod_sdk::lekko::client::v1beta1::{
    GetBoolValueRequest, GetFloatValueRequest, GetIntValueRequest, GetJsonValueRequest,
    GetProtoValueRequest, RepositoryKey, Value, GetStringValueRequest,
};

pub fn init() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    )
}

// Inserting this information should be done automatically by tonic-build 0.9 but
// we need to upgrade.
pub struct Method {
    pub service: String,
    pub method: String,
}

// This is just the starting point for tracing.
// There are a few low hanging fruit:
// - Responding correctly with error messages
// - Pass Method around correctly to give one log line per request
// - Printing values of eval along with the context in a type safe way
//   - this can probably done with an associated type for the trait.
struct TraceInfo {
    rk: RepositoryKey,
    namespace: String,
    feature: String,
    // if debug log level is enabled, we copy the input to print.
    debug_context_res: Option<HashMap<String, Value>>,
}

pub trait InsertTrace {
    fn insert_trace<M>(&self, message: tonic::Response<M>) -> tonic::Response<M>;
}

impl InsertTrace for GetBoolValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

impl InsertTrace for GetIntValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

impl InsertTrace for GetFloatValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

impl InsertTrace for GetStringValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

impl InsertTrace for GetJsonValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

impl InsertTrace for GetProtoValueRequest {
    fn insert_trace<M>(&self, mut message: tonic::Response<M>) -> tonic::Response<M> {
        let debug_context_res = if log_enabled!(log::Level::Debug) {
            Some(self.context.clone())
        } else {
            None
        };
        message.extensions_mut().insert({
            TraceInfo {
                // we can unwrap this since we can assume that the client would
                // have failed already
                rk: self.repo_key.clone().unwrap(),
                namespace: self.namespace.clone(),
                feature: self.key.clone(),
                debug_context_res,
            }
        });
        message
    }
}

pub fn get_trace_string(ext: &http::Extensions) -> Option<String> {
    ext.get::<TraceInfo>().map(|ti| {
        let mut text = format!("{}/{}/{}", ti.rk.repo_name, ti.namespace, ti.feature);
        if let Some(debug_context) = &ti.debug_context_res {
            text = format!("{} context: {:?}", text, debug_context)
        }
        text
    })
}

pub fn http_uri_to_method(uri: String) -> Method {
    let splits: Vec<&str> = uri.split('/').collect();
    assert!(splits.len() > 2);
    Method {
        method: splits.last().unwrap().to_string(),
        service: splits[splits.len() - 2].to_owned(),
    }
}
