#[doc = "Reader of register FDR"]
pub type R = crate::R<u32, super::FDR>;
#[doc = "Writer for register FDR"]
pub type W = crate::W<u32, super::FDR>;
#[doc = "Register FDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STEP`"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Divider Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM_A {
    #[doc = "0: The divider is switched off, fFD = 0."]
    VALUE1 = 0,
    #[doc = "1: Normal divider mode selected."]
    VALUE2 = 1,
    #[doc = "2: Fractional divider mode selected."]
    VALUE3 = 2,
    #[doc = "3: The divider is switched off, fFD = 0."]
    VALUE4 = 3,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DM`"]
pub type DM_R = crate::R<u8, DM_A>;
impl DM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            0 => DM_A::VALUE1,
            1 => DM_A::VALUE2,
            2 => DM_A::VALUE3,
            3 => DM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DM_A::VALUE4
    }
}
#[doc = "Write proxy for field `DM`"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DM_A::VALUE1)
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DM_A::VALUE2)
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DM_A::VALUE3)
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
}
