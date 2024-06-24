/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
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
            #[allow(clippy::while_let_on_iterator)]
            while let Some(component) = components.next() {
                full_path.push(component);
            }
            full_path.to_string_lossy().into_owned()
        }
        Err(_) => cmd,
    }
}
