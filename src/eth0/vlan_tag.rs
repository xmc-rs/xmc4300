#[doc = "Register `VLAN_TAG` reader"]
pub type R = crate::R<VLAN_TAG_SPEC>;
#[doc = "Register `VLAN_TAG` writer"]
pub type W = crate::W<VLAN_TAG_SPEC>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames"]
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VTIM_R = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type ESVL_R = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type VTHM_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<VLAN_TAG_SPEC> {
        VL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<VLAN_TAG_SPEC> {
        ETV_W::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<VLAN_TAG_SPEC> {
        VTIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<VLAN_TAG_SPEC> {
        ESVL_W::new(self, 18)
    }
}
#[doc = "VLAN Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLAN_TAG_SPEC;
impl crate::RegisterSpec for VLAN_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_tag::R`](R) reader structure"]
impl crate::Readable for VLAN_TAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vlan_tag::W`](W) writer structure"]
impl crate::Writable for VLAN_TAG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VLAN_TAG to value 0"]
impl crate::Resettable for VLAN_TAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
