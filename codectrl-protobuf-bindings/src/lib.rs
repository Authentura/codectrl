/// This is the module to emulate the structure of the protobuf file imports to
/// prevent any build issues.

pub mod data {
    pub mod backtrace_data {
        tonic::include_proto!("codectrl.data.backtrace_data");
    }
    pub mod log {
        tonic::include_proto!("codectrl.data.log");
    }

    pub use backtrace_data::*;
    pub use log::*;
}

pub mod logs_service {
    tonic::include_proto!("codectrl.logs_service");

    pub use log_server_server::{
        LogServer as LogServerTrait, LogServerServer as LogServerService,
    };

    pub use log_client_server::{
        LogClient as LogClientTrait, LogClientServer as LogClientService,
    };
}
