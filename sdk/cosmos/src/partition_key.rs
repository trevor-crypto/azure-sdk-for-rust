use crate::headers;
use http::request::Builder;
use serde::Serialize;

/// CosmosDB partition key. Every CosmosDB entity must implement it.
pub trait CosmosEntity<'a, T: Serialize + 'a> {
    /// This function returns the partition key value as reference. Implement it by returning
    /// a reference of your partition key.
    fn partition_key(&'a self) -> T;
}

// Here we do not implement add_as_header because the trait does not support errors and serializing
// with serde_json returns a Result. I am not sure why a serialization could fail (memory
// allocation)? In case we are confident that no errors should arise we can implement the trait and just
// unwrap the result of serde_json::to_string.
pub(crate) fn add_as_header_to_builder<'a, T: Serialize + 'a, P: CosmosEntity<'a, T> + 'a>(
    pk: &'a P,
    builder: Builder,
) -> Result<Builder, serde_json::Error> {
    // this must be serialized as an array even tough CosmosDB supports only a sigle
    // partition key.
    let serialized = serde_json::to_string(&[pk.partition_key()])?;
    Ok(builder.header(headers::HEADER_DOCUMENTDB_PARTITIONKEY, &serialized))
}
