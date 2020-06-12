/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub(super) fn to_full_path<S>(cmd: S) -> String
where
    S: Into<String>,
{
    use std::path::Path;
    use std::path::Component;

    let cmd = cmd.into();
    let path = Path::new(&cmd);

    let mut components = path.components();
    let is_absolute = components.next() == Some(Component::RootDir);

    if !is_absolute {
        return cmd;
    }

    let parent = match components.next() {
        Some(Component::Normal(path)) => path.to_str().unwrap(),
        _ => return cmd,
    };

    match std::env::var(parent) {
        Ok(to_dir) => {
            let mut full_path = std::path::PathBuf::from(to_dir);

            // TODO: optimize this
            while let Some(component) = components.next() {
                full_path.push(component);
            }
            full_path.to_string_lossy().into_owned()
        }
        Err(_) => cmd,
    }
}
