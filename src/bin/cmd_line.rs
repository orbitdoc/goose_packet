extern crate goose_packet;

use pnet::datalink::{self,interfaces,Channel, NetworkInterface, MacAddr};
//use chrono::prelude::*;
use goose_packet::pdu::{IECGoosePdu,EthernetHeader,IECData,encodeEthernetHeader,encodeIECGoosePdu,getTimeMs};
//use goose_packet::ber::{encode_boolean};
fn display_buffer( buffer: &[u8], size:usize){
    //print!("[");
    for i in 0..std::cmp::min(buffer.len(),size){
        if (i)%8==0 {
            print!("{:06x} ",i);
        }
        print!("{:02x} ",buffer[i]);
        if (i+1)%8==0 {
            print!("\n");
        }
    }
    //println!("]");
    print!("\n");
}
fn build_packet( in_packet: & [u8], out_packet: &mut [u8], length: usize){
    out_packet.copy_from_slice(&in_packet[..length]);
    display_buffer(out_packet,length);
}
fn main(){
    let mut ether_header= EthernetHeader{
        srcAddr:[00 as u8;6],
        dstAddr:[0x00, 0x02,0x03,0x04,0x00,0x06],
        TPID:[0x81,0x00],
        TCI:[0x80,0x01], //?
        ehterType:[0x88,0xB8],
        APPID:[0x01,0x01],//?
        length:[0x00,0x00]
    };

    let mut goose= IECGoosePdu{
        gocbRef:"testGoose".to_string(),
        timeAllowedtoLive:6400,
        datSet:"test_datSet".to_string(),
        goID:"test_ID".to_string(),
        t:getTimeMs(),
        stNum:12,
        sqNum:23,
        simulation:false,
        confRev:5,
        ndsCom:false,
        numDatSetEntries:1,
        allData:
            vec![
            IECData::integer(2),
            IECData::integer(234),
            IECData::integer(234567890),
            IECData::array(
                vec![
                    IECData::integer(-2),
                    IECData::integer(-234),
                    IECData::integer(-234567890),
                    ]),
            IECData::structure(
                vec![
                    IECData::unsigned(456),
                    IECData::float(0.123),
                    IECData::octet_string(getTimeMs().to_vec()),
                    IECData::utc_time(getTimeMs())
                    ]),            
            IECData::boolean(true),
            IECData::boolean(false),
            IECData::visible_string("abc234".to_string()),
            IECData::mms_string("hÃllo".to_string()),
            IECData::bit_string{padding:3,val:1}
            ]
        
        };


    let mut buffer=[0 as u8;256];
    let mut pos=26;
    pos=encodeIECGoosePdu(&goose,& mut buffer,pos);
    let goose_length=pos-26+8;
    let legnth_byte=goose_length.to_be_bytes();
    ether_header.length.copy_from_slice(&legnth_byte[6..]);
    encodeEthernetHeader(&ether_header,& mut buffer,0);

    let interfaces = interfaces();
    //for interface in interfaces.iter() {
    //    println!("{}", interface);
    //}
    
    let interface_name = 19;
    let interface_names_match =
        |iface: &NetworkInterface| iface.index == interface_name;

    // Find the network interface with the provided name
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();


    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };



        tx.build_and_send(1, pos, &mut |packet: &mut [u8]| {
            build_packet(&buffer, packet,pos);
        });
    
}