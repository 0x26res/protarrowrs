use std::fs;

use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use tempfile;

fn main() {
    // Here we define `.proto` file source, we are not generating rust sources for it.
    let proto = "syntax = 'proto3'; message Mmm { int32 aaa = 1; }";

    let temp_dir = tempfile::tempdir().unwrap();
    let tempfile = temp_dir.path().join("mmm.proto");
    // For now we need to write files to the disk.
    fs::write(&tempfile, proto).unwrap();

    // Parse text `.proto` file to `FileDescriptorProto` message.
    // Note this API is not stable and subject to change.
    // But binary protos can always be generated manually with `protoc` command.
    let mut file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes(&[temp_dir.path().to_path_buf()])
        .input(&tempfile)
        .parse_and_typecheck()
        .unwrap()
        .file_descriptors;
    assert_eq!(1, file_descriptor_protos.len());

    // This is our .proto file converted to `FileDescriptorProto` from `descriptor.proto`.
    let file_descriptor_proto: FileDescriptorProto = file_descriptor_protos.pop().unwrap();
    // Now this `FileDescriptorProto` initialized for reflective access.
    let file_descriptor: FileDescriptor =
        FileDescriptor::new_dynamic(file_descriptor_proto, &[]).unwrap();
    // Find the message.
    let mmm_descriptor = file_descriptor
        .message_by_package_relative_name("Mmm")
        .unwrap();
    // Create an empty message.
    let mut mmm: Box<dyn MessageDyn> = mmm_descriptor.new_instance();
    // Find the field.
    let aaa_field = mmm_descriptor.field_by_name("aaa").unwrap();
    // Set field.
    aaa_field.set_singular_field(&mut *mmm, ReflectValueBox::I32(42));

    let mut boxes: Vec<Box<dyn MessageDyn>> = Vec::new();
    boxes.push(mmm.clone());
    let mmm2: &mut dyn MessageDyn = boxes.get_mut(0).unwrap().as_mut();
    //let _mmm4: &mut Box<dyn MessageDyn> = boxes.get(0).unwrap().as_mut();
    //let mut mmm3 : Box<dyn MessageDyn>= mmm2.clone();
    aaa_field.set_singular_field(&mut *mmm2, ReflectValueBox::I32(45));

    aaa_field.set_singular_field(&mut *mmm.clone(), ReflectValueBox::I32(45));
    aaa_field.set_singular_field(&mut *mmm.clone_box(), ReflectValueBox::I32(45));

    // Now serialize it to binary format.
    // field number = 1
    // wire_type = 0 (varint)
    // tag = (1 << 3) | 0 = 8
    // value = 42
    assert_eq!(&[8, 42], mmm.write_to_bytes_dyn().unwrap().as_slice());

    // Print it as text format.
    assert_eq!("aaa: 42", protobuf::text_format::print_to_string(&*mmm));
    assert_eq!("aaa: 45", protobuf::text_format::print_to_string(&*mmm2));
    let _mmm4: &dyn MessageDyn = boxes.get(0).unwrap().as_ref();
    assert_eq!("aaa: 45", protobuf::text_format::print_to_string(_mmm4));
}
