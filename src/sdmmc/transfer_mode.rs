#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TRANSFER_MODE {
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
#[doc = "Possible values of the field `CMD_COMP_ATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_COMP_ATAR {
    #[doc = "Device will send command completion Signal"]
    VALUE1,
    #[doc = "Device will not send command completion Signal"]
    VALUE2,
}
impl CMD_COMP_ATAR {
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
            CMD_COMP_ATAR::VALUE1 => true,
            CMD_COMP_ATAR::VALUE2 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_COMP_ATAR {
        match value {
            true => CMD_COMP_ATAR::VALUE1,
            false => CMD_COMP_ATAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMD_COMP_ATAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMD_COMP_ATAR::VALUE2
    }
}
#[doc = "Possible values of the field `MULTI_BLOCK_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTI_BLOCK_SELECTR {
    #[doc = "Single Block"]
    VALUE1,
    #[doc = "Multiple Block"]
    VALUE2,
}
impl MULTI_BLOCK_SELECTR {
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
            MULTI_BLOCK_SELECTR::VALUE1 => false,
            MULTI_BLOCK_SELECTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MULTI_BLOCK_SELECTR {
        match value {
            false => MULTI_BLOCK_SELECTR::VALUE1,
            true => MULTI_BLOCK_SELECTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MULTI_BLOCK_SELECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MULTI_BLOCK_SELECTR::VALUE2
    }
}
#[doc = "Possible values of the field `TX_DIR_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DIR_SELECTR {
    #[doc = "Write (Host to Card)"]
    VALUE1,
    #[doc = "Read (Card to Host)"]
    VALUE2,
}
impl TX_DIR_SELECTR {
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
            TX_DIR_SELECTR::VALUE1 => false,
            TX_DIR_SELECTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DIR_SELECTR {
        match value {
            false => TX_DIR_SELECTR::VALUE1,
            true => TX_DIR_SELECTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TX_DIR_SELECTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TX_DIR_SELECTR::VALUE2
    }
}
#[doc = "Possible values of the field `ACMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_ENR {
    #[doc = "Auto Command Disabled"]
    VALUE1,
    #[doc = "Auto CMD12 Enable"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMD_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMD_ENR::VALUE1 => 0,
            ACMD_ENR::VALUE2 => 1,
            ACMD_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMD_ENR {
        match value {
            0 => ACMD_ENR::VALUE1,
            1 => ACMD_ENR::VALUE2,
            i => ACMD_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACMD_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACMD_ENR::VALUE2
    }
}
#[doc = "Possible values of the field `BLOCK_COUNT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_COUNT_ENR {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl BLOCK_COUNT_ENR {
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
            BLOCK_COUNT_ENR::VALUE1 => false,
            BLOCK_COUNT_ENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLOCK_COUNT_ENR {
        match value {
            false => BLOCK_COUNT_ENR::VALUE1,
            true => BLOCK_COUNT_ENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BLOCK_COUNT_ENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BLOCK_COUNT_ENR::VALUE2
    }
}
#[doc = "Values that can be written to the field `CMD_COMP_ATA`"]
pub enum CMD_COMP_ATAW {
    #[doc = "Device will send command completion Signal"]
    VALUE1,
    #[doc = "Device will not send command completion Signal"]
    VALUE2,
}
impl CMD_COMP_ATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_COMP_ATAW::VALUE1 => true,
            CMD_COMP_ATAW::VALUE2 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_COMP_ATAW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_COMP_ATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_COMP_ATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Device will send command completion Signal"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_COMP_ATAW::VALUE1)
    }
    #[doc = "Device will not send command completion Signal"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_COMP_ATAW::VALUE2)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MULTI_BLOCK_SELECT`"]
pub enum MULTI_BLOCK_SELECTW {
    #[doc = "Single Block"]
    VALUE1,
    #[doc = "Multiple Block"]
    VALUE2,
}
impl MULTI_BLOCK_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MULTI_BLOCK_SELECTW::VALUE1 => false,
            MULTI_BLOCK_SELECTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTI_BLOCK_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTI_BLOCK_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTI_BLOCK_SELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single Block"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULTI_BLOCK_SELECTW::VALUE1)
    }
    #[doc = "Multiple Block"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULTI_BLOCK_SELECTW::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_DIR_SELECT`"]
pub enum TX_DIR_SELECTW {
    #[doc = "Write (Host to Card)"]
    VALUE1,
    #[doc = "Read (Card to Host)"]
    VALUE2,
}
impl TX_DIR_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DIR_SELECTW::VALUE1 => false,
            TX_DIR_SELECTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DIR_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DIR_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DIR_SELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_DIR_SELECTW::VALUE1)
    }
    #[doc = "Read (Card to Host)"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_DIR_SELECTW::VALUE2)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMD_EN`"]
pub enum ACMD_ENW {
    #[doc = "Auto Command Disabled"]
    VALUE1,
    #[doc = "Auto CMD12 Enable"]
    VALUE2,
}
impl ACMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMD_ENW::VALUE1 => 0,
            ACMD_ENW::VALUE2 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMD_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Auto Command Disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACMD_ENW::VALUE1)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACMD_ENW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLOCK_COUNT_EN`"]
pub enum BLOCK_COUNT_ENW {
    #[doc = "Disable"]
    VALUE1,
    #[doc = "Enable"]
    VALUE2,
}
impl BLOCK_COUNT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLOCK_COUNT_ENW::VALUE1 => false,
            BLOCK_COUNT_ENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_COUNT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_COUNT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLOCK_COUNT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BLOCK_COUNT_ENW::VALUE1)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BLOCK_COUNT_ENW::VALUE2)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline]
    pub fn cmd_comp_ata(&self) -> CMD_COMP_ATAR {
        CMD_COMP_ATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline]
    pub fn multi_block_select(&self) -> MULTI_BLOCK_SELECTR {
        MULTI_BLOCK_SELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn tx_dir_select(&self) -> TX_DIR_SELECTR {
        TX_DIR_SELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline]
    pub fn acmd_en(&self) -> ACMD_ENR {
        ACMD_ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn block_count_en(&self) -> BLOCK_COUNT_ENR {
        BLOCK_COUNT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline]
    pub fn cmd_comp_ata(&mut self) -> _CMD_COMP_ATAW {
        _CMD_COMP_ATAW { w: self }
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline]
    pub fn multi_block_select(&mut self) -> _MULTI_BLOCK_SELECTW {
        _MULTI_BLOCK_SELECTW { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn tx_dir_select(&mut self) -> _TX_DIR_SELECTW {
        _TX_DIR_SELECTW { w: self }
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline]
    pub fn acmd_en(&mut self) -> _ACMD_ENW {
        _ACMD_ENW { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn block_count_en(&mut self) -> _BLOCK_COUNT_ENW {
        _BLOCK_COUNT_ENW { w: self }
    }
}
