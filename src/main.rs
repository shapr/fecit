use zfec_rs::Fec;

fn main() {

    let message = b"xyz";

    let fec = Fec::new(3, 5).unwrap();

    let (mut encoded_chunks, padding) = fec.encode(&message[..]).unwrap();
    // encoded_chunks.remove(2);
    // let decoded_message = fec.decode(&encoded_chunks, padding).unwrap();

    // let (chunk_vec, chunk_i): (Vec<u8>, usize) = chunk.into();
    // let prettier : Vec<(Vec<u8>, usize)> = encoded_chunks.iter().map(|c| c.into() ).collect();
    // assert_eq!(message.to_vec(), decoded_message);

    // let val: Vec<u8> = vec![120, 121, 122];
    // let chunk = Chunk::new(val.clone(), 0);
    // let (chunk_vec, chunk_i): (Vec<u8>, usize) = chunk.into();
    // assert_eq!(val, chunk_vec);

    println!("{:?}", encoded_chunks);
}
