use anyhow::{anyhow, Result};
use bitvec::{field::BitField as _, prelude as bv};
use itertools::Itertools;
use nom::{
    bytes::complete::take,
    combinator::map,
    multi::{count, many0},
    Finish, IResult,
};
use nom_bitvec::BSlice;

#[derive(Debug, Clone)]

struct PacketHeader {
    version: u8,
    type_id: u8,
}

#[derive(Debug, Clone)]
enum PacketType {
    Literal(usize),
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    subpackets: Vec<Packet>,
}

fn parse_header(input: BSlice<bv::Msb0, u8>) -> IResult<BSlice<bv::Msb0, u8>, PacketHeader> {
    let (input, version) = map(take(3u8), |bits: BSlice<bv::Msb0, u8>| bits.0.load_be())(input)?;
    let (input, type_id) = map(take(3u8), |bits: BSlice<bv::Msb0, u8>| bits.0.load_be())(input)?;

    let header = PacketHeader { version, type_id };
    Ok((input, header))
}

fn parse_literal(mut input: BSlice<bv::Msb0, u8>) -> IResult<BSlice<bv::Msb0, u8>, usize> {
    let mut val: bv::BitVec<bv::Msb0, u8> = bv::BitVec::new();
    loop {
        let res = map(take(1u8), |bits: BSlice<bv::Msb0, u8>| bits[0])(input)?;
        input = res.0;
        let not_last = res.1;

        let res = take(4u8)(input)?;
        input = res.0;
        let bits = res.1;

        val.extend(bits.0);
        if !not_last {
            break;
        }
    }

    Ok((input, val.load_be()))
}

fn parse_subpackets(mut input: BSlice<bv::Msb0, u8>) -> IResult<BSlice<bv::Msb0, u8>, Vec<Packet>> {
    let res = map(take(1u8), |bits: BSlice<bv::Msb0, u8>| bits.0.load())(input)?;
    input = res.0;
    let length_type_id: usize = res.1;

    let subpackets = if length_type_id == 0 {
        let res = map(take(15u8), |bits: BSlice<bv::Msb0, u8>| bits.0.load_be())(input)?;
        input = res.0;
        let sub_packet_length = res.1;
        let subpackets = BSlice(&input.0[..sub_packet_length]);

        let res = many0(parse_packet)(subpackets)?;
        let subpackets = res.1;
        assert!(res.0 .0.is_empty());

        input = BSlice(&input.0[sub_packet_length..]);

        subpackets
    } else {
        let res = map(take(11u8), |bits: BSlice<bv::Msb0, u8>| bits.0.load_be())(input)?;
        input = res.0;
        let num_sub_packets = res.1;

        let res = count(parse_packet, num_sub_packets)(input)?;
        input = res.0;

        res.1
    };

    Ok((input, subpackets))
}

fn parse_packet(mut input: BSlice<bv::Msb0, u8>) -> IResult<BSlice<bv::Msb0, u8>, Packet> {
    let res = parse_header(input)?;
    input = res.0;
    let header = res.1;

    let (packet_type, subpackets) = match header.type_id {
        4 => {
            let res = parse_literal(input)?;
            input = res.0;
            (PacketType::Literal(res.1), Vec::new())
        }
        _ => {
            let res = parse_subpackets(input)?;
            input = res.0;
            let subpackets = res.1;
            let packet_type = match header.type_id {
                0 => PacketType::Sum,
                1 => PacketType::Product,
                2 => PacketType::Minimum,
                3 => PacketType::Maximum,
                5 => PacketType::GreaterThan,
                6 => PacketType::LessThan,
                7 => PacketType::EqualTo,
                _ => panic!("Unexpected type id {}", header.type_id), // FIXME: how to use custom error with nom
            };
            (packet_type, subpackets)
        }
    };

    let packet = Packet {
        version: header.version,
        packet_type,
        subpackets,
    };
    Ok((input, packet))
}

fn parse_input(input: &str) -> Result<Packet> {
    let bits: bv::BitVec<bv::Msb0, u8> = input
        .trim_end()
        .chars()
        .chunks(2)
        .into_iter()
        .map(|nibbles| {
            nibbles.enumerate().try_fold(0u8, |acc, (i, nibble)| {
                let val = (nibble
                    .to_digit(16)
                    .ok_or_else(|| anyhow!("Invalid hex digit {}", nibble))?
                    as u8)
                    << ((1 - i as u8) * 4);
                Ok(acc | val)
            })
        })
        .collect::<Result<bv::BitVec<bv::Msb0, u8>>>()?;

    let packet = parse_packet(BSlice(bits.as_bitslice()))
        .finish()
        .map_err(|e| anyhow!("error: {:?}", e))?
        .1;
    Ok(packet)
}

fn sum_versions(packet: &Packet) -> usize {
    match packet.packet_type {
        PacketType::Literal(_) => packet.version as usize,
        _ => packet.version as usize + packet.subpackets.iter().map(sum_versions).sum::<usize>(),
    }
}

fn evaluate(packet: &Packet) -> usize {
    match packet.packet_type {
        PacketType::Literal(val) => val,
        PacketType::Sum => packet.subpackets.iter().map(evaluate).sum(),
        PacketType::Product => packet.subpackets.iter().map(evaluate).product(),
        PacketType::Minimum => packet.subpackets.iter().map(evaluate).min().unwrap(),
        PacketType::Maximum => packet.subpackets.iter().map(evaluate).max().unwrap(),
        PacketType::GreaterThan => {
            assert_eq!(packet.subpackets.len(), 2);
            (evaluate(&packet.subpackets[0]) > evaluate(&packet.subpackets[1])) as _
        }
        PacketType::LessThan => {
            assert_eq!(packet.subpackets.len(), 2);
            (evaluate(&packet.subpackets[0]) < evaluate(&packet.subpackets[1])) as _
        }
        PacketType::EqualTo => {
            assert_eq!(packet.subpackets.len(), 2);
            (evaluate(&packet.subpackets[0]) == evaluate(&packet.subpackets[1])) as _
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day16")?;
    let packet = parse_input(&input)?;

    let result_a = sum_versions(&packet);
    assert_eq!(result_a, 893);
    println!("Day 16, part A: {}", result_a);

    let result_b = evaluate(&packet);
    assert_eq!(result_b, 4358595186090);
    println!("Day 16, part B: {}", result_b);

    Ok(())
}
