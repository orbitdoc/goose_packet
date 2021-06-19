pub fn decode_boolean(value:&mut bool,buffer: &[u8],pos:usize) ->usize{
    *value=buffer[pos]!=0;
    pos+1
}

pub fn decode_string(value:& mut String,buffer: &[u8],pos:usize,length:usize) ->usize{
    *value=String::from_utf8_lossy(&buffer[pos..pos+length]).to_string();
    pos+length    
}

pub fn decode_octet_string(value:& mut [u8],buffer: &[u8],pos:usize,length:usize) ->usize{
    value[0..length].copy_from_slice(&buffer[pos..pos+length]);
    pos+length
}


pub fn decompress_integer(value: & mut [u8],buffer: &[u8],pos:usize,length:usize) {
    let mut fill=0x00;
    if buffer[pos] &0x80 == 0x80 {
        fill=0xff;
    }
    let fill_length=value.len()-length;    
    for i in 0..value.len()-fill_length{
        value[i]=fill;
    }
    value[fill_length..].copy_from_slice(&buffer[pos..pos+length]);

}
pub fn decode_interger_8(value:&mut i8, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;1];
    bytes[0..].copy_from_slice(&buffer[pos..pos+length]);
    *value=i8::from_be_bytes(bytes);
    pos+length
}

pub fn decode_interger_16(value:&mut i16, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;2];
    bytes[0..].copy_from_slice(&buffer[pos..pos+length]);
    *value=i16::from_be_bytes(bytes);
    pos+length
}

pub fn decode_interger(value:&mut i32, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;4];

    decompress_integer(& mut bytes,buffer,pos,length);
    *value=i32::from_be_bytes(bytes);
    pos+length
}

pub fn decode_interger_64(value:&mut i64, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;8];

    decompress_integer(& mut bytes,buffer,pos,length);
    *value=i64::from_be_bytes(bytes);
    pos+length
}

pub fn decode_unsigned_8(value:&mut u8, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;1];
    bytes[0..].copy_from_slice(&buffer[pos..pos+length]);
    *value=u8::from_be_bytes(bytes);
    pos+length
}

pub fn decode_unsigned_16(value:&mut u16, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;2];
    bytes[0..].copy_from_slice(&buffer[pos..pos+length]);
    *value=u16::from_be_bytes(bytes);
    pos+length
}
pub fn decode_unsigned(value:&mut u32, buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;4];
    decompress_integer(& mut bytes,buffer,pos,length);
    *value=u32::from_be_bytes(bytes);
    pos+length
}


pub fn decode_float(value:&mut f32,buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;4];
    
    bytes.copy_from_slice(&buffer[pos+1..pos+5]);
    *value=f32::from_be_bytes(bytes);
    pos+length
}

pub fn decode_float_64(value:&mut f64,buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut bytes=[0 as u8;8];
    
    bytes.copy_from_slice(&buffer[pos+1..pos+9]);
    *value=f64::from_be_bytes(bytes);
    pos+length
}

pub fn decode_bit_string(value:& mut [u8],padding:&mut u8,buffer: &[u8],pos:usize,length:usize) ->usize{
    let mut new_pos=pos;

    *padding=buffer[new_pos];
    new_pos+=1;

    for i in 0..value.len()  {
        value[value.len()-i-1]=buffer[new_pos+i].reverse_bits();
    }
    new_pos+length
    
}
pub fn decode_tag_length(tag:&mut u8,value:&mut usize,buffer: &[u8],pos:usize) ->usize{

    let mut new_pos=pos;
    *tag=buffer[new_pos];
    new_pos+=1;

    match buffer[new_pos] {
        0x81=>{ 
            new_pos+=1;  
            *value=buffer[new_pos] as usize;
            new_pos+=1;  

        },
        0x82=>{
            new_pos+=1;  
            *value= buffer[new_pos] as usize *256;
            new_pos+=1;     
            *value+= (buffer[new_pos])as usize;
            new_pos+=1;     

        },
        0x83=>{
            new_pos+=1;  
            *value= buffer[new_pos] as usize *0x10000;
            new_pos+=1;     
            *value+= buffer[new_pos]as usize *0x100;
            new_pos+=1;     
            *value+= (buffer[new_pos])as usize;
            new_pos+=1;         
    
        },
        _=>{
            if buffer[new_pos]>0x83 {panic!("unexpexted legnth");}
            *value=buffer[new_pos]  as usize;
            new_pos+=1;  

        }
    }


    new_pos
}