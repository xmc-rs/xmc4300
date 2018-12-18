#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MII_CONT_STAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `W_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_ENR {
    #[doc = "Write disabled"]
    VALUE1,
    #[doc = "Write enabled"]
    VALUE2,
}
impl W_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            W_ENR::VALUE1 => false,
            W_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> W_ENR {
        match value {
            false => W_ENR::VALUE1,
            true => W_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == W_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == W_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `MIC_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIC_PDIR {
    #[doc = "Only ECAT control"]
    VALUE1,
    #[doc = "PDI control possible"]
    VALUE2,
}
impl MIC_PDIR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MIC_PDIR::VALUE1 => false,
            MIC_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIC_PDIR {
        match value {
            false => MIC_PDIR::VALUE1,
            true => MIC_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MIC_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MIC_PDIR::VALUE2
    }
}
#[doc = "Possible values of the field `MI_LD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI_LDR {
    #[doc = "Not available"]
    VALUE1,
    #[doc = "MI link detection active"]
    VALUE2,
}
impl MI_LDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MI_LDR::VALUE1 => false,
            MI_LDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI_LDR {
        match value {
            false => MI_LDR::VALUE1,
            true => MI_LDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MI_LDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MI_LDR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct PHY_ADDRR {
    bits: u8,
}
impl PHY_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD_REG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_REGR {
    #[doc = "No command/MII idle (clear error bits)"]
    VALUE1,
    #[doc = "Read"]
    VALUE2,
    #[doc = "Write"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMD_REGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_REGR::VALUE1 => 0,
            CMD_REGR::VALUE2 => 1,
            CMD_REGR::VALUE3 => 2,
            CMD_REGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_REGR {
        match value {
            0 => CMD_REGR::VALUE1,
            1 => CMD_REGR::VALUE2,
            2 => CMD_REGR::VALUE3,
            i => CMD_REGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_REGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_REGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMD_REGR::VALUE3
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Last Command was successful"]
    VALUE1,
    #[doc = "Invalid command or write command without Write Enable"]
    VALUE2,
}
impl ERRORR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ERRORR::VALUE1 => false,
            ERRORR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORR {
        match value {
            false => ERRORR::VALUE1,
            true => ERRORR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ERRORR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ERRORR::VALUE2
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "MI control state machine is idle"]
    VALUE1,
    #[doc = "MI control state machine is active"]
    VALUE2,
}
impl BUSYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BUSYR::VALUE1 => false,
            BUSYR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::VALUE1,
            true => BUSYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BUSYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BUSYR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CMD_REG`"]
pub enum CMD_REGW {
    #[doc = "No command/MII idle (clear error bits)"]
    VALUE1,
    #[doc = "Read"]
    VALUE2,
    #[doc = "Write"]
    VALUE3,
}
impl CMD_REGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_REGW::VALUE1 => 0,
            CMD_REGW::VALUE2 => 1,
            CMD_REGW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_REGW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_REGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_REGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No command/MII idle (clear error bits)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE1)
    }
    #[doc = "Read"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE2)
    }
    #[doc = "Write"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_REGW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Write enable"]
    #[inline]
    pub fn w_en(&self) -> W_ENR {
        W_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Management Interface can be controlled by PDI"]
    #[inline]
    pub fn mic_pdi(&self) -> MIC_PDIR {
        MIC_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - MI link detection"]
    #[inline]
    pub fn mi_ld(&self) -> MI_LDR {
        MI_LDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 3:7 - PHY address of port 0"]
    #[inline]
    pub fn phy_addr(&self) -> PHY_ADDRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        PHY_ADDRR { bits }
    }
    #[doc = "Bits 8:9 - Command register"]
    #[inline]
    pub fn cmd_reg(&self) -> CMD_REGR {
        CMD_REGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 14 - Command error"]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - Command register"]
    #[inline]
    pub fn cmd_reg(&mut self) -> _CMD_REGW {
        _CMD_REGW { w: self }
    }
}
