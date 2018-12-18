#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDR1 {
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
pub struct PD8R {
    bits: u8,
}
impl PD8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD9R {
    bits: u8,
}
impl PD9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD10R {
    bits: u8,
}
impl PD10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD11R {
    bits: u8,
}
impl PD11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD12R {
    bits: u8,
}
impl PD12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD13R {
    bits: u8,
}
impl PD13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD14R {
    bits: u8,
}
impl PD14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD15R {
    bits: u8,
}
impl PD15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PD8W<'a> {
    w: &'a mut W,
}
impl<'a> _PD8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD9W<'a> {
    w: &'a mut W,
}
impl<'a> _PD9W<'a> {
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
pub struct _PD10W<'a> {
    w: &'a mut W,
}
impl<'a> _PD10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD11W<'a> {
    w: &'a mut W,
}
impl<'a> _PD11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD12W<'a> {
    w: &'a mut W,
}
impl<'a> _PD12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD13W<'a> {
    w: &'a mut W,
}
impl<'a> _PD13W<'a> {
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
#[doc = r" Proxy"]
pub struct _PD14W<'a> {
    w: &'a mut W,
}
impl<'a> _PD14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PD15W<'a> {
    w: &'a mut W,
}
impl<'a> _PD15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline]
    pub fn pd8(&self) -> PD8R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD8R { bits }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline]
    pub fn pd9(&self) -> PD9R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD9R { bits }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline]
    pub fn pd10(&self) -> PD10R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD10R { bits }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline]
    pub fn pd11(&self) -> PD11R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD11R { bits }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline]
    pub fn pd12(&self) -> PD12R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD12R { bits }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline]
    pub fn pd13(&self) -> PD13R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD13R { bits }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline]
    pub fn pd14(&self) -> PD14R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD14R { bits }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline]
    pub fn pd15(&self) -> PD15R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD15R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 572662306 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline]
    pub fn pd8(&mut self) -> _PD8W {
        _PD8W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline]
    pub fn pd9(&mut self) -> _PD9W {
        _PD9W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline]
    pub fn pd10(&mut self) -> _PD10W {
        _PD10W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline]
    pub fn pd11(&mut self) -> _PD11W {
        _PD11W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline]
    pub fn pd12(&mut self) -> _PD12W {
        _PD12W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline]
    pub fn pd13(&mut self) -> _PD13W {
        _PD13W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline]
    pub fn pd14(&mut self) -> _PD14W {
        _PD14W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline]
    pub fn pd15(&mut self) -> _PD15W {
        _PD15W { w: self }
    }
}
