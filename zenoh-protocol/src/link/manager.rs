//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::sync::Arc;

#[cfg(feature = "tcp")]
use crate::link::tcp::ManagerTcp;
#[cfg(feature = "udp")]
use crate::link::udp::ManagerUdp;
use crate::link::{LinkManager, LocatorProtocol};
use crate::session::SessionManagerInner;

pub struct LinkManagerBuilder;

impl LinkManagerBuilder {
    pub(crate) fn make(
        manager: Arc<SessionManagerInner>,
        protocol: &LocatorProtocol,
    ) -> LinkManager {
        match protocol {
            #[cfg(feature = "tcp")]
            LocatorProtocol::Tcp => Arc::new(ManagerTcp::new(manager)),
            #[cfg(feature = "udp")]
            LocatorProtocol::Udp => Arc::new(ManagerUdp::new(manager)),
        }
    }
}
