#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRG {
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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "The fractional divider frequency fFD is selected."]
    VALUE1,
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    VALUE3,
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::VALUE1 => 0,
            CLKSELR::VALUE3 => 2,
            CLKSELR::VALUE4 => 3,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::VALUE1,
            2 => CLKSELR::VALUE3,
            3 => CLKSELR::VALUE4,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLKSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLKSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLKSELR::VALUE4
    }
}
#[doc = "Possible values of the field `TMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMENR {
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    VALUE1,
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    VALUE2,
}
impl TMENR {
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
            TMENR::VALUE1 => false,
            TMENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMENR {
        match value {
            false => TMENR::VALUE1,
            true => TMENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TMENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TMENR::VALUE2
    }
}
#[doc = "Possible values of the field `PPPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPPENR {
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    VALUE1,
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    VALUE2,
}
impl PPPENR {
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
            PPPENR::VALUE1 => false,
            PPPENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPPENR {
        match value {
            false => PPPENR::VALUE1,
            true => PPPENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PPPENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PPPENR::VALUE2
    }
}
#[doc = "Possible values of the field `CTQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTQSELR {
    #[doc = "fCTQIN = fPDIV"]
    VALUE1,
    #[doc = "fCTQIN = fPPP"]
    VALUE2,
    #[doc = "fCTQIN = fSCLK"]
    VALUE3,
    #[doc = "fCTQIN = fMCLK"]
    VALUE4,
}
impl CTQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTQSELR::VALUE1 => 0,
            CTQSELR::VALUE2 => 1,
            CTQSELR::VALUE3 => 2,
            CTQSELR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTQSELR {
        match value {
            0 => CTQSELR::VALUE1,
            1 => CTQSELR::VALUE2,
            2 => CTQSELR::VALUE3,
            3 => CTQSELR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CTQSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CTQSELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CTQSELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CTQSELR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct PCTQR {
    bits: u8,
}
impl PCTQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCTQR {
    bits: u8,
}
impl DCTQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PDIVR {
    bits: u16,
}
impl PDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SCLKOSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLKOSELR {
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    VALUE1,
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    VALUE2,
}
impl SCLKOSELR {
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
            SCLKOSELR::VALUE1 => false,
            SCLKOSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLKOSELR {
        match value {
            false => SCLKOSELR::VALUE1,
            true => SCLKOSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCLKOSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCLKOSELR::VALUE2
    }
}
#[doc = "Possible values of the field `MCLKCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKCFGR {
    #[doc = "The passive level is 0."]
    VALUE1,
    #[doc = "The passive level is 1."]
    VALUE2,
}
impl MCLKCFGR {
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
            MCLKCFGR::VALUE1 => false,
            MCLKCFGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLKCFGR {
        match value {
            false => MCLKCFGR::VALUE1,
            true => MCLKCFGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MCLKCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MCLKCFGR::VALUE2
    }
}
#[doc = "Possible values of the field `SCLKCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLKCFGR {
    #[doc = "The passive level is 0 and the delay is disabled."]
    VALUE1,
    #[doc = "The passive level is 1 and the delay is disabled."]
    VALUE2,
    #[doc = "The passive level is 0 and the delay is enabled."]
    VALUE3,
    #[doc = "The passive level is 1 and the delay is enabled."]
    VALUE4,
}
impl SCLKCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLKCFGR::VALUE1 => 0,
            SCLKCFGR::VALUE2 => 1,
            SCLKCFGR::VALUE3 => 2,
            SCLKCFGR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLKCFGR {
        match value {
            0 => SCLKCFGR::VALUE1,
            1 => SCLKCFGR::VALUE2,
            2 => SCLKCFGR::VALUE3,
            3 => SCLKCFGR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCLKCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCLKCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SCLKCFGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SCLKCFGR::VALUE4
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "The fractional divider frequency fFD is selected."]
    VALUE1,
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    VALUE3,
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    VALUE4,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::VALUE1 => 0,
            CLKSELW::VALUE3 => 2,
            CLKSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The fractional divider frequency fFD is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE1)
    }
    #[doc = "The trigger signal DX1T defines fPIN. Signal MCLK toggles with fPIN."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE3)
    }
    #[doc = "Signal MCLK corresponds to the DX1S signal and the frequency fPIN is derived from the rising edges of DX1S."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKSELW::VALUE4)
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
#[doc = "Values that can be written to the field `TMEN`"]
pub enum TMENW {
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    VALUE1,
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    VALUE2,
}
impl TMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMENW::VALUE1 => false,
            TMENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timing measurement is disabled: The trigger signals DX0T and DX1T are ignored."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMENW::VALUE1)
    }
    #[doc = "Timing measurement is enabled: The 10-bit counter is incremented by 1 with fPPP and stops counting when reaching its maximum value. If one of the trigger signals DX0T or DX1T become active, the counter value is captured into bit field CTV, the counter is cleared and a transmit shift event is generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMENW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPPEN`"]
pub enum PPPENW {
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    VALUE1,
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    VALUE2,
}
impl PPPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPPENW::VALUE1 => false,
            PPPENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PPPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 2:1 divider for fPPP is disabled. fPPP = fPIN"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPPENW::VALUE1)
    }
    #[doc = "The 2:1 divider for fPPP is enabled. fPPP = fMCLK = fPIN / 2."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPPENW::VALUE2)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTQSEL`"]
pub enum CTQSELW {
    #[doc = "fCTQIN = fPDIV"]
    VALUE1,
    #[doc = "fCTQIN = fPPP"]
    VALUE2,
    #[doc = "fCTQIN = fSCLK"]
    VALUE3,
    #[doc = "fCTQIN = fMCLK"]
    VALUE4,
}
impl CTQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTQSELW::VALUE1 => 0,
            CTQSELW::VALUE2 => 1,
            CTQSELW::VALUE3 => 2,
            CTQSELW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CTQSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTQSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTQSELW::VALUE1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTQSELW::VALUE2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CTQSELW::VALUE3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CTQSELW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCTQW<'a> {
    w: &'a mut W,
}
impl<'a> _PCTQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCTQW<'a> {
    w: &'a mut W,
}
impl<'a> _DCTQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLKOSEL`"]
pub enum SCLKOSELW {
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    VALUE1,
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    VALUE2,
}
impl SCLKOSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLKOSELW::VALUE1 => false,
            SCLKOSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLKOSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLKOSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLKOSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCLK from the baud rate generator is selected as the SCLKOUT input source."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKOSELW::VALUE1)
    }
    #[doc = "The transmit shift clock from DX1 input stage is selected as the SCLKOUT input source."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKOSELW::VALUE2)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCLKCFG`"]
pub enum MCLKCFGW {
    #[doc = "The passive level is 0."]
    VALUE1,
    #[doc = "The passive level is 1."]
    VALUE2,
}
impl MCLKCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLKCFGW::VALUE1 => false,
            MCLKCFGW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLKCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLKCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLKCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The passive level is 0."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLKCFGW::VALUE1)
    }
    #[doc = "The passive level is 1."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLKCFGW::VALUE2)
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
#[doc = "Values that can be written to the field `SCLKCFG`"]
pub enum SCLKCFGW {
    #[doc = "The passive level is 0 and the delay is disabled."]
    VALUE1,
    #[doc = "The passive level is 1 and the delay is disabled."]
    VALUE2,
    #[doc = "The passive level is 0 and the delay is enabled."]
    VALUE3,
    #[doc = "The passive level is 1 and the delay is enabled."]
    VALUE4,
}
impl SCLKCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLKCFGW::VALUE1 => 0,
            SCLKCFGW::VALUE2 => 1,
            SCLKCFGW::VALUE3 => 2,
            SCLKCFGW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLKCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLKCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLKCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The passive level is 0 and the delay is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCLKCFGW::VALUE1)
    }
    #[doc = "The passive level is 1 and the delay is disabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCLKCFGW::VALUE2)
    }
    #[doc = "The passive level is 0 and the delay is enabled."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCLKCFGW::VALUE3)
    }
    #[doc = "The passive level is 1 and the delay is enabled."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCLKCFGW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline]
    pub fn tmen(&self) -> TMENR {
        TMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline]
    pub fn pppen(&self) -> PPPENR {
        PPPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline]
    pub fn ctqsel(&self) -> CTQSELR {
        CTQSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline]
    pub fn pctq(&self) -> PCTQR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCTQR { bits }
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline]
    pub fn dctq(&self) -> DCTQR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCTQR { bits }
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline]
    pub fn pdiv(&self) -> PDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PDIVR { bits }
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline]
    pub fn sclkosel(&self) -> SCLKOSELR {
        SCLKOSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline]
    pub fn mclkcfg(&self) -> MCLKCFGR {
        MCLKCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline]
    pub fn sclkcfg(&self) -> SCLKCFGR {
        SCLKCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Clock Selection"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 3 - Timing Measurement Enable"]
    #[inline]
    pub fn tmen(&mut self) -> _TMENW {
        _TMENW { w: self }
    }
    #[doc = "Bit 4 - Enable 2:1 Divider for fPPP"]
    #[inline]
    pub fn pppen(&mut self) -> _PPPENW {
        _PPPENW { w: self }
    }
    #[doc = "Bits 6:7 - Input Selection for CTQ"]
    #[inline]
    pub fn ctqsel(&mut self) -> _CTQSELW {
        _CTQSELW { w: self }
    }
    #[doc = "Bits 8:9 - Pre-Divider for Time Quanta Counter"]
    #[inline]
    pub fn pctq(&mut self) -> _PCTQW {
        _PCTQW { w: self }
    }
    #[doc = "Bits 10:14 - Denominator for Time Quanta Counter"]
    #[inline]
    pub fn dctq(&mut self) -> _DCTQW {
        _DCTQW { w: self }
    }
    #[doc = "Bits 16:25 - Divider Mode: Divider Factor to Generate fPDIV"]
    #[inline]
    pub fn pdiv(&mut self) -> _PDIVW {
        _PDIVW { w: self }
    }
    #[doc = "Bit 28 - Shift Clock Output Select"]
    #[inline]
    pub fn sclkosel(&mut self) -> _SCLKOSELW {
        _SCLKOSELW { w: self }
    }
    #[doc = "Bit 29 - Master Clock Configuration"]
    #[inline]
    pub fn mclkcfg(&mut self) -> _MCLKCFGW {
        _MCLKCFGW { w: self }
    }
    #[doc = "Bits 30:31 - Shift Clock Output Configuration"]
    #[inline]
    pub fn sclkcfg(&mut self) -> _SCLKCFGW {
        _SCLKCFGW { w: self }
    }
}
