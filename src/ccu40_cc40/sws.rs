#[doc = "Writer for register SWS"]
pub type W = crate::W<u32, super::SWS>;
#[doc = "Register SWS `reset()`'s with value 0"]
impl crate::ResetValue for super::SWS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPM`"]
pub struct SPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPM_W<'a> {
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
#[doc = "Write proxy for field `SOM`"]
pub struct SOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOM_W<'a> {
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
#[doc = "Write proxy for field `SCMU`"]
pub struct SCMU_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMU_W<'a> {
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
#[doc = "Write proxy for field `SCMD`"]
pub struct SCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMD_W<'a> {
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
#[doc = "Write proxy for field `SE0A`"]
pub struct SE0A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0A_W<'a> {
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
#[doc = "Write proxy for field `SE1A`"]
pub struct SE1A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1A_W<'a> {
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
#[doc = "Write proxy for field `SE2A`"]
pub struct SE2A_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2A_W<'a> {
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
#[doc = "Write proxy for field `STRPF`"]
pub struct STRPF_W<'a> {
    w: &'a mut W,
}
impl<'a> STRPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Period match while counting up set"]
    #[inline(always)]
    pub fn spm(&mut self) -> SPM_W {
        SPM_W { w: self }
    }
    #[doc = "Bit 1 - One match while counting down set"]
    #[inline(always)]
    pub fn som(&mut self) -> SOM_W {
        SOM_W { w: self }
    }
    #[doc = "Bit 2 - Compare match while counting up set"]
    #[inline(always)]
    pub fn scmu(&mut self) -> SCMU_W {
        SCMU_W { w: self }
    }
    #[doc = "Bit 3 - Compare match while counting down set"]
    #[inline(always)]
    pub fn scmd(&mut self) -> SCMD_W {
        SCMD_W { w: self }
    }
    #[doc = "Bit 8 - Event 0 detection set"]
    #[inline(always)]
    pub fn se0a(&mut self) -> SE0A_W {
        SE0A_W { w: self }
    }
    #[doc = "Bit 9 - Event 1 detection set"]
    #[inline(always)]
    pub fn se1a(&mut self) -> SE1A_W {
        SE1A_W { w: self }
    }
    #[doc = "Bit 10 - Event 2 detection set"]
    #[inline(always)]
    pub fn se2a(&mut self) -> SE2A_W {
        SE2A_W { w: self }
    }
    #[doc = "Bit 11 - Trap Flag status set"]
    #[inline(always)]
    pub fn strpf(&mut self) -> STRPF_W {
        STRPF_W { w: self }
    }
}
