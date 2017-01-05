#![feature(test)]

extern crate test;
extern crate kvproto;
extern crate protobuf;

use test::Bencher;
use protobuf::core::Message;
//use protobuf::core::CodedMessage;

use kvproto::raft_serverpb::RaftMessage;
use kvproto::eraftpb::MessageType;

#[bench]
fn bench_codec(b: &mut Bencher) {
    let mut v = Vec::with_capacity(1024);
   let mut m = RaftMessage::new();
	m.set_region_id(1);
	m.mut_region_epoch().set_conf_ver(2);
	m.mut_region_epoch().set_version(3);
	m.mut_from_peer().set_store_id(4);
	m.mut_from_peer().set_id(5);
	m.mut_to_peer().set_store_id(6);
	m.mut_to_peer().set_id(7);
    m.mut_message().set_to(8);
    m.mut_message().set_msg_type(MessageType::MsgHeartbeat);
    m.mut_message().set_commit(9);
    m.write_to_writer(&mut v).unwrap();
    b.iter(|| {
        let mut res = RaftMessage::new();
        res.merge_from_bytes(&v).unwrap();
		// v.clear();
    });
}

