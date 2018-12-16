#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDR0 {
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
pub struct PD0R {
    bits: u8,
}
impl PD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD1R {
    bits: u8,
}
impl PD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD2R {
    bits: u8,
}
impl PD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD3R {
    bits: u8,
}
impl PD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD4R {
    bits: u8,
}
impl PD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD5R {
    bits: u8,
}
impl PD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD6R {
    bits: u8,
}
impl PD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PD7R {
    bits: u8,
}
impl PD7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PD0W<'a> {
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
pub struct _PD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PD1W<'a> {
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
pub struct _PD2W<'a> {
    w: &'a mut W,
}
impl<'a> _PD2W<'a> {
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
pub struct _PD3W<'a> {
    w: &'a mut W,
}
impl<'a> _PD3W<'a> {
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
pub struct _PD4W<'a> {
    w: &'a mut W,
}
impl<'a> _PD4W<'a> {
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
pub struct _PD5W<'a> {
    w: &'a mut W,
}
impl<'a> _PD5W<'a> {
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
pub struct _PD6W<'a> {
    w: &'a mut W,
}
impl<'a> _PD6W<'a> {
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
pub struct _PD7W<'a> {
    w: &'a mut W,
}
impl<'a> _PD7W<'a> {
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline]
    pub fn pd0(&self) -> PD0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD0R { bits }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline]
    pub fn pd1(&self) -> PD1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD1R { bits }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline]
    pub fn pd2(&self) -> PD2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD2R { bits }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline]
    pub fn pd3(&self) -> PD3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD3R { bits }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline]
    pub fn pd4(&self) -> PD4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD4R { bits }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline]
    pub fn pd5(&self) -> PD5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD5R { bits }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline]
    pub fn pd6(&self) -> PD6R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD6R { bits }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline]
    pub fn pd7(&self) -> PD7R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PD7R { bits }
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline]
    pub fn pd0(&mut self) -> _PD0W {
        _PD0W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline]
    pub fn pd1(&mut self) -> _PD1W {
        _PD1W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline]
    pub fn pd2(&mut self) -> _PD2W {
        _PD2W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline]
    pub fn pd3(&mut self) -> _PD3W {
        _PD3W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline]
    pub fn pd4(&mut self) -> _PD4W {
        _PD4W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline]
    pub fn pd5(&mut self) -> _PD5W {
        _PD5W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline]
    pub fn pd6(&mut self) -> _PD6W {
        _PD6W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline]
    pub fn pd7(&mut self) -> _PD7W {
        _PD7W { w: self }
    }
}
