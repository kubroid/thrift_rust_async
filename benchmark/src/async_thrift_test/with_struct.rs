// Autogenerated by Thrift Compiler ()
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate async_thrift;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use async_thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use async_thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TAsyncInputProtocol, TAsyncOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use async_thrift::protocol::field_id;
use async_thrift::protocol::verify_expected_message_type;
use async_thrift::protocol::verify_expected_sequence_number;
use async_thrift::protocol::verify_expected_service_call;
use async_thrift::protocol::verify_required_field_exists;
use async_trait::async_trait;
use self::async_thrift::server::TAsyncProcessor;

//
// Input
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Input {
  pub num1: Option<i32>,
  pub num2: Option<i32>,
  pub comment: Option<String>,
}

impl Input {
  pub fn new<F1, F2, F3>(num1: F1, num2: F2, comment: F3) -> Input where F1: Into<Option<i32>>, F2: Into<Option<i32>>, F3: Into<Option<String>> {
    Input {
      num1: num1.into(),
      num2: num2.into(),
      comment: comment.into(),
    }
  }
  pub async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<Input> {
    i_prot.read_struct_begin().await?;
    let mut f_1: Option<i32> = Some(0);
    let mut f_2: Option<i32> = Some(0);
    let mut f_3: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32().await?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i32().await?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string().await?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    let ret = Input {
      num1: f_1,
      num2: f_2,
      comment: f_3,
    };
    Ok(ret)
  }
  pub async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Input");
    o_prot.write_struct_begin(&struct_ident).await?;
    if let Some(fld_var) = self.num1 {
      o_prot.write_field_begin(&TFieldIdentifier::new("num1", TType::I32, 1)).await?;
      o_prot.write_i32(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.num2 {
      o_prot.write_field_begin(&TFieldIdentifier::new("num2", TType::I32, 2)).await?;
      o_prot.write_i32(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.comment {
      o_prot.write_field_begin(&TFieldIdentifier::new("comment", TType::String, 3)).await?;
      o_prot.write_string(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
}

impl Default for Input {
  fn default() -> Self {
    Input{
      num1: Some(0),
      num2: Some(0),
      comment: Some("".to_owned()),
    }
  }
}

//
// Output
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Output {
  pub res: Option<i32>,
  pub comment: Option<String>,
}

impl Output {
  pub fn new<F1, F2>(res: F1, comment: F2) -> Output where F1: Into<Option<i32>>, F2: Into<Option<String>> {
    Output {
      res: res.into(),
      comment: comment.into(),
    }
  }
  pub async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<Output> {
    i_prot.read_struct_begin().await?;
    let mut f_1: Option<i32> = Some(0);
    let mut f_2: Option<String> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32().await?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string().await?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    let ret = Output {
      res: f_1,
      comment: f_2,
    };
    Ok(ret)
  }
  pub async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Output");
    o_prot.write_struct_begin(&struct_ident).await?;
    if let Some(fld_var) = self.res {
      o_prot.write_field_begin(&TFieldIdentifier::new("res", TType::I32, 1)).await?;
      o_prot.write_i32(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.comment {
      o_prot.write_field_begin(&TFieldIdentifier::new("comment", TType::String, 2)).await?;
      o_prot.write_string(fld_var).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
}

impl Default for Output {
  fn default() -> Self {
    Output{
      res: Some(0),
      comment: Some("".to_owned()),
    }
  }
}

//
// Calculator service client
//

#[async_trait]
pub trait TCalculatorSyncClient {
  async fn add(&mut self, param: Input) -> async_thrift::Result<Output>;
}

pub trait TCalculatorSyncClientMarker {}

pub struct CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  _i_prot: IP,
  _o_prot: OP,
  _sequence_number: i32,
}

impl <IP, OP> CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  pub fn new(input_protocol: IP, output_protocol: OP) -> CalculatorSyncClient<IP, OP> {
    CalculatorSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
  }
}

impl <IP, OP> TThriftClient for CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
  fn i_prot_mut(&mut self) -> &mut (dyn TAsyncInputProtocol + Send) { &mut self._i_prot }
  fn o_prot_mut(&mut self) -> &mut (dyn TAsyncOutputProtocol + Send) { &mut self._o_prot }
  fn sequence_number(&self) -> i32 { self._sequence_number }
  fn increment_sequence_number(&mut self) -> i32 { self._sequence_number += 1; self._sequence_number }
}

impl <IP, OP> TCalculatorSyncClientMarker for CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {}

#[async_trait]
impl <C: TThriftClient + TCalculatorSyncClientMarker+ Send> TCalculatorSyncClient for C {
  async fn add(&mut self, param: Input) -> async_thrift::Result<Output> {
    (
      {
        self.increment_sequence_number();
        let message_ident = TMessageIdentifier::new("add", TMessageType::Call, self.sequence_number());
        let call_args = CalculatorAddArgs { param: param };
        self.o_prot_mut().write_message_begin(&message_ident).await?;
        call_args.write_to_out_protocol(self.o_prot_mut()).await?;
        self.o_prot_mut().write_message_end().await?;
        self.o_prot_mut().flush().await
      }
    )?;
    {
      let message_ident = self.i_prot_mut().read_message_begin().await?;
      verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
      verify_expected_service_call("add", &message_ident.name)?;
      if message_ident.message_type == TMessageType::Exception {
        let remote_error = async_thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut()).await?;
        self.i_prot_mut().read_message_end().await?;
        return Err(async_thrift::Error::Application(remote_error))
      }
      verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
      let result = CalculatorAddResult::read_from_in_protocol(self.i_prot_mut()).await?;
      self.i_prot_mut().read_message_end().await?;
      result.ok_or()
    }
  }
}

//
// Calculator service processor
//

#[async_trait]
pub trait CalculatorSyncHandler {
  async fn handle_add(&self, param: Input) -> async_thrift::Result<Output>;
}

pub struct CalculatorSyncProcessor<H: CalculatorSyncHandler> {
  handler: H,
}

impl <H: CalculatorSyncHandler> CalculatorSyncProcessor<H> {
  pub fn new(handler: H) -> CalculatorSyncProcessor<H> {
    CalculatorSyncProcessor {
      handler,
    }
  }
  async fn process_add(&self, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    TCalculatorProcessFunctions::process_add(&self.handler, incoming_sequence_number, i_prot, o_prot).await
  }
}

pub struct TCalculatorProcessFunctions;

impl TCalculatorProcessFunctions {
  pub async fn process_add<H: CalculatorSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let args = CalculatorAddArgs::read_from_in_protocol(i_prot).await?;
    match handler.handle_add(args.param).await {
      Ok(handler_return) => {
        let message_ident = TMessageIdentifier::new("add", TMessageType::Reply, incoming_sequence_number);
        o_prot.write_message_begin(&message_ident).await?;
        let ret = CalculatorAddResult { result_value: Some(handler_return) };
        ret.write_to_out_protocol(o_prot).await?;
        o_prot.write_message_end().await?;
        o_prot.flush().await
      },
      Err(e) => {
        match e {
          async_thrift::Error::Application(app_err) => {
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
          _ => {
            let ret_err = {
              ApplicationError::new(
                ApplicationErrorKind::Unknown,
                e.description()
              )
            };
            let message_ident = TMessageIdentifier::new("add", TMessageType::Exception, incoming_sequence_number);
            o_prot.write_message_begin(&message_ident).await?;
            async_thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot).await?;
            o_prot.write_message_end().await?;
            o_prot.flush().await
          },
        }
      },
    }
  }
}

#[async_trait]
impl <H: CalculatorSyncHandler + Send + Sync> TAsyncProcessor for CalculatorSyncProcessor<H> {
  async fn process(&self, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let message_ident = i_prot.read_message_begin().await?;
    let res = match &*message_ident.name {
      "add" => {
        self.process_add(message_ident.sequence_number, i_prot, o_prot).await
      },
      method => {
        Err(
          async_thrift::Error::Application(
            ApplicationError::new(
              ApplicationErrorKind::UnknownMethod,
              format!("unknown method {}", method)
            )
          )
        )
      },
    };
    async_thrift::server::handle_process_result(&message_ident, res, o_prot).await
  }
}

//
// CalculatorAddArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CalculatorAddArgs {
  param: Input,
}

impl CalculatorAddArgs {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<CalculatorAddArgs> {
    i_prot.read_struct_begin().await?;
    let mut f_1: Option<Input> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = Input::read_from_in_protocol(i_prot).await?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    verify_required_field_exists("CalculatorAddArgs.param", &f_1)?;
    let ret = CalculatorAddArgs {
      param: f_1.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("add_args");
    o_prot.write_struct_begin(&struct_ident).await?;
    o_prot.write_field_begin(&TFieldIdentifier::new("param", TType::Struct, 1)).await?;
    self.param.write_to_out_protocol(o_prot).await?;
    o_prot.write_field_end().await?;
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
}

//
// CalculatorAddResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CalculatorAddResult {
  result_value: Option<Output>,
}

impl CalculatorAddResult {
  async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift::Result<CalculatorAddResult> {
    i_prot.read_struct_begin().await?;
    let mut f_0: Option<Output> = None;
    loop {
      let field_ident = i_prot.read_field_begin().await?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        0 => {
          let val = Output::read_from_in_protocol(i_prot).await?;
          f_0 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type).await?;
        },
      };
      i_prot.read_field_end().await?;
    }
    i_prot.read_struct_end().await?;
    let ret = CalculatorAddResult {
      result_value: f_0,
    };
    Ok(ret)
  }
  async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("CalculatorAddResult");
    o_prot.write_struct_begin(&struct_ident).await?;
    if let Some(ref fld_var) = self.result_value {
      o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0)).await?;
      fld_var.write_to_out_protocol(o_prot).await?;
      o_prot.write_field_end().await?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop().await?;
    o_prot.write_struct_end().await
  }
  fn ok_or(self) -> async_thrift::Result<Output> {
    if self.result_value.is_some() {
      Ok(self.result_value.unwrap())
    } else {
      Err(
        async_thrift::Error::Application(
          ApplicationError::new(
            ApplicationErrorKind::MissingResult,
            "no result received for CalculatorAdd"
          )
        )
      )
    }
  }
}
