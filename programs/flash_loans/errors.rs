use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
  #[msg("No access allowed")]
  NotAllowed
}