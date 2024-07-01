#[doc = "Register `HCCHAR` reader"]
pub type R = crate::R<HCCHAR_SPEC>;
#[doc = "Register `HCCHAR` writer"]
pub type W = crate::W<HCCHAR_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNum` reader - Endpoint Number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNum` writer - Endpoint Number"]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPDIR_A {
    #[doc = "0: OUT"]
    VALUE1 = 0,
    #[doc = "1: IN"]
    VALUE2 = 1,
}
impl From<EPDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDir` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader<EPDIR_A>;
impl EPDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPDIR_A {
        match self.bits {
            false => EPDIR_A::VALUE1,
            true => EPDIR_A::VALUE2,
        }
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPDIR_A::VALUE1
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPDIR_A::VALUE2
    }
}
#[doc = "Field `EPDir` writer - Endpoint Direction"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG, EPDIR_A>;
impl<'a, REG> EPDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OUT"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPDIR_A::VALUE1)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPDIR_A::VALUE2)
    }
}
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    VALUE1 = 0,
    #[doc = "1: Isochronous"]
    VALUE2 = 1,
    #[doc = "2: Bulk"]
    VALUE3 = 2,
    #[doc = "3: Interrupt"]
    VALUE4 = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for EPTYPE_A {}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::VALUE1,
            1 => EPTYPE_A::VALUE2,
            2 => EPTYPE_A::VALUE3,
            3 => EPTYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPTYPE_A::VALUE1
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPTYPE_A::VALUE2
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EPTYPE_A::VALUE3
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EPTYPE_A::VALUE4
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EPTYPE_A, crate::Safe>;
impl<'a, REG> EPTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE4)
    }
}
#[doc = "Multi Count / Error Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MC_EC_A {
    #[doc = "1: 1 transaction"]
    VALUE2 = 1,
    #[doc = "2: 2 transactions to be issued for this endpoint per frame"]
    VALUE3 = 2,
    #[doc = "3: 3 transactions to be issued for this endpoint per frame"]
    VALUE4 = 3,
}
impl From<MC_EC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_EC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MC_EC_A {
    type Ux = u8;
}
impl crate::IsEnum for MC_EC_A {}
#[doc = "Field `MC_EC` reader - Multi Count / Error Count"]
pub type MC_EC_R = crate::FieldReader<MC_EC_A>;
impl MC_EC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MC_EC_A> {
        match self.bits {
            1 => Some(MC_EC_A::VALUE2),
            2 => Some(MC_EC_A::VALUE3),
            3 => Some(MC_EC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "1 transaction"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MC_EC_A::VALUE2
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MC_EC_A::VALUE3
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MC_EC_A::VALUE4
    }
}
#[doc = "Field `MC_EC` writer - Multi Count / Error Count"]
pub type MC_EC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MC_EC_A>;
impl<'a, REG> MC_EC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transaction"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MC_EC_A::VALUE2)
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MC_EC_A::VALUE3)
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MC_EC_A::VALUE4)
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub type DEV_ADDR_R = crate::FieldReader;
#[doc = "Field `DevAddr` writer - Device Address"]
pub type DEV_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Odd Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD_FRM_A {
    #[doc = "0: Even frame"]
    VALUE1 = 0,
    #[doc = "1: Odd frame"]
    VALUE2 = 1,
}
impl From<ODD_FRM_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_FRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OddFrm` reader - Odd Frame"]
pub type ODD_FRM_R = crate::BitReader<ODD_FRM_A>;
impl ODD_FRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD_FRM_A {
        match self.bits {
            false => ODD_FRM_A::VALUE1,
            true => ODD_FRM_A::VALUE2,
        }
    }
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ODD_FRM_A::VALUE1
    }
    #[doc = "Odd frame"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ODD_FRM_A::VALUE2
    }
}
#[doc = "Field `OddFrm` writer - Odd Frame"]
pub type ODD_FRM_W<'a, REG> = crate::BitWriter<'a, REG, ODD_FRM_A>;
impl<'a, REG> ODD_FRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_FRM_A::VALUE1)
    }
    #[doc = "Odd frame"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_FRM_A::VALUE2)
    }
}
#[doc = "Field `ChDis` reader - Channel Disable"]
pub type CH_DIS_R = crate::BitReader;
#[doc = "Field `ChDis` writer - Channel Disable"]
pub type CH_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_ENA_A {
    #[doc = "0: Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    VALUE1 = 0,
    #[doc = "1: Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    VALUE2 = 1,
}
impl From<CH_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CH_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ChEna` reader - Channel Enable"]
pub type CH_ENA_R = crate::BitReader<CH_ENA_A>;
impl CH_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_ENA_A {
        match self.bits {
            false => CH_ENA_A::VALUE1,
            true => CH_ENA_A::VALUE2,
        }
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_ENA_A::VALUE1
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_ENA_A::VALUE2
    }
}
#[doc = "Field `ChEna` writer - Channel Enable"]
pub type CH_ENA_W<'a, REG> = crate::BitWriter<'a, REG, CH_ENA_A>;
impl<'a, REG> CH_ENA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CH_ENA_A::VALUE1)
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CH_ENA_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    pub fn mc_ec(&self) -> MC_EC_R {
        MC_EC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn odd_frm(&self) -> ODD_FRM_R {
        ODD_FRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn ch_dis(&self) -> CH_DIS_R {
        CH_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn ch_ena(&self) -> CH_ENA_R {
        CH_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<HCCHAR_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<HCCHAR_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<HCCHAR_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HCCHAR_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    #[must_use]
    pub fn mc_ec(&mut self) -> MC_EC_W<HCCHAR_SPEC> {
        MC_EC_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<HCCHAR_SPEC> {
        DEV_ADDR_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    #[must_use]
    pub fn odd_frm(&mut self) -> ODD_FRM_W<HCCHAR_SPEC> {
        ODD_FRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_dis(&mut self) -> CH_DIS_W<HCCHAR_SPEC> {
        CH_DIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena(&mut self) -> CH_ENA_W<HCCHAR_SPEC> {
        CH_ENA_W::new(self, 31)
    }
}
#[doc = "Host Channel Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR_SPEC;
impl crate::RegisterSpec for HCCHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar::R`](R) reader structure"]
impl crate::Readable for HCCHAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar::W`](W) writer structure"]
impl crate::Writable for HCCHAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR to value 0"]
impl crate::Resettable for HCCHAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
