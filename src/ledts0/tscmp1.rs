#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSCMP1 {
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
pub struct CMP_TS4R {
    bits: u8,
}
impl CMP_TS4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_TS5R {
    bits: u8,
}
impl CMP_TS5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_TS6R {
    bits: u8,
}
impl CMP_TS6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_TS7R {
    bits: u8,
}
impl CMP_TS7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CMP_TS4W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_TS4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP_TS5W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_TS5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP_TS6W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_TS6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP_TS7W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_TS7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts4(&self) -> CMP_TS4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_TS4R { bits }
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts5(&self) -> CMP_TS5R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_TS5R { bits }
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts6(&self) -> CMP_TS6R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_TS6R { bits }
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts7(&self) -> CMP_TS7R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_TS7R { bits }
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
    #[doc = "Bits 0:7 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts4(&mut self) -> _CMP_TS4W {
        _CMP_TS4W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts5(&mut self) -> _CMP_TS5W {
        _CMP_TS5W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts6(&mut self) -> _CMP_TS6W {
        _CMP_TS6W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for Touch-Sense TSIN\\[x\\]"]
    #[inline]
    pub fn cmp_ts7(&mut self) -> _CMP_TS7W {
        _CMP_TS7W { w: self }
    }
}
