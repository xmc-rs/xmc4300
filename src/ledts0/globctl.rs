#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBCTL {
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
pub struct TS_ENR {
    bits: bool,
}
impl TS_ENR {
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
pub struct LD_ENR {
    bits: bool,
}
impl LD_ENR {
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
#[doc = "Possible values of the field `CMTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTRR {
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    VALUE1,
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    VALUE2,
}
impl CMTRR {
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
            CMTRR::VALUE1 => false,
            CMTRR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMTRR {
        match value {
            false => CMTRR::VALUE1,
            true => CMTRR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMTRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMTRR::VALUE2
    }
}
#[doc = "Possible values of the field `ENSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSYNCR {
    #[doc = "No synchronization"]
    VALUE1,
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    VALUE2,
}
impl ENSYNCR {
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
            ENSYNCR::VALUE1 => false,
            ENSYNCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENSYNCR {
        match value {
            false => ENSYNCR::VALUE1,
            true => ENSYNCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENSYNCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ENSYNCR::VALUE2
    }
}
#[doc = "Possible values of the field `SUSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFGR {
    #[doc = "Ignore suspend request"]
    VALUE1,
    #[doc = "Enable suspend according to request"]
    VALUE2,
}
impl SUSCFGR {
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
            SUSCFGR::VALUE1 => false,
            SUSCFGR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSCFGR {
        match value {
            false => SUSCFGR::VALUE1,
            true => SUSCFGR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFGR::VALUE2
    }
}
#[doc = "Possible values of the field `MASKVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKVALR {
    #[doc = "Mask LSB bit"]
    VALUE1,
    #[doc = "Mask 2 LSB bits"]
    VALUE2,
    #[doc = "Mask 8 LSB bits"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKVALR::VALUE1 => 0,
            MASKVALR::VALUE2 => 1,
            MASKVALR::VALUE3 => 7,
            MASKVALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKVALR {
        match value {
            0 => MASKVALR::VALUE1,
            1 => MASKVALR::VALUE2,
            7 => MASKVALR::VALUE3,
            i => MASKVALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MASKVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MASKVALR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MASKVALR::VALUE3
    }
}
#[doc = "Possible values of the field `FENVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FENVALR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl FENVALR {
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
            FENVALR::VALUE1 => false,
            FENVALR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FENVALR {
        match value {
            false => FENVALR::VALUE1,
            true => FENVALR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FENVALR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FENVALR::VALUE2
    }
}
#[doc = "Possible values of the field `ITS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITS_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ITS_ENR {
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
            ITS_ENR::VALUE1 => false,
            ITS_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITS_ENR {
        match value {
            false => ITS_ENR::VALUE1,
            true => ITS_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ITS_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ITS_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `ITF_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITF_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ITF_ENR {
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
            ITF_ENR::VALUE1 => false,
            ITF_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITF_ENR {
        match value {
            false => ITF_ENR::VALUE1,
            true => ITF_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ITF_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ITF_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `ITP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITP_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    VALUE2,
}
impl ITP_ENR {
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
            ITP_ENR::VALUE1 => false,
            ITP_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITP_ENR {
        match value {
            false => ITP_ENR::VALUE1,
            true => ITP_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ITP_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ITP_ENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CLK_PSR {
    bits: u16,
}
impl CLK_PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TS_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _LD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LD_ENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMTR`"]
pub enum CMTRW {
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    VALUE1,
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    VALUE2,
}
impl CMTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMTRW::VALUE1 => false,
            CMTRW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMTRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMTRW::VALUE1)
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMTRW::VALUE2)
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
#[doc = "Values that can be written to the field `ENSYNC`"]
pub enum ENSYNCW {
    #[doc = "No synchronization"]
    VALUE1,
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    VALUE2,
}
impl ENSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENSYNCW::VALUE1 => false,
            ENSYNCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No synchronization"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENSYNCW::VALUE1)
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENSYNCW::VALUE2)
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
#[doc = "Values that can be written to the field `SUSCFG`"]
pub enum SUSCFGW {
    #[doc = "Ignore suspend request"]
    VALUE1,
    #[doc = "Enable suspend according to request"]
    VALUE2,
}
impl SUSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSCFGW::VALUE1 => false,
            SUSCFGW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ignore suspend request"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE1)
    }
    #[doc = "Enable suspend according to request"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFGW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASKVAL`"]
pub enum MASKVALW {
    #[doc = "Mask LSB bit"]
    VALUE1,
    #[doc = "Mask 2 LSB bits"]
    VALUE2,
    #[doc = "Mask 8 LSB bits"]
    VALUE3,
}
impl MASKVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKVALW::VALUE1 => 0,
            MASKVALW::VALUE2 => 1,
            MASKVALW::VALUE3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKVALW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKVALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Mask LSB bit"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MASKVALW::VALUE1)
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MASKVALW::VALUE2)
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MASKVALW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FENVAL`"]
pub enum FENVALW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl FENVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FENVALW::VALUE1 => false,
            FENVALW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FENVALW<'a> {
    w: &'a mut W,
}
impl<'a> _FENVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FENVALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FENVALW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FENVALW::VALUE2)
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
#[doc = "Values that can be written to the field `ITS_EN`"]
pub enum ITS_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ITS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITS_ENW::VALUE1 => false,
            ITS_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITS_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITS_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `ITF_EN`"]
pub enum ITF_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl ITF_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITF_ENW::VALUE1 => false,
            ITF_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITF_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITF_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITF_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITF_ENW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITP_EN`"]
pub enum ITP_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    VALUE2,
}
impl ITP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITP_ENW::VALUE1 => false,
            ITP_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ITP_ENW::VALUE1)
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ITP_ENW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CLK_PSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_PSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline]
    pub fn ts_en(&self) -> TS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TS_ENR { bits }
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline]
    pub fn ld_en(&self) -> LD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LD_ENR { bits }
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline]
    pub fn cmtr(&self) -> CMTRR {
        CMTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline]
    pub fn ensync(&self) -> ENSYNCR {
        ENSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline]
    pub fn suscfg(&self) -> SUSCFGR {
        SUSCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline]
    pub fn maskval(&self) -> MASKVALR {
        MASKVALR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline]
    pub fn fenval(&self) -> FENVALR {
        FENVALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline]
    pub fn its_en(&self) -> ITS_ENR {
        ITS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline]
    pub fn itf_en(&self) -> ITF_ENR {
        ITF_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline]
    pub fn itp_en(&self) -> ITP_ENR {
        ITP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline]
    pub fn clk_ps(&self) -> CLK_PSR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CLK_PSR { bits }
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
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline]
    pub fn ts_en(&mut self) -> _TS_ENW {
        _TS_ENW { w: self }
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline]
    pub fn ld_en(&mut self) -> _LD_ENW {
        _LD_ENW { w: self }
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline]
    pub fn cmtr(&mut self) -> _CMTRW {
        _CMTRW { w: self }
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline]
    pub fn ensync(&mut self) -> _ENSYNCW {
        _ENSYNCW { w: self }
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline]
    pub fn suscfg(&mut self) -> _SUSCFGW {
        _SUSCFGW { w: self }
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline]
    pub fn maskval(&mut self) -> _MASKVALW {
        _MASKVALW { w: self }
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline]
    pub fn fenval(&mut self) -> _FENVALW {
        _FENVALW { w: self }
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline]
    pub fn its_en(&mut self) -> _ITS_ENW {
        _ITS_ENW { w: self }
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline]
    pub fn itf_en(&mut self) -> _ITF_ENW {
        _ITF_ENW { w: self }
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline]
    pub fn itp_en(&mut self) -> _ITP_ENW {
        _ITP_ENW { w: self }
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline]
    pub fn clk_ps(&mut self) -> _CLK_PSW {
        _CLK_PSW { w: self }
    }
}
