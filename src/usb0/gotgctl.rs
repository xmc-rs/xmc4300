#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GotgctlSpec>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GotgctlSpec>;
#[doc = "Session Request Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SesReqScs {
    #[doc = "0: Session request failure"]
    Value1 = 0,
    #[doc = "1: Session request success"]
    Value2 = 1,
}
impl From<SesReqScs> for bool {
    #[inline(always)]
    fn from(variant: SesReqScs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SesReqScs` reader - Session Request Success"]
pub type SesReqScsR = crate::BitReader<SesReqScs>;
impl SesReqScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SesReqScs {
        match self.bits {
            false => SesReqScs::Value1,
            true => SesReqScs::Value2,
        }
    }
    #[doc = "Session request failure"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SesReqScs::Value1
    }
    #[doc = "Session request success"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SesReqScs::Value2
    }
}
#[doc = "Session Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SesReq {
    #[doc = "0: No session request"]
    Value1 = 0,
    #[doc = "1: Session request"]
    Value2 = 1,
}
impl From<SesReq> for bool {
    #[inline(always)]
    fn from(variant: SesReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SesReq` reader - Session Request"]
pub type SesReqR = crate::BitReader<SesReq>;
impl SesReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SesReq {
        match self.bits {
            false => SesReq::Value1,
            true => SesReq::Value2,
        }
    }
    #[doc = "No session request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SesReq::Value1
    }
    #[doc = "Session request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SesReq::Value2
    }
}
#[doc = "Field `SesReq` writer - Session Request"]
pub type SesReqW<'a, REG> = crate::BitWriter<'a, REG, SesReq>;
impl<'a, REG> SesReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No session request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SesReq::Value1)
    }
    #[doc = "Session request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SesReq::Value2)
    }
}
#[doc = "VBUS Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbvalidOvEn {
    #[doc = "0: Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    Value1 = 0,
    #[doc = "1: Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    Value2 = 1,
}
impl From<VbvalidOvEn> for bool {
    #[inline(always)]
    fn from(variant: VbvalidOvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VbvalidOvEn` reader - VBUS Valid Override Enable"]
pub type VbvalidOvEnR = crate::BitReader<VbvalidOvEn>;
impl VbvalidOvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbvalidOvEn {
        match self.bits {
            false => VbvalidOvEn::Value1,
            true => VbvalidOvEn::Value2,
        }
    }
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbvalidOvEn::Value1
    }
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VbvalidOvEn::Value2
    }
}
#[doc = "Field `VbvalidOvEn` writer - VBUS Valid Override Enable"]
pub type VbvalidOvEnW<'a, REG> = crate::BitWriter<'a, REG, VbvalidOvEn>;
impl<'a, REG> VbvalidOvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override is disabled and vbus valid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbvalidOvEn::Value1)
    }
    #[doc = "Internally vbus valid received from the PHY is overridden with GOTGCTL.VbvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VbvalidOvEn::Value2)
    }
}
#[doc = "VBUS Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbvalidOvVal {
    #[doc = "0: vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    Value1 = 0,
    #[doc = "1: vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    Value2 = 1,
}
impl From<VbvalidOvVal> for bool {
    #[inline(always)]
    fn from(variant: VbvalidOvVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VbvalidOvVal` reader - VBUS Valid Override Value"]
pub type VbvalidOvValR = crate::BitReader<VbvalidOvVal>;
impl VbvalidOvValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbvalidOvVal {
        match self.bits {
            false => VbvalidOvVal::Value1,
            true => VbvalidOvVal::Value2,
        }
    }
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VbvalidOvVal::Value1
    }
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VbvalidOvVal::Value2
    }
}
#[doc = "Field `VbvalidOvVal` writer - VBUS Valid Override Value"]
pub type VbvalidOvValW<'a, REG> = crate::BitWriter<'a, REG, VbvalidOvVal>;
impl<'a, REG> VbvalidOvValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vbusvalid value is 0# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VbvalidOvVal::Value1)
    }
    #[doc = "vbusvalid value is 1# when GOTGCTL.VbvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VbvalidOvVal::Value2)
    }
}
#[doc = "A-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvalidOvEn {
    #[doc = "0: Override is disabled and Avalid signal from the PHY is used internally by the core."]
    Value1 = 0,
    #[doc = "1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    Value2 = 1,
}
impl From<AvalidOvEn> for bool {
    #[inline(always)]
    fn from(variant: AvalidOvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AvalidOvEn` reader - A-Peripheral Session Valid Override Enable"]
pub type AvalidOvEnR = crate::BitReader<AvalidOvEn>;
impl AvalidOvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AvalidOvEn {
        match self.bits {
            false => AvalidOvEn::Value1,
            true => AvalidOvEn::Value2,
        }
    }
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AvalidOvEn::Value1
    }
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AvalidOvEn::Value2
    }
}
#[doc = "Field `AvalidOvEn` writer - A-Peripheral Session Valid Override Enable"]
pub type AvalidOvEnW<'a, REG> = crate::BitWriter<'a, REG, AvalidOvEn>;
impl<'a, REG> AvalidOvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override is disabled and Avalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AvalidOvEn::Value1)
    }
    #[doc = "Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AvalidOvEn::Value2)
    }
}
#[doc = "A-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvalidOvVal {
    #[doc = "0: Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    Value1 = 0,
    #[doc = "1: Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    Value2 = 1,
}
impl From<AvalidOvVal> for bool {
    #[inline(always)]
    fn from(variant: AvalidOvVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AvalidOvVal` reader - A-Peripheral Session Valid Override Value"]
pub type AvalidOvValR = crate::BitReader<AvalidOvVal>;
impl AvalidOvValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AvalidOvVal {
        match self.bits {
            false => AvalidOvVal::Value1,
            true => AvalidOvVal::Value2,
        }
    }
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AvalidOvVal::Value1
    }
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AvalidOvVal::Value2
    }
}
#[doc = "Field `AvalidOvVal` writer - A-Peripheral Session Valid Override Value"]
pub type AvalidOvValW<'a, REG> = crate::BitWriter<'a, REG, AvalidOvVal>;
impl<'a, REG> AvalidOvValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Avalid value is 0# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AvalidOvVal::Value1)
    }
    #[doc = "Avalid value is 1# when GOTGCTL.AvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AvalidOvVal::Value2)
    }
}
#[doc = "B-Peripheral Session Valid Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BvalidOvEn {
    #[doc = "0: Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    Value1 = 0,
    #[doc = "1: Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    Value2 = 1,
}
impl From<BvalidOvEn> for bool {
    #[inline(always)]
    fn from(variant: BvalidOvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BvalidOvEn` reader - B-Peripheral Session Valid Override Enable"]
pub type BvalidOvEnR = crate::BitReader<BvalidOvEn>;
impl BvalidOvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BvalidOvEn {
        match self.bits {
            false => BvalidOvEn::Value1,
            true => BvalidOvEn::Value2,
        }
    }
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BvalidOvEn::Value1
    }
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BvalidOvEn::Value2
    }
}
#[doc = "Field `BvalidOvEn` writer - B-Peripheral Session Valid Override Enable"]
pub type BvalidOvEnW<'a, REG> = crate::BitWriter<'a, REG, BvalidOvEn>;
impl<'a, REG> BvalidOvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override is disabled and Bvalid signal from the PHY is used internally by the core."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BvalidOvEn::Value1)
    }
    #[doc = "Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BvalidOvEn::Value2)
    }
}
#[doc = "B-Peripheral Session Valid Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BvalidOvVal {
    #[doc = "0: Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    Value1 = 0,
    #[doc = "1: Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    Value2 = 1,
}
impl From<BvalidOvVal> for bool {
    #[inline(always)]
    fn from(variant: BvalidOvVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BvalidOvVal` reader - B-Peripheral Session Valid Override Value"]
pub type BvalidOvValR = crate::BitReader<BvalidOvVal>;
impl BvalidOvValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BvalidOvVal {
        match self.bits {
            false => BvalidOvVal::Value1,
            true => BvalidOvVal::Value2,
        }
    }
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BvalidOvVal::Value1
    }
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BvalidOvVal::Value2
    }
}
#[doc = "Field `BvalidOvVal` writer - B-Peripheral Session Valid Override Value"]
pub type BvalidOvValW<'a, REG> = crate::BitWriter<'a, REG, BvalidOvVal>;
impl<'a, REG> BvalidOvValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bvalid value is 0# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BvalidOvVal::Value1)
    }
    #[doc = "Bvalid value is 1# when GOTGCTL.BvalidOvEn = 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BvalidOvVal::Value2)
    }
}
#[doc = "Host Negotiation Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HstNegScs {
    #[doc = "0: Host negotiation failure"]
    Value1 = 0,
    #[doc = "1: Host negotiation success"]
    Value2 = 1,
}
impl From<HstNegScs> for bool {
    #[inline(always)]
    fn from(variant: HstNegScs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HstNegScs` reader - Host Negotiation Success"]
pub type HstNegScsR = crate::BitReader<HstNegScs>;
impl HstNegScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HstNegScs {
        match self.bits {
            false => HstNegScs::Value1,
            true => HstNegScs::Value2,
        }
    }
    #[doc = "Host negotiation failure"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HstNegScs::Value1
    }
    #[doc = "Host negotiation success"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HstNegScs::Value2
    }
}
#[doc = "HNP Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hnpreq {
    #[doc = "0: No HNP request"]
    Value1 = 0,
    #[doc = "1: HNP request"]
    Value2 = 1,
}
impl From<Hnpreq> for bool {
    #[inline(always)]
    fn from(variant: Hnpreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPReq` reader - HNP Request"]
pub type HnpreqR = crate::BitReader<Hnpreq>;
impl HnpreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hnpreq {
        match self.bits {
            false => Hnpreq::Value1,
            true => Hnpreq::Value2,
        }
    }
    #[doc = "No HNP request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hnpreq::Value1
    }
    #[doc = "HNP request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hnpreq::Value2
    }
}
#[doc = "Field `HNPReq` writer - HNP Request"]
pub type HnpreqW<'a, REG> = crate::BitWriter<'a, REG, Hnpreq>;
impl<'a, REG> HnpreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No HNP request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpreq::Value1)
    }
    #[doc = "HNP request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpreq::Value2)
    }
}
#[doc = "Host Set HNP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HstSetHnpen {
    #[doc = "0: Host Set HNP is not enabled"]
    Value1 = 0,
    #[doc = "1: Host Set HNP is enabled"]
    Value2 = 1,
}
impl From<HstSetHnpen> for bool {
    #[inline(always)]
    fn from(variant: HstSetHnpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HstSetHNPEn` reader - Host Set HNP Enable"]
pub type HstSetHnpenR = crate::BitReader<HstSetHnpen>;
impl HstSetHnpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HstSetHnpen {
        match self.bits {
            false => HstSetHnpen::Value1,
            true => HstSetHnpen::Value2,
        }
    }
    #[doc = "Host Set HNP is not enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HstSetHnpen::Value1
    }
    #[doc = "Host Set HNP is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HstSetHnpen::Value2
    }
}
#[doc = "Field `HstSetHNPEn` writer - Host Set HNP Enable"]
pub type HstSetHnpenW<'a, REG> = crate::BitWriter<'a, REG, HstSetHnpen>;
impl<'a, REG> HstSetHnpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host Set HNP is not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HstSetHnpen::Value1)
    }
    #[doc = "Host Set HNP is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HstSetHnpen::Value2)
    }
}
#[doc = "Device HNP Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevHnpen {
    #[doc = "0: HNP is not enabled in the application"]
    Value1 = 0,
    #[doc = "1: HNP is enabled in the application"]
    Value2 = 1,
}
impl From<DevHnpen> for bool {
    #[inline(always)]
    fn from(variant: DevHnpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DevHNPEn` reader - Device HNP Enabled"]
pub type DevHnpenR = crate::BitReader<DevHnpen>;
impl DevHnpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevHnpen {
        match self.bits {
            false => DevHnpen::Value1,
            true => DevHnpen::Value2,
        }
    }
    #[doc = "HNP is not enabled in the application"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DevHnpen::Value1
    }
    #[doc = "HNP is enabled in the application"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DevHnpen::Value2
    }
}
#[doc = "Field `DevHNPEn` writer - Device HNP Enabled"]
pub type DevHnpenW<'a, REG> = crate::BitWriter<'a, REG, DevHnpen>;
impl<'a, REG> DevHnpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HNP is not enabled in the application"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DevHnpen::Value1)
    }
    #[doc = "HNP is enabled in the application"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DevHnpen::Value2)
    }
}
#[doc = "Connector ID Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConlDsts {
    #[doc = "0: The USB core is in A-Device mode"]
    Value1 = 0,
    #[doc = "1: The USB core is in B-Device mode"]
    Value2 = 1,
}
impl From<ConlDsts> for bool {
    #[inline(always)]
    fn from(variant: ConlDsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ConlDSts` reader - Connector ID Status"]
pub type ConlDstsR = crate::BitReader<ConlDsts>;
impl ConlDstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConlDsts {
        match self.bits {
            false => ConlDsts::Value1,
            true => ConlDsts::Value2,
        }
    }
    #[doc = "The USB core is in A-Device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ConlDsts::Value1
    }
    #[doc = "The USB core is in B-Device mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ConlDsts::Value2
    }
}
#[doc = "Long/Short Debounce Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbncTime {
    #[doc = "0: Long debounce time, used for physical connections (100 ms + 2.5 us)"]
    Value1 = 0,
    #[doc = "1: Short debounce time, used for soft connections (2.5 us)"]
    Value2 = 1,
}
impl From<DbncTime> for bool {
    #[inline(always)]
    fn from(variant: DbncTime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DbncTime` reader - Long/Short Debounce Time"]
pub type DbncTimeR = crate::BitReader<DbncTime>;
impl DbncTimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbncTime {
        match self.bits {
            false => DbncTime::Value1,
            true => DbncTime::Value2,
        }
    }
    #[doc = "Long debounce time, used for physical connections (100 ms + 2.5 us)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DbncTime::Value1
    }
    #[doc = "Short debounce time, used for soft connections (2.5 us)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DbncTime::Value2
    }
}
#[doc = "A-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsesVid {
    #[doc = "0: A-session is not valid"]
    Value1 = 0,
    #[doc = "1: A-session is valid"]
    Value2 = 1,
}
impl From<AsesVid> for bool {
    #[inline(always)]
    fn from(variant: AsesVid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASesVId` reader - A-Session Valid"]
pub type AsesVidR = crate::BitReader<AsesVid>;
impl AsesVidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AsesVid {
        match self.bits {
            false => AsesVid::Value1,
            true => AsesVid::Value2,
        }
    }
    #[doc = "A-session is not valid"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AsesVid::Value1
    }
    #[doc = "A-session is valid"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AsesVid::Value2
    }
}
#[doc = "B-Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BsesVld {
    #[doc = "0: B-session is not valid."]
    Value1 = 0,
    #[doc = "1: B-session is valid."]
    Value2 = 1,
}
impl From<BsesVld> for bool {
    #[inline(always)]
    fn from(variant: BsesVld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSesVld` reader - B-Session Valid"]
pub type BsesVldR = crate::BitReader<BsesVld>;
impl BsesVldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BsesVld {
        match self.bits {
            false => BsesVld::Value1,
            true => BsesVld::Value2,
        }
    }
    #[doc = "B-session is not valid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BsesVld::Value1
    }
    #[doc = "B-session is valid."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BsesVld::Value2
    }
}
#[doc = "OTG Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Otgver {
    #[doc = "0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    Value1 = 0,
    #[doc = "1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    Value2 = 1,
}
impl From<Otgver> for bool {
    #[inline(always)]
    fn from(variant: Otgver) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGVer` reader - OTG Version"]
pub type OtgverR = crate::BitReader<Otgver>;
impl OtgverR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Otgver {
        match self.bits {
            false => Otgver::Value1,
            true => Otgver::Value2,
        }
    }
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Otgver::Value1
    }
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Otgver::Value2
    }
}
#[doc = "Field `OTGVer` writer - OTG Version"]
pub type OtgverW<'a, REG> = crate::BitWriter<'a, REG, Otgver>;
impl<'a, REG> OtgverW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Otgver::Value1)
    }
    #[doc = "OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Otgver::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Session Request Success"]
    #[inline(always)]
    pub fn ses_req_scs(&self) -> SesReqScsR {
        SesReqScsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn ses_req(&self) -> SesReqR {
        SesReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalid_ov_en(&self) -> VbvalidOvEnR {
        VbvalidOvEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    pub fn vbvalid_ov_val(&self) -> VbvalidOvValR {
        VbvalidOvValR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalid_ov_en(&self) -> AvalidOvEnR {
        AvalidOvEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn avalid_ov_val(&self) -> AvalidOvValR {
        AvalidOvValR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalid_ov_en(&self) -> BvalidOvEnR {
        BvalidOvEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    pub fn bvalid_ov_val(&self) -> BvalidOvValR {
        BvalidOvValR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline(always)]
    pub fn hst_neg_scs(&self) -> HstNegScsR {
        HstNegScsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HnpreqR {
        HnpreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hst_set_hnpen(&self) -> HstSetHnpenR {
        HstSetHnpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn dev_hnpen(&self) -> DevHnpenR {
        DevHnpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline(always)]
    pub fn conl_dsts(&self) -> ConlDstsR {
        ConlDstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline(always)]
    pub fn dbnc_time(&self) -> DbncTimeR {
        DbncTimeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn ases_vid(&self) -> AsesVidR {
        AsesVidR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline(always)]
    pub fn bses_vld(&self) -> BsesVldR {
        BsesVldR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OtgverR {
        OtgverR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    #[must_use]
    pub fn ses_req(&mut self) -> SesReqW<GotgctlSpec> {
        SesReqW::new(self, 1)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalid_ov_en(&mut self) -> VbvalidOvEnW<GotgctlSpec> {
        VbvalidOvEnW::new(self, 2)
    }
    #[doc = "Bit 3 - VBUS Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalid_ov_val(&mut self) -> VbvalidOvValW<GotgctlSpec> {
        VbvalidOvValW::new(self, 3)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avalid_ov_en(&mut self) -> AvalidOvEnW<GotgctlSpec> {
        AvalidOvEnW::new(self, 4)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn avalid_ov_val(&mut self) -> AvalidOvValW<GotgctlSpec> {
        AvalidOvValW::new(self, 5)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvalid_ov_en(&mut self) -> BvalidOvEnW<GotgctlSpec> {
        BvalidOvEnW::new(self, 6)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn bvalid_ov_val(&mut self) -> BvalidOvValW<GotgctlSpec> {
        BvalidOvValW::new(self, 7)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HnpreqW<GotgctlSpec> {
        HnpreqW::new(self, 9)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hst_set_hnpen(&mut self) -> HstSetHnpenW<GotgctlSpec> {
        HstSetHnpenW::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dev_hnpen(&mut self) -> DevHnpenW<GotgctlSpec> {
        DevHnpenW::new(self, 11)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OtgverW<GotgctlSpec> {
        OtgverW::new(self, 20)
    }
}
#[doc = "Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgctlSpec;
impl crate::RegisterSpec for GotgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GotgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GotgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GotgctlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
