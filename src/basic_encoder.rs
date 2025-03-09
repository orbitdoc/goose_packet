pub fn encode_boolean(tag:u8,value:bool,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    if !fill
    {
        return 1;
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=1;
    new_pos+=1;
    buffer[new_pos]= if value{0xff} else{0x00};
    new_pos+=1;
    new_pos
}

pub fn encode_string(tag:u8,value:& String,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    let bytes=value.as_bytes();
    
    if !fill{
        return bytes.len();
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=bytes.len() as u8; //?
    new_pos+=1;
    for i in 0..bytes.len(){
        buffer[new_pos]=bytes[i];
        new_pos+=1;
    }
    new_pos
}

pub fn encode_octet_string(tag:u8,value:& [u8],buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    
    if !fill{
        return value.len();
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=value.len() as u8; //?
    new_pos+=1;

    buffer[new_pos..new_pos+value.len()].copy_from_slice(value);
    new_pos+value.len()
}


pub fn encode_interger_general(tag:u8,value:  &[u8],buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    
    compress_interger(tag,value,buffer,pos,fill)      
}

pub fn encode_unsigned_general(tag:u8,value:  &[u8],buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    let prepend=[&[0x00 as u8], value].concat();
    compress_interger(tag,&prepend,buffer,pos,fill)      
}

pub fn encode_interger(tag:u8,value: i32,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    let compressed = value.to_be_bytes();
    compress_interger(tag,&compressed,buffer,pos,fill)      
}

pub fn encode_unsigned(tag:u8,value: u32,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    //println!("u32: {}",value);
    let compressed = value.to_be_bytes();
    let mut prepend=vec![0x00 as u8];
    prepend.extend(&compressed);
    compress_interger(tag,&prepend,buffer,pos,fill)
}

pub fn compress_interger(tag:u8,compressed: &[u8],buffer: &mut[u8],pos:usize,fill:bool) ->usize{

    let mut compress_start:usize=0;
    while compress_start<(compressed.len()-1) {
        if (compressed[compress_start] == 0x00 ) && (compressed[compress_start+1] &0x80 ==0) {
            compress_start+=1;
            continue;
        }
        if (compressed[compress_start] == 0xff ) && (compressed[compress_start+1] &0x80 == 0x80) {
            compress_start+=1;
            continue;
        }
        break;
    }
    //println!("{:?},{}",compressed,compress_start);

    let compressed_size=compressed.len()-compress_start;

    if !fill{
        return compressed_size;
    }

    let mut new_pos=pos;
   
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=compressed_size as u8; //?
    new_pos+=1;

    buffer[new_pos..new_pos+compressed_size].copy_from_slice(&compressed[compress_start..]);
    new_pos+compressed_size
}
pub fn encode_float_general(tag:u8,bytes:&[u8],buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    
    if !fill{
        return bytes.len()+1;
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=(bytes.len()+1) as u8; //?
    new_pos+=1;
    buffer[new_pos]=0x08; //exponent
    new_pos+=1;
    buffer[new_pos..new_pos+bytes.len()].copy_from_slice(&bytes);

    new_pos+=bytes.len();
    new_pos
}

pub fn encode_float(tag:u8,value:f32,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    let bytes=value.to_be_bytes();
    
    if !fill{
        return bytes.len()+1;
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=(bytes.len()+1) as u8; //?
    new_pos+=1;
    buffer[new_pos]=0x08; //exponent
    new_pos+=1;
    buffer[new_pos..new_pos+bytes.len()].copy_from_slice(&bytes);

    new_pos+=bytes.len();
    new_pos
}
pub fn encode_bit_string(tag:u8,value:& [u8],padding: u8,buffer: &mut[u8],pos:usize,fill:bool) ->usize{
    //println!("u32: {}",value);

    if !fill{
        return value.len()+1;
    }

    let mut new_pos=pos;
   
    buffer[new_pos]=tag;
    new_pos+=1;
    buffer[new_pos]=(value.len()+1) as u8; //?
    new_pos+=1;

    buffer[new_pos]=padding;
    new_pos+=1;
    
    for i in 0..value.len()  {
        buffer[new_pos+i]=value[value.len()-i-1].reverse_bits();
    }
    new_pos+value.len()
}
pub fn encode_tag_length(tag:u8,value: usize,buffer: &mut[u8],pos:usize,fill:bool) ->usize{

    if !fill {
        return size_length(value);
    }

    let mut new_pos=pos;
    buffer[new_pos]=tag;
    new_pos+=1;

    if value<128 {
        buffer[new_pos]=value as u8;
        new_pos+=1;
    }
    else if value<256 {
        buffer[new_pos]=0x81;
        new_pos+=1;  
        buffer[new_pos]=value as u8;
        new_pos+=1;       
    }
    else if value<65535 {
        buffer[new_pos]=0x82;
        new_pos+=1;  
        buffer[new_pos]=(value/256) as u8;
        new_pos+=1;     
        buffer[new_pos]=(value&0xff) as u8;
        new_pos+=1;     
    }
    else {

        buffer[new_pos]=0x83;
        new_pos+=1;  
        buffer[new_pos]=(value/0x10000) as u8;
        new_pos+=1;     
        buffer[new_pos]=((value& 0xffff) / 0x100) as u8;
        new_pos+=1;     
        buffer[new_pos]=(value&0xff) as u8;
        new_pos+=1;         

    }

    new_pos
}

pub fn size_length(value: usize) ->usize
{
    if value<128 {
        return 1;
    }
    else if value<256 {
        return 2;
    }
    else if value<65535 {
        return 3;
    }
    else {       
        return 4;
    }
}