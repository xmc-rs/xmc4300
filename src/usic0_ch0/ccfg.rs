#[doc = "Reader of register CCFG"]
pub type R = crate::R<u32, super::CCFG>;
#[doc = "SSC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SSC`"]
pub type SSC_R = crate::R<bool, SSC_A>;
impl SSC_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "ASC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<bool, ASC_A>;
impl ASC_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "IIC Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IIC`"]
pub type IIC_R = crate::R<bool, IIC_A>;
impl IIC_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "IIS Protocol Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IIS`"]
pub type IIS_R = crate::R<bool, IIS_A>;
impl IIS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Receive FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RB`"]
pub type RB_R = crate::R<bool, RB_A>;
impl RB_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Transmit FIFO Buffer Available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TB`"]
pub type TB_R = crate::R<bool, TB_A>;
impl TB_R {
    #[doc = r"Get enumerated values variant"]
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
        SSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ASC Protocol Available"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IIC Protocol Available"]
    #[inline(always)]
    pub fn iic(&self) -> IIC_R {
        IIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IIS Protocol Available"]
    #[inline(always)]
    pub fn iis(&self) -> IIS_R {
        IIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Available"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Buffer Available"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
