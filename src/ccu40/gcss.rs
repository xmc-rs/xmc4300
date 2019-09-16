#[doc = "Writer for register GCSS"]
pub type W = crate::W<u32, super::GCSS>;
#[doc = "Register GCSS `reset()`'s with value 0"]
impl crate::ResetValue for super::GCSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `S0SE`"]
pub struct S0SE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0SE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `S0DSE`"]
pub struct S0DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `S0PSE`"]
pub struct S0PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `S1SE`"]
pub struct S1SE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1SE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `S1DSE`"]
pub struct S1DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1DSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `S1PSE`"]
pub struct S1PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `S2SE`"]
pub struct S2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> S2SE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `S2DSE`"]
pub struct S2DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S2DSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `S2PSE`"]
pub struct S2PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S2PSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `S3SE`"]
pub struct S3SE_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `S3DSE`"]
pub struct S3DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S3DSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `S3PSE`"]
pub struct S3PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> S3PSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `S0STS`"]
pub struct S0STS_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `S1STS`"]
pub struct S1STS_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `S2STS`"]
pub struct S2STS_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `S3STS`"]
pub struct S3STS_W<'a> {
    w: &'a mut W,
}
impl<'a> S3STS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer set enable"]
    #[inline(always)]
    pub fn s0se(&mut self) -> S0SE_W {
        S0SE_W { w: self }
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer set enable"]
    #[inline(always)]
    pub fn s0dse(&mut self) -> S0DSE_W {
        S0DSE_W { w: self }
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer set enable"]
    #[inline(always)]
    pub fn s0pse(&mut self) -> S0PSE_W {
        S0PSE_W { w: self }
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer set enable"]
    #[inline(always)]
    pub fn s1se(&mut self) -> S1SE_W {
        S1SE_W { w: self }
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer set enable"]
    #[inline(always)]
    pub fn s1dse(&mut self) -> S1DSE_W {
        S1DSE_W { w: self }
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer set enable"]
    #[inline(always)]
    pub fn s1pse(&mut self) -> S1PSE_W {
        S1PSE_W { w: self }
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer set enable"]
    #[inline(always)]
    pub fn s2se(&mut self) -> S2SE_W {
        S2SE_W { w: self }
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer set enable"]
    #[inline(always)]
    pub fn s2dse(&mut self) -> S2DSE_W {
        S2DSE_W { w: self }
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer set enable"]
    #[inline(always)]
    pub fn s2pse(&mut self) -> S2PSE_W {
        S2PSE_W { w: self }
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer set enable"]
    #[inline(always)]
    pub fn s3se(&mut self) -> S3SE_W {
        S3SE_W { w: self }
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer set enable"]
    #[inline(always)]
    pub fn s3dse(&mut self) -> S3DSE_W {
        S3DSE_W { w: self }
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer set enable"]
    #[inline(always)]
    pub fn s3pse(&mut self) -> S3PSE_W {
        S3PSE_W { w: self }
    }
    #[doc = "Bit 16 - Slice 0 status bit set"]
    #[inline(always)]
    pub fn s0sts(&mut self) -> S0STS_W {
        S0STS_W { w: self }
    }
    #[doc = "Bit 17 - Slice 1 status bit set"]
    #[inline(always)]
    pub fn s1sts(&mut self) -> S1STS_W {
        S1STS_W { w: self }
    }
    #[doc = "Bit 18 - Slice 2 status bit set"]
    #[inline(always)]
    pub fn s2sts(&mut self) -> S2STS_W {
        S2STS_W { w: self }
    }
    #[doc = "Bit 19 - Slice 3 status bit set"]
    #[inline(always)]
    pub fn s3sts(&mut self) -> S3STS_W {
        S3STS_W { w: self }
    }
}
