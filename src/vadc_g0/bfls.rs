#[doc = "Writer for register BFLS"]
pub type W = crate::W<u32, super::BFLS>;
#[doc = "Register BFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::BFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Boundary Flag 0 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFC0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFC0`"]
pub struct BFC0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFC0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC0_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC0_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 1 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFC1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC1_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFC1`"]
pub struct BFC1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFC1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC1_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC1_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 2 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFC2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC2_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFC2`"]
pub struct BFC2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFC2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC2_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC2_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 3 Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFC3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit BFLy"]
    VALUE2 = 1,
}
impl From<BFC3_AW> for bool {
    #[inline(always)]
    fn from(variant: BFC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFC3`"]
pub struct BFC3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFC3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFC3_AW::VALUE1)
    }
    #[doc = "Clear bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFC3_AW::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Boundary Flag 0 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFS0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS0_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFS0`"]
pub struct BFS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFS0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS0_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS0_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 1 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFS1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS1_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFS1`"]
pub struct BFS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFS1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS1_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS1_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 2 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFS2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS2_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFS2`"]
pub struct BFS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFS2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS2_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS2_AW::VALUE2)
    }
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
#[doc = "Boundary Flag 3 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFS3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Set bit BFLy"]
    VALUE2 = 1,
}
impl From<BFS3_AW> for bool {
    #[inline(always)]
    fn from(variant: BFS3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BFS3`"]
pub struct BFS3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFS3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFS3_AW::VALUE1)
    }
    #[doc = "Set bit BFLy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFS3_AW::VALUE2)
    }
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
    #[doc = "Bit 0 - Boundary Flag 0 Clear"]
    #[inline(always)]
    pub fn bfc0(&mut self) -> BFC0_W {
        BFC0_W { w: self }
    }
    #[doc = "Bit 1 - Boundary Flag 1 Clear"]
    #[inline(always)]
    pub fn bfc1(&mut self) -> BFC1_W {
        BFC1_W { w: self }
    }
    #[doc = "Bit 2 - Boundary Flag 2 Clear"]
    #[inline(always)]
    pub fn bfc2(&mut self) -> BFC2_W {
        BFC2_W { w: self }
    }
    #[doc = "Bit 3 - Boundary Flag 3 Clear"]
    #[inline(always)]
    pub fn bfc3(&mut self) -> BFC3_W {
        BFC3_W { w: self }
    }
    #[doc = "Bit 16 - Boundary Flag 0 Set"]
    #[inline(always)]
    pub fn bfs0(&mut self) -> BFS0_W {
        BFS0_W { w: self }
    }
    #[doc = "Bit 17 - Boundary Flag 1 Set"]
    #[inline(always)]
    pub fn bfs1(&mut self) -> BFS1_W {
        BFS1_W { w: self }
    }
    #[doc = "Bit 18 - Boundary Flag 2 Set"]
    #[inline(always)]
    pub fn bfs2(&mut self) -> BFS2_W {
        BFS2_W { w: self }
    }
    #[doc = "Bit 19 - Boundary Flag 3 Set"]
    #[inline(always)]
    pub fn bfs3(&mut self) -> BFS3_W {
        BFS3_W { w: self }
    }
}
