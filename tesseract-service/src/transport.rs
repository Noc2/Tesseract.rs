//===------------ transport.rs --------------------------------------------===//
//  Copyright 2021, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait TransportProcessor {
    //for the sake of simplicity at the current stage
    //transports are limited to OneToOne Request/Response operation
    async fn process(self: Arc<Self>, data: &[u8]) -> Vec<u8>;
}

pub trait BoundTransport {}

pub trait Transport {
    fn bind(self, processor: Arc<dyn TransportProcessor + Send + Sync>) -> Box<dyn BoundTransport>;
}
