#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGL {
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
pub struct RELOAD_DSTR {
    bits: bool,
}
impl RELOAD_DSTR {
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
pub struct RELOAD_SRCR {
    bits: bool,
}
impl RELOAD_SRCR {
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
pub struct MAX_ABRSTR {
    bits: u16,
}
impl MAX_ABRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRC_HS_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HS_POLR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl SRC_HS_POLR {
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
            SRC_HS_POLR::VALUE1 => false,
            SRC_HS_POLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRC_HS_POLR {
        match value {
            false => SRC_HS_POLR::VALUE1,
            true => SRC_HS_POLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRC_HS_POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRC_HS_POLR::VALUE2
    }
}
#[doc = "Possible values of the field `DST_HS_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_HS_POLR {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl DST_HS_POLR {
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
            DST_HS_POLR::VALUE1 => false,
            DST_HS_POLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DST_HS_POLR {
        match value {
            false => DST_HS_POLR::VALUE1,
            true => DST_HS_POLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DST_HS_POLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DST_HS_POLR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_BR {
    bits: bool,
}
impl LOCK_BR {
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
pub struct LOCK_CHR {
    bits: bool,
}
impl LOCK_CHR {
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
#[doc = "Possible values of the field `LOCK_B_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_B_LR {
    #[doc = "Over complete DMA transfer"]
    VALUE1,
    #[doc = "Over complete DMA block transfer"]
    VALUE2,
    #[doc = "Over complete DMA transaction"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_B_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_B_LR::VALUE1 => 0,
            LOCK_B_LR::VALUE2 => 1,
            LOCK_B_LR::VALUE3 => 2,
            LOCK_B_LR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_B_LR {
        match value {
            0 => LOCK_B_LR::VALUE1,
            1 => LOCK_B_LR::VALUE2,
            2 => LOCK_B_LR::VALUE3,
            i => LOCK_B_LR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_B_LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_B_LR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_B_LR::VALUE3
    }
}
#[doc = "Possible values of the field `LOCK_CH_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_CH_LR {
    #[doc = "Over complete DMA transfer"]
    VALUE1,
    #[doc = "Over complete DMA block transfer"]
    VALUE2,
    #[doc = "Over complete DMA transaction"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_CH_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_CH_LR::VALUE1 => 0,
            LOCK_CH_LR::VALUE2 => 1,
            LOCK_CH_LR::VALUE3 => 2,
            LOCK_CH_LR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_CH_LR {
        match value {
            0 => LOCK_CH_LR::VALUE1,
            1 => LOCK_CH_LR::VALUE2,
            2 => LOCK_CH_LR::VALUE3,
            i => LOCK_CH_LR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_CH_LR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_CH_LR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_CH_LR::VALUE3
    }
}
#[doc = "Possible values of the field `HS_SEL_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRCR {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1,
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2,
}
impl HS_SEL_SRCR {
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
            HS_SEL_SRCR::VALUE1 => false,
            HS_SEL_SRCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_SEL_SRCR {
        match value {
            false => HS_SEL_SRCR::VALUE1,
            true => HS_SEL_SRCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_SRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_SRCR::VALUE2
    }
}
#[doc = "Possible values of the field `HS_SEL_DST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_DSTR {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1,
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2,
}
impl HS_SEL_DSTR {
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
            HS_SEL_DSTR::VALUE1 => false,
            HS_SEL_DSTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_SEL_DSTR {
        match value {
            false => HS_SEL_DSTR::VALUE1,
            true => HS_SEL_DSTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_DSTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_DSTR::VALUE2
    }
}
#[doc = "Possible values of the field `FIFO_EMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_EMPTYR {
    #[doc = "Channel FIFO empty"]
    VALUE1,
    #[doc = "Channel FIFO not empty"]
    VALUE2,
}
impl FIFO_EMPTYR {
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
            FIFO_EMPTYR::VALUE1 => true,
            FIFO_EMPTYR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_EMPTYR {
        match value {
            true => FIFO_EMPTYR::VALUE1,
            false => FIFO_EMPTYR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_EMPTYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_EMPTYR::VALUE2
    }
}
#[doc = "Possible values of the field `CH_SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_SUSPR {
    #[doc = "Not suspended."]
    VALUE1,
    #[doc = "Suspend DMA transfer from the source."]
    VALUE2,
}
impl CH_SUSPR {
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
            CH_SUSPR::VALUE1 => false,
            CH_SUSPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH_SUSPR {
        match value {
            false => CH_SUSPR::VALUE1,
            true => CH_SUSPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CH_SUSPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CH_SUSPR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct CH_PRIORR {
    bits: u8,
}
impl CH_PRIORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RELOAD_DSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOAD_DSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RELOAD_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOAD_SRCW<'a> {
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
pub struct _MAX_ABRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_ABRSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC_HS_POL`"]
pub enum SRC_HS_POLW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl SRC_HS_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRC_HS_POLW::VALUE1 => false,
            SRC_HS_POLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_HS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_HS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_HS_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_HS_POLW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_HS_POLW::VALUE2)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_HS_POL`"]
pub enum DST_HS_POLW {
    #[doc = "Active high"]
    VALUE1,
    #[doc = "Active low"]
    VALUE2,
}
impl DST_HS_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DST_HS_POLW::VALUE1 => false,
            DST_HS_POLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_HS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_HS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_HS_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active high"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_HS_POLW::VALUE1)
    }
    #[doc = "Active low"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_HS_POLW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _LOCK_BW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_BW<'a> {
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
#[doc = r" Proxy"]
pub struct _LOCK_CHW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_CHW<'a> {
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
#[doc = "Values that can be written to the field `LOCK_B_L`"]
pub enum LOCK_B_LW {
    #[doc = "Over complete DMA transfer"]
    VALUE1,
    #[doc = "Over complete DMA block transfer"]
    VALUE2,
    #[doc = "Over complete DMA transaction"]
    VALUE3,
}
impl LOCK_B_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_B_LW::VALUE1 => 0,
            LOCK_B_LW::VALUE2 => 1,
            LOCK_B_LW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_B_LW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_B_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_B_LW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_B_LW::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_B_LW::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_B_LW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_CH_L`"]
pub enum LOCK_CH_LW {
    #[doc = "Over complete DMA transfer"]
    VALUE1,
    #[doc = "Over complete DMA block transfer"]
    VALUE2,
    #[doc = "Over complete DMA transaction"]
    VALUE3,
}
impl LOCK_CH_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_CH_LW::VALUE1 => 0,
            LOCK_CH_LW::VALUE2 => 1,
            LOCK_CH_LW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_CH_LW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_CH_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_CH_LW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_CH_LW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HS_SEL_SRC`"]
pub enum HS_SEL_SRCW {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1,
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2,
}
impl HS_SEL_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_SEL_SRCW::VALUE1 => false,
            HS_SEL_SRCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_SEL_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_SEL_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_SEL_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_SRCW::VALUE2)
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
#[doc = "Values that can be written to the field `HS_SEL_DST`"]
pub enum HS_SEL_DSTW {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1,
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2,
}
impl HS_SEL_DSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_SEL_DSTW::VALUE1 => false,
            HS_SEL_DSTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_SEL_DSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_SEL_DSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_SEL_DSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_DSTW::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_DSTW::VALUE2)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH_SUSP`"]
pub enum CH_SUSPW {
    #[doc = "Not suspended."]
    VALUE1,
    #[doc = "Suspend DMA transfer from the source."]
    VALUE2,
}
impl CH_SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH_SUSPW::VALUE1 => false,
            CH_SUSPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH_SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_SUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_SUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not suspended."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_SUSPW::VALUE1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_SUSPW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _CH_PRIORW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_PRIORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline]
    pub fn reload_dst(&self) -> RELOAD_DSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RELOAD_DSTR { bits }
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline]
    pub fn reload_src(&self) -> RELOAD_SRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RELOAD_SRCR { bits }
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline]
    pub fn max_abrst(&self) -> MAX_ABRSTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MAX_ABRSTR { bits }
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline]
    pub fn src_hs_pol(&self) -> SRC_HS_POLR {
        SRC_HS_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline]
    pub fn dst_hs_pol(&self) -> DST_HS_POLR {
        DST_HS_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline]
    pub fn lock_b(&self) -> LOCK_BR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCK_BR { bits }
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline]
    pub fn lock_ch(&self) -> LOCK_CHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCK_CHR { bits }
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline]
    pub fn lock_b_l(&self) -> LOCK_B_LR {
        LOCK_B_LR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline]
    pub fn lock_ch_l(&self) -> LOCK_CH_LR {
        LOCK_CH_LR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline]
    pub fn hs_sel_src(&self) -> HS_SEL_SRCR {
        HS_SEL_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline]
    pub fn hs_sel_dst(&self) -> HS_SEL_DSTR {
        HS_SEL_DSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline]
    pub fn fifo_empty(&self) -> FIFO_EMPTYR {
        FIFO_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline]
    pub fn ch_susp(&self) -> CH_SUSPR {
        CH_SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline]
    pub fn ch_prior(&self) -> CH_PRIORR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH_PRIORR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3584 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline]
    pub fn reload_dst(&mut self) -> _RELOAD_DSTW {
        _RELOAD_DSTW { w: self }
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline]
    pub fn reload_src(&mut self) -> _RELOAD_SRCW {
        _RELOAD_SRCW { w: self }
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline]
    pub fn max_abrst(&mut self) -> _MAX_ABRSTW {
        _MAX_ABRSTW { w: self }
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline]
    pub fn src_hs_pol(&mut self) -> _SRC_HS_POLW {
        _SRC_HS_POLW { w: self }
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline]
    pub fn dst_hs_pol(&mut self) -> _DST_HS_POLW {
        _DST_HS_POLW { w: self }
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline]
    pub fn lock_b(&mut self) -> _LOCK_BW {
        _LOCK_BW { w: self }
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline]
    pub fn lock_ch(&mut self) -> _LOCK_CHW {
        _LOCK_CHW { w: self }
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline]
    pub fn lock_b_l(&mut self) -> _LOCK_B_LW {
        _LOCK_B_LW { w: self }
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline]
    pub fn lock_ch_l(&mut self) -> _LOCK_CH_LW {
        _LOCK_CH_LW { w: self }
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline]
    pub fn hs_sel_src(&mut self) -> _HS_SEL_SRCW {
        _HS_SEL_SRCW { w: self }
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline]
    pub fn hs_sel_dst(&mut self) -> _HS_SEL_DSTW {
        _HS_SEL_DSTW { w: self }
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline]
    pub fn ch_susp(&mut self) -> _CH_SUSPW {
        _CH_SUSPW { w: self }
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline]
    pub fn ch_prior(&mut self) -> _CH_PRIORW {
        _CH_PRIORW { w: self }
    }
}
