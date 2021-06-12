#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::encoder::{*};
use crate::decoder::{*};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum IECData{
    array(Vec<IECData>),
    structure(Vec<IECData>),
    boolean(bool),

    int8(i8),
    int16(i16),
    int32(i32),
    int64(i64),

    int8u(u8),
    int16u(u16),
    int32u(u32),

    float32(f32),
    float64(f64),

    visible_string(String),
    mms_string(String),
    bit_string{ padding: u8, val: u16 },
    octet_string(Vec<u8>),
    utc_time([u8;8])
}
#[derive(Debug,Default)]
pub struct EthernetHeader {
    pub srcAddr:[u8;6],
    pub dstAddr:[u8;6],
    pub TPID:[u8;2],
    pub TCI:[u8;2],
    pub ehterType:[u8;2],
    pub APPID:[u8;2],
    pub length:[u8;2]
}

#[derive(Debug,Default)]
pub struct IECGoosePdu {
    pub gocbRef: String,
    pub timeAllowedtoLive: u32,
    pub datSet: String,
    pub goID: String,
    pub t: [u8;8],
    pub stNum: u32,
    pub sqNum: u32,
    pub simulation: bool,
    pub confRev: u32,
    pub ndsCom: bool,
    pub numDatSetEntries: u32,
    pub allData: Vec<IECData>
}

impl IECGoosePdu {
    /// Simulate a fern growing for one day.
    pub fn report(&mut self) {
        println!("gocbRef:{},data:{:?}",self.gocbRef,self.allData);
    }
}
pub fn encodeGooseFrame(header: & mut EthernetHeader, pdu: & IECGoosePdu, buffer: &mut[u8], pos:usize) ->usize{
    let mut new_pos=pos+ 26;
    new_pos=encodeIECGoosePdu(pdu,buffer,new_pos);
    let goose_length=new_pos-26+8;
    let legnth_byte=goose_length.to_be_bytes();
    header.length.copy_from_slice(&legnth_byte[6..]);
    encodeEthernetHeader(header,buffer,0);
    display_buffer(&buffer[pos..],new_pos);
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

    buffer[new_pos..new_pos+2].copy_from_slice(&header.APPID);
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&header.length);
    new_pos=new_pos+2;
    println!("length {:?}",header.length);

    buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 1
    new_pos=new_pos+2;

    buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 2
    new_pos=new_pos+2;

    new_pos

}

pub fn encodeIECGoosePdu(pdu: & IECGoosePdu, buffer: &mut[u8], pos:usize) ->usize{

    let (goosePduLength,dataSetSize)=sizeIECGoosePdu(pdu,buffer);
    fillIECGoosePdu(pdu, buffer, pos, goosePduLength, dataSetSize)
}

pub fn sizeIECGoosePdu(pdu: & IECGoosePdu, buffer: &mut[u8]) ->(usize,usize){

    let mut goosePduLength=0;
    
    let mut size;
    let fill= false;

    size=encode_string(0x80, &pdu.gocbRef, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_unsigned(0x81, pdu.timeAllowedtoLive, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_string(0x82, &pdu.datSet, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_string(0x83, &pdu.goID, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_octet_string(0x84, &pdu.t, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_unsigned(0x85, pdu.stNum, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_unsigned(0x86, pdu.sqNum, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_boolean(0x87, pdu.simulation, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_unsigned(0x88, pdu.confRev, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_boolean(0x89, pdu.ndsCom, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    size=encode_unsigned(0x8a, pdu.numDatSetEntries, buffer, 0, fill);
    goosePduLength+=1+size_length(size)+size;

    let dataSetSize=sizeIECData(&pdu,buffer);
    goosePduLength+=1+size_length(dataSetSize)+dataSetSize;

    (goosePduLength,dataSetSize)

}

pub fn fillIECGoosePdu(pdu: & IECGoosePdu, buffer: &mut[u8], pos:usize, goosePduLength: usize, dataSetSize: usize) ->usize{

    let mut new_pos=pos;
    let fill= true;

    new_pos=encode_tag_length(0x61,goosePduLength,buffer,new_pos,fill );

    new_pos=encode_string(0x80, &pdu.gocbRef, buffer, new_pos, fill);

    new_pos=encode_unsigned(0x81, pdu.timeAllowedtoLive, buffer, new_pos, fill);

    new_pos=encode_string(0x82, &pdu.datSet, buffer, new_pos, fill);
    
    new_pos=encode_string(0x83, &pdu.goID, buffer, new_pos, fill);

    new_pos=encode_octet_string(0x84, &pdu.t, buffer, new_pos, fill);

    new_pos=encode_unsigned(0x85, pdu.stNum, buffer, new_pos, fill);

    new_pos=encode_unsigned(0x86, pdu.sqNum, buffer, new_pos, fill);

    new_pos=encode_boolean(0x87, pdu.simulation, buffer, new_pos, fill);

    new_pos=encode_unsigned(0x88, pdu.confRev, buffer, new_pos, fill);

    new_pos=encode_boolean(0x89, pdu.ndsCom, buffer, new_pos, fill);

    new_pos=encode_unsigned(0x8a, pdu.numDatSetEntries, buffer, new_pos, fill);

    new_pos=encode_tag_length(0xab,dataSetSize,buffer,new_pos,fill );

    new_pos=encodeIECData(pdu, buffer, new_pos);

    new_pos

}


pub fn sizeIECData(pdu: & IECGoosePdu, buffer: &mut[u8]) ->usize{

    let mut dataSetSize=0;

    for i in 0..pdu.allData.len() {
        dataSetSize+=sizeIECDataElement(&pdu.allData[i], buffer);
    }
    //println!("dataSetSize {}",dataSetSize);
    dataSetSize


}

pub fn sizeIECDataElement(data: & IECData, buffer: &mut[u8]) ->usize{
    
    let fill= false;

    let dataSetSize=match  data{
        IECData::boolean (val)=> encode_boolean(0, *val, buffer, 0, fill),

        IECData::int8 (val)=> encode_interger_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::int16 (val)=> encode_interger_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::int32 (val)=> encode_interger_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::int64 (val)=> encode_interger_general(0, &val.to_be_bytes(), buffer, 0, fill),
  
        IECData::int8u (val)=> encode_unsigned_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::int16u (val)=> encode_unsigned_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::int32u (val)=> encode_unsigned_general(0, &val.to_be_bytes(), buffer, 0, fill),

        IECData::float32 (val)=> encode_float_general(0, &val.to_be_bytes(), buffer, 0, fill),
        IECData::float64 (val)=> encode_float_general(0, &val.to_be_bytes(), buffer, 0, fill),

        IECData::visible_string (val)=> encode_string(0, val, buffer, 0, fill),
        IECData::mms_string (val)=> encode_string(0, val, buffer, 0, fill),
        IECData::bit_string{padding,val}=>encode_bit_string(0, *val, *padding,buffer, 0, fill),
        IECData::array (val)=>encode_array(0,&val,buffer,0,fill),
        IECData::structure (val)=>encode_structure(0,&val,buffer,0,fill),
        IECData::octet_string (val)=>encode_octet_string(0,&val,buffer,0,fill),
        IECData::utc_time (val)=>encode_octet_string(0,val,buffer,0,fill),
        _=>{println!("unkowntype in sizeIECDataElement");0}
    };
    //println!("length {},dataSetSize {}",size_length(dataSetSize),dataSetSize);

    1+size_length(dataSetSize)+dataSetSize

}
pub fn encodeIECData(pdu: & IECGoosePdu, buffer: &mut[u8], pos:usize) ->usize{

    let mut new_pos=pos;

    for i in 0..pdu.allData.len() {
        new_pos=encodeIECDataElement(&pdu.allData[i], buffer, new_pos);
    }

    new_pos

}

pub fn encode_array(tag:u8, value: &[IECData],buffer: &mut[u8],pos:usize,fill:bool) ->usize{

    let mut element_size=0;
    for i in 0..value.len(){
        element_size+=sizeIECDataElement(&value[i],buffer);
    }
    
    if !fill {
        return element_size;
    }
    let mut new_pos=pos;
    new_pos=encode_tag_length(tag,element_size,buffer,new_pos,fill);
    for i in 0..value.len() {
        new_pos=encodeIECDataElement(&value[i], buffer, new_pos);
    }

    new_pos
}

pub fn encode_structure(tag:u8, value: &[IECData],buffer: &mut[u8],pos:usize,fill:bool) ->usize{

    encode_array(tag,value,buffer,pos,fill)
}

pub fn encodeIECDataElement(data: & IECData, buffer: &mut[u8], pos:usize) ->usize{
    
    let fill= true;
    let new_pos=pos;

    let new_pos=match  data{
        IECData::boolean (val)=> encode_boolean(0x83, *val, buffer, new_pos, fill),

        IECData::int8 (val)=> encode_interger_general(0x85, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::int16 (val)=> encode_interger_general(0x85, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::int32 (val)=> encode_interger_general(0x85, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::int64 (val)=> encode_interger_general(0x85, &val.to_be_bytes(), buffer, new_pos, fill),

        IECData::int8u (val)=> encode_unsigned_general(0x86, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::int16u (val)=> encode_unsigned_general(0x86, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::int32u (val)=> encode_unsigned_general(0x86, &val.to_be_bytes(), buffer, new_pos, fill),

        IECData::float32 (val)=> encode_float_general(0x87, &val.to_be_bytes(), buffer, new_pos, fill),
        IECData::float64 (val)=> encode_float_general(0x87, &val.to_be_bytes(), buffer, new_pos, fill),

        IECData::visible_string (val)=> encode_string(0x8a, val, buffer, new_pos, fill),
        IECData::mms_string (val)=> encode_string(0x90, val, buffer, new_pos, fill),
        IECData::bit_string{padding,val}=>encode_bit_string(0x84, *val, *padding,buffer, new_pos, fill),
        IECData::array(val)=>encode_array(0xa1,&val,buffer,new_pos,fill),
        IECData::structure(val)=>encode_structure(0xa2,&val,buffer,new_pos,fill),
        IECData::octet_string(val)=> encode_octet_string(0x89, &val, buffer, new_pos, fill),
        IECData::utc_time(val)=> encode_octet_string(0x91, val, buffer, new_pos, fill),
        _=>{panic!("unknown data type");}
    };

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

fn display_buffer( buffer: &[u8], size:usize){
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
    new_pos=new_pos+2;

    header.TCI.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;

    header.ehterType.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;

    header.APPID.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;

    header.length.copy_from_slice(&buffer[new_pos..new_pos+2]);
    new_pos=new_pos+2;
    println!("dcode header {:?}",header);

    //buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 1
    new_pos=new_pos+2;

    //buffer[new_pos..new_pos+2].copy_from_slice(&[0 ;2]); // reserved 2
    new_pos=new_pos+2;

    new_pos

}
pub fn decodeIECDataElement(buffer: &[u8], pos:usize) ->(usize,IECData){
    
    let mut new_pos=pos;

    let mut tag:u8=0;
    let mut length:usize=0;
    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);

    match  tag{
        0x83=> {
            let mut val:bool=false;
            new_pos=decode_boolean(& mut val, buffer, new_pos);
            return (new_pos,IECData::boolean(val));
        },
        0x85=>{
            match length{
                1=>{
                    let mut val:i8=0;
                    new_pos=decode_interger_8(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int8 (val));
                },
                2=>{
                    let mut val:i16=0;
                    new_pos=decode_interger_16(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int16 (val));
                },
                3..=4=>{
                    let mut val:i32=0;
                    new_pos=decode_interger(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int32 (val));
                }
                5..=8=>{
                    let mut val:i64=0;
                    new_pos=decode_interger_64(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int64 (val));                },
                _=>{
                    panic!("oversize interger");
                }
            }
        },
        0x86=>{
            match length{
                1=>{
                    let mut val:u8=0;
                    new_pos=decode_unsigned_8(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int8u (val));
                },
                2=>{
                    let mut val:u16=0;
                    new_pos=decode_unsigned_16(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int16u (val));
                },
                3..=4=>{
                    let mut val:u32=0;
                    new_pos=decode_unsigned(& mut val, buffer, new_pos,length);
                    return (new_pos,IECData::int32u (val));
                },
                5=>{
                    let mut val:u32=0;
                    new_pos=decode_unsigned(& mut val, buffer, new_pos+1,length-1);
                    return (new_pos,IECData::int32u (val));
                },
                6..=8=>{
                    panic!("oversize interger");

                },
                _=>{
                    panic!("oversize interger");
                }
            }
        },
        0x87=>{
            match length{
                5=>{
                    let mut val:f32=0.0;
                    new_pos=decode_float(&mut val,buffer, new_pos, length);
                    return (new_pos,IECData::float32(val));
                },
                9=>{
                    let mut val:f64=0.0;
                    new_pos=decode_float_64(&mut val,buffer, new_pos, length);
                    return (new_pos,IECData::float64(val));
                },
                _=>{
                    panic!("unknown size float");

                }

            }

        },
        0x8a=>{
            let mut val:String="".to_string();
            new_pos=decode_string(&mut val,buffer,new_pos,length);
            return (new_pos,IECData::visible_string (val));
        },
        0x90=>{
            let mut val:String="".to_string();
            new_pos=decode_string(&mut val,buffer,new_pos,length);
            return (new_pos,IECData::mms_string (val));
        },      
        0x84=>{
            let mut padding:u8=0;
            let mut val:u16=0;
            new_pos=decode_bit_string(&mut val,&mut padding,buffer,new_pos,length);
            return (new_pos,IECData::bit_string {val,padding});            
        },
        0xa1=>{
            let mut val:Vec<IECData>=vec![];
            new_pos=decodeIECData(&mut val,buffer,new_pos,new_pos+length);
            return (new_pos,IECData::array (val));            
        },
        0xa2=>{
            let mut val:Vec<IECData>=vec![];
            new_pos=decodeIECData(&mut val,buffer,new_pos,new_pos+length);
            return (new_pos,IECData::structure (val));            
        },        
        0x89=>{
            let mut val:Vec<u8>=vec![0;length];
            new_pos=decode_octet_string(&mut val,buffer,new_pos,length);
            return (new_pos,IECData::octet_string (val));        
        },
        0x91=>{
            let mut val=[0 as u8;8];
            new_pos=decode_octet_string(&mut val,buffer,new_pos,length);
            return (new_pos,IECData::utc_time (val));        
        },
        _=>{panic!("unknown data type");}
    };

}

pub fn decodeIECData(data: &mut Vec<IECData>, buffer: &[u8], pos:usize, end:usize) ->usize{

    let mut new_pos=pos;

    loop {
        let (next_pos, new_data)=decodeIECDataElement(buffer, new_pos);
        data.push(new_data);
        new_pos=next_pos;
        if new_pos>= end {
            break;
        }
    }

    new_pos

}
pub fn decodeIECGoosePdu(pdu: & mut IECGoosePdu, buffer: &[u8], pos:usize) ->usize{

    let mut new_pos=pos;
    let mut tag:u8=0;
    let mut length:usize=0;

    //goosePduLength
    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_string(&mut pdu.gocbRef,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_unsigned(&mut pdu.timeAllowedtoLive,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_string(&mut pdu.datSet,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_string(&mut pdu.goID,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_octet_string(&mut pdu.t,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_unsigned(&mut pdu.stNum,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_unsigned(&mut pdu.sqNum,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_boolean(&mut pdu.simulation,buffer,new_pos);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_unsigned(&mut pdu.confRev,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_boolean(&mut pdu.ndsCom,buffer,new_pos);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decode_unsigned(&mut pdu.numDatSetEntries,buffer,new_pos,length);

    new_pos=decode_tag_length(&mut tag,&mut length,buffer,new_pos);
    new_pos=decodeIECData(&mut pdu.allData,buffer,new_pos,new_pos+length);

    print!("decode pdu: {:?}",pdu);
    new_pos

}