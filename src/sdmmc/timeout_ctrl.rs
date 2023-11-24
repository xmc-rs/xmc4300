#[doc = "Register `TIMEOUT_CTRL` reader"]
pub type R = crate::R<TIMEOUT_CTRL_SPEC>;
#[doc = "Register `TIMEOUT_CTRL` writer"]
pub type W = crate::W<TIMEOUT_CTRL_SPEC>;
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` reader - Data Timeout Counter Value"]
pub type DAT_TIMEOUT_CNT_VAL_R = crate::FieldReader<DAT_TIMEOUT_CNT_VAL_A>;
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAT_TIMEOUT_CNT_VAL_A {
    #[doc = "0: TMCLK * 2^13"]
    VALUE1 = 0,
    #[doc = "1: TMCLK * 2^14"]
    VALUE2 = 1,
    #[doc = "14: TMCLK * 2^27"]
    VALUE3 = 14,
}
impl From<DAT_TIMEOUT_CNT_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_TIMEOUT_CNT_VAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAT_TIMEOUT_CNT_VAL_A {
    type Ux = u8;
}
impl DAT_TIMEOUT_CNT_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAT_TIMEOUT_CNT_VAL_A> {
        match self.bits {
            0 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE1),
            1 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE2),
            14 => Some(DAT_TIMEOUT_CNT_VAL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "TMCLK * 2^13"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE1
    }
    #[doc = "TMCLK * 2^14"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE2
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DAT_TIMEOUT_CNT_VAL_A::VALUE3
    }
}
#[doc = "Field `DAT_TIMEOUT_CNT_VAL` writer - Data Timeout Counter Value"]
pub type DAT_TIMEOUT_CNT_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DAT_TIMEOUT_CNT_VAL_A>;
impl<'a, REG> DAT_TIMEOUT_CNT_VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMCLK * 2^13"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE1)
    }
    #[doc = "TMCLK * 2^14"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE2)
    }
    #[doc = "TMCLK * 2^27"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_TIMEOUT_CNT_VAL_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dat_timeout_cnt_val(&self) -> DAT_TIMEOUT_CNT_VAL_R {
        DAT_TIMEOUT_CNT_VAL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dat_timeout_cnt_val(&mut self) -> DAT_TIMEOUT_CNT_VAL_W<TIMEOUT_CTRL_SPEC> {
        DAT_TIMEOUT_CNT_VAL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_CTRL_SPEC;
impl crate::RegisterSpec for TIMEOUT_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timeout_ctrl::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctrl::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CTRL to value 0"]
impl crate::Resettable for TIMEOUT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
