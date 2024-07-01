#[doc = "Register `BLOCK_GAP_CTRL` reader"]
pub type R = crate::R<BLOCK_GAP_CTRL_SPEC>;
#[doc = "Register `BLOCK_GAP_CTRL` writer"]
pub type W = crate::W<BLOCK_GAP_CTRL_SPEC>;
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
#[doc = "Field `STOP_AT_BLOCK_GAP` reader - Stop At Block Gap Request"]
pub type STOP_AT_BLOCK_GAP_R = crate::BitReader<STOP_AT_BLOCK_GAP_A>;
impl STOP_AT_BLOCK_GAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_AT_BLOCK_GAP_A {
        match self.bits {
            false => STOP_AT_BLOCK_GAP_A::VALUE1,
            true => STOP_AT_BLOCK_GAP_A::VALUE2,
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STOP_AT_BLOCK_GAP_A::VALUE1
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STOP_AT_BLOCK_GAP_A::VALUE2
    }
}
#[doc = "Field `STOP_AT_BLOCK_GAP` writer - Stop At Block Gap Request"]
pub type STOP_AT_BLOCK_GAP_W<'a, REG> = crate::BitWriter<'a, REG, STOP_AT_BLOCK_GAP_A>;
impl<'a, REG> STOP_AT_BLOCK_GAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE1)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE2)
    }
}
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
#[doc = "Field `CONTINUE_REQ` reader - Continue Request"]
pub type CONTINUE_REQ_R = crate::BitReader<CONTINUE_REQ_A>;
impl CONTINUE_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONTINUE_REQ_A {
        match self.bits {
            false => CONTINUE_REQ_A::VALUE1,
            true => CONTINUE_REQ_A::VALUE2,
        }
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CONTINUE_REQ_A::VALUE1
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CONTINUE_REQ_A::VALUE2
    }
}
#[doc = "Field `CONTINUE_REQ` writer - Continue Request"]
pub type CONTINUE_REQ_W<'a, REG> = crate::BitWriter<'a, REG, CONTINUE_REQ_A>;
impl<'a, REG> CONTINUE_REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CONTINUE_REQ_A::VALUE1)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CONTINUE_REQ_A::VALUE2)
    }
}
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
#[doc = "Field `READ_WAIT_CTRL` reader - Read Wait Control"]
pub type READ_WAIT_CTRL_R = crate::BitReader<READ_WAIT_CTRL_A>;
impl READ_WAIT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> READ_WAIT_CTRL_A {
        match self.bits {
            false => READ_WAIT_CTRL_A::VALUE1,
            true => READ_WAIT_CTRL_A::VALUE2,
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == READ_WAIT_CTRL_A::VALUE1
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == READ_WAIT_CTRL_A::VALUE2
    }
}
#[doc = "Field `READ_WAIT_CTRL` writer - Read Wait Control"]
pub type READ_WAIT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, READ_WAIT_CTRL_A>;
impl<'a, REG> READ_WAIT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(READ_WAIT_CTRL_A::VALUE1)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(READ_WAIT_CTRL_A::VALUE2)
    }
}
#[doc = "Field `INT_AT_BLOCK_GAP` reader - Interrupt At Block Gap"]
pub type INT_AT_BLOCK_GAP_R = crate::BitReader;
#[doc = "Field `INT_AT_BLOCK_GAP` writer - Interrupt At Block Gap"]
pub type INT_AT_BLOCK_GAP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn stop_at_block_gap(&mut self) -> STOP_AT_BLOCK_GAP_W<BLOCK_GAP_CTRL_SPEC> {
        STOP_AT_BLOCK_GAP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W<BLOCK_GAP_CTRL_SPEC> {
        CONTINUE_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn read_wait_ctrl(&mut self) -> READ_WAIT_CTRL_W<BLOCK_GAP_CTRL_SPEC> {
        READ_WAIT_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_block_gap(&mut self) -> INT_AT_BLOCK_GAP_W<BLOCK_GAP_CTRL_SPEC> {
        INT_AT_BLOCK_GAP_W::new(self, 3)
    }
}
#[doc = "Block Gap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_gap_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_gap_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_GAP_CTRL_SPEC;
impl crate::RegisterSpec for BLOCK_GAP_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`block_gap_ctrl::R`](R) reader structure"]
impl crate::Readable for BLOCK_GAP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`block_gap_ctrl::W`](W) writer structure"]
impl crate::Writable for BLOCK_GAP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BLOCK_GAP_CTRL to value 0"]
impl crate::Resettable for BLOCK_GAP_CTRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
