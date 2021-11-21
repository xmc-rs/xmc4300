#[doc = "Register `BLOCK_GAP_CTRL` reader"]
pub struct R(crate::R<BLOCK_GAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_GAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_GAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_GAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_GAP_CTRL` writer"]
pub struct W(crate::W<BLOCK_GAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_GAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BLOCK_GAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_GAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_AT_BLOCK_GAP` reader - Interrupt At Block Gap"]
pub struct INT_AT_BLOCK_GAP_R(crate::FieldReader<bool, bool>);
impl INT_AT_BLOCK_GAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_AT_BLOCK_GAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_AT_BLOCK_GAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_AT_BLOCK_GAP` writer - Interrupt At Block Gap"]
pub struct INT_AT_BLOCK_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_AT_BLOCK_GAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WAIT_CTRL_A {
    #[doc = "0: Disable Read Wait Control"]
    VALUE1 = 0,
    #[doc = "1: Enable Read Wait Control"]
    VALUE2 = 1,
}
impl From<READ_WAIT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: READ_WAIT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_WAIT_CTRL` reader - Read Wait Control"]
pub struct READ_WAIT_CTRL_R(crate::FieldReader<bool, READ_WAIT_CTRL_A>);
impl READ_WAIT_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_WAIT_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_WAIT_CTRL_A {
        match self.bits {
            false => READ_WAIT_CTRL_A::VALUE1,
            true => READ_WAIT_CTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == READ_WAIT_CTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == READ_WAIT_CTRL_A::VALUE2
    }
}
impl core::ops::Deref for READ_WAIT_CTRL_R {
    type Target = crate::FieldReader<bool, READ_WAIT_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_WAIT_CTRL` writer - Read Wait Control"]
pub struct READ_WAIT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WAIT_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_WAIT_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRL_A::VALUE1)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTINUE_REQ_A {
    #[doc = "0: Ignored"]
    VALUE1 = 0,
    #[doc = "1: Restart"]
    VALUE2 = 1,
}
impl From<CONTINUE_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CONTINUE_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTINUE_REQ` reader - Continue Request"]
pub struct CONTINUE_REQ_R(crate::FieldReader<bool, CONTINUE_REQ_A>);
impl CONTINUE_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONTINUE_REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTINUE_REQ_A {
        match self.bits {
            false => CONTINUE_REQ_A::VALUE1,
            true => CONTINUE_REQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CONTINUE_REQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CONTINUE_REQ_A::VALUE2
    }
}
impl core::ops::Deref for CONTINUE_REQ_R {
    type Target = crate::FieldReader<bool, CONTINUE_REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTINUE_REQ` writer - Continue Request"]
pub struct CONTINUE_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUE_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTINUE_REQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CONTINUE_REQ_A::VALUE1)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CONTINUE_REQ_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AT_BLOCK_GAP_A {
    #[doc = "0: Transfer"]
    VALUE1 = 0,
    #[doc = "1: Stop"]
    VALUE2 = 1,
}
impl From<STOP_AT_BLOCK_GAP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_AT_BLOCK_GAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` reader - Stop At Block Gap Request"]
pub struct STOP_AT_BLOCK_GAP_R(crate::FieldReader<bool, STOP_AT_BLOCK_GAP_A>);
impl STOP_AT_BLOCK_GAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_AT_BLOCK_GAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_AT_BLOCK_GAP_A {
        match self.bits {
            false => STOP_AT_BLOCK_GAP_A::VALUE1,
            true => STOP_AT_BLOCK_GAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STOP_AT_BLOCK_GAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STOP_AT_BLOCK_GAP_A::VALUE2
    }
}
impl core::ops::Deref for STOP_AT_BLOCK_GAP_R {
    type Target = crate::FieldReader<bool, STOP_AT_BLOCK_GAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` writer - Stop At Block Gap Request"]
pub struct STOP_AT_BLOCK_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_AT_BLOCK_GAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AT_BLOCK_GAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE1)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&self) -> INT_AT_BLOCK_GAP_R {
        INT_AT_BLOCK_GAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&self) -> READ_WAIT_CTRL_R {
        READ_WAIT_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&self) -> STOP_AT_BLOCK_GAP_R {
        STOP_AT_BLOCK_GAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&mut self) -> INT_AT_BLOCK_GAP_W {
        INT_AT_BLOCK_GAP_W { w: self }
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&mut self) -> READ_WAIT_CTRL_W {
        READ_WAIT_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W {
        CONTINUE_REQ_W { w: self }
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&mut self) -> STOP_AT_BLOCK_GAP_W {
        STOP_AT_BLOCK_GAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Gap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_gap_ctrl](index.html) module"]
pub struct BLOCK_GAP_CTRL_SPEC;
impl crate::RegisterSpec for BLOCK_GAP_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [block_gap_ctrl::R](R) reader structure"]
impl crate::Readable for BLOCK_GAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_gap_ctrl::W](W) writer structure"]
impl crate::Writable for BLOCK_GAP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLOCK_GAP_CTRL to value 0"]
impl crate::Resettable for BLOCK_GAP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
