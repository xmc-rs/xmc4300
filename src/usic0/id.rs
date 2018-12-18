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
#[doc = r" Value of the field"]
pub struct MOD_TYPER {
    bits: u8,
}
impl MOD_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MOD_TYPER { bits }
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
