#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCFG {
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
#[doc = "Possible values of the field `DevSpd`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVSPDR {
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEVSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEVSPDR::VALUE4 => 3,
            DEVSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEVSPDR {
        match value {
            3 => DEVSPDR::VALUE4,
            i => DEVSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DEVSPDR::VALUE4
    }
}
#[doc = "Possible values of the field `NZStsOUTHShk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NZSTSOUTHSHKR {
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1,
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2,
}
impl NZSTSOUTHSHKR {
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
            NZSTSOUTHSHKR::VALUE1 => true,
            NZSTSOUTHSHKR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NZSTSOUTHSHKR {
        match value {
            true => NZSTSOUTHSHKR::VALUE1,
            false => NZSTSOUTHSHKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NZSTSOUTHSHKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NZSTSOUTHSHKR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct DEVADDRR {
    bits: u8,
}
impl DEVADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PerFrInt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERFRINTR {
    #[doc = "80% of the frame interval"]
    VALUE1,
    #[doc = "85%"]
    VALUE2,
    #[doc = "90%"]
    VALUE3,
    #[doc = "95%"]
    VALUE4,
}
impl PERFRINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERFRINTR::VALUE1 => 0,
            PERFRINTR::VALUE2 => 1,
            PERFRINTR::VALUE3 => 2,
            PERFRINTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERFRINTR {
        match value {
            0 => PERFRINTR::VALUE1,
            1 => PERFRINTR::VALUE2,
            2 => PERFRINTR::VALUE3,
            3 => PERFRINTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PERFRINTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PERFRINTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PERFRINTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PERFRINTR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct DESCDMAR {
    bits: bool,
}
impl DESCDMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PerSchIntvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERSCHINTVLR {
    #[doc = "25% of frame."]
    VALUE1,
    #[doc = "50% of frame."]
    VALUE2,
    #[doc = "75% of frame."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERSCHINTVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERSCHINTVLR::VALUE1 => 0,
            PERSCHINTVLR::VALUE2 => 1,
            PERSCHINTVLR::VALUE3 => 2,
            PERSCHINTVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERSCHINTVLR {
        match value {
            0 => PERSCHINTVLR::VALUE1,
            1 => PERSCHINTVLR::VALUE2,
            2 => PERSCHINTVLR::VALUE3,
            i => PERSCHINTVLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PERSCHINTVLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PERSCHINTVLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PERSCHINTVLR::VALUE3
    }
}
#[doc = "Values that can be written to the field `DevSpd`"]
pub enum DEVSPDW {
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4,
}
impl DEVSPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEVSPDW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVSPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVSPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DEVSPDW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NZStsOUTHShk`"]
pub enum NZSTSOUTHSHKW {
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1,
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2,
}
impl NZSTSOUTHSHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NZSTSOUTHSHKW::VALUE1 => true,
            NZSTSOUTHSHKW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NZSTSOUTHSHKW<'a> {
    w: &'a mut W,
}
impl<'a> _NZSTSOUTHSHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NZSTSOUTHSHKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHKW::VALUE1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHKW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEVADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PerFrInt`"]
pub enum PERFRINTW {
    #[doc = "80% of the frame interval"]
    VALUE1,
    #[doc = "85%"]
    VALUE2,
    #[doc = "90%"]
    VALUE3,
    #[doc = "95%"]
    VALUE4,
}
impl PERFRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERFRINTW::VALUE1 => 0,
            PERFRINTW::VALUE2 => 1,
            PERFRINTW::VALUE3 => 2,
            PERFRINTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERFRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PERFRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERFRINTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "80% of the frame interval"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERFRINTW::VALUE1)
    }
    #[doc = "85%"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERFRINTW::VALUE2)
    }
    #[doc = "90%"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERFRINTW::VALUE3)
    }
    #[doc = "95%"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PERFRINTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DESCDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DESCDMAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PerSchIntvl`"]
pub enum PERSCHINTVLW {
    #[doc = "25% of frame."]
    VALUE1,
    #[doc = "50% of frame."]
    VALUE2,
    #[doc = "75% of frame."]
    VALUE3,
}
impl PERSCHINTVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERSCHINTVLW::VALUE1 => 0,
            PERSCHINTVLW::VALUE2 => 1,
            PERSCHINTVLW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERSCHINTVLW<'a> {
    w: &'a mut W,
}
impl<'a> _PERSCHINTVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERSCHINTVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "25% of frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERSCHINTVLW::VALUE1)
    }
    #[doc = "50% of frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERSCHINTVLW::VALUE2)
    }
    #[doc = "75% of frame."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERSCHINTVLW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline]
    pub fn dev_spd(&self) -> DEVSPDR {
        DEVSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline]
    pub fn nzsts_outhshk(&self) -> NZSTSOUTHSHKR {
        NZSTSOUTHSHKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline]
    pub fn dev_addr(&self) -> DEVADDRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVADDRR { bits }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline]
    pub fn per_fr_int(&self) -> PERFRINTR {
        PERFRINTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline]
    pub fn desc_dma(&self) -> DESCDMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DESCDMAR { bits }
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline]
    pub fn per_sch_intvl(&self) -> PERSCHINTVLR {
        PERSCHINTVLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 136314880 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline]
    pub fn dev_spd(&mut self) -> _DEVSPDW {
        _DEVSPDW { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline]
    pub fn nzsts_outhshk(&mut self) -> _NZSTSOUTHSHKW {
        _NZSTSOUTHSHKW { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline]
    pub fn dev_addr(&mut self) -> _DEVADDRW {
        _DEVADDRW { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline]
    pub fn per_fr_int(&mut self) -> _PERFRINTW {
        _PERFRINTW { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline]
    pub fn desc_dma(&mut self) -> _DESCDMAW {
        _DESCDMAW { w: self }
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline]
    pub fn per_sch_intvl(&mut self) -> _PERSCHINTVLW {
        _PERSCHINTVLW { w: self }
    }
}
