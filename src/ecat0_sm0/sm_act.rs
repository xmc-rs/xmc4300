#[doc = "Register `SM_ACT` reader"]
pub struct R(crate::R<SM_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_ACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SM_EN` reader - SyncManager Enable/Disable"]
pub type SM_EN_R = crate::BitReader<SM_EN_A>;
#[doc = "SyncManager Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM_EN_A {
    #[doc = "0: Disable: Access to Memory without SyncManager control"]
    VALUE1 = 0,
    #[doc = "1: Enable: SyncManager is active and controls Memory area set in configuration"]
    VALUE2 = 1,
}
impl From<SM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_EN_A {
        match self.bits {
            false => SM_EN_A::VALUE1,
            true => SM_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SM_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SM_EN_A::VALUE2
    }
}
#[doc = "Field `REP_REQ` reader - Repeat Request"]
pub type REP_REQ_R = crate::BitReader<bool>;
#[doc = "Field `LE_ECAT` reader - LatchEvent ECAT"]
pub type LE_ECAT_R = crate::BitReader<LE_ECAT_A>;
#[doc = "LatchEvent ECAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LE_ECAT_A {
    #[doc = "0: No"]
    VALUE1 = 0,
    #[doc = "1: Generate Latch event if EtherCAT master issues a buffer exchange"]
    VALUE2 = 1,
}
impl From<LE_ECAT_A> for bool {
    #[inline(always)]
    fn from(variant: LE_ECAT_A) -> Self {
        variant as u8 != 0
    }
}
impl LE_ECAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LE_ECAT_A {
        match self.bits {
            false => LE_ECAT_A::VALUE1,
            true => LE_ECAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LE_ECAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LE_ECAT_A::VALUE2
    }
}
#[doc = "Field `LE_PDI` reader - LatchEvent PDI"]
pub type LE_PDI_R = crate::BitReader<LE_PDI_A>;
#[doc = "LatchEvent PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LE_PDI_A {
    #[doc = "0: No"]
    VALUE1 = 0,
    #[doc = "1: Generate Latch events if PDI issues a buffer exchange or if PDI accesses buffer start address"]
    VALUE2 = 1,
}
impl From<LE_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: LE_PDI_A) -> Self {
        variant as u8 != 0
    }
}
impl LE_PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LE_PDI_A {
        match self.bits {
            false => LE_PDI_A::VALUE1,
            true => LE_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LE_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LE_PDI_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SyncManager Enable/Disable"]
    #[inline(always)]
    pub fn sm_en(&self) -> SM_EN_R {
        SM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat Request"]
    #[inline(always)]
    pub fn rep_req(&self) -> REP_REQ_R {
        REP_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LatchEvent ECAT"]
    #[inline(always)]
    pub fn le_ecat(&self) -> LE_ECAT_R {
        LE_ECAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LatchEvent PDI"]
    #[inline(always)]
    pub fn le_pdi(&self) -> LE_PDI_R {
        LE_PDI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Activate SyncManager 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm_act](index.html) module"]
pub struct SM_ACT_SPEC;
impl crate::RegisterSpec for SM_ACT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sm_act::R](R) reader structure"]
impl crate::Readable for SM_ACT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SM_ACT to value 0"]
impl crate::Resettable for SM_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
