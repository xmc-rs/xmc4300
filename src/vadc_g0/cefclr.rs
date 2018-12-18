#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CEFCLR {
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
#[doc = "Values that can be written to the field `CEV0`"]
pub enum CEV0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV0W::VALUE1 => false,
            CEV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV0W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV0W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV1`"]
pub enum CEV1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV1W::VALUE1 => false,
            CEV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV1W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV1W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV2`"]
pub enum CEV2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV2W::VALUE1 => false,
            CEV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV2W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV2W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV3`"]
pub enum CEV3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV3W::VALUE1 => false,
            CEV3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV3W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV3W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV4`"]
pub enum CEV4W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV4W::VALUE1 => false,
            CEV4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV4W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV4W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV4W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV5`"]
pub enum CEV5W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV5W::VALUE1 => false,
            CEV5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV5W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV5W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV5W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV6`"]
pub enum CEV6W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV6W::VALUE1 => false,
            CEV6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV6W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV6W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV6W::VALUE2)
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
#[doc = "Values that can be written to the field `CEV7`"]
pub enum CEV7W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    VALUE2,
}
impl CEV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEV7W::VALUE1 => false,
            CEV7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEV7W<'a> {
    w: &'a mut W,
}
impl<'a> _CEV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEV7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEV7W::VALUE1)
    }
    #[doc = "Clear the channel event flag in GxCEFLAG"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEV7W::VALUE2)
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
    #[doc = "Bit 0 - Clear Channel Event for Channel 0"]
    #[inline]
    pub fn cev0(&mut self) -> _CEV0W {
        _CEV0W { w: self }
    }
    #[doc = "Bit 1 - Clear Channel Event for Channel 1"]
    #[inline]
    pub fn cev1(&mut self) -> _CEV1W {
        _CEV1W { w: self }
    }
    #[doc = "Bit 2 - Clear Channel Event for Channel 2"]
    #[inline]
    pub fn cev2(&mut self) -> _CEV2W {
        _CEV2W { w: self }
    }
    #[doc = "Bit 3 - Clear Channel Event for Channel 3"]
    #[inline]
    pub fn cev3(&mut self) -> _CEV3W {
        _CEV3W { w: self }
    }
    #[doc = "Bit 4 - Clear Channel Event for Channel 4"]
    #[inline]
    pub fn cev4(&mut self) -> _CEV4W {
        _CEV4W { w: self }
    }
    #[doc = "Bit 5 - Clear Channel Event for Channel 5"]
    #[inline]
    pub fn cev5(&mut self) -> _CEV5W {
        _CEV5W { w: self }
    }
    #[doc = "Bit 6 - Clear Channel Event for Channel 6"]
    #[inline]
    pub fn cev6(&mut self) -> _CEV6W {
        _CEV6W { w: self }
    }
    #[doc = "Bit 7 - Clear Channel Event for Channel 7"]
    #[inline]
    pub fn cev7(&mut self) -> _CEV7W {
        _CEV7W { w: self }
    }
}
