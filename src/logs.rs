use crate::data::CANData;
use std::path::{Path};
use std::iter::{Iterator, IntoIterator};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Lines};
use std::ops::Div;

pub struct CANDump {
    file: File
}

pub struct CANDumpEntry {
    interface: String,
    can_id: u32,
    data: Vec<u8>
}

pub struct CANDumpIterator {
    lines: Lines<BufReader<File>>
}

impl CANDump {
    pub fn open<P>(path: P) -> io::Result<CANDump>
    where
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        Ok(CANDump {file})
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

impl Iterator for CANDumpIterator {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl IntoIterator for CANDump {
    type Item = ();
    type IntoIter = CANDumpIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpIterator {
            lines: io::BufReader::new(self.into_inner()).lines()
        }
    }
}

pub struct CANDumpLog {
    file: File
}

#[derive(Debug, PartialEq)]
pub struct CANDumpLogEntry {
    timestamp: f64,
    interface: String,
    can_id: u32,
    data: Vec<u8>,
    flag: Option<u8>
}

impl CANDumpLogEntry {
    pub fn new(timestamp: f64, interface: &str, can_id:u32, data: &[u8], flag: Option<u8>) -> Self {
        CANDumpLogEntry {
            timestamp,
            interface: String::from(interface),
            can_id,
            data: Vec::<_>::from(data),
            flag
        }
    }
}

impl CANData for CANDumpLogEntry {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }
}

pub struct CANDumpLogIterator {
    lines: Lines<BufReader<File>>
}

impl CANDumpLog {
    pub fn open<P>(path: P) -> io::Result<CANDumpLog>
    where
        P: AsRef<Path>
    {
        let file = File::open(path)?;
        Ok(CANDumpLog {file})
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

impl Iterator for CANDumpLogIterator {
    type Item = CANDumpLogEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next();
            match line {
                Some(Ok(line)) => {
                    if let [timestamp_string, interface, can_id_data_string] = line
                        .split(" ")
                        .take(3)
                        .collect::<Vec<_>>()[..]
                    {
                        let timestamp = match timestamp_string[1..timestamp_string.len() - 1]
                            .parse::<f64>()
                        {
                            Ok(timestamp) => timestamp,
                            Err(_) => continue
                        };

                        if let [can_id_string, _, data_string] = can_id_data_string
                            .split("#")
                            .take(3)
                            .collect::<Vec<_>>()[..]
                        {
                            let can_id = match u32::from_str_radix(can_id_string, 16) {
                                Ok(can_id) => can_id,
                                Err(_) => continue
                            };

                            let flag = match u8::from_str_radix(&data_string[0..1], 16) {
                                Ok(flag) => flag,
                                Err(_) => continue
                            };

                            let mut data = Vec::new();
                            for i in 0..(data_string.len() - 1).div(2) {
                                let value = u8::from_str_radix(&data_string[2*i+1..2*i+2+1], 16).unwrap();
                                data.push(value);
                            }

                            return Some(Self::Item::new(timestamp, interface, can_id, &data, Some(flag)));
                        }

                        if let [can_id_string, data_string] = can_id_data_string
                            .split("#")
                            .take(2)
                            .collect::<Vec<_>>()[..]
                        {
                            let can_id = match u32::from_str_radix(can_id_string, 16) {
                                Ok(can_id) => can_id,
                                Err(_) => continue
                            };

                            let mut data = Vec::new();
                            for i in 0..data_string.len().div(2) {
                                data.push(u8::from_str_radix(&data_string[2*i..2*i+2], 16).unwrap())
                            }

                            return Some(Self::Item::new(timestamp, interface, can_id, &data, None));
                        }
                    }
                },
                Some(Err(_)) => continue,
                None => return None
            }
        }
    }
}

impl IntoIterator for CANDumpLog {
    type Item = CANDumpLogEntry;
    type IntoIter = CANDumpLogIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpLogIterator {
            lines: io::BufReader::new(self.into_inner()).lines()
        }
    }
}