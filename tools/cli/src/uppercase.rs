/*
 * Copyright 2021 Fluence Labs Limited
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

/// Convert first letter of the given string to uppercase
pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use crate::uppercase::uppercase_first_letter;

    #[test]
    fn uppercase() {
        let s = String::from("hello, world!");
        let result = uppercase_first_letter(s);
        assert_eq!(s, "Hello, world!");
    }

    #[test]
    fn identity() {
        let s = String::from("Hello, world!");
        let result = uppercase_first_letter(s);
        assert_eq!(s, "Hello, world!");
    }
}
