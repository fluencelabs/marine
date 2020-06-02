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

pub struct SlicePrettyPrinter<'a>(pub &'a [u8]);

impl<'a> std::fmt::LowerHex for SlicePrettyPrinter<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmtr.write_fmt(format_args!("0x"))?;
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::UpperHex for SlicePrettyPrinter<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmtr.write_fmt(format_args!("0x"))?;
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02X}", byte))?;
        }
        Ok(())
    }
}
