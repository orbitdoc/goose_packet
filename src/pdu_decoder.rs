#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::types::{*};
use crate::basic_decoder::{*};


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
            let mut val:Vec<u8>=vec![0;length-1];
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