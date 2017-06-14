/*
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate t2plugin;
extern crate libc;
#[macro_use]
extern crate lazy_static;

use std::str;

use t2plugin::{T2Plugin, Header, BinaryType, output_string, output_num, output_nums};
use t2plugin::nethdr::{Packet, Flow, L4Type};
use t2plugin::slread::{SliceReader, TrimBytes};


//  ------------  Plugin per flow structure  ------------

struct RustTemplate {
    // example variables
    int1: u64,
    float2: f64,
    strings: Vec<String>,
}


//  ------------  Plugin interface implementation  ------------

impl T2Plugin for RustTemplate {
    fn new() -> RustTemplate {
        RustTemplate {
            int1: 0,
            float2: 0f64,
            strings: Vec::new(),
        }
    }

    fn get_dependencies() -> Vec<&'static str> {
        // this method can be deleted if the plugin does not have dependencies
        vec!["basicFlow", "httpSniffer"]
    }

    fn initialize() {
        // this method can be deleted if the plugin does not need to initialize variables on
        // startup
        println!("Initializing RustTemplate plugin");
    }

    fn print_header() -> Header {
        let mut header = Header::new();

        // 1st column with a repetitive u64 value
        header.add_simple_col("Example column 1", "example1", true, BinaryType::bt_uint_64);

        // 2nd column with a compound value (u16, string)
        header.add_compound_col("Example column 2", "example2", false, 
                                &[BinaryType::bt_uint_16, BinaryType::bt_string]);

        header
    }

    #[allow(unused_variables)]
    fn on_flow_generated(&mut self, packet: &Packet, flow: &mut Flow) {
        // this method can be deleted if empty

        self.float2 = packet.timestamp();
    }

    #[allow(unused_variables)]
    fn claim_l2_info(packet: &Packet, plugin: Option<&mut RustTemplate>, flow: Option<&mut Flow>) {
        // this method can be deleted if empty
        
        if let Some(plugin) = plugin {
            // do layer 2 flow related processing here
        }
    }

    #[allow(unused_variables)]
    fn claim_l3_info(packet: &Packet) {
        // this method can be deleted if empty
    }

    fn claim_l4_info(&mut self, packet: &Packet, flow: &mut Flow) {
        // this method can be deleted if empty

        self.int1 += packet.packet_len as u64;
    }

    #[allow(unused_variables)]
    fn on_flow_terminate(&mut self, flow: &mut Flow) {
        // this method can be deleted if empty

        // 1st column: output repetitive u64
        output_nums(...);

        // 2nd column: output compound (u16, string)
        output_num(... as u16);
        output_string(...);
    }

    fn on_application_terminate() {
        // this method can be deleted if empty
    }
}

t2plugin!(RustTemplate);
