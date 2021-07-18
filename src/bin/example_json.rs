use goose_packet::{pdu::getTimeMs, types::IECData};

fn main(){
    let current_time=getTimeMs();

    let goose_data=vec![
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
        ];
    let serialized = serde_json::to_string(&goose_data).unwrap();
    println!("serialized = {}", serialized);

    let deserialized :Vec<IECData>= serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}