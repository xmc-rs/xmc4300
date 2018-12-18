#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KSCFG {
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
#[doc = "Possible values of the field `MODEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODENR {
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1,
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2,
}
impl MODENR {
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
            MODENR::VALUE1 => false,
            MODENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODENR {
        match value {
            false => MODENR::VALUE1,
            true => MODENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MODENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MODENR::VALUE2
    }
}
#[doc = "Possible values of the field `NOMCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOMCFGR {
    #[doc = "Run mode 0 is selected."]
    VALUE1,
    #[doc = "Run mode 1 is selected."]
    VALUE2,
    #[doc = "Stop mode 0 is selected."]
    VALUE3,
    #[doc = "Stop mode 1 is selected."]
    VALUE4,
}
impl NOMCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NOMCFGR::VALUE1 => 0,
            NOMCFGR::VALUE2 => 1,
            NOMCFGR::VALUE3 => 2,
            NOMCFGR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NOMCFGR {
        match value {
            0 => NOMCFGR::VALUE1,
            1 => NOMCFGR::VALUE2,
            2 => NOMCFGR::VALUE3,
            3 => NOMCFGR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NOMCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NOMCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == NOMCFGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == NOMCFGR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct SUMCFGR {
    bits: u8,
}
impl SUMCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MODEN`"]
pub enum MODENW {
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1,
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2,
}
impl MODENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODENW::VALUE1 => false,
            MODENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODENW<'a> {
    w: &'a mut W,
}
impl<'a> _MODENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODENW::VALUE1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODENW::VALUE2)
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
#[doc = "Values that can be written to the field `BPMODEN`"]
pub enum BPMODENW {
    #[doc = "MODEN is not changed."]
    VALUE1,
    #[doc = "MODEN is updated with the written value."]
    VALUE2,
}
impl BPMODENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BPMODENW::VALUE1 => false,
            BPMODENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BPMODENW<'a> {
    w: &'a mut W,
}
impl<'a> _BPMODENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BPMODENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MODEN is not changed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPMODENW::VALUE1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPMODENW::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOMCFG`"]
pub enum NOMCFGW {
    #[doc = "Run mode 0 is selected."]
    VALUE1,
    #[doc = "Run mode 1 is selected."]
    VALUE2,
    #[doc = "Stop mode 0 is selected."]
    VALUE3,
    #[doc = "Stop mode 1 is selected."]
    VALUE4,
}
impl NOMCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NOMCFGW::VALUE1 => 0,
            NOMCFGW::VALUE2 => 1,
            NOMCFGW::VALUE3 => 2,
            NOMCFGW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOMCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _NOMCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOMCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Run mode 0 is selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOMCFGW::VALUE1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOMCFGW::VALUE2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(NOMCFGW::VALUE3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(NOMCFGW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BPNOM`"]
pub enum BPNOMW {
    #[doc = "NOMCFG is not changed."]
    VALUE1,
    #[doc = "NOMCFG is updated with the written value."]
    VALUE2,
}
impl BPNOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BPNOMW::VALUE1 => false,
            BPNOMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BPNOMW<'a> {
    w: &'a mut W,
}
impl<'a> _BPNOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BPNOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NOMCFG is not changed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPNOMW::VALUE1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPNOMW::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUMCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SUMCFGW<'a> {
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
#[doc = "Values that can be written to the field `BPSUM`"]
pub enum BPSUMW {
    #[doc = "SUMCFG is not changed."]
    VALUE1,
    #[doc = "SUMCFG is updated with the written value."]
    VALUE2,
}
impl BPSUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BPSUMW::VALUE1 => false,
            BPSUMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BPSUMW<'a> {
    w: &'a mut W,
}
impl<'a> _BPSUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BPSUMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SUMCFG is not changed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPSUMW::VALUE1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPSUMW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Module Enable"]
    #[inline]
    pub fn moden(&self) -> MODENR {
        MODENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline]
    pub fn nomcfg(&self) -> NOMCFGR {
        NOMCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline]
    pub fn sumcfg(&self) -> SUMCFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUMCFGR { bits }
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
    #[doc = "Bit 0 - Module Enable"]
    #[inline]
    pub fn moden(&mut self) -> _MODENW {
        _MODENW { w: self }
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline]
    pub fn bpmoden(&mut self) -> _BPMODENW {
        _BPMODENW { w: self }
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline]
    pub fn nomcfg(&mut self) -> _NOMCFGW {
        _NOMCFGW { w: self }
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline]
    pub fn bpnom(&mut self) -> _BPNOMW {
        _BPNOMW { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline]
    pub fn sumcfg(&mut self) -> _SUMCFGW {
        _SUMCFGW { w: self }
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline]
    pub fn bpsum(&mut self) -> _BPSUMW {
        _BPSUMW { w: self }
    }
}
