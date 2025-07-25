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

use super::type_config::TypeConfig;
use crate::{
    controller::{journal::StorageEngineController, mqtt::MqttController},
    core::cache::CacheManager,
    raft::route::apply::StorageDriver,
};
use grpc_clients::pool::ClientPool;
use openraft::Raft;
use rocksdb_engine::RocksDBEngine;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use tracing::{error, info};

pub fn monitoring_leader_transition(
    raft: &Raft<TypeConfig>,
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    cache_manager: Arc<CacheManager>,
    client_pool: Arc<ClientPool>,
    raft_machine_apply: Arc<StorageDriver>,
) {
    let mut metrics_rx = raft.metrics();
    let (stop_send, _) = broadcast::channel::<bool>(2);
    let mut controller_running = false;
    tokio::spawn(async move {
        let mut last_leader: Option<u64> = None;
        loop {
            match metrics_rx.changed().await {
                Ok(_) => {
                    let mm = metrics_rx.borrow().clone();

                    if let Some(current_leader) = mm.current_leader {
                        if last_leader != Some(current_leader) {
                            if mm.id == current_leader {
                                info!(
                                    "Leader transition has occurred. current leader is Node {:?}. Previous leader was Node {:?}.",
                                    mm.current_leader, last_leader
                                );
                                start_controller(
                                    &rocksdb_engine_handler,
                                    &cache_manager,
                                    &client_pool,
                                    &raft_machine_apply,
                                    stop_send.clone(),
                                );
                                controller_running = true;
                            } else if controller_running {
                                stop_controller(stop_send.clone());
                                controller_running = false
                            }
                            last_leader = Some(current_leader);
                        }
                    }
                }
                Err(changed_err) => {
                    error!(
                    "Error while watching metrics_rx: {}; quitting monitoring_leader_transition() loop",
                    changed_err);
                }
            }
        }
    });
}

pub fn start_controller(
    rocksdb_engine_handler: &Arc<RocksDBEngine>,
    cache_manager: &Arc<CacheManager>,
    client_pool: &Arc<ClientPool>,
    raft_machine_apply: &Arc<StorageDriver>,
    stop_send: Sender<bool>,
) {
    let mqtt_controller = MqttController::new(
        rocksdb_engine_handler.clone(),
        cache_manager.clone(),
        client_pool.clone(),
        stop_send.clone(),
    );
    tokio::spawn(async move {
        mqtt_controller.start().await;
    });

    let journal_controller = StorageEngineController::new(
        raft_machine_apply.clone(),
        cache_manager.clone(),
        client_pool.clone(),
    );
    tokio::spawn(async move {
        journal_controller.start().await;
    });
}

pub fn stop_controller(stop_send: Sender<bool>) {
    if let Err(e) = stop_send.send(true) {
        error!(
            "Failed to send stop signal, Failure to stop controller,Error message:{}",
            e
        );
    }
}
