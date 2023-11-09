use std::{fmt::Display, error::Error};

pub(super) type Value = [u8; 2]; // Big endian
pub(super) type Registry = usize;

#[derive(Debug)]
pub struct RegistryBankError;

impl Display for RegistryBankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Write failed")
    }
}

impl Error for RegistryBankError {}

#[derive(Debug)]
pub(super) struct RegistryBank<const N: usize> {
    registries: [Value; N]
}

impl<const N: usize> RegistryBank<{N}> {
    pub fn new(array: [Value; N]) -> Self {
        Self {
            registries: array
        }
    }

    pub fn read_i16(&self, registry: Registry) -> Option<i16> {
        let value = self.registries.get(registry)?;
        Some(i16::from_be_bytes(*value))
    }

    pub fn read_u16(&self, registry: Registry) -> Option<u16> {
        let value = self.registries.get(registry)?;
        Some(u16::from_be_bytes(*value))
    }

    pub fn write_i16(&mut self, registry: Registry, value: i16) -> Result<(), RegistryBankError> {
        let reg = self.registries.get_mut(registry).ok_or(RegistryBankError)?;
        let write = i16::to_be_bytes(value);
        reg.copy_from_slice(&write);
        Ok(())
    }

    pub fn write_u16(&mut self, registry: Registry, value: u16) -> Result<(), RegistryBankError> {
        let reg = self.registries.get_mut(registry).ok_or(RegistryBankError)?;
        let write = u16::to_be_bytes(value);
        reg.copy_from_slice(&write);
        Ok(())
    }
}