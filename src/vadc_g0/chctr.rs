#[doc = "Register `CHCTR[%s]` reader"]
pub type R = crate::R<ChctrSpec>;
#[doc = "Register `CHCTR[%s]` writer"]
pub type W = crate::W<ChctrSpec>;
#[doc = "Input Class Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iclsel {
    #[doc = "0: Use group-specific class 0"]
    Value1 = 0,
    #[doc = "1: Use group-specific class 1"]
    Value2 = 1,
    #[doc = "2: Use global class 0"]
    Value3 = 2,
    #[doc = "3: Use global class 1"]
    Value4 = 3,
}
impl From<Iclsel> for u8 {
    #[inline(always)]
    fn from(variant: Iclsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iclsel {
    type Ux = u8;
}
impl crate::IsEnum for Iclsel {}
#[doc = "Field `ICLSEL` reader - Input Class Select"]
pub type IclselR = crate::FieldReader<Iclsel>;
impl IclselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iclsel {
        match self.bits {
            0 => Iclsel::Value1,
            1 => Iclsel::Value2,
            2 => Iclsel::Value3,
            3 => Iclsel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iclsel::Value1
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iclsel::Value2
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Iclsel::Value3
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Iclsel::Value4
    }
}
#[doc = "Field `ICLSEL` writer - Input Class Select"]
pub type IclselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iclsel, crate::Safe>;
impl<'a, REG> IclselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific class 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Iclsel::Value1)
    }
    #[doc = "Use group-specific class 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Iclsel::Value2)
    }
    #[doc = "Use global class 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Iclsel::Value3)
    }
    #[doc = "Use global class 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Iclsel::Value4)
    }
}
#[doc = "Lower Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bndsell {
    #[doc = "0: Use group-specific boundary 0"]
    Value1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    Value2 = 1,
    #[doc = "2: Use global boundary 0"]
    Value3 = 2,
    #[doc = "3: Use global boundary 1"]
    Value4 = 3,
}
impl From<Bndsell> for u8 {
    #[inline(always)]
    fn from(variant: Bndsell) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bndsell {
    type Ux = u8;
}
impl crate::IsEnum for Bndsell {}
#[doc = "Field `BNDSELL` reader - Lower Boundary Select"]
pub type BndsellR = crate::FieldReader<Bndsell>;
impl BndsellR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bndsell {
        match self.bits {
            0 => Bndsell::Value1,
            1 => Bndsell::Value2,
            2 => Bndsell::Value3,
            3 => Bndsell::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bndsell::Value1
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bndsell::Value2
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bndsell::Value3
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bndsell::Value4
    }
}
#[doc = "Field `BNDSELL` writer - Lower Boundary Select"]
pub type BndsellW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bndsell, crate::Safe>;
impl<'a, REG> BndsellW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bndsell::Value1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bndsell::Value2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bndsell::Value3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bndsell::Value4)
    }
}
#[doc = "Upper Boundary Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bndselu {
    #[doc = "0: Use group-specific boundary 0"]
    Value1 = 0,
    #[doc = "1: Use group-specific boundary 1"]
    Value2 = 1,
    #[doc = "2: Use global boundary 0"]
    Value3 = 2,
    #[doc = "3: Use global boundary 1"]
    Value4 = 3,
}
impl From<Bndselu> for u8 {
    #[inline(always)]
    fn from(variant: Bndselu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bndselu {
    type Ux = u8;
}
impl crate::IsEnum for Bndselu {}
#[doc = "Field `BNDSELU` reader - Upper Boundary Select"]
pub type BndseluR = crate::FieldReader<Bndselu>;
impl BndseluR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bndselu {
        match self.bits {
            0 => Bndselu::Value1,
            1 => Bndselu::Value2,
            2 => Bndselu::Value3,
            3 => Bndselu::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bndselu::Value1
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bndselu::Value2
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bndselu::Value3
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bndselu::Value4
    }
}
#[doc = "Field `BNDSELU` writer - Upper Boundary Select"]
pub type BndseluW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bndselu, crate::Safe>;
impl<'a, REG> BndseluW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use group-specific boundary 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bndselu::Value1)
    }
    #[doc = "Use group-specific boundary 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bndselu::Value2)
    }
    #[doc = "Use global boundary 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bndselu::Value3)
    }
    #[doc = "Use global boundary 1"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bndselu::Value4)
    }
}
#[doc = "Channel Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chevmode {
    #[doc = "0: Never"]
    Value1 = 0,
    #[doc = "1: NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    Value2 = 1,
    #[doc = "2: NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    Value3 = 2,
    #[doc = "3: NCM: Always (ignore band) FCM: If result switches to either level"]
    Value4 = 3,
}
impl From<Chevmode> for u8 {
    #[inline(always)]
    fn from(variant: Chevmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chevmode {
    type Ux = u8;
}
impl crate::IsEnum for Chevmode {}
#[doc = "Field `CHEVMODE` reader - Channel Event Mode"]
pub type ChevmodeR = crate::FieldReader<Chevmode>;
impl ChevmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chevmode {
        match self.bits {
            0 => Chevmode::Value1,
            1 => Chevmode::Value2,
            2 => Chevmode::Value3,
            3 => Chevmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Never"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Chevmode::Value1
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Chevmode::Value2
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Chevmode::Value3
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Chevmode::Value4
    }
}
#[doc = "Field `CHEVMODE` writer - Channel Event Mode"]
pub type ChevmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chevmode, crate::Safe>;
impl<'a, REG> ChevmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Chevmode::Value1)
    }
    #[doc = "NCM: If result is inside the boundary band FCM: If result becomes high (above cmp. val.)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Chevmode::Value2)
    }
    #[doc = "NCM: If result is outside the boundary band FCM: If result becomes low (below cmp. val.)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Chevmode::Value3)
    }
    #[doc = "NCM: Always (ignore band) FCM: If result switches to either level"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Chevmode::Value4)
    }
}
#[doc = "Synchronization Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: No synchroniz. request, standalone operation"]
    Value1 = 0,
    #[doc = "1: Request a synchronized conversion of this channel (only taken into account for a master)"]
    Value2 = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Synchronization Request"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Value1,
            true => Sync::Value2,
        }
    }
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync::Value1
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync::Value2
    }
}
#[doc = "Field `SYNC` writer - Synchronization Request"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchroniz. request, standalone operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Value1)
    }
    #[doc = "Request a synchronized conversion of this channel (only taken into account for a master)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Value2)
    }
}
#[doc = "Reference Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refsel {
    #[doc = "0: Standard reference input VAREF"]
    Value1 = 0,
    #[doc = "1: Alternate reference input from CH0"]
    Value2 = 1,
}
impl From<Refsel> for bool {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSEL` reader - Reference Input Selection"]
pub type RefselR = crate::BitReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            false => Refsel::Value1,
            true => Refsel::Value2,
        }
    }
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Refsel::Value1
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Refsel::Value2
    }
}
#[doc = "Field `REFSEL` writer - Reference Input Selection"]
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard reference input VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Value1)
    }
    #[doc = "Alternate reference input from CH0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Value2)
    }
}
#[doc = "Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resreg {
    #[doc = "0: Store result in group result register GxRES0"]
    Value1 = 0,
    #[doc = "15: Store result in group result register GxRES15"]
    Value2 = 15,
}
impl From<Resreg> for u8 {
    #[inline(always)]
    fn from(variant: Resreg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resreg {
    type Ux = u8;
}
impl crate::IsEnum for Resreg {}
#[doc = "Field `RESREG` reader - Result Register"]
pub type ResregR = crate::FieldReader<Resreg>;
impl ResregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resreg> {
        match self.bits {
            0 => Some(Resreg::Value1),
            15 => Some(Resreg::Value2),
            _ => None,
        }
    }
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Resreg::Value1
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Resreg::Value2
    }
}
#[doc = "Field `RESREG` writer - Result Register"]
pub type ResregW<'a, REG> = crate::FieldWriter<'a, REG, 4, Resreg>;
impl<'a, REG> ResregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Store result in group result register GxRES0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Resreg::Value1)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Resreg::Value2)
    }
}
#[doc = "Result Target for Background Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Restbs {
    #[doc = "0: Store results in the selected group result register"]
    Value1 = 0,
    #[doc = "1: Store results in the global result register"]
    Value2 = 1,
}
impl From<Restbs> for bool {
    #[inline(always)]
    fn from(variant: Restbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTBS` reader - Result Target for Background Source"]
pub type RestbsR = crate::BitReader<Restbs>;
impl RestbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Restbs {
        match self.bits {
            false => Restbs::Value1,
            true => Restbs::Value2,
        }
    }
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Restbs::Value1
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Restbs::Value2
    }
}
#[doc = "Field `RESTBS` writer - Result Target for Background Source"]
pub type RestbsW<'a, REG> = crate::BitWriter<'a, REG, Restbs>;
impl<'a, REG> RestbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Store results in the selected group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Restbs::Value1)
    }
    #[doc = "Store results in the global result register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Restbs::Value2)
    }
}
#[doc = "Result Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Respos {
    #[doc = "0: Store results left-aligned"]
    Value1 = 0,
    #[doc = "1: Store results right-aligned"]
    Value2 = 1,
}
impl From<Respos> for bool {
    #[inline(always)]
    fn from(variant: Respos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESPOS` reader - Result Position"]
pub type ResposR = crate::BitReader<Respos>;
impl ResposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Respos {
        match self.bits {
            false => Respos::Value1,
            true => Respos::Value2,
        }
    }
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Respos::Value1
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Respos::Value2
    }
}
#[doc = "Field `RESPOS` writer - Result Position"]
pub type ResposW<'a, REG> = crate::BitWriter<'a, REG, Respos>;
impl<'a, REG> ResposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Store results left-aligned"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Respos::Value1)
    }
    #[doc = "Store results right-aligned"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Respos::Value2)
    }
}
#[doc = "Broken Wire Detection Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bwdch {
    #[doc = "0: Select VAGND"]
    Value1 = 0,
    #[doc = "1: Select VAREF"]
    Value2 = 1,
}
impl From<Bwdch> for u8 {
    #[inline(always)]
    fn from(variant: Bwdch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bwdch {
    type Ux = u8;
}
impl crate::IsEnum for Bwdch {}
#[doc = "Field `BWDCH` reader - Broken Wire Detection Channel"]
pub type BwdchR = crate::FieldReader<Bwdch>;
impl BwdchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bwdch> {
        match self.bits {
            0 => Some(Bwdch::Value1),
            1 => Some(Bwdch::Value2),
            _ => None,
        }
    }
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bwdch::Value1
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bwdch::Value2
    }
}
#[doc = "Field `BWDCH` writer - Broken Wire Detection Channel"]
pub type BwdchW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bwdch>;
impl<'a, REG> BwdchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select VAGND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwdch::Value1)
    }
    #[doc = "Select VAREF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bwdch::Value2)
    }
}
#[doc = "Broken Wire Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwden {
    #[doc = "0: Normal operation"]
    Value1 = 0,
    #[doc = "1: Additional preparation phase is enabled"]
    Value2 = 1,
}
impl From<Bwden> for bool {
    #[inline(always)]
    fn from(variant: Bwden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWDEN` reader - Broken Wire Detection Enable"]
pub type BwdenR = crate::BitReader<Bwden>;
impl BwdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwden {
        match self.bits {
            false => Bwden::Value1,
            true => Bwden::Value2,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bwden::Value1
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bwden::Value2
    }
}
#[doc = "Field `BWDEN` writer - Broken Wire Detection Enable"]
pub type BwdenW<'a, REG> = crate::BitWriter<'a, REG, Bwden>;
impl<'a, REG> BwdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwden::Value1)
    }
    #[doc = "Additional preparation phase is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bwden::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    pub fn iclsel(&self) -> IclselR {
        IclselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    pub fn bndsell(&self) -> BndsellR {
        BndsellR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    pub fn bndselu(&self) -> BndseluR {
        BndseluR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    pub fn chevmode(&self) -> ChevmodeR {
        ChevmodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    pub fn resreg(&self) -> ResregR {
        ResregR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    pub fn restbs(&self) -> RestbsR {
        RestbsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    pub fn respos(&self) -> ResposR {
        ResposR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    pub fn bwdch(&self) -> BwdchR {
        BwdchR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    pub fn bwden(&self) -> BwdenR {
        BwdenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Class Select"]
    #[inline(always)]
    #[must_use]
    pub fn iclsel(&mut self) -> IclselW<ChctrSpec> {
        IclselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Lower Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndsell(&mut self) -> BndsellW<ChctrSpec> {
        BndsellW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Upper Boundary Select"]
    #[inline(always)]
    #[must_use]
    pub fn bndselu(&mut self) -> BndseluW<ChctrSpec> {
        BndseluW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Channel Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chevmode(&mut self) -> ChevmodeW<ChctrSpec> {
        ChevmodeW::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronization Request"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<ChctrSpec> {
        SyncW::new(self, 10)
    }
    #[doc = "Bit 11 - Reference Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<ChctrSpec> {
        RefselW::new(self, 11)
    }
    #[doc = "Bits 16:19 - Result Register"]
    #[inline(always)]
    #[must_use]
    pub fn resreg(&mut self) -> ResregW<ChctrSpec> {
        ResregW::new(self, 16)
    }
    #[doc = "Bit 20 - Result Target for Background Source"]
    #[inline(always)]
    #[must_use]
    pub fn restbs(&mut self) -> RestbsW<ChctrSpec> {
        RestbsW::new(self, 20)
    }
    #[doc = "Bit 21 - Result Position"]
    #[inline(always)]
    #[must_use]
    pub fn respos(&mut self) -> ResposW<ChctrSpec> {
        ResposW::new(self, 21)
    }
    #[doc = "Bits 28:29 - Broken Wire Detection Channel"]
    #[inline(always)]
    #[must_use]
    pub fn bwdch(&mut self) -> BwdchW<ChctrSpec> {
        BwdchW::new(self, 28)
    }
    #[doc = "Bit 30 - Broken Wire Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwden(&mut self) -> BwdenW<ChctrSpec> {
        BwdenW::new(self, 30)
    }
}
#[doc = "Channel Ctrl. Reg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrSpec;
impl crate::RegisterSpec for ChctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctr::R`](R) reader structure"]
impl crate::Readable for ChctrSpec {}
#[doc = "`write(|w| ..)` method takes [`chctr::W`](W) writer structure"]
impl crate::Writable for ChctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTR[%s]
to value 0"]
impl crate::Resettable for ChctrSpec {
    const RESET_VALUE: u32 = 0;
}
