#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSLEEPCR {
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
#[doc = "Possible values of the field `SYSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSSELR {
    #[doc = "fOFI clock"]
    CONST_0,
    #[doc = "fPLL clock"]
    CONST_1,
}
impl SYSSELR {
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
            SYSSELR::CONST_0 => false,
            SYSSELR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSSELR {
        match value {
            false => SYSSELR::CONST_0,
            true => SYSSELR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSELR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSELR::CONST_1
    }
}
#[doc = "Possible values of the field `FPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDNR {
    #[doc = "Flash power down module"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl FPDNR {
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
            FPDNR::CONST_1 => true,
            FPDNR::CONST_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPDNR {
        match value {
            true => FPDNR::CONST_1,
            false => FPDNR::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == FPDNR::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == FPDNR::CONST_0
    }
}
#[doc = "Possible values of the field `PLLPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPDNR {
    #[doc = "Switch off main PLL"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl PLLPDNR {
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
            PLLPDNR::CONST_1 => true,
            PLLPDNR::CONST_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLPDNR {
        match value {
            true => PLLPDNR::CONST_1,
            false => PLLPDNR::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == PLLPDNR::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == PLLPDNR::CONST_0
    }
}
#[doc = "Possible values of the field `VCOPDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOPDNR {
    #[doc = "Switch off VCO of main PLL"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl VCOPDNR {
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
            VCOPDNR::CONST_1 => true,
            VCOPDNR::CONST_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCOPDNR {
        match value {
            true => VCOPDNR::CONST_1,
            false => VCOPDNR::CONST_0,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == VCOPDNR::CONST_1
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == VCOPDNR::CONST_0
    }
}
#[doc = "Possible values of the field `USBCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCRR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl USBCRR {
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
            USBCRR::CONST_0 => false,
            USBCRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBCRR {
        match value {
            false => USBCRR::CONST_0,
            true => USBCRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == USBCRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == USBCRR::CONST_1
    }
}
#[doc = "Possible values of the field `MMCCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCCRR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl MMCCRR {
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
            MMCCRR::CONST_0 => false,
            MMCCRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCCRR {
        match value {
            false => MMCCRR::CONST_0,
            true => MMCCRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == MMCCRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == MMCCRR::CONST_1
    }
}
#[doc = "Possible values of the field `ETH0CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH0CRR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ETH0CRR {
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
            ETH0CRR::CONST_0 => false,
            ETH0CRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH0CRR {
        match value {
            false => ETH0CRR::CONST_0,
            true => ETH0CRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0CRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0CRR::CONST_1
    }
}
#[doc = "Possible values of the field `CCUCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCRR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl CCUCRR {
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
            CCUCRR::CONST_0 => false,
            CCUCRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUCRR {
        match value {
            false => CCUCRR::CONST_0,
            true => CCUCRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == CCUCRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == CCUCRR::CONST_1
    }
}
#[doc = "Possible values of the field `WDTCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCRR {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl WDTCRR {
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
            WDTCRR::CONST_0 => false,
            WDTCRR::CONST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTCRR {
        match value {
            false => WDTCRR::CONST_0,
            true => WDTCRR::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline]
    pub fn is_const_0(&self) -> bool {
        *self == WDTCRR::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline]
    pub fn is_const_1(&self) -> bool {
        *self == WDTCRR::CONST_1
    }
}
#[doc = "Values that can be written to the field `SYSSEL`"]
pub enum SYSSELW {
    #[doc = "fOFI clock"]
    CONST_0,
    #[doc = "fPLL clock"]
    CONST_1,
}
impl SYSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSSELW::CONST_0 => false,
            SYSSELW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fOFI clock"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SYSSELW::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SYSSELW::CONST_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPDN`"]
pub enum FPDNW {
    #[doc = "Flash power down module"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl FPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPDNW::CONST_1 => true,
            FPDNW::CONST_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _FPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash power down module"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FPDNW::CONST_1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FPDNW::CONST_0)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLPDN`"]
pub enum PLLPDNW {
    #[doc = "Switch off main PLL"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl PLLPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLPDNW::CONST_1 => true,
            PLLPDNW::CONST_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PLLPDNW::CONST_1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PLLPDNW::CONST_0)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VCOPDN`"]
pub enum VCOPDNW {
    #[doc = "Switch off VCO of main PLL"]
    CONST_1,
    #[doc = "No effect"]
    CONST_0,
}
impl VCOPDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VCOPDNW::CONST_1 => true,
            VCOPDNW::CONST_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VCOPDNW<'a> {
    w: &'a mut W,
}
impl<'a> _VCOPDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VCOPDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(VCOPDNW::CONST_1)
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(VCOPDNW::CONST_0)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBCR`"]
pub enum USBCRW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl USBCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBCRW::CONST_0 => false,
            USBCRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCRW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBCRW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBCRW::CONST_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MMCCR`"]
pub enum MMCCRW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl MMCCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCCRW::CONST_0 => false,
            MMCCRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCCRW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCCRW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCCRW::CONST_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETH0CR`"]
pub enum ETH0CRW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl ETH0CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH0CRW::CONST_0 => false,
            ETH0CRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH0CRW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH0CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH0CRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ETH0CRW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ETH0CRW::CONST_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCUCR`"]
pub enum CCUCRW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl CCUCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUCRW::CONST_0 => false,
            CCUCRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUCRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(CCUCRW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(CCUCRW::CONST_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTCR`"]
pub enum WDTCRW {
    #[doc = "Disabled"]
    CONST_0,
    #[doc = "Enabled"]
    CONST_1,
}
impl WDTCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTCRW::CONST_0 => false,
            WDTCRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTCRW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(WDTCRW::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(WDTCRW::CONST_1)
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&self) -> SYSSELR {
        SYSSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline]
    pub fn fpdn(&self) -> FPDNR {
        FPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline]
    pub fn pllpdn(&self) -> PLLPDNR {
        PLLPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline]
    pub fn vcopdn(&self) -> VCOPDNR {
        VCOPDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn usbcr(&self) -> USBCRR {
        USBCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn mmccr(&self) -> MMCCRR {
        MMCCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn eth0cr(&self) -> ETH0CRR {
        ETH0CRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn ccucr(&self) -> CCUCRR {
        CCUCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn wdtcr(&self) -> WDTCRR {
        WDTCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline]
    pub fn syssel(&mut self) -> _SYSSELW {
        _SYSSELW { w: self }
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline]
    pub fn fpdn(&mut self) -> _FPDNW {
        _FPDNW { w: self }
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline]
    pub fn pllpdn(&mut self) -> _PLLPDNW {
        _PLLPDNW { w: self }
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline]
    pub fn vcopdn(&mut self) -> _VCOPDNW {
        _VCOPDNW { w: self }
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn usbcr(&mut self) -> _USBCRW {
        _USBCRW { w: self }
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn mmccr(&mut self) -> _MMCCRW {
        _MMCCRW { w: self }
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn eth0cr(&mut self) -> _ETH0CRW {
        _ETH0CRW { w: self }
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn ccucr(&mut self) -> _CCUCRW {
        _CCUCRW { w: self }
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline]
    pub fn wdtcr(&mut self) -> _WDTCRW {
        _WDTCRW { w: self }
    }
}
