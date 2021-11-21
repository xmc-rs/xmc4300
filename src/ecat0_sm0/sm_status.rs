#[doc = "Register `SM_STATUS` reader"]
pub struct R(crate::R<SM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct INT_W_R(crate::FieldReader<bool, INT_W_A>);
impl INT_W_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_W_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_W_A {
        match self.bits {
            false => INT_W_A::VALUE1,
            true => INT_W_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INT_W_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INT_W_A::VALUE2
    }
}
impl core::ops::Deref for INT_W_R {
    type Target = crate::FieldReader<bool, INT_W_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct INT_R_R(crate::FieldReader<bool, INT_R_A>);
impl INT_R_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_R_A {
        match self.bits {
            false => INT_R_A::VALUE1,
            true => INT_R_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == INT_R_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == INT_R_A::VALUE2
    }
}
impl core::ops::Deref for INT_R_R {
    type Target = crate::FieldReader<bool, INT_R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mailbox mode: mailbox status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MB_STATUS_R(crate::FieldReader<bool, MB_STATUS_A>);
impl MB_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB_STATUS_A {
        match self.bits {
            false => MB_STATUS_A::VALUE1,
            true => MB_STATUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MB_STATUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MB_STATUS_A::VALUE2
    }
}
impl core::ops::Deref for MB_STATUS_R {
    type Target = crate::FieldReader<bool, MB_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Buffered mode: buffer status (last written buffer)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `BUF_STATUS` reader - Buffered mode: buffer status (last written buffer)"]
pub struct BUF_STATUS_R(crate::FieldReader<u8, BUF_STATUS_A>);
impl BUF_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUF_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF_STATUS_A {
        match self.bits {
            0 => BUF_STATUS_A::VALUE1,
            1 => BUF_STATUS_A::VALUE2,
            2 => BUF_STATUS_A::VALUE3,
            3 => BUF_STATUS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUF_STATUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUF_STATUS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == BUF_STATUS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == BUF_STATUS_A::VALUE4
    }
}
impl core::ops::Deref for BUF_STATUS_R {
    type Target = crate::FieldReader<u8, BUF_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct R_BUF_IU_R(crate::FieldReader<bool, R_BUF_IU_A>);
impl R_BUF_IU_R {
    pub(crate) fn new(bits: bool) -> Self {
        R_BUF_IU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R_BUF_IU_A {
        match self.bits {
            false => R_BUF_IU_A::VALUE1,
            true => R_BUF_IU_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == R_BUF_IU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == R_BUF_IU_A::VALUE2
    }
}
impl core::ops::Deref for R_BUF_IU_R {
    type Target = crate::FieldReader<bool, R_BUF_IU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write buffer in use (opened)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct W_BUF_IU_R(crate::FieldReader<bool, W_BUF_IU_A>);
impl W_BUF_IU_R {
    pub(crate) fn new(bits: bool) -> Self {
        W_BUF_IU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_BUF_IU_A {
        match self.bits {
            false => W_BUF_IU_A::VALUE1,
            true => W_BUF_IU_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == W_BUF_IU_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == W_BUF_IU_A::VALUE2
    }
}
impl core::ops::Deref for W_BUF_IU_R {
    type Target = crate::FieldReader<bool, W_BUF_IU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Write"]
    #[inline(always)]
    pub fn int_w(&self) -> INT_W_R {
        INT_W_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Read"]
    #[inline(always)]
    pub fn int_r(&self) -> INT_R_R {
        INT_R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mailbox mode: mailbox status"]
    #[inline(always)]
    pub fn mb_status(&self) -> MB_STATUS_R {
        MB_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Buffered mode: buffer status (last written buffer)"]
    #[inline(always)]
    pub fn buf_status(&self) -> BUF_STATUS_R {
        BUF_STATUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Read buffer in use (opened)"]
    #[inline(always)]
    pub fn r_buf_iu(&self) -> R_BUF_IU_R {
        R_BUF_IU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write buffer in use (opened)"]
    #[inline(always)]
    pub fn w_buf_iu(&self) -> W_BUF_IU_R {
        W_BUF_IU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Status Register SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_status](index.html) module"]
pub struct SM_STATUS_SPEC;
impl crate::RegisterSpec for SM_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sm_status::R](R) reader structure"]
impl crate::Readable for SM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM_STATUS to value 0x30"]
impl crate::Resettable for SM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
