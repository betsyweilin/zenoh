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
use super::ZInt;

// Properties accepted in open()
pub const ZN_USER_KEY: ZInt = 0x50;
pub const ZN_PASSWD_KEY: ZInt = 0x51;

// Properties returned by info()
pub const ZN_INFO_PID_KEY: ZInt      = 0x00;
pub const ZN_INFO_PEER_KEY: ZInt     = 0x01;
pub const ZN_INFO_PEER_PID_KEY: ZInt = 0x02;