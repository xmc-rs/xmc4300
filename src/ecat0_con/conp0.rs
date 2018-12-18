#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONP0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RXD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD0R {
    #[doc = "Data input RXD0A is selected"]
    VALUE1,
    #[doc = "Data input RXD0B is selected"]
    VALUE2,
    #[doc = "Data input RXD0C is selected"]
    VALUE3,
    #[doc = "Data input RXD0D is selected"]
    VALUE4,
}
impl RXD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD0R::VALUE1 => 0,
            RXD0R::VALUE2 => 1,
            RXD0R::VALUE3 => 2,
            RXD0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD0R {
        match value {
            0 => RXD0R::VALUE1,
            1 => RXD0R::VALUE2,
            2 => RXD0R::VALUE3,
            3 => RXD0R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD0R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD1R {
    #[doc = "Data input RXD1A is selected"]
    VALUE1,
    #[doc = "Data input RXD1B is selected"]
    VALUE2,
    #[doc = "Data input RXD1C is selected"]
    VALUE3,
    #[doc = "Data input RXD1D is selected"]
    VALUE4,
}
impl RXD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD1R::VALUE1 => 0,
            RXD1R::VALUE2 => 1,
            RXD1R::VALUE3 => 2,
            RXD1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD1R {
        match value {
            0 => RXD1R::VALUE1,
            1 => RXD1R::VALUE2,
            2 => RXD1R::VALUE3,
            3 => RXD1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD1R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD2R {
    #[doc = "Data input RXD2A is selected"]
    VALUE1,
    #[doc = "Data input RXD2B is selected"]
    VALUE2,
    #[doc = "Data input RXD2C is selected"]
    VALUE3,
    #[doc = "Data input RXD2D is selected"]
    VALUE4,
}
impl RXD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD2R::VALUE1 => 0,
            RXD2R::VALUE2 => 1,
            RXD2R::VALUE3 => 2,
            RXD2R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD2R {
        match value {
            0 => RXD2R::VALUE1,
            1 => RXD2R::VALUE2,
            2 => RXD2R::VALUE3,
            3 => RXD2R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD2R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD3R {
    #[doc = "Data input RXD3A is selected"]
    VALUE1,
    #[doc = "Data input RXD3B is selected"]
    VALUE2,
    #[doc = "Data input RXD3C is selected"]
    VALUE3,
    #[doc = "Data input RXD3D is selected"]
    VALUE4,
}
impl RXD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD3R::VALUE1 => 0,
            RXD3R::VALUE2 => 1,
            RXD3R::VALUE3 => 2,
            RXD3R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD3R {
        match value {
            0 => RXD3R::VALUE1,
            1 => RXD3R::VALUE2,
            2 => RXD3R::VALUE3,
            3 => RXD3R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD3R::VALUE4
    }
}
#[doc = "Possible values of the field `RX_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ERRR {
    #[doc = "Data input RX_ERRA is selected"]
    VALUE1,
    #[doc = "Data input RX_ERRB is selected"]
    VALUE2,
    #[doc = "Data input RX_ERRC is selected"]
    VALUE3,
    #[doc = "Data input RX_ERRD is selected"]
    VALUE4,
}
impl RX_ERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_ERRR::VALUE1 => 0,
            RX_ERRR::VALUE2 => 1,
            RX_ERRR::VALUE3 => 2,
            RX_ERRR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_ERRR {
        match value {
            0 => RX_ERRR::VALUE1,
            1 => RX_ERRR::VALUE2,
            2 => RX_ERRR::VALUE3,
            3 => RX_ERRR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RX_ERRR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RX_ERRR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RX_ERRR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RX_ERRR::VALUE4
    }
}
#[doc = "Possible values of the field `RX_DV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DVR {
    #[doc = "Data input RX_DVA is selected"]
    VALUE1,
    #[doc = "Data input RX_DVB is selected"]
    VALUE2,
    #[doc = "Data input RX_DVC is selected"]
    VALUE3,
    #[doc = "Data input RX_DVD is selected"]
    VALUE4,
}
impl RX_DVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_DVR::VALUE1 => 0,
            RX_DVR::VALUE2 => 1,
            RX_DVR::VALUE3 => 2,
            RX_DVR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_DVR {
        match value {
            0 => RX_DVR::VALUE1,
            1 => RX_DVR::VALUE2,
            2 => RX_DVR::VALUE3,
            3 => RX_DVR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RX_DVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RX_DVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RX_DVR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RX_DVR::VALUE4
    }
}
#[doc = "Possible values of the field `RX_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CLKR {
    #[doc = "Clock input RX_CLKA"]
    VALUE1,
    #[doc = "Clock input RX_CLKB"]
    VALUE2,
    #[doc = "Clock input RX_CLKC"]
    VALUE3,
    #[doc = "Clock input RX_CLKD"]
    VALUE4,
}
impl RX_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_CLKR::VALUE1 => 0,
            RX_CLKR::VALUE2 => 1,
            RX_CLKR::VALUE3 => 2,
            RX_CLKR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_CLKR {
        match value {
            0 => RX_CLKR::VALUE1,
            1 => RX_CLKR::VALUE2,
            2 => RX_CLKR::VALUE3,
            3 => RX_CLKR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RX_CLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RX_CLKR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RX_CLKR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RX_CLKR::VALUE4
    }
}
#[doc = "Possible values of the field `LINK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINKR {
    #[doc = "PHY LINKA"]
    VALUE1,
    #[doc = "PHY LINKB"]
    VALUE2,
    #[doc = "PHY LINKC"]
    VALUE3,
    #[doc = "PHY LINKD"]
    VALUE4,
}
impl LINKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LINKR::VALUE1 => 0,
            LINKR::VALUE2 => 1,
            LINKR::VALUE3 => 2,
            LINKR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LINKR {
        match value {
            0 => LINKR::VALUE1,
            1 => LINKR::VALUE2,
            2 => LINKR::VALUE3,
            3 => LINKR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LINKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LINKR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == LINKR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == LINKR::VALUE4
    }
}
#[doc = "Possible values of the field `TX_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CLKR {
    #[doc = "Clock input TX_CLKA"]
    VALUE1,
    #[doc = "Clock input TX_CLKB"]
    VALUE2,
    #[doc = "Clock input TX_CLKC"]
    VALUE3,
    #[doc = "Clock input TX_CLKD"]
    VALUE4,
}
impl TX_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_CLKR::VALUE1 => 0,
            TX_CLKR::VALUE2 => 1,
            TX_CLKR::VALUE3 => 2,
            TX_CLKR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_CLKR {
        match value {
            0 => TX_CLKR::VALUE1,
            1 => TX_CLKR::VALUE2,
            2 => TX_CLKR::VALUE3,
            3 => TX_CLKR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TX_CLKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TX_CLKR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TX_CLKR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TX_CLKR::VALUE4
    }
}
#[doc = "Possible values of the field `TX_SHIFT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_SHIFTR {
    #[doc = "0 ns"]
    VALUE1,
    #[doc = "10 ns"]
    VALUE2,
    #[doc = "20 ns"]
    VALUE3,
    #[doc = "30 ns"]
    VALUE4,
}
impl TX_SHIFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_SHIFTR::VALUE1 => 0,
            TX_SHIFTR::VALUE2 => 1,
            TX_SHIFTR::VALUE3 => 2,
            TX_SHIFTR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_SHIFTR {
        match value {
            0 => TX_SHIFTR::VALUE1,
            1 => TX_SHIFTR::VALUE2,
            2 => TX_SHIFTR::VALUE3,
            3 => TX_SHIFTR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TX_SHIFTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TX_SHIFTR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == TX_SHIFTR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == TX_SHIFTR::VALUE4
    }
}
#[doc = "Values that can be written to the field `RXD0`"]
pub enum RXD0W {
    #[doc = "Data input RXD0A is selected"]
    VALUE1,
    #[doc = "Data input RXD0B is selected"]
    VALUE2,
    #[doc = "Data input RXD0C is selected"]
    VALUE3,
    #[doc = "Data input RXD0D is selected"]
    VALUE4,
}
impl RXD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD0W::VALUE1 => 0,
            RXD0W::VALUE2 => 1,
            RXD0W::VALUE3 => 2,
            RXD0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD0W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD0A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD0W::VALUE1)
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD0W::VALUE2)
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD0W::VALUE3)
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD0W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD1`"]
pub enum RXD1W {
    #[doc = "Data input RXD1A is selected"]
    VALUE1,
    #[doc = "Data input RXD1B is selected"]
    VALUE2,
    #[doc = "Data input RXD1C is selected"]
    VALUE3,
    #[doc = "Data input RXD1D is selected"]
    VALUE4,
}
impl RXD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD1W::VALUE1 => 0,
            RXD1W::VALUE2 => 1,
            RXD1W::VALUE3 => 2,
            RXD1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD1A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD1W::VALUE1)
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD1W::VALUE2)
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD1W::VALUE3)
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD2`"]
pub enum RXD2W {
    #[doc = "Data input RXD2A is selected"]
    VALUE1,
    #[doc = "Data input RXD2B is selected"]
    VALUE2,
    #[doc = "Data input RXD2C is selected"]
    VALUE3,
    #[doc = "Data input RXD2D is selected"]
    VALUE4,
}
impl RXD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD2W::VALUE1 => 0,
            RXD2W::VALUE2 => 1,
            RXD2W::VALUE3 => 2,
            RXD2W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD2W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD2A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD2W::VALUE1)
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD2W::VALUE2)
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD2W::VALUE3)
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD2W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD3`"]
pub enum RXD3W {
    #[doc = "Data input RXD3A is selected"]
    VALUE1,
    #[doc = "Data input RXD3B is selected"]
    VALUE2,
    #[doc = "Data input RXD3C is selected"]
    VALUE3,
    #[doc = "Data input RXD3D is selected"]
    VALUE4,
}
impl RXD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD3W::VALUE1 => 0,
            RXD3W::VALUE2 => 1,
            RXD3W::VALUE3 => 2,
            RXD3W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD3W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD3A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD3W::VALUE1)
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD3W::VALUE2)
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD3W::VALUE3)
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD3W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_ERR`"]
pub enum RX_ERRW {
    #[doc = "Data input RX_ERRA is selected"]
    VALUE1,
    #[doc = "Data input RX_ERRB is selected"]
    VALUE2,
    #[doc = "Data input RX_ERRC is selected"]
    VALUE3,
    #[doc = "Data input RX_ERRD is selected"]
    VALUE4,
}
impl RX_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_ERRW::VALUE1 => 0,
            RX_ERRW::VALUE2 => 1,
            RX_ERRW::VALUE3 => 2,
            RX_ERRW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ERRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RX_ERRA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_ERRW::VALUE1)
    }
    #[doc = "Data input RX_ERRB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_ERRW::VALUE2)
    }
    #[doc = "Data input RX_ERRC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_ERRW::VALUE3)
    }
    #[doc = "Data input RX_ERRD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_ERRW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_DV`"]
pub enum RX_DVW {
    #[doc = "Data input RX_DVA is selected"]
    VALUE1,
    #[doc = "Data input RX_DVB is selected"]
    VALUE2,
    #[doc = "Data input RX_DVC is selected"]
    VALUE3,
    #[doc = "Data input RX_DVD is selected"]
    VALUE4,
}
impl RX_DVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_DVW::VALUE1 => 0,
            RX_DVW::VALUE2 => 1,
            RX_DVW::VALUE3 => 2,
            RX_DVW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DVW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RX_DVA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_DVW::VALUE1)
    }
    #[doc = "Data input RX_DVB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_DVW::VALUE2)
    }
    #[doc = "Data input RX_DVC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_DVW::VALUE3)
    }
    #[doc = "Data input RX_DVD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_DVW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_CLK`"]
pub enum RX_CLKW {
    #[doc = "Clock input RX_CLKA"]
    VALUE1,
    #[doc = "Clock input RX_CLKB"]
    VALUE2,
    #[doc = "Clock input RX_CLKC"]
    VALUE3,
    #[doc = "Clock input RX_CLKD"]
    VALUE4,
}
impl RX_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_CLKW::VALUE1 => 0,
            RX_CLKW::VALUE2 => 1,
            RX_CLKW::VALUE3 => 2,
            RX_CLKW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_CLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock input RX_CLKA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RX_CLKW::VALUE1)
    }
    #[doc = "Clock input RX_CLKB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RX_CLKW::VALUE2)
    }
    #[doc = "Clock input RX_CLKC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RX_CLKW::VALUE3)
    }
    #[doc = "Clock input RX_CLKD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RX_CLKW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LINK`"]
pub enum LINKW {
    #[doc = "PHY LINKA"]
    VALUE1,
    #[doc = "PHY LINKB"]
    VALUE2,
    #[doc = "PHY LINKC"]
    VALUE3,
    #[doc = "PHY LINKD"]
    VALUE4,
}
impl LINKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LINKW::VALUE1 => 0,
            LINKW::VALUE2 => 1,
            LINKW::VALUE3 => 2,
            LINKW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINKW<'a> {
    w: &'a mut W,
}
impl<'a> _LINKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PHY LINKA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LINKW::VALUE1)
    }
    #[doc = "PHY LINKB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LINKW::VALUE2)
    }
    #[doc = "PHY LINKC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(LINKW::VALUE3)
    }
    #[doc = "PHY LINKD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(LINKW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_CLK`"]
pub enum TX_CLKW {
    #[doc = "Clock input TX_CLKA"]
    VALUE1,
    #[doc = "Clock input TX_CLKB"]
    VALUE2,
    #[doc = "Clock input TX_CLKC"]
    VALUE3,
    #[doc = "Clock input TX_CLKD"]
    VALUE4,
}
impl TX_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_CLKW::VALUE1 => 0,
            TX_CLKW::VALUE2 => 1,
            TX_CLKW::VALUE3 => 2,
            TX_CLKW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_CLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock input TX_CLKA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_CLKW::VALUE1)
    }
    #[doc = "Clock input TX_CLKB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_CLKW::VALUE2)
    }
    #[doc = "Clock input TX_CLKC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TX_CLKW::VALUE3)
    }
    #[doc = "Clock input TX_CLKD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TX_CLKW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_SHIFT`"]
pub enum TX_SHIFTW {
    #[doc = "0 ns"]
    VALUE1,
    #[doc = "10 ns"]
    VALUE2,
    #[doc = "20 ns"]
    VALUE3,
    #[doc = "30 ns"]
    VALUE4,
}
impl TX_SHIFTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_SHIFTW::VALUE1 => 0,
            TX_SHIFTW::VALUE2 => 1,
            TX_SHIFTW::VALUE3 => 2,
            TX_SHIFTW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_SHIFTW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_SHIFTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_SHIFTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 ns"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_SHIFTW::VALUE1)
    }
    #[doc = "10 ns"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_SHIFTW::VALUE2)
    }
    #[doc = "20 ns"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(TX_SHIFTW::VALUE3)
    }
    #[doc = "30 ns"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(TX_SHIFTW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline]
    pub fn rxd0(&self) -> RXD0R {
        RXD0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline]
    pub fn rxd1(&self) -> RXD1R {
        RXD1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline]
    pub fn rxd2(&self) -> RXD2R {
        RXD2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline]
    pub fn rxd3(&self) -> RXD3R {
        RXD3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline]
    pub fn rx_err(&self) -> RX_ERRR {
        RX_ERRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline]
    pub fn rx_dv(&self) -> RX_DVR {
        RX_DVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline]
    pub fn rx_clk(&self) -> RX_CLKR {
        RX_CLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline]
    pub fn link(&self) -> LINKR {
        LINKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline]
    pub fn tx_clk(&self) -> TX_CLKR {
        TX_CLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline]
    pub fn tx_shift(&self) -> TX_SHIFTR {
        TX_SHIFTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - PORT0 Receive Input 0 Select"]
    #[inline]
    pub fn rxd0(&mut self) -> _RXD0W {
        _RXD0W { w: self }
    }
    #[doc = "Bits 2:3 - Port0 Receive Input 1 Select"]
    #[inline]
    pub fn rxd1(&mut self) -> _RXD1W {
        _RXD1W { w: self }
    }
    #[doc = "Bits 4:5 - Port0 Receive Input 2 Select"]
    #[inline]
    pub fn rxd2(&mut self) -> _RXD2W {
        _RXD2W { w: self }
    }
    #[doc = "Bits 6:7 - Port0 Receive Input 3 Select"]
    #[inline]
    pub fn rxd3(&mut self) -> _RXD3W {
        _RXD3W { w: self }
    }
    #[doc = "Bits 8:9 - Port0 MII RX ERROR Input Select"]
    #[inline]
    pub fn rx_err(&mut self) -> _RX_ERRW {
        _RX_ERRW { w: self }
    }
    #[doc = "Bits 10:11 - Port0 MII RX DV Input Select"]
    #[inline]
    pub fn rx_dv(&mut self) -> _RX_DVW {
        _RX_DVW { w: self }
    }
    #[doc = "Bits 12:13 - Port0 MII RX Clock Input Select"]
    #[inline]
    pub fn rx_clk(&mut self) -> _RX_CLKW {
        _RX_CLKW { w: self }
    }
    #[doc = "Bits 16:17 - Port0 PHY Link Input Select"]
    #[inline]
    pub fn link(&mut self) -> _LINKW {
        _LINKW { w: self }
    }
    #[doc = "Bits 28:29 - Port0 MII TX Clock Input Select"]
    #[inline]
    pub fn tx_clk(&mut self) -> _TX_CLKW {
        _TX_CLKW { w: self }
    }
    #[doc = "Bits 30:31 - Port0 Manual TX Shift configuration"]
    #[inline]
    pub fn tx_shift(&mut self) -> _TX_SHIFTW {
        _TX_SHIFTW { w: self }
    }
}
