#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLEARERR {
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
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::VALUE1 => false,
            CH0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0W::VALUE2)
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
#[doc = "Values that can be written to the field `CH1`"]
pub enum CH1W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::VALUE1 => false,
            CH1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1W::VALUE2)
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
#[doc = "Values that can be written to the field `CH2`"]
pub enum CH2W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::VALUE1 => false,
            CH2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2W::VALUE2)
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
#[doc = "Values that can be written to the field `CH3`"]
pub enum CH3W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::VALUE1 => false,
            CH3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3W::VALUE2)
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
#[doc = "Values that can be written to the field `CH4`"]
pub enum CH4W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::VALUE1 => false,
            CH4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4W::VALUE2)
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
#[doc = "Values that can be written to the field `CH5`"]
pub enum CH5W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::VALUE1 => false,
            CH5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5W::VALUE2)
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
#[doc = "Values that can be written to the field `CH6`"]
pub enum CH6W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::VALUE1 => false,
            CH6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6W::VALUE2)
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
#[doc = "Values that can be written to the field `CH7`"]
pub enum CH7W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "clear status"]
    VALUE2,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::VALUE1 => false,
            CH7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7W::VALUE1)
    }
    #[doc = "clear status"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7W::VALUE2)
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
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Clear Interrupt Status and Raw Status for channel 4"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Clear Interrupt Status and Raw Status for channel 5"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Clear Interrupt Status and Raw Status for channel 6"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Clear Interrupt Status and Raw Status for channel 7"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
}
