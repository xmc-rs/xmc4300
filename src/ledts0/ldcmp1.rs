#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LDCMP1 {
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
pub struct CMP_LD4R {
    bits: u8,
}
impl CMP_LD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_LD5R {
    bits: u8,
}
impl CMP_LD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_LD6R {
    bits: u8,
}
impl CMP_LD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP_LDA_TSCOMR {
    bits: u8,
}
impl CMP_LDA_TSCOMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CMP_LD4W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_LD4W<'a> {
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
pub struct _CMP_LD5W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_LD5W<'a> {
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
pub struct _CMP_LD6W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_LD6W<'a> {
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
pub struct _CMP_LDA_TSCOMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMP_LDA_TSCOMW<'a> {
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
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld4(&self) -> CMP_LD4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_LD4R { bits }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld5(&self) -> CMP_LD5R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_LD5R { bits }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld6(&self) -> CMP_LD6R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_LD6R { bits }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline]
    pub fn cmp_lda_tscom(&self) -> CMP_LDA_TSCOMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP_LDA_TSCOMR { bits }
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
    #[doc = "Bits 0:7 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld4(&mut self) -> _CMP_LD4W {
        _CMP_LD4W { w: self }
    }
    #[doc = "Bits 8:15 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld5(&mut self) -> _CMP_LD5W {
        _CMP_LD5W { w: self }
    }
    #[doc = "Bits 16:23 - Compare Value for LED COL\\[x\\]"]
    #[inline]
    pub fn cmp_ld6(&mut self) -> _CMP_LD6W {
        _CMP_LD6W { w: self }
    }
    #[doc = "Bits 24:31 - Compare Value for LED COLA / Common Compare Value for Touch-sense Pad Turns"]
    #[inline]
    pub fn cmp_lda_tscom(&mut self) -> _CMP_LDA_TSCOMW {
        _CMP_LDA_TSCOMW { w: self }
    }
}
