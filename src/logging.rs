//! Module contains logfile types used to access the underlying CAN-bus data.

use crate::data::CANRead;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::iter::{IntoIterator, Iterator};
use std::ops::Div;
use std::path::Path;
use std::str::FromStr;

///
///
/// # Format
/// ```bash
/// can0 00000042 [0]
/// can0 1FF [1] 01
/// vcan0 00001337 [8] 01 02 03 04 05 06 07 08
/// ```
/// # Example
/// ```no_run
/// use cantools::logging::CANDump;
/// let file = CANDump::open("raw_file");
/// ```
pub struct CANDump {
    file: File,
}

impl CANDump {
    pub fn open<P>(path: P) -> io::Result<CANDump>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(CANDump { file })
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

#[derive(Debug, PartialEq)]
pub struct CANDumpEntry {
    interface: String,
    can_id: u32,
    data: Vec<u8>,
}

impl CANRead for CANDumpEntry {
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn dlc(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum CANDumpEntryConstructionError {
    EmptyInterface,
}

impl CANDumpEntry {
    pub fn new(
        interface: &str,
        can_id: u32,
        data: Vec<u8>,
    ) -> Result<Self, CANDumpEntryConstructionError> {
        if interface.is_empty() {
            Err(CANDumpEntryConstructionError::EmptyInterface)
        } else {
            Ok(CANDumpEntry {
                interface: String::from(interface),
                can_id,
                data,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CANDumpEntryParseError {
    MissingInterfaceData,
    MissingCanIdData,
    MissingDlcData,
    ParseDlcError,
    ParseCanIdError,
    ParseCanDataError,
    DlcDataMismatch,
    ConstructionError(CANDumpEntryConstructionError),
}

impl FromStr for CANDumpEntry {
    type Err = CANDumpEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split(' ').collect::<Vec<_>>();

        let interface = match splits.get(0).copied() {
            Some(interface) => interface,
            None => return Err(CANDumpEntryParseError::MissingInterfaceData),
        };

        let can_id = match splits.get(1).copied().map(|e| u32::from_str_radix(e, 16)) {
            Some(Ok(can_id)) => can_id,
            _ => return Err(CANDumpEntryParseError::MissingCanIdData),
        };

        let dlc_string = match splits.get(2).copied() {
            Some(dlc_string) => dlc_string,
            None => return Err(CANDumpEntryParseError::MissingDlcData),
        };

        let dlc = match dlc_string[1..dlc_string.len() - 1].parse::<usize>() {
            Ok(dlc) => dlc,
            Err(_) => return Err(CANDumpEntryParseError::ParseDlcError),
        };

        let mut data = Vec::new();
        for entry in splits.into_iter().skip(3) {
            match u8::from_str_radix(entry, 16) {
                Ok(value) => data.push(value),
                _ => return Err(CANDumpEntryParseError::ParseCanDataError),
            }
        }

        if dlc != data.len() {
            return Err(CANDumpEntryParseError::DlcDataMismatch);
        }

        match CANDumpEntry::new(interface, can_id, data) {
            Ok(entry) => Ok(entry),
            Err(err) => Err(CANDumpEntryParseError::ConstructionError(err)),
        }
    }
}

impl ToString for CANDumpEntry {
    fn to_string(&self) -> String {
        let data_string = self
            .data
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "{} {:08X} [{}] {}",
            self.interface,
            self.can_id,
            self.data.len(),
            data_string
        )
    }
}

pub struct CANDumpIterator {
    lines: Lines<BufReader<File>>,
}

impl Iterator for CANDumpIterator {
    type Item = CANDumpEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next();
            match line {
                Some(Ok(line)) => match line.parse::<Self::Item>() {
                    Ok(entry) => return Some(entry),
                    Err(_) => continue,
                },
                Some(Err(_)) => continue,
                None => return None,
            }
        }
    }
}

impl IntoIterator for CANDump {
    type Item = CANDumpEntry;
    type IntoIter = CANDumpIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpIterator {
            lines: io::BufReader::new(self.into_inner()).lines(),
        }
    }
}

pub struct CANDumpLog {
    file: File,
}

impl CANDumpLog {
    pub fn open<P>(path: P) -> io::Result<CANDumpLog>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(CANDumpLog { file })
    }

    pub fn into_inner(self) -> File {
        self.file
    }
}

#[derive(Debug, PartialEq)]
pub struct CANDumpLogEntry {
    timestamp: f64,
    interface: String,
    can_id: u32,
    data: Vec<u8>,
    flag: Option<u8>,
}

#[derive(Debug, PartialEq)]
pub enum CANDumpLogEntryConstructionError {
    InvalidTimestamp,
    EmptyInterface,
    InvalidFlagValue,
}

impl CANDumpLogEntry {
    pub fn new(
        timestamp: f64,
        interface: &str,
        can_id: u32,
        data: Vec<u8>,
        flag: Option<u8>,
    ) -> Result<Self, CANDumpLogEntryConstructionError> {
        if timestamp.is_nan() || timestamp.is_infinite() {
            return Err(CANDumpLogEntryConstructionError::InvalidTimestamp);
        }

        if interface.is_empty() {
            return Err(CANDumpLogEntryConstructionError::EmptyInterface);
        }

        if let Some(x) = flag {
            if x > 0x0F {
                return Err(CANDumpLogEntryConstructionError::InvalidFlagValue);
            };
        }

        Ok(CANDumpLogEntry {
            timestamp,
            interface: String::from(interface),
            can_id,
            data,
            flag,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum CANDumpLogEntryParseError {
    MissingTimestampData,
    ParseTimestampError,
    MissingInterfaceData,
    MissingCompoundCanData,
    MissingCanIdData,
    MissingCanData,
    MissingFlagData,
    ParseCanIdError,
    ParseCanDataError,
    ParseFlagError,
    ConstructionError(CANDumpLogEntryConstructionError),
    Unspecified,
}

impl FromStr for CANDumpLogEntry {
    type Err = CANDumpLogEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split(' ').take(3).collect::<Vec<_>>();

        let timestamp = match splits.get(0).copied() {
            Some(timestamp) => timestamp,
            None => return Err(CANDumpLogEntryParseError::MissingTimestampData),
        };

        let timestamp = match timestamp[1..timestamp.len() - 1].parse::<f64>() {
            Ok(timestamp) => timestamp,
            Err(_) => return Err(CANDumpLogEntryParseError::ParseTimestampError),
        };

        let interface = match splits.get(1).copied() {
            Some(interface) => interface,
            None => return Err(CANDumpLogEntryParseError::MissingInterfaceData),
        };

        let can_data = match splits.get(2).copied() {
            Some(can_data) => can_data,
            None => return Err(CANDumpLogEntryParseError::MissingCompoundCanData),
        };

        let can_data_splits = can_data.split('#').take(3).collect::<Vec<_>>();

        return match can_data_splits.len() {
            2 => {
                let can_id_string = match can_data_splits.get(0).copied() {
                    Some(can_id_string) => can_id_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanIdData),
                };

                let can_id = match u32::from_str_radix(can_id_string, 16) {
                    Ok(can_id) => can_id,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseCanIdError),
                };

                let data_string = match can_data_splits.get(1).copied() {
                    Some(data_string) => data_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanData),
                };

                let mut data = Vec::new();
                for i in 0..data_string.len().div(2) {
                    match u8::from_str_radix(&data_string[2 * i..2 * i + 2], 16) {
                        Ok(value) => data.push(value),
                        Err(_) => return Err(CANDumpLogEntryParseError::ParseCanDataError),
                    };
                }

                match CANDumpLogEntry::new(timestamp, interface, can_id, data, None) {
                    Ok(entry) => Ok(entry),
                    Err(err) => Err(CANDumpLogEntryParseError::ConstructionError(err)),
                }
            }
            3 => {
                let can_id_string = match can_data_splits.get(0).copied() {
                    Some(can_id_string) => can_id_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanIdData),
                };

                let can_id = match u32::from_str_radix(can_id_string, 16) {
                    Ok(can_id) => can_id,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseCanIdError),
                };

                let data_string = match can_data_splits.get(2).copied() {
                    Some(data_string) => data_string,
                    None => return Err(CANDumpLogEntryParseError::MissingCanData),
                };

                let flag_string = match data_string.get(0..1) {
                    Some(flag_string) => flag_string,
                    None => return Err(CANDumpLogEntryParseError::MissingFlagData),
                };

                let flag = match u8::from_str_radix(flag_string, 16) {
                    Ok(flag) => flag,
                    Err(_) => return Err(CANDumpLogEntryParseError::ParseFlagError),
                };

                let mut data = Vec::new();
                for i in 0..(data_string.len() - 1).div(2) {
                    match u8::from_str_radix(&data_string[2 * i + 1..2 * i + 2 + 1], 16) {
                        Ok(value) => data.push(value),
                        Err(_) => return Err(CANDumpLogEntryParseError::ParseCanDataError),
                    };
                }

                match CANDumpLogEntry::new(timestamp, interface, can_id, data, Some(flag)) {
                    Ok(entry) => Ok(entry),
                    Err(err) => Err(CANDumpLogEntryParseError::ConstructionError(err)),
                }
            }
            _ => Err(CANDumpLogEntryParseError::Unspecified),
        };
    }
}

impl ToString for CANDumpLogEntry {
    fn to_string(&self) -> String {
        let data_string = self
            .data
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join("");

        return match self.flag {
            Some(flag) => {
                format!(
                    "({}) {} {:08X}##{:1X}{}",
                    self.timestamp, self.interface, self.can_id, flag, data_string
                )
            }
            None => {
                format!(
                    "({}) {} {:08X}#{}",
                    self.timestamp, self.interface, self.can_id, data_string
                )
            }
        };
    }
}

impl CANRead for CANDumpLogEntry {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }
}

pub struct CANDumpLogIterator {
    lines: Lines<BufReader<File>>,
}

impl Iterator for CANDumpLogIterator {
    type Item = CANDumpLogEntry;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next();
            match line {
                Some(Ok(line)) => match line.parse::<CANDumpLogEntry>() {
                    Ok(entry) => return Some(entry),
                    Err(_) => continue,
                },
                Some(Err(_)) => continue,
                None => return None,
            }
        }
    }
}

impl IntoIterator for CANDumpLog {
    type Item = CANDumpLogEntry;
    type IntoIter = CANDumpLogIterator;
    fn into_iter(self) -> Self::IntoIter {
        CANDumpLogIterator {
            lines: io::BufReader::new(self.into_inner()).lines(),
        }
    }
}
