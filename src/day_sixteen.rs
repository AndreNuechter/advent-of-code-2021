#[derive(Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    value: i64,
    sub_packets: Option<Vec<Packet>>,
}

pub fn day_sixteen(input: &str) {
    let bits: String = input
        .chars()
        .map(|hex| {
            format!(
                "{:0>4b}",
                i64::from_str_radix(&hex.to_string(), 16).unwrap()
            )
        })
        .collect();
    let (
        Packet {
            version,
            sub_packets,
            value,
            ..
        },
        _remaining_bits,
    ) = parse_packet(&bits);
    let mut version_sum = version as u16;

    if sub_packets.is_some() {
        version_sum += sub_packets
            .unwrap()
            .iter()
            .map(get_sub_packet_version_sum)
            .sum::<u16>();
    }

    println!("pt 1:");
    println!("sum of versions {}", version_sum);
    println!("pt 2:");
    println!("calculated value: {}", value);
}

fn get_sub_packet_version_sum(packet: &Packet) -> u16 {
    if packet.sub_packets.is_none() {
        packet.version as u16
    } else {
        packet.version as u16
            + packet
                .clone()
                .sub_packets
                .unwrap()
                .iter()
                .map(get_sub_packet_version_sum)
                .sum::<u16>()
    }
}

fn parse_packet(packet: &str) -> (Packet, &str) {
    // 1st 3 bit => packet version
    // 2nd 3 bit => type ID
    let (version, type_id) = get_header(packet);

    println!("packet version: {}, type ID: {}", version, type_id);

    let new_packet: Packet;
    let remaining_bits: &str;

    // 4 => literal value (single binary number; padded with zeroes to have length % 4 == 0; and broken into 4-bit groups, ea prefixed w 1/0, 0 indicating the last group of a packet(so groups are of length 5))
    if type_id == 4 {
        let (value, offset) = get_literal_value(&packet[6..]);
        new_packet = Packet {
            version,
            type_id,
            value,
            sub_packets: None,
        };
        remaining_bits = &packet[6 + offset..];
        println!("literal value: {}", value);
    // _ => operator packet (contains one or more packets)
    } else {
        // 7th bit => length type ID, indicating one of two modes for designation of sub-packets
        let length_type_id = binary_string_to_decimal(&packet[6..7]);
        // bits after length type ID + 15/11 bit for field info => sub-packets
        let mut sub_packets: Vec<Packet> = Vec::new();
        let mut offset;
        println!("length type id {}", length_type_id);

        // 0 => next 15 bit indicate total length in bits of sub-packets
        if length_type_id == 0 {
            let sub_packet_length = binary_string_to_decimal(&packet[7..22]) as usize;
            println!("next {} bits contain subpackets", sub_packet_length);
            offset = 22 + sub_packet_length;
            let mut temp = &packet[22..offset];

            while temp.len() > 0 {
                let (new_packet, remaining_bits) = parse_packet(temp);
                sub_packets.push(new_packet);
                temp = remaining_bits;
            }
        // 1 => next 11 bit represent # of sub-packets contained by this packet
        } else {
            let sub_packet_count = binary_string_to_decimal(&packet[7..18]);
            println!("{} subpackets", sub_packet_count);
            offset = 18;
            sub_packets = (0..sub_packet_count)
                .map(|_| {
                    let current_slice = &packet[offset..];
                    let (new_packet, remaining_bits) = parse_packet(current_slice);
                    let packet_length = current_slice.len() - remaining_bits.len();
                    println!("packet_length {} bits", packet_length);
                    offset += packet_length;
                    new_packet
                })
                .collect();
        }

        let mut iter = sub_packets.iter().map(|packet| packet.value);
        let value = match type_id {
            1 => iter.product::<i64>(),
            2 => iter.min().unwrap(),
            3 => iter.max().unwrap(),
            5 => {
                if iter.next().unwrap() > iter.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if iter.next().unwrap() < iter.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if iter.next().unwrap() == iter.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => iter.sum::<i64>(),
        };

        new_packet = Packet {
            version,
            type_id,
            value,
            sub_packets: Some(sub_packets),
        };
        remaining_bits = &packet[offset..];
    }

    (new_packet, remaining_bits)
}

fn get_header(packet: &str) -> (u8, u8) {
    (
        binary_string_to_decimal(&packet[0..3]) as u8,
        binary_string_to_decimal(&packet[3..6]) as u8,
    )
}

fn get_literal_value(packet: &str) -> (i64, usize) {
    let mut result = String::new();
    let mut end = 0;

    for offset in (0..packet.len()).step_by(5) {
        end = offset + 5;
        result.push_str(&packet[(offset + 1)..end]);

        if packet[offset..(offset + 1)] == *"0" {
            break;
        }
    }

    (binary_string_to_decimal(&result), end)
}

fn binary_string_to_decimal(binary_string: &str) -> i64 {
    i64::from_str_radix(&binary_string.to_string(), 2).unwrap()
}
