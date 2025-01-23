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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/*");
    // Journal Engine
    tonic_build::configure().build_server(true).compile_protos(
        &[
            "src/journal_server/proto/admin.proto",
            "src/journal_server/proto/engine.proto",
            "src/journal_server/proto/inner.proto",
            "src/journal_server/proto/record.proto",
        ],
        &["src/journal_server/proto/"], // specify the root location to search proto dependencies
    )?;

    // MQTT Broker
    tonic_build::configure().build_server(true).compile_protos(
        &[
            "src/broker_mqtt/proto/admin.proto",
            "src/broker_mqtt/proto/inner.proto",
        ],
        &["src/broker_mqtt/proto"], // specify the root location to search proto dependencies
    )?;

    // Placement Center
    let config = {
        let mut c = prost_build::Config::new();
        c.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
        c.service_generator(tonic_build::configure().service_generator());
        c
    };
    prost_validate_build::Builder::new().compile_protos_with_config(
        config,
        &[
            "src/placement_center/proto/journal.proto",
            "src/placement_center/proto/kv.proto",
            "src/placement_center/proto/mqtt.proto",
            "src/placement_center/proto/inner.proto",
            "src/placement_center/proto/openraft.proto",
        ],
        &[
            "src/placement_center/proto",
            "src/prost_validation_types/proto",
        ], // specify the root location to search proto dependencies
    )?;
    Ok(())
}
