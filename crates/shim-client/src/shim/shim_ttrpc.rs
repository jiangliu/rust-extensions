// This file is generated by ttrpc-compiler 0.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct TaskClient {
    client: ::ttrpc::Client,
}

impl TaskClient {
    pub fn new(client: ::ttrpc::Client) -> Self {
        TaskClient {
            client: client,
        }
    }

    pub fn state(&self, ctx: ttrpc::context::Context, req: &super::shim::StateRequest) -> ::ttrpc::Result<super::shim::StateResponse> {
        let mut cres = super::shim::StateResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "State", cres);
        Ok(cres)
    }

    pub fn create(&self, ctx: ttrpc::context::Context, req: &super::shim::CreateTaskRequest) -> ::ttrpc::Result<super::shim::CreateTaskResponse> {
        let mut cres = super::shim::CreateTaskResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Create", cres);
        Ok(cres)
    }

    pub fn start(&self, ctx: ttrpc::context::Context, req: &super::shim::StartRequest) -> ::ttrpc::Result<super::shim::StartResponse> {
        let mut cres = super::shim::StartResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Start", cres);
        Ok(cres)
    }

    pub fn delete(&self, ctx: ttrpc::context::Context, req: &super::shim::DeleteRequest) -> ::ttrpc::Result<super::shim::DeleteResponse> {
        let mut cres = super::shim::DeleteResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Delete", cres);
        Ok(cres)
    }

    pub fn pids(&self, ctx: ttrpc::context::Context, req: &super::shim::PidsRequest) -> ::ttrpc::Result<super::shim::PidsResponse> {
        let mut cres = super::shim::PidsResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Pids", cres);
        Ok(cres)
    }

    pub fn pause(&self, ctx: ttrpc::context::Context, req: &super::shim::PauseRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Pause", cres);
        Ok(cres)
    }

    pub fn resume(&self, ctx: ttrpc::context::Context, req: &super::shim::ResumeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Resume", cres);
        Ok(cres)
    }

    pub fn checkpoint(&self, ctx: ttrpc::context::Context, req: &super::shim::CheckpointTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Checkpoint", cres);
        Ok(cres)
    }

    pub fn kill(&self, ctx: ttrpc::context::Context, req: &super::shim::KillRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Kill", cres);
        Ok(cres)
    }

    pub fn exec(&self, ctx: ttrpc::context::Context, req: &super::shim::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Exec", cres);
        Ok(cres)
    }

    pub fn resize_pty(&self, ctx: ttrpc::context::Context, req: &super::shim::ResizePtyRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "ResizePty", cres);
        Ok(cres)
    }

    pub fn close_io(&self, ctx: ttrpc::context::Context, req: &super::shim::CloseIORequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "CloseIO", cres);
        Ok(cres)
    }

    pub fn update(&self, ctx: ttrpc::context::Context, req: &super::shim::UpdateTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Update", cres);
        Ok(cres)
    }

    pub fn wait(&self, ctx: ttrpc::context::Context, req: &super::shim::WaitRequest) -> ::ttrpc::Result<super::shim::WaitResponse> {
        let mut cres = super::shim::WaitResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Wait", cres);
        Ok(cres)
    }

    pub fn stats(&self, ctx: ttrpc::context::Context, req: &super::shim::StatsRequest) -> ::ttrpc::Result<super::shim::StatsResponse> {
        let mut cres = super::shim::StatsResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Stats", cres);
        Ok(cres)
    }

    pub fn connect(&self, ctx: ttrpc::context::Context, req: &super::shim::ConnectRequest) -> ::ttrpc::Result<super::shim::ConnectResponse> {
        let mut cres = super::shim::ConnectResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Connect", cres);
        Ok(cres)
    }

    pub fn shutdown(&self, ctx: ttrpc::context::Context, req: &super::shim::ShutdownRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.task.v2.Task", "Shutdown", cres);
        Ok(cres)
    }
}

struct StateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for StateMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, StateRequest, state);
        Ok(())
    }
}

struct CreateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CreateMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, CreateTaskRequest, create);
        Ok(())
    }
}

struct StartMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for StartMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, StartRequest, start);
        Ok(())
    }
}

struct DeleteMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for DeleteMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, DeleteRequest, delete);
        Ok(())
    }
}

struct PidsMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for PidsMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, PidsRequest, pids);
        Ok(())
    }
}

struct PauseMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for PauseMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, PauseRequest, pause);
        Ok(())
    }
}

struct ResumeMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ResumeMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, ResumeRequest, resume);
        Ok(())
    }
}

struct CheckpointMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CheckpointMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, CheckpointTaskRequest, checkpoint);
        Ok(())
    }
}

struct KillMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for KillMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, KillRequest, kill);
        Ok(())
    }
}

struct ExecMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ExecMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, ExecProcessRequest, exec);
        Ok(())
    }
}

struct ResizePtyMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ResizePtyMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, ResizePtyRequest, resize_pty);
        Ok(())
    }
}

struct CloseIoMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CloseIoMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, CloseIORequest, close_io);
        Ok(())
    }
}

struct UpdateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for UpdateMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, UpdateTaskRequest, update);
        Ok(())
    }
}

struct WaitMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for WaitMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, WaitRequest, wait);
        Ok(())
    }
}

struct StatsMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for StatsMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, StatsRequest, stats);
        Ok(())
    }
}

struct ConnectMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ConnectMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, ConnectRequest, connect);
        Ok(())
    }
}

struct ShutdownMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ShutdownMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, shim, ShutdownRequest, shutdown);
        Ok(())
    }
}

pub trait Task {
    fn state(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::StateRequest) -> ::ttrpc::Result<super::shim::StateResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/State is not supported".to_string())))
    }
    fn create(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::CreateTaskRequest) -> ::ttrpc::Result<super::shim::CreateTaskResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Create is not supported".to_string())))
    }
    fn start(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::StartRequest) -> ::ttrpc::Result<super::shim::StartResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Start is not supported".to_string())))
    }
    fn delete(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::DeleteRequest) -> ::ttrpc::Result<super::shim::DeleteResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Delete is not supported".to_string())))
    }
    fn pids(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::PidsRequest) -> ::ttrpc::Result<super::shim::PidsResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Pids is not supported".to_string())))
    }
    fn pause(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::PauseRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Pause is not supported".to_string())))
    }
    fn resume(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::ResumeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Resume is not supported".to_string())))
    }
    fn checkpoint(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::CheckpointTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Checkpoint is not supported".to_string())))
    }
    fn kill(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::KillRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Kill is not supported".to_string())))
    }
    fn exec(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Exec is not supported".to_string())))
    }
    fn resize_pty(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::ResizePtyRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/ResizePty is not supported".to_string())))
    }
    fn close_io(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::CloseIORequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/CloseIO is not supported".to_string())))
    }
    fn update(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::UpdateTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Update is not supported".to_string())))
    }
    fn wait(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::WaitRequest) -> ::ttrpc::Result<super::shim::WaitResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Wait is not supported".to_string())))
    }
    fn stats(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::StatsRequest) -> ::ttrpc::Result<super::shim::StatsResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Stats is not supported".to_string())))
    }
    fn connect(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::ConnectRequest) -> ::ttrpc::Result<super::shim::ConnectResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Connect is not supported".to_string())))
    }
    fn shutdown(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::shim::ShutdownRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Shutdown is not supported".to_string())))
    }
}

pub fn create_task(service: Arc<std::boxed::Box<dyn Task + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/containerd.task.v2.Task/State".to_string(),
                    std::boxed::Box::new(StateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Create".to_string(),
                    std::boxed::Box::new(CreateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Start".to_string(),
                    std::boxed::Box::new(StartMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Delete".to_string(),
                    std::boxed::Box::new(DeleteMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Pids".to_string(),
                    std::boxed::Box::new(PidsMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Pause".to_string(),
                    std::boxed::Box::new(PauseMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Resume".to_string(),
                    std::boxed::Box::new(ResumeMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Checkpoint".to_string(),
                    std::boxed::Box::new(CheckpointMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Kill".to_string(),
                    std::boxed::Box::new(KillMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Exec".to_string(),
                    std::boxed::Box::new(ExecMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/ResizePty".to_string(),
                    std::boxed::Box::new(ResizePtyMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/CloseIO".to_string(),
                    std::boxed::Box::new(CloseIoMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Update".to_string(),
                    std::boxed::Box::new(UpdateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Wait".to_string(),
                    std::boxed::Box::new(WaitMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Stats".to_string(),
                    std::boxed::Box::new(StatsMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Connect".to_string(),
                    std::boxed::Box::new(ConnectMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Shutdown".to_string(),
                    std::boxed::Box::new(ShutdownMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods
}
