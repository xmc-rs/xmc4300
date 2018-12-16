#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BFLS {
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
#[doc = "Values that can be written to the field `BFC0`"]
pub enum BFC0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit BFLy"]
    VALUE2,
}
impl BFC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFC0W::VALUE1 => false,
            BFC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFC0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC0W::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC0W::VALUE2)
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
#[doc = "Values that can be written to the field `BFC1`"]
pub enum BFC1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit BFLy"]
    VALUE2,
}
impl BFC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFC1W::VALUE1 => false,
            BFC1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFC1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC1W::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC1W::VALUE2)
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
#[doc = "Values that can be written to the field `BFC2`"]
pub enum BFC2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit BFLy"]
    VALUE2,
}
impl BFC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFC2W::VALUE1 => false,
            BFC2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFC2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC2W::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC2W::VALUE2)
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
#[doc = "Values that can be written to the field `BFC3`"]
pub enum BFC3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Clear bit BFLy"]
    VALUE2,
}
impl BFC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFC3W::VALUE1 => false,
            BFC3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFC3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFC3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC3W::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC3W::VALUE2)
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
#[doc = "Values that can be written to the field `BFS0`"]
pub enum BFS0W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Set bit BFLy"]
    VALUE2,
}
impl BFS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFS0W::VALUE1 => false,
            BFS0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFS0W<'a> {
    w: &'a mut W,
}
impl<'a> _BFS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS0W::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS0W::VALUE2)
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
#[doc = "Values that can be written to the field `BFS1`"]
pub enum BFS1W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Set bit BFLy"]
    VALUE2,
}
impl BFS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFS1W::VALUE1 => false,
            BFS1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFS1W<'a> {
    w: &'a mut W,
}
impl<'a> _BFS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS1W::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS1W::VALUE2)
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
#[doc = "Values that can be written to the field `BFS2`"]
pub enum BFS2W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Set bit BFLy"]
    VALUE2,
}
impl BFS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFS2W::VALUE1 => false,
            BFS2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFS2W<'a> {
    w: &'a mut W,
}
impl<'a> _BFS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS2W::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS2W::VALUE2)
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
#[doc = "Values that can be written to the field `BFS3`"]
pub enum BFS3W {
    #[doc = "No action"]
    VALUE1,
    #[doc = "Set bit BFLy"]
    VALUE2,
}
impl BFS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFS3W::VALUE1 => false,
            BFS3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFS3W<'a> {
    w: &'a mut W,
}
impl<'a> _BFS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS3W::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS3W::VALUE2)
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
    #[doc = "Bit 0 - Boundary Flag 0 Clear"]
    #[inline]
    pub fn bfc0(&mut self) -> _BFC0W {
        _BFC0W { w: self }
    }
    #[doc = "Bit 1 - Boundary Flag 1 Clear"]
    #[inline]
    pub fn bfc1(&mut self) -> _BFC1W {
        _BFC1W { w: self }
    }
    #[doc = "Bit 2 - Boundary Flag 2 Clear"]
    #[inline]
    pub fn bfc2(&mut self) -> _BFC2W {
        _BFC2W { w: self }
    }
    #[doc = "Bit 3 - Boundary Flag 3 Clear"]
    #[inline]
    pub fn bfc3(&mut self) -> _BFC3W {
        _BFC3W { w: self }
    }
    #[doc = "Bit 16 - Boundary Flag 0 Set"]
    #[inline]
    pub fn bfs0(&mut self) -> _BFS0W {
        _BFS0W { w: self }
    }
    #[doc = "Bit 17 - Boundary Flag 1 Set"]
    #[inline]
    pub fn bfs1(&mut self) -> _BFS1W {
        _BFS1W { w: self }
    }
    #[doc = "Bit 18 - Boundary Flag 2 Set"]
    #[inline]
    pub fn bfs2(&mut self) -> _BFS2W {
        _BFS2W { w: self }
    }
    #[doc = "Bit 19 - Boundary Flag 3 Set"]
    #[inline]
    pub fn bfs3(&mut self) -> _BFS3W {
        _BFS3W { w: self }
    }
}
