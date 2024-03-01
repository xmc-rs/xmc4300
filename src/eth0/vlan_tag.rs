#[doc = "Register `VLAN_TAG` reader"]
pub type R = crate::R<VlanTagSpec>;
#[doc = "Register `VLAN_TAG` writer"]
pub type W = crate::W<VlanTagSpec>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames"]
pub type VlR = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames"]
pub type VlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type EtvR = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VtimR = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type EsvlR = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type EsvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type VthmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VlR {
        VlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VtimR {
        VtimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> EsvlR {
        EsvlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VthmR {
        VthmR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VlW<VlanTagSpec> {
        VlW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<VlanTagSpec> {
        EtvW::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VtimW<VlanTagSpec> {
        VtimW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> EsvlW<VlanTagSpec> {
        EsvlW::new(self, 18)
    }
}
#[doc = "VLAN Tag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanTagSpec;
impl crate::RegisterSpec for VlanTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_tag::R`](R) reader structure"]
impl crate::Readable for VlanTagSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_tag::W`](W) writer structure"]
impl crate::Writable for VlanTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VLAN_TAG to value 0"]
impl crate::Resettable for VlanTagSpec {
    const RESET_VALUE: u32 = 0;
}
