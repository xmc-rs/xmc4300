#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIEPCTL_INTBULK {
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
#[doc = r" Value of the field"]
pub struct MPSR {
    bits: u16,
}
impl MPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBACTEPR {
    bits: bool,
}
impl USBACTEPR {
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
#[doc = "Possible values of the field `DPID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIDR {
    #[doc = "DATA0"]
    VALUE1,
    #[doc = "DATA1"]
    VALUE2,
}
impl DPIDR {
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
            DPIDR::VALUE1 => false,
            DPIDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPIDR {
        match value {
            false => DPIDR::VALUE1,
            true => DPIDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DPIDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DPIDR::VALUE2
    }
}
#[doc = "Possible values of the field `NAKSts`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAKSTSR {
    #[doc = "The core is transmitting non-NAK handshakes based on the FIFO status."]
    VALUE1,
    #[doc = "The core is transmitting NAK handshakes on this endpoint."]
    VALUE2,
}
impl NAKSTSR {
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
            NAKSTSR::VALUE1 => false,
            NAKSTSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NAKSTSR {
        match value {
            false => NAKSTSR::VALUE1,
            true => NAKSTSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NAKSTSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NAKSTSR::VALUE2
    }
}
#[doc = "Possible values of the field `EPType`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPER {
    #[doc = "Control"]
    VALUE1,
    #[doc = "Isochronous"]
    VALUE2,
    #[doc = "Bulk"]
    VALUE3,
    #[doc = "Interrupt"]
    VALUE4,
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPTYPER::VALUE1 => 0,
            EPTYPER::VALUE2 => 1,
            EPTYPER::VALUE3 => 2,
            EPTYPER::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPTYPER {
        match value {
            0 => EPTYPER::VALUE1,
            1 => EPTYPER::VALUE2,
            2 => EPTYPER::VALUE3,
            3 => EPTYPER::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EPTYPER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EPTYPER::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EPTYPER::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EPTYPER::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct SNPR {
    bits: bool,
}
impl SNPR {
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
#[doc = r" Value of the field"]
pub struct STALLR {
    bits: bool,
}
impl STALLR {
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
#[doc = r" Value of the field"]
pub struct TXFNUMR {
    bits: u8,
}
impl TXFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPDISR {
    bits: bool,
}
impl EPDISR {
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
#[doc = r" Value of the field"]
pub struct EPENAR {
    bits: bool,
}
impl EPENAR {
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
#[doc = r" Proxy"]
pub struct _MPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MPSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBACTEPW<'a> {
    w: &'a mut W,
}
impl<'a> _USBACTEPW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPType`"]
pub enum EPTYPEW {
    #[doc = "Control"]
    VALUE1,
    #[doc = "Isochronous"]
    VALUE2,
    #[doc = "Bulk"]
    VALUE3,
    #[doc = "Interrupt"]
    VALUE4,
}
impl EPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPTYPEW::VALUE1 => 0,
            EPTYPEW::VALUE2 => 1,
            EPTYPEW::VALUE3 => 2,
            EPTYPEW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPTYPEW::VALUE1)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPTYPEW::VALUE2)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EPTYPEW::VALUE3)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EPTYPEW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SNPW<'a> {
    w: &'a mut W,
}
impl<'a> _SNPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CNAKW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SNAKW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SETD0PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _SETD0PIDW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SETD1PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _SETD1PIDW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDISW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPENAW<'a> {
    w: &'a mut W,
}
impl<'a> _EPENAW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&self) -> MPSR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MPSR { bits }
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline]
    pub fn usbact_ep(&self) -> USBACTEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBACTEPR { bits }
    }
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        DPIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline]
    pub fn naksts(&self) -> NAKSTSR {
        NAKSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        EPTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline]
    pub fn snp(&self) -> SNPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SNPR { bits }
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline]
    pub fn stall(&self) -> STALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STALLR { bits }
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline]
    pub fn tx_fnum(&self) -> TXFNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXFNUMR { bits }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline]
    pub fn epdis(&self) -> EPDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPDISR { bits }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline]
    pub fn epena(&self) -> EPENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPENAR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&mut self) -> _MPSW {
        _MPSW { w: self }
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline]
    pub fn usbact_ep(&mut self) -> _USBACTEPW {
        _USBACTEPW { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline]
    pub fn snp(&mut self) -> _SNPW {
        _SNPW { w: self }
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline]
    pub fn stall(&mut self) -> _STALLW {
        _STALLW { w: self }
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline]
    pub fn tx_fnum(&mut self) -> _TXFNUMW {
        _TXFNUMW { w: self }
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline]
    pub fn cnak(&mut self) -> _CNAKW {
        _CNAKW { w: self }
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline]
    pub fn snak(&mut self) -> _SNAKW {
        _SNAKW { w: self }
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline]
    pub fn set_d0pid(&mut self) -> _SETD0PIDW {
        _SETD0PIDW { w: self }
    }
    #[doc = "Bit 29 - 29 Set DATA1 PID"]
    #[inline]
    pub fn set_d1pid(&mut self) -> _SETD1PIDW {
        _SETD1PIDW { w: self }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline]
    pub fn epdis(&mut self) -> _EPDISW {
        _EPDISW { w: self }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline]
    pub fn epena(&mut self) -> _EPENAW {
        _EPENAW { w: self }
    }
}
