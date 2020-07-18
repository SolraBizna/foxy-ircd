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

pub mod message;
pub use message::Message;
pub mod db;
pub use db::*;
pub mod case;
pub use case::*;
pub mod invocation;
pub use invocation::*;
pub mod connection;
pub use connection::*;

fn main() {
    let Invocation { mut runtime }
    = match get_invocation(|x| println!("{}", x.peer_addr().unwrap())) {
        Some(x) => x,
        None => std::process::exit(1),
    };
    let (mut send_quit, mut recv_quit) = tokio::sync::mpsc::channel(1);
    ctrlc::set_handler(move || {
        let _ = send_quit.try_send("control-C");
    }).unwrap();
    let reason = runtime.block_on(async {
        recv_quit.recv().await.unwrap()
    });
    eprintln!("\nShutting down server due to {}.", reason);
    // Try to be patient and let ongoing tasks finish, but don't block for more
    // than 15 seconds.
    runtime.shutdown_timeout(std::time::Duration::new(15, 0));
}
