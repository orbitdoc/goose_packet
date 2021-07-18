#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::types::{*};

use crate::pdu_encoder::{*};
use crate::pdu_decoder::{*};

use std::time::{SystemTime, UNIX_EPOCH};


pub fn encodeGooseFrame(header: & mut EthernetHeader, pdu: & IECGoosePdu, buffer: &mut[u8], pos:usize) ->usize{
    let mut new_pos=pos+ 26;
    new_pos=encodeIECGoosePdu(pdu,buffer,new_pos);
    let goose_length=new_pos-26+8;
    let legnth_byte=goose_length.to_be_bytes();
    header.length.copy_from_slice(&legnth_byte[6..]);
    encodeEthernetHeader(header,buffer,pos);
    //display_buffer(&buffer[pos..],new_pos);
    new_pos
}
pub fn encodeEthernetHeader(header: & EthernetHeader, buffer: &mut[u8], pos:usize) ->usize{

    let mut new_pos=pos;

    buffer[new_pos..new_pos+6].copy_from_slice(&header.dstAddr);
    new_pos=new_pos+6;

    buffer[new_pos..new_pos+6].copy_from_slice(&header.srcAddr);
    new_pos=new_pos+6;

    buffer[new_pos..new_pos+2].copy_from_slice(&header.TPID);
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&header.TCI);
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&header.ehterType);
    new_pos=new_pos+2;


    // Start of GOOSE length
    buffer[new_pos..new_pos+2].copy_from_slice(&header.APPID);
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&header.length);
    new_pos=new_pos+2;
    //println!("length {:?}",header.length);

    buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 1
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 2
    new_pos=new_pos+2;

    new_pos

}



pub fn getTimeMs()->[u8;8]{
    let mut time_array=[0 as u8;8];
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    //println!("{:?}", since_the_epoch);
    let seconds= since_the_epoch.as_secs() as u32;
    let sec_array=seconds.to_be_bytes();
    let subsec_nano=(since_the_epoch.subsec_micros() as f32 * 4294.967296) as u32;
    let nano_array=subsec_nano.to_be_bytes();
    time_array[0..4].copy_from_slice(&sec_array);
    time_array[4..7].copy_from_slice(&nano_array[..3]);
    time_array[7]=0x18;
    time_array
}

pub fn display_buffer( buffer: &[u8], size:usize){
    for i in 0..std::cmp::min(buffer.len(),size){
        if (i)%8==0 {
            print!("{:06x} ",i);
        }
        print!("{:02x} ",buffer[i]);
        if (i+1)%8==0 {
            print!("\n");
        }
    }
    print!("\n");
}

pub fn decodeGooseFrame(header: & mut EthernetHeader, pdu: &  mut IECGoosePdu, buffer: &[u8], pos:usize) ->usize{
    let mut new_pos=pos;
    new_pos=decodeEthernetHeader(header,buffer,new_pos);
    if new_pos==0 { 
        return new_pos;
    }
    new_pos=decodeIECGoosePdu(pdu,buffer,new_pos);
    new_pos
}

pub fn decodeEthernetHeader(header: & mut EthernetHeader, buffer: &[u8], pos:usize) ->usize{

    let mut new_pos=pos;

    header.dstAddr.copy_from_slice(&buffer[new_pos..new_pos+6]);
    new_pos=new_pos+6;

    header.srcAddr.copy_from_slice(&buffer[new_pos..new_pos+6]);
    new_pos=new_pos+6;

    header.TPID.copy_from_slice(&buffer[new_pos..new_pos+2]);

    if header.TPID ==[0x81,0x00]{ //if vlan
        new_pos=new_pos+2;

        header.TCI.copy_from_slice(&buffer[new_pos..new_pos+2]);
        new_pos=new_pos+2;
        //https://github.com/libpnet/libpnet/issues/460
        //println!("vlan stripped");
    }


    header.ehterType.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;
    if header.ehterType !=[0x88,0xb8]
    {
        return pos;
    }
    header.APPID.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;

    header.length.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;
    //println!("decode header {:?}",header);

    new_pos=new_pos+2;  // reserved 1
    
    new_pos=new_pos+2; // reserved 2

    new_pos

}