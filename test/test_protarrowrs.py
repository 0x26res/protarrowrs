from ptars import HandlerPool
from ptars_protos import simple_pb2


def test_generate_proto():
    pool = HandlerPool()

    handler = pool.get_for_message(simple_pb2.SimpleMessage.DESCRIPTOR)
    protos = [
        simple_pb2.SimpleMessage(
            int64_value=123,
            uint32_value=456,
        ),
        simple_pb2.SimpleMessage(int64_value=0, uint32_value=789),
        simple_pb2.SimpleMessage(
            double_value=1.0,
            float_value=2.0,
            int32_value=3,
            int64_value=4,
            uint32_value=5,
            uint64_value=6,
            sint32_value=7,
            sint64_value=8,
            fixed32_value=9,
            fixed64_value=10,
            sfixed32_value=11,
            sfixed64_value=12,
            bool_value=True,
            string_value="14",
            bytes_value=b"15",
        ),
    ]
    message_payloads = [p.SerializeToString() for p in protos]
    table = handler.list_to_table(message_payloads)

    assert table["int64_value"].to_pylist() == [123, 0, 4]
    assert table["uint32_value"].to_pylist() == [456, 789, 5]
    assert table["bool_value"].to_pylist() == [False, False, True]
    assert table["string_value"].to_pylist() == ["", "", "14"]
    assert table["bytes_value"].to_pylist() == [b"", b"", b"15"]
