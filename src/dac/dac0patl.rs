#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAC0PATL {
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
pub struct PAT0R {
    bits: u8,
}
impl PAT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAT1R {
    bits: u8,
}
impl PAT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAT2R {
    bits: u8,
}
impl PAT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAT3R {
    bits: u8,
}
impl PAT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAT4R {
    bits: u8,
}
impl PAT4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAT5R {
    bits: u8,
}
impl PAT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PAT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT2W<'a> {
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
pub struct _PAT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAT4W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAT5W<'a> {
    w: &'a mut W,
}
impl<'a> _PAT5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC0"]
    #[inline]
    pub fn pat0(&self) -> PAT0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT0R { bits }
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC0"]
    #[inline]
    pub fn pat1(&self) -> PAT1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT1R { bits }
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC0"]
    #[inline]
    pub fn pat2(&self) -> PAT2R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT2R { bits }
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC0"]
    #[inline]
    pub fn pat3(&self) -> PAT3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT3R { bits }
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC0"]
    #[inline]
    pub fn pat4(&self) -> PAT4R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT4R { bits }
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC0"]
    #[inline]
    pub fn pat5(&self) -> PAT5R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAT5R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 896053440 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC0"]
    #[inline]
    pub fn pat0(&mut self) -> _PAT0W {
        _PAT0W { w: self }
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC0"]
    #[inline]
    pub fn pat1(&mut self) -> _PAT1W {
        _PAT1W { w: self }
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC0"]
    #[inline]
    pub fn pat2(&mut self) -> _PAT2W {
        _PAT2W { w: self }
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC0"]
    #[inline]
    pub fn pat3(&mut self) -> _PAT3W {
        _PAT3W { w: self }
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC0"]
    #[inline]
    pub fn pat4(&mut self) -> _PAT4W {
        _PAT4W { w: self }
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC0"]
    #[inline]
    pub fn pat5(&mut self) -> _PAT5W {
        _PAT5W { w: self }
    }
}
