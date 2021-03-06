use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum SendError {}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum ProducerCreateError {}
