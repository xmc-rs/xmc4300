#[doc = "Register `SM_STATUS` reader"]
pub type R = crate::R<SM_STATUS_SPEC>;
#[doc = "Interrupt Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_W_A {
    #[doc = "0: Interrupt cleared after first byte of buffer was read"]
    VALUE1 = 0,
    #[doc = "1: Interrupt after buffer was completely and successfully written"]
    VALUE2 = 1,
}
impl From<INT_W_A> for bool {
    #[inline(always)]
    fn from(variant: INT_W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_W` reader - Interrupt Write"]
pub type INT_W_R = crate::BitReader<INT_W_A>;
impl INT_W_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_W_A {
        match self.bits {
            false => INT_W_A::VALUE1,
            true => INT_W_A::VALUE2,
        }
    }
    #[doc = "Interrupt cleared after first byte of buffer was read"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_W_A::VALUE1
    }
    #[doc = "Interrupt after buffer was completely and successfully written"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_W_A::VALUE2
    }
}
#[doc = "Interrupt Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_R_A {
    #[doc = "0: Interrupt cleared after first byte of buffer was written"]
    VALUE1 = 0,
    #[doc = "1: Interrupt after buffer was completely and successful read"]
    VALUE2 = 1,
}
impl From<INT_R_A> for bool {
    #[inline(always)]
    fn from(variant: INT_R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_R` reader - Interrupt Read"]
pub type INT_R_R = crate::BitReader<INT_R_A>;
impl INT_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_R_A {
        match self.bits {
            false => INT_R_A::VALUE1,
            true => INT_R_A::VALUE2,
        }
    }
    #[doc = "Interrupt cleared after first byte of buffer was written"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_R_A::VALUE1
    }
    #[doc = "Interrupt after buffer was completely and successful read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_R_A::VALUE2
    }
}
#[doc = "Mailbox mode: mailbox status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB_STATUS_A {
    #[doc = "0: Mailbox empty"]
    VALUE1 = 0,
    #[doc = "1: Mailbox full"]
    VALUE2 = 1,
}
impl From<MB_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MB_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB_STATUS` reader - Mailbox mode: mailbox status"]
pub type MB_STATUS_R = crate::BitReader<MB_STATUS_A>;
impl MB_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MB_STATUS_A {
        match self.bits {
            false => MB_STATUS_A::VALUE1,
            true => MB_STATUS_A::VALUE2,
        }
    }
    #[doc = "Mailbox empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MB_STATUS_A::VALUE1
    }
    #[doc = "Mailbox full"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MB_STATUS_A::VALUE2
    }
}
#[doc = "Buffered mode: buffer status (last written buffer)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUF_STATUS_A {
    #[doc = "0: 1. buffer"]
    VALUE1 = 0,
    #[doc = "1: 2. buffer"]
    VALUE2 = 1,
    #[doc = "2: 3. buffer"]
    VALUE3 = 2,
    #[doc = "3: (no buffer written)"]
    VALUE4 = 3,
}
impl From<BUF_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: BUF_STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUF_STATUS_A {
    type Ux = u8;
}
impl crate::IsEnum for BUF_STATUS_A {}
#[doc = "Field `BUF_STATUS` reader - Buffered mode: buffer status (last written buffer)"]
pub type BUF_STATUS_R = crate::FieldReader<BUF_STATUS_A>;
impl BUF_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUF_STATUS_A {
        match self.bits {
            0 => BUF_STATUS_A::VALUE1,
            1 => BUF_STATUS_A::VALUE2,
            2 => BUF_STATUS_A::VALUE3,
            3 => BUF_STATUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "1. buffer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUF_STATUS_A::VALUE1
    }
    #[doc = "2. buffer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUF_STATUS_A::VALUE2
    }
    #[doc = "3. buffer"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BUF_STATUS_A::VALUE3
    }
    #[doc = "(no buffer written)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BUF_STATUS_A::VALUE4
    }
}
#[doc = "Read buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R_BUF_IU_A {
    #[doc = "0: buffer not in use"]
    VALUE1 = 0,
    #[doc = "1: buffer in use"]
    VALUE2 = 1,
}
impl From<R_BUF_IU_A> for bool {
    #[inline(always)]
    fn from(variant: R_BUF_IU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_BUF_IU` reader - Read buffer in use (opened)"]
pub type R_BUF_IU_R = crate::BitReader<R_BUF_IU_A>;
impl R_BUF_IU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> R_BUF_IU_A {
        match self.bits {
            false => R_BUF_IU_A::VALUE1,
            true => R_BUF_IU_A::VALUE2,
        }
    }
    #[doc = "buffer not in use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == R_BUF_IU_A::VALUE1
    }
    #[doc = "buffer in use"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == R_BUF_IU_A::VALUE2
    }
}
#[doc = "Write buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W_BUF_IU_A {
    #[doc = "0: buffer not in use"]
    VALUE1 = 0,
    #[doc = "1: buffer in use"]
    VALUE2 = 1,
}
impl From<W_BUF_IU_A> for bool {
    #[inline(always)]
    fn from(variant: W_BUF_IU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_BUF_IU` reader - Write buffer in use (opened)"]
pub type W_BUF_IU_R = crate::BitReader<W_BUF_IU_A>;
impl W_BUF_IU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> W_BUF_IU_A {
        match self.bits {
            false => W_BUF_IU_A::VALUE1,
            true => W_BUF_IU_A::VALUE2,
        }
    }
    #[doc = "buffer not in use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == W_BUF_IU_A::VALUE1
    }
    #[doc = "buffer in use"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_BUF_IU_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Write"]
    #[inline(always)]
    pub fn int_w(&self) -> INT_W_R {
        INT_W_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Read"]
    #[inline(always)]
    pub fn int_r(&self) -> INT_R_R {
        INT_R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox mode: mailbox status"]
    #[inline(always)]
    pub fn mb_status(&self) -> MB_STATUS_R {
        MB_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Buffered mode: buffer status (last written buffer)"]
    #[inline(always)]
    pub fn buf_status(&self) -> BUF_STATUS_R {
        BUF_STATUS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Read buffer in use (opened)"]
    #[inline(always)]
    pub fn r_buf_iu(&self) -> R_BUF_IU_R {
        R_BUF_IU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write buffer in use (opened)"]
    #[inline(always)]
    pub fn w_buf_iu(&self) -> W_BUF_IU_R {
        W_BUF_IU_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register SyncManager 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sm_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_STATUS_SPEC;
impl crate::RegisterSpec for SM_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_status::R`](R) reader structure"]
impl crate::Readable for SM_STATUS_SPEC {}
#[doc = "`reset()` method sets SM_STATUS to value 0x30"]
impl crate::Resettable for SM_STATUS_SPEC {
    const RESET_VALUE: u8 = 0x30;
}
