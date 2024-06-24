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

#[macro_export]
macro_rules! call_faas {
    ($faas:expr, $module_name:expr, $func_name:expr, $args:expr) => {
        $faas
            .call_with_json_async($module_name, $func_name, $args, <_>::default())
            .await
            .unwrap_or_else(|e| panic!("faas failed with {:?}", e))
    };
}
