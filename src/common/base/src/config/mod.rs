// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use broker_mqtt::BrokerMQTTConfig;
use std::fs;
use std::path;
use toml;

pub mod broker_mqtt;
pub mod common;
pub mod default_mqtt;
pub mod journal_server;
pub mod placement_center;

pub const DEFAULT_MQTT_SERVER_CONFIG: &str = "config/mqtt-server.toml";
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";
pub const DEFAULT_JOURNAL_SERVER_CONFIG: &str = "config/journal-server.toml";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
