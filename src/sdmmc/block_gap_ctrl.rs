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
#[doc = "Field `STOP_AT_BLOCK_GAP` reader - Stop At Block Gap Request"]
pub type STOP_AT_BLOCK_GAP_R = crate::BitReader<STOP_AT_BLOCK_GAP_A>;
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STOP_AT_BLOCK_GAP_R {
    #[doc = "Get enumerated values variant"]
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
        *self == STOP_AT_BLOCK_GAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STOP_AT_BLOCK_GAP_A::VALUE2
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` writer - Stop At Block Gap Request"]
pub type STOP_AT_BLOCK_GAP_W<'a, const O: u8> = crate::BitWriter<'a, u8, BLOCK_GAP_CTRL_SPEC, STOP_AT_BLOCK_GAP_A, O>;
impl<'a, const O: u8> STOP_AT_BLOCK_GAP_W<'a, O> {
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
}
#[doc = "Field `CONTINUE_REQ` reader - Continue Request"]
pub type CONTINUE_REQ_R = crate::BitReader<CONTINUE_REQ_A>;
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CONTINUE_REQ_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CONTINUE_REQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CONTINUE_REQ_A::VALUE2
    }
}
#[doc = "Field `CONTINUE_REQ` writer - Continue Request"]
pub type CONTINUE_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, BLOCK_GAP_CTRL_SPEC, CONTINUE_REQ_A, O>;
impl<'a, const O: u8> CONTINUE_REQ_W<'a, O> {
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
}
#[doc = "Field `READ_WAIT_CTRL` reader - Read Wait Control"]
pub type READ_WAIT_CTRL_R = crate::BitReader<READ_WAIT_CTRL_A>;
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl READ_WAIT_CTRL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == READ_WAIT_CTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == READ_WAIT_CTRL_A::VALUE2
    }
}
#[doc = "Field `READ_WAIT_CTRL` writer - Read Wait Control"]
pub type READ_WAIT_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u8, BLOCK_GAP_CTRL_SPEC, READ_WAIT_CTRL_A, O>;
impl<'a, const O: u8> READ_WAIT_CTRL_W<'a, O> {
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
}
#[doc = "Field `INT_AT_BLOCK_GAP` reader - Interrupt At Block Gap"]
pub type INT_AT_BLOCK_GAP_R = crate::BitReader<bool>;
#[doc = "Field `INT_AT_BLOCK_GAP` writer - Interrupt At Block Gap"]
pub type INT_AT_BLOCK_GAP_W<'a, const O: u8> = crate::BitWriter<'a, u8, BLOCK_GAP_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&self) -> STOP_AT_BLOCK_GAP_R {
        STOP_AT_BLOCK_GAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&self) -> READ_WAIT_CTRL_R {
        READ_WAIT_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&self) -> INT_AT_BLOCK_GAP_R {
        INT_AT_BLOCK_GAP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    #[must_use]
    pub fn stop_at_block_gap(&mut self) -> STOP_AT_BLOCK_GAP_W<0> {
        STOP_AT_BLOCK_GAP_W::new(self)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W<1> {
        CONTINUE_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn read_wait_ctrl(&mut self) -> READ_WAIT_CTRL_W<2> {
        READ_WAIT_CTRL_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_block_gap(&mut self) -> INT_AT_BLOCK_GAP_W<3> {
        INT_AT_BLOCK_GAP_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_GAP_CTRL to value 0"]
impl crate::Resettable for BLOCK_GAP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
