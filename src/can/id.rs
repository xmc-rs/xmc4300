#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct MOD_REVR {
    bits: u8,
}
impl MOD_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MOD_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_TYPER {
    #[doc = "Define the module as a 32-bit module."]
    VALUE1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MOD_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MOD_TYPER::VALUE1 => 192,
            MOD_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MOD_TYPER {
        match value {
            192 => MOD_TYPER::VALUE1,
            i => MOD_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MOD_TYPER::VALUE1
    }
}
#[doc = r" Value of the field"]
pub struct MOD_NUMBERR {
    bits: u16,
}
impl MOD_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Module Revision Number"]
    #[inline]
    pub fn mod_rev(&self) -> MOD_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MOD_REVR { bits }
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline]
    pub fn mod_type(&self) -> MOD_TYPER {
        MOD_TYPER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Module Number Value"]
    #[inline]
    pub fn mod_number(&self) -> MOD_NUMBERR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MOD_NUMBERR { bits }
    }
}
