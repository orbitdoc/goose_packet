#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::types::{*};
use crate::basic_encoder::{*};

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
        IECData::bit_string{padding,val}=>encode_bit_string(0, val, *padding,buffer, 0, fill),
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
        IECData::bit_string{padding,val}=>encode_bit_string(0x84, val, *padding,buffer, new_pos, fill),
        IECData::array(val)=>encode_array(0xa1,&val,buffer,new_pos,fill),
        IECData::structure(val)=>encode_structure(0xa2,&val,buffer,new_pos,fill),
        IECData::octet_string(val)=> encode_octet_string(0x89, &val, buffer, new_pos, fill),
        IECData::utc_time(val)=> encode_octet_string(0x91, val, buffer, new_pos, fill),
        _=>{panic!("unknown data type");}
    };

    new_pos

}