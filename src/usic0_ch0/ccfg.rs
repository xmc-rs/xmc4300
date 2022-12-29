#[doc = "Register `CCFG` reader"]
pub struct R(crate::R<CCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSC` reader - SSC Protocol Available"]
pub type SSC_R = crate::BitReader<SSC_A>;
#[doc = "SSC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSC_A {
    #[doc = "0: The SSC protocol is not available."]
    VALUE1 = 0,
    #[doc = "1: The SSC protocol is available."]
    VALUE2 = 1,
}
impl From<SSC_A> for bool {
    #[inline(always)]
    fn from(variant: SSC_A) -> Self {
        variant as u8 != 0
    }
}
impl SSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSC_A {
        match self.bits {
            false => SSC_A::VALUE1,
            true => SSC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSC_A::VALUE2
    }
}
#[doc = "Field `ASC` reader - ASC Protocol Available"]
pub type ASC_R = crate::BitReader<ASC_A>;
#[doc = "ASC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASC_A {
    #[doc = "0: The ASC protocol is not available."]
    VALUE1 = 0,
    #[doc = "1: The ASC protocol is available."]
    VALUE2 = 1,
}
impl From<ASC_A> for bool {
    #[inline(always)]
    fn from(variant: ASC_A) -> Self {
        variant as u8 != 0
    }
}
impl ASC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASC_A {
        match self.bits {
            false => ASC_A::VALUE1,
            true => ASC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ASC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ASC_A::VALUE2
    }
}
#[doc = "Field `IIC` reader - IIC Protocol Available"]
pub type IIC_R = crate::BitReader<IIC_A>;
#[doc = "IIC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIC_A {
    #[doc = "0: The IIC protocol is not available."]
    VALUE1 = 0,
    #[doc = "1: The IIC protocol is available."]
    VALUE2 = 1,
}
impl From<IIC_A> for bool {
    #[inline(always)]
    fn from(variant: IIC_A) -> Self {
        variant as u8 != 0
    }
}
impl IIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIC_A {
        match self.bits {
            false => IIC_A::VALUE1,
            true => IIC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IIC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IIC_A::VALUE2
    }
}
#[doc = "Field `IIS` reader - IIS Protocol Available"]
pub type IIS_R = crate::BitReader<IIS_A>;
#[doc = "IIS Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIS_A {
    #[doc = "0: The IIS protocol is not available."]
    VALUE1 = 0,
    #[doc = "1: The IIS protocol is available."]
    VALUE2 = 1,
}
impl From<IIS_A> for bool {
    #[inline(always)]
    fn from(variant: IIS_A) -> Self {
        variant as u8 != 0
    }
}
impl IIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIS_A {
        match self.bits {
            false => IIS_A::VALUE1,
            true => IIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IIS_A::VALUE2
    }
}
#[doc = "Field `RB` reader - Receive FIFO Buffer Available"]
pub type RB_R = crate::BitReader<RB_A>;
#[doc = "Receive FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RB_A {
    #[doc = "0: A receive FIFO buffer is not available."]
    VALUE1 = 0,
    #[doc = "1: A receive FIFO buffer is available."]
    VALUE2 = 1,
}
impl From<RB_A> for bool {
    #[inline(always)]
    fn from(variant: RB_A) -> Self {
        variant as u8 != 0
    }
}
impl RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RB_A {
        match self.bits {
            false => RB_A::VALUE1,
            true => RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RB_A::VALUE2
    }
}
#[doc = "Field `TB` reader - Transmit FIFO Buffer Available"]
pub type TB_R = crate::BitReader<TB_A>;
#[doc = "Transmit FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TB_A {
    #[doc = "0: A transmit FIFO buffer is not available."]
    VALUE1 = 0,
    #[doc = "1: A transmit FIFO buffer is available."]
    VALUE2 = 1,
}
impl From<TB_A> for bool {
    #[inline(always)]
    fn from(variant: TB_A) -> Self {
        variant as u8 != 0
    }
}
impl TB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TB_A {
        match self.bits {
            false => TB_A::VALUE1,
            true => TB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SSC Protocol Available"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASC Protocol Available"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IIC Protocol Available"]
    #[inline(always)]
    pub fn iic(&self) -> IIC_R {
        IIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IIS Protocol Available"]
    #[inline(always)]
    pub fn iis(&self) -> IIS_R {
        IIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Available"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Buffer Available"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg](index.html) module"]
pub struct CCFG_SPEC;
impl crate::RegisterSpec for CCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg::R](R) reader structure"]
impl crate::Readable for CCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCFG to value 0xcf"]
impl crate::Resettable for CCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xcf;
}
