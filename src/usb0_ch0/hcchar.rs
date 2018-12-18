#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCCHAR {
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
pub struct EPNUMR {
    bits: u8,
}
impl EPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EPDir`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIRR {
    #[doc = "OUT"]
    VALUE1,
    #[doc = "IN"]
    VALUE2,
}
impl EPDIRR {
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
            EPDIRR::VALUE1 => false,
            EPDIRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPDIRR {
        match value {
            false => EPDIRR::VALUE1,
            true => EPDIRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EPDIRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EPDIRR::VALUE2
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
#[doc = "Possible values of the field `MC_EC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_ECR {
    #[doc = "1 transaction"]
    VALUE2,
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    VALUE3,
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MC_ECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MC_ECR::VALUE2 => 1,
            MC_ECR::VALUE3 => 2,
            MC_ECR::VALUE4 => 3,
            MC_ECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MC_ECR {
        match value {
            1 => MC_ECR::VALUE2,
            2 => MC_ECR::VALUE3,
            3 => MC_ECR::VALUE4,
            i => MC_ECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MC_ECR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MC_ECR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MC_ECR::VALUE4
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
#[doc = "Possible values of the field `OddFrm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODDFRMR {
    #[doc = "Even frame"]
    VALUE1,
    #[doc = "Odd frame"]
    VALUE2,
}
impl ODDFRMR {
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
            ODDFRMR::VALUE1 => false,
            ODDFRMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODDFRMR {
        match value {
            false => ODDFRMR::VALUE1,
            true => ODDFRMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ODDFRMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ODDFRMR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CHDISR {
    bits: bool,
}
impl CHDISR {
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
#[doc = "Possible values of the field `ChEna`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHENAR {
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    VALUE1,
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    VALUE2,
}
impl CHENAR {
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
            CHENAR::VALUE1 => false,
            CHENAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHENAR {
        match value {
            false => CHENAR::VALUE1,
            true => CHENAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHENAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CHENAR::VALUE2
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
pub struct _EPNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPDir`"]
pub enum EPDIRW {
    #[doc = "OUT"]
    VALUE1,
    #[doc = "IN"]
    VALUE2,
}
impl EPDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPDIRW::VALUE1 => false,
            EPDIRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OUT"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPDIRW::VALUE1)
    }
    #[doc = "IN"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPDIRW::VALUE2)
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
#[doc = "Values that can be written to the field `MC_EC`"]
pub enum MC_ECW {
    #[doc = "1 transaction"]
    VALUE2,
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    VALUE3,
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    VALUE4,
}
impl MC_ECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MC_ECW::VALUE2 => 1,
            MC_ECW::VALUE3 => 2,
            MC_ECW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MC_ECW<'a> {
    w: &'a mut W,
}
impl<'a> _MC_ECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MC_ECW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 transaction"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MC_ECW::VALUE2)
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MC_ECW::VALUE3)
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MC_ECW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OddFrm`"]
pub enum ODDFRMW {
    #[doc = "Even frame"]
    VALUE1,
    #[doc = "Odd frame"]
    VALUE2,
}
impl ODDFRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODDFRMW::VALUE1 => false,
            ODDFRMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODDFRMW<'a> {
    w: &'a mut W,
}
impl<'a> _ODDFRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODDFRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even frame"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ODDFRMW::VALUE1)
    }
    #[doc = "Odd frame"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ODDFRMW::VALUE2)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CHDISW<'a> {
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
#[doc = "Values that can be written to the field `ChEna`"]
pub enum CHENAW {
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    VALUE1,
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    VALUE2,
}
impl CHENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHENAW::VALUE1 => false,
            CHENAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CHENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHENAW::VALUE1)
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHENAW::VALUE2)
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
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline]
    pub fn epnum(&self) -> EPNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPNUMR { bits }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&self) -> EPDIRR {
        EPDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline]
    pub fn mc_ec(&self) -> MC_ECR {
        MC_ECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline]
    pub fn dev_addr(&self) -> DEVADDRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVADDRR { bits }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline]
    pub fn odd_frm(&self) -> ODDFRMR {
        ODDFRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline]
    pub fn ch_dis(&self) -> CHDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHDISR { bits }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline]
    pub fn ch_ena(&self) -> CHENAR {
        CHENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&mut self) -> _MPSW {
        _MPSW { w: self }
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline]
    pub fn epnum(&mut self) -> _EPNUMW {
        _EPNUMW { w: self }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&mut self) -> _EPDIRW {
        _EPDIRW { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline]
    pub fn mc_ec(&mut self) -> _MC_ECW {
        _MC_ECW { w: self }
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline]
    pub fn dev_addr(&mut self) -> _DEVADDRW {
        _DEVADDRW { w: self }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline]
    pub fn odd_frm(&mut self) -> _ODDFRMW {
        _ODDFRMW { w: self }
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline]
    pub fn ch_dis(&mut self) -> _CHDISW {
        _CHDISW { w: self }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline]
    pub fn ch_ena(&mut self) -> _CHENAW {
        _CHENAW { w: self }
    }
}
