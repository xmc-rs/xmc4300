#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSCR {
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
}
#[doc = "Values that can be written to the field `CST0`"]
pub enum CST0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST0W::VALUE1 => false,
            CST0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST0W<'a> {
    w: &'a mut W,
}
impl<'a> _CST0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST0W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST0W::VALUE2)
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
#[doc = "Values that can be written to the field `CST1`"]
pub enum CST1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST1W::VALUE1 => false,
            CST1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST1W<'a> {
    w: &'a mut W,
}
impl<'a> _CST1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST1W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST1W::VALUE2)
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
#[doc = "Values that can be written to the field `CST2`"]
pub enum CST2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST2W::VALUE1 => false,
            CST2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST2W<'a> {
    w: &'a mut W,
}
impl<'a> _CST2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST2W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST2W::VALUE2)
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
#[doc = "Values that can be written to the field `CST3`"]
pub enum CST3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST3W::VALUE1 => false,
            CST3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST3W<'a> {
    w: &'a mut W,
}
impl<'a> _CST3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST3W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST3W::VALUE2)
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
#[doc = "Values that can be written to the field `CST4`"]
pub enum CST4W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST4W::VALUE1 => false,
            CST4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST4W<'a> {
    w: &'a mut W,
}
impl<'a> _CST4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST4W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST4W::VALUE2)
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
#[doc = "Values that can be written to the field `CST5`"]
pub enum CST5W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST5W::VALUE1 => false,
            CST5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST5W<'a> {
    w: &'a mut W,
}
impl<'a> _CST5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST5W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST5W::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CST6`"]
pub enum CST6W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST6W::VALUE1 => false,
            CST6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST6W<'a> {
    w: &'a mut W,
}
impl<'a> _CST6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST6W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST6W::VALUE2)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CST7`"]
pub enum CST7W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST7W::VALUE1 => false,
            CST7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST7W<'a> {
    w: &'a mut W,
}
impl<'a> _CST7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST7W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST7W::VALUE2)
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
#[doc = "Values that can be written to the field `CST8`"]
pub enum CST8W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST8W::VALUE1 => false,
            CST8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST8W<'a> {
    w: &'a mut W,
}
impl<'a> _CST8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST8W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST8W::VALUE2)
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
#[doc = "Values that can be written to the field `CST9`"]
pub enum CST9W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.STx is cleared."]
    VALUE2,
}
impl CST9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST9W::VALUE1 => false,
            CST9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST9W<'a> {
    w: &'a mut W,
}
impl<'a> _CST9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CST9W::VALUE1)
    }
    #[doc = "Flag PSR.STx is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CST9W::VALUE2)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRSIF`"]
pub enum CRSIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.RSIF is cleared."]
    VALUE2,
}
impl CRSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRSIFW::VALUE1 => false,
            CRSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRSIFW::VALUE1)
    }
    #[doc = "Flag PSR.RSIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CDLIF`"]
pub enum CDLIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.DLIF is cleared."]
    VALUE2,
}
impl CDLIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDLIFW::VALUE1 => false,
            CDLIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDLIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CDLIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDLIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDLIFW::VALUE1)
    }
    #[doc = "Flag PSR.DLIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDLIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CTSIF`"]
pub enum CTSIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.TSIF is cleared."]
    VALUE2,
}
impl CTSIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSIFW::VALUE1 => false,
            CTSIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTSIFW::VALUE1)
    }
    #[doc = "Flag PSR.TSIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTSIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CTBIF`"]
pub enum CTBIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.TBIF is cleared."]
    VALUE2,
}
impl CTBIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTBIFW::VALUE1 => false,
            CTBIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTBIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTBIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTBIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTBIFW::VALUE1)
    }
    #[doc = "Flag PSR.TBIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTBIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CRIF`"]
pub enum CRIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.RIF is cleared."]
    VALUE2,
}
impl CRIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRIFW::VALUE1 => false,
            CRIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CRIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRIFW::VALUE1)
    }
    #[doc = "Flag PSR.RIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CAIF`"]
pub enum CAIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.AIF is cleared."]
    VALUE2,
}
impl CAIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAIFW::VALUE1 => false,
            CAIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CAIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CAIFW::VALUE1)
    }
    #[doc = "Flag PSR.AIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CAIFW::VALUE2)
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
#[doc = "Values that can be written to the field `CBRGIF`"]
pub enum CBRGIFW {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Flag PSR.BRGIF is cleared."]
    VALUE2,
}
impl CBRGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CBRGIFW::VALUE1 => false,
            CBRGIFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CBRGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CBRGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CBRGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CBRGIFW::VALUE1)
    }
    #[doc = "Flag PSR.BRGIF is cleared."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CBRGIFW::VALUE2)
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
    #[doc = "Bit 0 - Clear Status Flag 0 in PSR"]
    #[inline]
    pub fn cst0(&mut self) -> _CST0W {
        _CST0W { w: self }
    }
    #[doc = "Bit 1 - Clear Status Flag 1 in PSR"]
    #[inline]
    pub fn cst1(&mut self) -> _CST1W {
        _CST1W { w: self }
    }
    #[doc = "Bit 2 - Clear Status Flag 2 in PSR"]
    #[inline]
    pub fn cst2(&mut self) -> _CST2W {
        _CST2W { w: self }
    }
    #[doc = "Bit 3 - Clear Status Flag 3 in PSR"]
    #[inline]
    pub fn cst3(&mut self) -> _CST3W {
        _CST3W { w: self }
    }
    #[doc = "Bit 4 - Clear Status Flag 4 in PSR"]
    #[inline]
    pub fn cst4(&mut self) -> _CST4W {
        _CST4W { w: self }
    }
    #[doc = "Bit 5 - Clear Status Flag 5 in PSR"]
    #[inline]
    pub fn cst5(&mut self) -> _CST5W {
        _CST5W { w: self }
    }
    #[doc = "Bit 6 - Clear Status Flag 6 in PSR"]
    #[inline]
    pub fn cst6(&mut self) -> _CST6W {
        _CST6W { w: self }
    }
    #[doc = "Bit 7 - Clear Status Flag 7 in PSR"]
    #[inline]
    pub fn cst7(&mut self) -> _CST7W {
        _CST7W { w: self }
    }
    #[doc = "Bit 8 - Clear Status Flag 8 in PSR"]
    #[inline]
    pub fn cst8(&mut self) -> _CST8W {
        _CST8W { w: self }
    }
    #[doc = "Bit 9 - Clear Status Flag 9 in PSR"]
    #[inline]
    pub fn cst9(&mut self) -> _CST9W {
        _CST9W { w: self }
    }
    #[doc = "Bit 10 - Clear Receiver Start Indication Flag"]
    #[inline]
    pub fn crsif(&mut self) -> _CRSIFW {
        _CRSIFW { w: self }
    }
    #[doc = "Bit 11 - Clear Data Lost Indication Flag"]
    #[inline]
    pub fn cdlif(&mut self) -> _CDLIFW {
        _CDLIFW { w: self }
    }
    #[doc = "Bit 12 - Clear Transmit Shift Indication Flag"]
    #[inline]
    pub fn ctsif(&mut self) -> _CTSIFW {
        _CTSIFW { w: self }
    }
    #[doc = "Bit 13 - Clear Transmit Buffer Indication Flag"]
    #[inline]
    pub fn ctbif(&mut self) -> _CTBIFW {
        _CTBIFW { w: self }
    }
    #[doc = "Bit 14 - Clear Receive Indication Flag"]
    #[inline]
    pub fn crif(&mut self) -> _CRIFW {
        _CRIFW { w: self }
    }
    #[doc = "Bit 15 - Clear Alternative Receive Indication Flag"]
    #[inline]
    pub fn caif(&mut self) -> _CAIFW {
        _CAIFW { w: self }
    }
    #[doc = "Bit 16 - Clear Baud Rate Generator Indication Flag"]
    #[inline]
    pub fn cbrgif(&mut self) -> _CBRGIFW {
        _CBRGIFW { w: self }
    }
}
