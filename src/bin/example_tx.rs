extern crate goose_packet;

use pnet::datalink::{self,interfaces,Channel, NetworkInterface};
use goose_packet::types::{IECGoosePdu,EthernetHeader,IECData};
use goose_packet::pdu::{encodeGooseFrame,getTimeMs,display_buffer};

use std::env;

const GOOSE_BUFFER_SIZE:usize = 512;

fn display_network_interfaces(){
    let interfaces = interfaces();
    for interface in interfaces.iter() {
        println!("interface  {}", interface.index);
        println!("\t name {}", interface.name);
        println!("\t ips {:?}", interface.ips);
        println!("\t description {}", interface.description);

    }
}

fn main(){
    let interface_name = match env::args().nth(1){
        Some (name)=>name,
        None=>{
            println!("please add an interface name as argument. the available interface in the system:");
            display_network_interfaces();
            panic!();
        }
    };

    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;
    let interfaces = interfaces();
    // Find the network interface with the provided name
    let interface = match interfaces.into_iter()
                              .filter(interface_names_match)
                              .next() {
        Some(val)=>val,
        _=>{
            println!("unknown interface name. the available interface in the system:");
            display_network_interfaces();
            panic!();           
        }

    };

    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ether_header= EthernetHeader{
        srcAddr:[00 as u8;6],
        dstAddr:[0x01,0x0C,0xCD,0x01,0x00,0x01],
        TPID:[0x81,0x00],
        TCI:[0x80,0x01], 
        ehterType:[0x88,0xB8],
        APPID:[0x01,0x01],
        length:[0x00,0x00]
    };
    let current_time=getTimeMs();
    let mut goose_pdu= IECGoosePdu{
        gocbRef:"testGoose".to_string(),
        timeAllowedtoLive:6400,
        datSet:"test_datSet".to_string(),
        goID:"test_ID".to_string(),
        t:current_time,
        stNum:12,
        sqNum:23,
        simulation:false,
        confRev:5,
        ndsCom:false,
        numDatSetEntries:1,
        allData:
            vec![
            IECData::int8(2),
            IECData::int32(234),
            IECData::int64(234567890),
            IECData::array(
                vec![
                    IECData::int8(-2),
                    IECData::int32(-234),
                    IECData::int64(-234567890),
                    ]),
            IECData::structure(
                vec![
                    IECData::int32u(4294967295),
                    IECData::float32(0.123),
                    IECData::octet_string(vec![0x22,0x33,0x66]),
                    IECData::utc_time(current_time)
                    ]),            
            IECData::boolean(true),
            IECData::boolean(false),
            IECData::visible_string("abc234".to_string()),
            IECData::mms_string("hÃllo".to_string()),
            IECData::bit_string{padding:3,val:vec![0x00,0x01]}
            ]
        
        };

    goose_pdu.numDatSetEntries=goose_pdu.allData.len() as u32;    
    let mut buffer=[0 as u8;GOOSE_BUFFER_SIZE];
    let goose_frame_size=encodeGooseFrame(&mut ether_header,&goose_pdu,&mut buffer,0);

    display_buffer(&buffer,goose_frame_size);

    //tx.build_and_send(1, goose_frame_size, &mut |packet: &mut [u8]| {
    //    packet.copy_from_slice(&buffer[..goose_frame_size]);
    //});

    tx.send_to(&buffer[..goose_frame_size],None);
    
}