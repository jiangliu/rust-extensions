/*
   Copyright The containerd Authors.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use containerd_shim as shim;

use log::info;
use shim::{api, TtrpcContext, TtrpcResult};

struct Service {
    exit: shim::ExitSignal,
}

impl shim::Shim for Service {
    type Error = shim::Error;

    fn new(
        _id: &str,
        _namespace: &str,
        _publisher: shim::RemotePublisher,
        _config: &mut shim::Config,
        exit: shim::ExitSignal,
    ) -> Self {
        Service { exit }
    }

    fn start_shim(&mut self, opts: shim::StartOpts) -> Result<String, shim::Error> {
        let address = shim::spawn(opts)?;
        Ok(address)
    }
}

impl shim::Task for Service {
    fn connect(
        &self,
        _ctx: &TtrpcContext,
        _req: api::ConnectRequest,
    ) -> TtrpcResult<api::ConnectResponse> {
        info!("Connect request");
        Ok(api::ConnectResponse {
            version: String::from("example"),
            ..Default::default()
        })
    }

    fn shutdown(&self, _ctx: &TtrpcContext, _req: api::ShutdownRequest) -> TtrpcResult<api::Empty> {
        info!("Shutdown request");
        self.exit.signal();
        Ok(api::Empty::default())
    }
}

fn main() {
    shim::run::<Service>("io.containerd.empty.v1")
}
