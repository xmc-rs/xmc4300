#[doc = "Register `SM_STATUS` reader"]
pub type R = crate::R<SmStatusSpec>;
#[doc = "Interrupt Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntW {
    #[doc = "0: Interrupt cleared after first byte of buffer was read"]
    Value1 = 0,
    #[doc = "1: Interrupt after buffer was completely and successfully written"]
    Value2 = 1,
}
impl From<IntW> for bool {
    #[inline(always)]
    fn from(variant: IntW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_W` reader - Interrupt Write"]
pub type IntWR = crate::BitReader<IntW>;
impl IntWR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntW {
        match self.bits {
            false => IntW::Value1,
            true => IntW::Value2,
        }
    }
    #[doc = "Interrupt cleared after first byte of buffer was read"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IntW::Value1
    }
    #[doc = "Interrupt after buffer was completely and successfully written"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IntW::Value2
    }
}
#[doc = "Interrupt Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntR {
    #[doc = "0: Interrupt cleared after first byte of buffer was written"]
    Value1 = 0,
    #[doc = "1: Interrupt after buffer was completely and successful read"]
    Value2 = 1,
}
impl From<IntR> for bool {
    #[inline(always)]
    fn from(variant: IntR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_R` reader - Interrupt Read"]
pub type IntRR = crate::BitReader<IntR>;
impl IntRR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntR {
        match self.bits {
            false => IntR::Value1,
            true => IntR::Value2,
        }
    }
    #[doc = "Interrupt cleared after first byte of buffer was written"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IntR::Value1
    }
    #[doc = "Interrupt after buffer was completely and successful read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IntR::Value2
    }
}
#[doc = "Mailbox mode: mailbox status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbStatus {
    #[doc = "0: Mailbox empty"]
    Value1 = 0,
    #[doc = "1: Mailbox full"]
    Value2 = 1,
}
impl From<MbStatus> for bool {
    #[inline(always)]
    fn from(variant: MbStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB_STATUS` reader - Mailbox mode: mailbox status"]
pub type MbStatusR = crate::BitReader<MbStatus>;
impl MbStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MbStatus {
        match self.bits {
            false => MbStatus::Value1,
            true => MbStatus::Value2,
        }
    }
    #[doc = "Mailbox empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MbStatus::Value1
    }
    #[doc = "Mailbox full"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MbStatus::Value2
    }
}
#[doc = "Buffered mode: buffer status (last written buffer)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BufStatus {
    #[doc = "0: 1. buffer"]
    Value1 = 0,
    #[doc = "1: 2. buffer"]
    Value2 = 1,
    #[doc = "2: 3. buffer"]
    Value3 = 2,
    #[doc = "3: (no buffer written)"]
    Value4 = 3,
}
impl From<BufStatus> for u8 {
    #[inline(always)]
    fn from(variant: BufStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BufStatus {
    type Ux = u8;
}
#[doc = "Field `BUF_STATUS` reader - Buffered mode: buffer status (last written buffer)"]
pub type BufStatusR = crate::FieldReader<BufStatus>;
impl BufStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BufStatus {
        match self.bits {
            0 => BufStatus::Value1,
            1 => BufStatus::Value2,
            2 => BufStatus::Value3,
            3 => BufStatus::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "1. buffer"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BufStatus::Value1
    }
    #[doc = "2. buffer"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BufStatus::Value2
    }
    #[doc = "3. buffer"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BufStatus::Value3
    }
    #[doc = "(no buffer written)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BufStatus::Value4
    }
}
#[doc = "Read buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBufIu {
    #[doc = "0: buffer not in use"]
    Value1 = 0,
    #[doc = "1: buffer in use"]
    Value2 = 1,
}
impl From<RBufIu> for bool {
    #[inline(always)]
    fn from(variant: RBufIu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_BUF_IU` reader - Read buffer in use (opened)"]
pub type RBufIuR = crate::BitReader<RBufIu>;
impl RBufIuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBufIu {
        match self.bits {
            false => RBufIu::Value1,
            true => RBufIu::Value2,
        }
    }
    #[doc = "buffer not in use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBufIu::Value1
    }
    #[doc = "buffer in use"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBufIu::Value2
    }
}
#[doc = "Write buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WBufIu {
    #[doc = "0: buffer not in use"]
    Value1 = 0,
    #[doc = "1: buffer in use"]
    Value2 = 1,
}
impl From<WBufIu> for bool {
    #[inline(always)]
    fn from(variant: WBufIu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_BUF_IU` reader - Write buffer in use (opened)"]
pub type WBufIuR = crate::BitReader<WBufIu>;
impl WBufIuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WBufIu {
        match self.bits {
            false => WBufIu::Value1,
            true => WBufIu::Value2,
        }
    }
    #[doc = "buffer not in use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WBufIu::Value1
    }
    #[doc = "buffer in use"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WBufIu::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Write"]
    #[inline(always)]
    pub fn int_w(&self) -> IntWR {
        IntWR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Read"]
    #[inline(always)]
    pub fn int_r(&self) -> IntRR {
        IntRR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox mode: mailbox status"]
    #[inline(always)]
    pub fn mb_status(&self) -> MbStatusR {
        MbStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Buffered mode: buffer status (last written buffer)"]
    #[inline(always)]
    pub fn buf_status(&self) -> BufStatusR {
        BufStatusR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Read buffer in use (opened)"]
    #[inline(always)]
    pub fn r_buf_iu(&self) -> RBufIuR {
        RBufIuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write buffer in use (opened)"]
    #[inline(always)]
    pub fn w_buf_iu(&self) -> WBufIuR {
        WBufIuR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmStatusSpec;
impl crate::RegisterSpec for SmStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_status::R`](R) reader structure"]
impl crate::Readable for SmStatusSpec {}
#[doc = "`reset()` method sets SM_STATUS to value 0x30"]
impl crate::Resettable for SmStatusSpec {
    const RESET_VALUE: u8 = 0x30;
}
