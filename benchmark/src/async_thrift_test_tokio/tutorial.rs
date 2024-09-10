// Autogenerated by Thrift Compiler ()
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate async_thrift_tokio;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use async_trait::async_trait;

use async_thrift_tokio::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use async_thrift_tokio::protocol::{TAsyncInputProtocol, TAsyncOutputProtocol, TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TSetIdentifier, TStructIdentifier, TType};
use async_thrift_tokio::protocol::field_id;
use async_thrift_tokio::protocol::verify_expected_message_type;
use async_thrift_tokio::protocol::verify_expected_sequence_number;
use async_thrift_tokio::protocol::verify_expected_service_call;
use async_thrift_tokio::protocol::verify_required_field_exists;
use async_thrift_tokio::server::TAsyncProcessor;

//
// Calculator service client
//

#[async_trait]
pub trait TCalculatorSyncClient {
    async fn ping(&mut self) -> async_thrift_tokio::Result<()>;
}

pub trait TCalculatorSyncClientMarker {}

pub struct CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
    _i_prot: IP,
    _o_prot: OP,
    _sequence_number: i32,
}

impl<IP, OP> CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
    pub fn new(input_protocol: IP, output_protocol: OP) -> CalculatorSyncClient<IP, OP> {
        CalculatorSyncClient { _i_prot: input_protocol, _o_prot: output_protocol, _sequence_number: 0 }
    }
}

impl<IP, OP> TThriftClient for CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {
    fn i_prot_mut(&mut self) -> &mut (dyn TAsyncInputProtocol + Send) { &mut self._i_prot }
    fn o_prot_mut(&mut self) -> &mut (dyn TAsyncOutputProtocol + Send) { &mut self._o_prot }
    fn sequence_number(&self) -> i32 { self._sequence_number }
    fn increment_sequence_number(&mut self) -> i32 {
        self._sequence_number += 1;
        self._sequence_number
    }
}

impl<IP, OP> TCalculatorSyncClientMarker for CalculatorSyncClient<IP, OP> where IP: TAsyncInputProtocol, OP: TAsyncOutputProtocol {}

#[async_trait]
impl<C: TThriftClient + TCalculatorSyncClientMarker + Send> TCalculatorSyncClient for C {
    async fn ping(&mut self) -> async_thrift_tokio::Result<()> {
        (
            {
                self.increment_sequence_number();
                let message_ident = TMessageIdentifier::new("ping", TMessageType::Call, self.sequence_number());
                let call_args = CalculatorPingArgs {};
                self.o_prot_mut().write_message_begin(&message_ident).await?;
                call_args.write_to_out_protocol(self.o_prot_mut()).await?;
                self.o_prot_mut().write_message_end().await?;
                self.o_prot_mut().flush().await
            }
        )?;
        {
            let message_ident = self.i_prot_mut().read_message_begin().await?;
            verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
            verify_expected_service_call("ping", &message_ident.name)?;
            if message_ident.message_type == TMessageType::Exception {
                let remote_error = async_thrift_tokio::Error::read_application_error_from_in_protocol(self.i_prot_mut()).await?;
                self.i_prot_mut().read_message_end().await?;
                return Err(async_thrift_tokio::Error::Application(remote_error));
            }
            verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
            let result = CalculatorPingResult::read_from_in_protocol(self.i_prot_mut()).await?;
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
    async fn handle_ping(&self) -> async_thrift_tokio::Result<()>;
}

pub struct CalculatorSyncProcessor<H: CalculatorSyncHandler> {
    handler: H,
}

impl<H: CalculatorSyncHandler> CalculatorSyncProcessor<H> {
    pub fn new(handler: H) -> CalculatorSyncProcessor<H> {
        CalculatorSyncProcessor {
            handler,
        }
    }
    async fn process_ping(&self, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift_tokio::Result<()> {
        TCalculatorProcessFunctions::process_ping(&self.handler, incoming_sequence_number, i_prot, o_prot).await
    }
}

pub struct TCalculatorProcessFunctions;

impl TCalculatorProcessFunctions {
    pub async fn process_ping<H: CalculatorSyncHandler>(handler: &H, incoming_sequence_number: i32, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift_tokio::Result<()> {
        let _ = CalculatorPingArgs::read_from_in_protocol(i_prot).await?;
        match handler.handle_ping().await {
            Ok(_) => {
                let message_ident = TMessageIdentifier::new("ping", TMessageType::Reply, incoming_sequence_number);
                o_prot.write_message_begin(&message_ident).await?;
                let ret = CalculatorPingResult {};
                ret.write_to_out_protocol(o_prot).await?;
                o_prot.write_message_end().await?;
                o_prot.flush().await
            }
            Err(e) => {
                match e {
                    async_thrift_tokio::Error::Application(app_err) => {
                        let message_ident = TMessageIdentifier::new("ping", TMessageType::Exception, incoming_sequence_number);
                        o_prot.write_message_begin(&message_ident).await?;
                        async_thrift_tokio::Error::write_application_error_to_out_protocol(&app_err, o_prot).await?;
                        o_prot.write_message_end().await?;
                        o_prot.flush().await
                    }
                    _ => {
                        let ret_err = {
                            ApplicationError::new(
                                ApplicationErrorKind::Unknown,
                                e.to_string(),
                            )
                        };
                        let message_ident = TMessageIdentifier::new("ping", TMessageType::Exception, incoming_sequence_number);
                        o_prot.write_message_begin(&message_ident).await?;
                        async_thrift_tokio::Error::write_application_error_to_out_protocol(&ret_err, o_prot).await?;
                        o_prot.write_message_end().await?;
                        o_prot.flush().await
                    }
                }
            }
        }
    }
}

#[async_trait]
impl<H: CalculatorSyncHandler + Send + Sync> TAsyncProcessor for CalculatorSyncProcessor<H> {
    async fn process(&self, i_prot: &mut (dyn TAsyncInputProtocol + Send), o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift_tokio::Result<()> {
        let message_ident = i_prot.read_message_begin().await?;
        let res = match &*message_ident.name {
            "ping" => {
                self.process_ping(message_ident.sequence_number, i_prot, o_prot).await
            }
            method => {
                Err(
                    async_thrift_tokio::Error::Application(
                        ApplicationError::new(
                            ApplicationErrorKind::UnknownMethod,
                            format!("unknown method {}", method),
                        )
                    )
                )
            }
        };
        async_thrift_tokio::server::handle_process_result(&message_ident, res, o_prot).await
    }
}

//
// CalculatorPingArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CalculatorPingArgs {}

impl CalculatorPingArgs {
    async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift_tokio::Result<CalculatorPingArgs> {
        i_prot.read_struct_begin().await?;
        loop {
            let field_ident = i_prot.read_field_begin().await?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                _ => {
                    i_prot.skip(field_ident.field_type).await?;
                }
            };
            i_prot.read_field_end().await?;
        }
        i_prot.read_struct_end().await?;
        let ret = CalculatorPingArgs {};
        Ok(ret)
    }
    async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift_tokio::Result<()> {
        let struct_ident = TStructIdentifier::new("ping_args");
        o_prot.write_struct_begin(&struct_ident).await?;
        o_prot.write_field_stop().await?;
        o_prot.write_struct_end().await
    }
}

//
// CalculatorPingResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CalculatorPingResult {}

impl CalculatorPingResult {
    async fn read_from_in_protocol(i_prot: &mut (dyn TAsyncInputProtocol + Send)) -> async_thrift_tokio::Result<CalculatorPingResult> {
        i_prot.read_struct_begin().await?;
        loop {
            let field_ident = i_prot.read_field_begin().await?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                _ => {
                    i_prot.skip(field_ident.field_type).await?;
                }
            };
            i_prot.read_field_end().await?;
        }
        i_prot.read_struct_end().await?;
        let ret = CalculatorPingResult {};
        Ok(ret)
    }
    async fn write_to_out_protocol(&self, o_prot: &mut (dyn TAsyncOutputProtocol + Send)) -> async_thrift_tokio::Result<()> {
        let struct_ident = TStructIdentifier::new("CalculatorPingResult");
        o_prot.write_struct_begin(&struct_ident).await?;
        o_prot.write_field_stop().await?;
        o_prot.write_struct_end().await
    }
    fn ok_or(self) -> async_thrift_tokio::Result<()> {
        Ok(())
    }
}

