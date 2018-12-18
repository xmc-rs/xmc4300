#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTLL {
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
pub struct LLP_SRC_ENR {
    bits: bool,
}
impl LLP_SRC_ENR {
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
pub struct LLP_DST_ENR {
    bits: bool,
}
impl LLP_DST_ENR {
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
pub struct TT_FCR {
    bits: u8,
}
impl TT_FCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DST_SCATTER_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_SCATTER_ENR {
    #[doc = "Scatter disabled"]
    VALUE1,
    #[doc = "Scatter enabled"]
    VALUE2,
}
impl DST_SCATTER_ENR {
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
            DST_SCATTER_ENR::VALUE1 => false,
            DST_SCATTER_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DST_SCATTER_ENR {
        match value {
            false => DST_SCATTER_ENR::VALUE1,
            true => DST_SCATTER_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DST_SCATTER_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DST_SCATTER_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `SRC_GATHER_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_GATHER_ENR {
    #[doc = "Gather disabled"]
    VALUE1,
    #[doc = "Gather enabled"]
    VALUE2,
}
impl SRC_GATHER_ENR {
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
            SRC_GATHER_ENR::VALUE1 => false,
            SRC_GATHER_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRC_GATHER_ENR {
        match value {
            false => SRC_GATHER_ENR::VALUE1,
            true => SRC_GATHER_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SRC_GATHER_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SRC_GATHER_ENR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SRC_MSIZER {
    bits: u8,
}
impl SRC_MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEST_MSIZER {
    bits: u8,
}
impl DEST_MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINCR {
    #[doc = "Increment"]
    VALUE1,
    #[doc = "Decrement"]
    VALUE2,
    #[doc = "No change"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SINCR::VALUE1 => 0,
            SINCR::VALUE2 => 1,
            SINCR::VALUE3 => 2,
            SINCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SINCR {
        match value {
            0 => SINCR::VALUE1,
            1 => SINCR::VALUE2,
            2 => SINCR::VALUE3,
            i => SINCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SINCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SINCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SINCR::VALUE3
    }
}
#[doc = "Possible values of the field `DINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINCR {
    #[doc = "Increment"]
    VALUE1,
    #[doc = "Decrement"]
    VALUE2,
    #[doc = "No change"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DINCR::VALUE1 => 0,
            DINCR::VALUE2 => 1,
            DINCR::VALUE3 => 2,
            DINCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DINCR {
        match value {
            0 => DINCR::VALUE1,
            1 => DINCR::VALUE2,
            2 => DINCR::VALUE3,
            i => DINCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DINCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DINCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DINCR::VALUE3
    }
}
#[doc = r" Value of the field"]
pub struct SRC_TR_WIDTHR {
    bits: u8,
}
impl SRC_TR_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DST_TR_WIDTHR {
    bits: u8,
}
impl DST_TR_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT_ENR {
    bits: bool,
}
impl INT_ENR {
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
pub struct _LLP_SRC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LLP_SRC_ENW<'a> {
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
pub struct _LLP_DST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LLP_DST_ENW<'a> {
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
pub struct _TT_FCW<'a> {
    w: &'a mut W,
}
impl<'a> _TT_FCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_SCATTER_EN`"]
pub enum DST_SCATTER_ENW {
    #[doc = "Scatter disabled"]
    VALUE1,
    #[doc = "Scatter enabled"]
    VALUE2,
}
impl DST_SCATTER_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DST_SCATTER_ENW::VALUE1 => false,
            DST_SCATTER_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_SCATTER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_SCATTER_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_SCATTER_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scatter disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_SCATTER_ENW::VALUE1)
    }
    #[doc = "Scatter enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_SCATTER_ENW::VALUE2)
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
#[doc = "Values that can be written to the field `SRC_GATHER_EN`"]
pub enum SRC_GATHER_ENW {
    #[doc = "Gather disabled"]
    VALUE1,
    #[doc = "Gather enabled"]
    VALUE2,
}
impl SRC_GATHER_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRC_GATHER_ENW::VALUE1 => false,
            SRC_GATHER_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_GATHER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_GATHER_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_GATHER_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gather disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_GATHER_ENW::VALUE1)
    }
    #[doc = "Gather enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_GATHER_ENW::VALUE2)
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
#[doc = r" Proxy"]
pub struct _SRC_MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_MSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEST_MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEST_MSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SINC`"]
pub enum SINCW {
    #[doc = "Increment"]
    VALUE1,
    #[doc = "Decrement"]
    VALUE2,
    #[doc = "No change"]
    VALUE3,
}
impl SINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SINCW::VALUE1 => 0,
            SINCW::VALUE2 => 1,
            SINCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Increment"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SINCW::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SINCW::VALUE2)
    }
    #[doc = "No change"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SINCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DINC`"]
pub enum DINCW {
    #[doc = "Increment"]
    VALUE1,
    #[doc = "Decrement"]
    VALUE2,
    #[doc = "No change"]
    VALUE3,
}
impl DINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DINCW::VALUE1 => 0,
            DINCW::VALUE2 => 1,
            DINCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Increment"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DINCW::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DINCW::VALUE2)
    }
    #[doc = "No change"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DINCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRC_TR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_TR_WIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_TR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_TR_WIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline]
    pub fn llp_src_en(&self) -> LLP_SRC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LLP_SRC_ENR { bits }
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline]
    pub fn llp_dst_en(&self) -> LLP_DST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LLP_DST_ENR { bits }
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline]
    pub fn tt_fc(&self) -> TT_FCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TT_FCR { bits }
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline]
    pub fn dst_scatter_en(&self) -> DST_SCATTER_ENR {
        DST_SCATTER_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline]
    pub fn src_gather_en(&self) -> SRC_GATHER_ENR {
        SRC_GATHER_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline]
    pub fn src_msize(&self) -> SRC_MSIZER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRC_MSIZER { bits }
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline]
    pub fn dest_msize(&self) -> DEST_MSIZER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEST_MSIZER { bits }
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline]
    pub fn sinc(&self) -> SINCR {
        SINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline]
    pub fn dinc(&self) -> DINCR {
        DINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRC_TR_WIDTHR { bits }
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DST_TR_WIDTHR { bits }
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline]
    pub fn int_en(&self) -> INT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INT_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3164161 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline]
    pub fn llp_src_en(&mut self) -> _LLP_SRC_ENW {
        _LLP_SRC_ENW { w: self }
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline]
    pub fn llp_dst_en(&mut self) -> _LLP_DST_ENW {
        _LLP_DST_ENW { w: self }
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline]
    pub fn tt_fc(&mut self) -> _TT_FCW {
        _TT_FCW { w: self }
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline]
    pub fn dst_scatter_en(&mut self) -> _DST_SCATTER_ENW {
        _DST_SCATTER_ENW { w: self }
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline]
    pub fn src_gather_en(&mut self) -> _SRC_GATHER_ENW {
        _SRC_GATHER_ENW { w: self }
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline]
    pub fn src_msize(&mut self) -> _SRC_MSIZEW {
        _SRC_MSIZEW { w: self }
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline]
    pub fn dest_msize(&mut self) -> _DEST_MSIZEW {
        _DEST_MSIZEW { w: self }
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline]
    pub fn sinc(&mut self) -> _SINCW {
        _SINCW { w: self }
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline]
    pub fn dinc(&mut self) -> _DINCW {
        _DINCW { w: self }
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline]
    pub fn src_tr_width(&mut self) -> _SRC_TR_WIDTHW {
        _SRC_TR_WIDTHW { w: self }
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline]
    pub fn dst_tr_width(&mut self) -> _DST_TR_WIDTHW {
        _DST_TR_WIDTHW { w: self }
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline]
    pub fn int_en(&mut self) -> _INT_ENW {
        _INT_ENW { w: self }
    }
}
