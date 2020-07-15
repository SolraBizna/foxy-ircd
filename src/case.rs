/*
 * This file is part of Foxy IRCd, copyright ©2020 Solra Bizna.
 *
 * Foxy IRCd is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * Foxy IRCd is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License along with
 * Foxy IRCd. If not, see <https://www.gnu.org/licenses/>.
 */

/// Upcase a byte.
///
/// Note: We use the "ascii" case mapping.
pub fn upcase(b: u8) -> u8 {
    if b >= b'a' && b <= b'z' { b & !0x20 }
    else { b }
}

/// Downcase a byte.
///
/// Note: We use the "ascii" case mapping.
pub fn downcase(b: u8) -> u8 {
    if b >= b'A' && b <= b'Z' { b | 0x20 }
    else { b }
}

