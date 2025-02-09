/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of Stalwart Mail Server.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use ahash::AHashMap;

use crate::{DirectoryOptions, LookupList, Principal};

pub mod config;
pub mod lookup;

#[derive(Default, Debug)]
pub struct MemoryDirectory {
    principals: AHashMap<String, Principal>,
    emails_to_names: AHashMap<String, Vec<EmailType>>,
    names_to_email: AHashMap<String, Vec<EmailType>>,
    domains: LookupList,
    opt: DirectoryOptions,
}

#[derive(Debug)]
enum EmailType {
    Primary(String),
    Alias(String),
    List(String),
}
