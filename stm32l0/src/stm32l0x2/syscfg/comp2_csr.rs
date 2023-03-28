#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable bit"]
pub type COMP2EN_R = crate::BitReader<COMP2EN_A>;
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator 2 switched OFF"]
    Disabled = 0,
    #[doc = "1: Comparator 2 switched ON"]
    Enabled = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::Disabled,
            true => COMP2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN_A::Enabled
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable bit"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2EN_A, O>;
impl<'a, const O: u8> COMP2EN_W<'a, O> {
    #[doc = "Comparator 2 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Disabled)
    }
    #[doc = "Comparator 2 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Enabled)
    }
}
#[doc = "Field `COMP2SPEED` reader - Comparator 2 power mode selection bit"]
pub type COMP2SPEED_R = crate::BitReader<COMP2SPEED_A>;
#[doc = "Comparator 2 power mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2SPEED_A {
    #[doc = "0: Slow speed"]
    Slow = 0,
    #[doc = "1: Fast speed"]
    Fast = 1,
}
impl From<COMP2SPEED_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2SPEED_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2SPEED_A {
        match self.bits {
            false => COMP2SPEED_A::Slow,
            true => COMP2SPEED_A::Fast,
        }
    }
    #[doc = "Checks if the value of the field is `Slow`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == COMP2SPEED_A::Slow
    }
    #[doc = "Checks if the value of the field is `Fast`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == COMP2SPEED_A::Fast
    }
}
#[doc = "Field `COMP2SPEED` writer - Comparator 2 power mode selection bit"]
pub type COMP2SPEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2SPEED_A, O>;
impl<'a, const O: u8> COMP2SPEED_W<'a, O> {
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(COMP2SPEED_A::Slow)
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(COMP2SPEED_A::Fast)
    }
}
#[doc = "Field `COMP2INNSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2INNSEL_R = crate::FieldReader<u8, COMP2INNSEL_A>;
#[doc = "Comparator 2 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INNSEL_A {
    #[doc = "0: VREFINT"]
    Vrefint = 0,
    #[doc = "1: PA2"]
    Pa2 = 1,
    #[doc = "2: PA4"]
    Pa4 = 2,
    #[doc = "3: PA5"]
    Pa5 = 3,
    #[doc = "4: 1/4 VREFINT"]
    VrefintDiv4 = 4,
    #[doc = "5: 1/2 VREFINT"]
    VrefintDiv2 = 5,
    #[doc = "6: 3/4 VREFINT"]
    VrefintDiv34 = 6,
    #[doc = "7: PB3"]
    Pb3 = 7,
}
impl From<COMP2INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INNSEL_A) -> Self {
        variant as _
    }
}
impl COMP2INNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2INNSEL_A {
        match self.bits {
            0 => COMP2INNSEL_A::Vrefint,
            1 => COMP2INNSEL_A::Pa2,
            2 => COMP2INNSEL_A::Pa4,
            3 => COMP2INNSEL_A::Pa5,
            4 => COMP2INNSEL_A::VrefintDiv4,
            5 => COMP2INNSEL_A::VrefintDiv2,
            6 => COMP2INNSEL_A::VrefintDiv34,
            7 => COMP2INNSEL_A::Pb3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Vrefint`"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        *self == COMP2INNSEL_A::Vrefint
    }
    #[doc = "Checks if the value of the field is `Pa2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INNSEL_A::Pa2
    }
    #[doc = "Checks if the value of the field is `Pa4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == COMP2INNSEL_A::Pa4
    }
    #[doc = "Checks if the value of the field is `Pa5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == COMP2INNSEL_A::Pa5
    }
    #[doc = "Checks if the value of the field is `VrefintDiv4`"]
    #[inline(always)]
    pub fn is_vrefint_div4(&self) -> bool {
        *self == COMP2INNSEL_A::VrefintDiv4
    }
    #[doc = "Checks if the value of the field is `VrefintDiv2`"]
    #[inline(always)]
    pub fn is_vrefint_div2(&self) -> bool {
        *self == COMP2INNSEL_A::VrefintDiv2
    }
    #[doc = "Checks if the value of the field is `VrefintDiv34`"]
    #[inline(always)]
    pub fn is_vrefint_div3_4(&self) -> bool {
        *self == COMP2INNSEL_A::VrefintDiv34
    }
    #[doc = "Checks if the value of the field is `Pb3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == COMP2INNSEL_A::Pb3
    }
}
#[doc = "Field `COMP2INNSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2INNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP2_CSR_SPEC, u8, COMP2INNSEL_A, 3, O>;
impl<'a, const O: u8> COMP2INNSEL_W<'a, O> {
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::Vrefint)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::Pa2)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::Pa5)
    }
    #[doc = "1/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VrefintDiv4)
    }
    #[doc = "1/2 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div2(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VrefintDiv2)
    }
    #[doc = "3/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div3_4(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::VrefintDiv34)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(COMP2INNSEL_A::Pb3)
    }
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2INPSEL_R = crate::FieldReader<u8, COMP2INPSEL_A>;
#[doc = "Comparator 2 Input Plus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INPSEL_A {
    #[doc = "0: PA3"]
    Pa3 = 0,
    #[doc = "1: PB4"]
    Pb4 = 1,
    #[doc = "2: PB5"]
    Pb5 = 2,
    #[doc = "3: PB6"]
    Pb6 = 3,
    #[doc = "4: PB7"]
    Pb7 = 4,
    #[doc = "5: PA7"]
    Pa7 = 5,
}
impl From<COMP2INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INPSEL_A) -> Self {
        variant as _
    }
}
impl COMP2INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INPSEL_A> {
        match self.bits {
            0 => Some(COMP2INPSEL_A::Pa3),
            1 => Some(COMP2INPSEL_A::Pb4),
            2 => Some(COMP2INPSEL_A::Pb5),
            3 => Some(COMP2INPSEL_A::Pb6),
            4 => Some(COMP2INPSEL_A::Pb7),
            5 => Some(COMP2INPSEL_A::Pa7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == COMP2INPSEL_A::Pa3
    }
    #[doc = "Checks if the value of the field is `Pb4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == COMP2INPSEL_A::Pb4
    }
    #[doc = "Checks if the value of the field is `Pb5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == COMP2INPSEL_A::Pb5
    }
    #[doc = "Checks if the value of the field is `Pb6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == COMP2INPSEL_A::Pb6
    }
    #[doc = "Checks if the value of the field is `Pb7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == COMP2INPSEL_A::Pb7
    }
    #[doc = "Checks if the value of the field is `Pa7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == COMP2INPSEL_A::Pa7
    }
}
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2INPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, COMP2INPSEL_A, 3, O>;
impl<'a, const O: u8> COMP2INPSEL_W<'a, O> {
    #[doc = "PA3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pa3)
    }
    #[doc = "PB4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pb4)
    }
    #[doc = "PB5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pb5)
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pb6)
    }
    #[doc = "PB7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pb7)
    }
    #[doc = "PA7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(COMP2INPSEL_A::Pa7)
    }
}
#[doc = "Field `COMP2LPTIMIN2` reader - Comparator 2 LPTIM input 2 propagation bit"]
pub type COMP2LPTIMIN2_R = crate::BitReader<COMP2LPTIMIN2_A>;
#[doc = "Comparator 2 LPTIM input 2 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LPTIMIN2_A {
    #[doc = "0: Comparator 2 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 2"]
    NotGated = 1,
}
impl From<COMP2LPTIMIN2_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2LPTIMIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LPTIMIN2_A {
        match self.bits {
            false => COMP2LPTIMIN2_A::Gated,
            true => COMP2LPTIMIN2_A::NotGated,
        }
    }
    #[doc = "Checks if the value of the field is `Gated`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP2LPTIMIN2_A::Gated
    }
    #[doc = "Checks if the value of the field is `NotGated`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP2LPTIMIN2_A::NotGated
    }
}
#[doc = "Field `COMP2LPTIMIN2` writer - Comparator 2 LPTIM input 2 propagation bit"]
pub type COMP2LPTIMIN2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2LPTIMIN2_A, O>;
impl<'a, const O: u8> COMP2LPTIMIN2_W<'a, O> {
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN2_A::Gated)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 2"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN2_A::NotGated)
    }
}
#[doc = "Field `COMP2LPTIMIN1` reader - Comparator 2 LPTIM input 1 propagation bit"]
pub type COMP2LPTIMIN1_R = crate::BitReader<COMP2LPTIMIN1_A>;
#[doc = "Comparator 2 LPTIM input 1 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LPTIMIN1_A {
    #[doc = "0: Comparator 2 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 1"]
    NotGated = 1,
}
impl From<COMP2LPTIMIN1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2LPTIMIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LPTIMIN1_A {
        match self.bits {
            false => COMP2LPTIMIN1_A::Gated,
            true => COMP2LPTIMIN1_A::NotGated,
        }
    }
    #[doc = "Checks if the value of the field is `Gated`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP2LPTIMIN1_A::Gated
    }
    #[doc = "Checks if the value of the field is `NotGated`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP2LPTIMIN1_A::NotGated
    }
}
#[doc = "Field `COMP2LPTIMIN1` writer - Comparator 2 LPTIM input 1 propagation bit"]
pub type COMP2LPTIMIN1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2LPTIMIN1_A, O>;
impl<'a, const O: u8> COMP2LPTIMIN1_W<'a, O> {
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN1_A::Gated)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP2LPTIMIN1_A::NotGated)
    }
}
#[doc = "Field `COMP2POLARITY` reader - Comparator 2 polarity selection bit"]
pub type COMP2POLARITY_R = crate::BitReader<COMP2POLARITY_A>;
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POLARITY_A {
    #[doc = "0: Comparator 2 output value not inverted"]
    NotInverted = 0,
    #[doc = "1: Comparator 2 output value inverted"]
    Inverted = 1,
}
impl From<COMP2POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POLARITY_A {
        match self.bits {
            false => COMP2POLARITY_A::NotInverted,
            true => COMP2POLARITY_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POLARITY_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POLARITY_A::Inverted
    }
}
#[doc = "Field `COMP2POLARITY` writer - Comparator 2 polarity selection bit"]
pub type COMP2POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2POLARITY_A, O>;
impl<'a, const O: u8> COMP2POLARITY_W<'a, O> {
    #[doc = "Comparator 2 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POLARITY_A::NotInverted)
    }
    #[doc = "Comparator 2 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POLARITY_A::Inverted)
    }
}
#[doc = "Field `COMP2VALUE` reader - Comparator 2 output status bit"]
pub type COMP2VALUE_R = crate::BitReader<COMP2VALUER_A>;
#[doc = "Comparator 2 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2VALUER_A {
    #[doc = "0: Comparator values are not equal"]
    NotEqual = 0,
    #[doc = "1: Comparator values are equal"]
    Equal = 1,
}
impl From<COMP2VALUER_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2VALUER_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2VALUER_A {
        match self.bits {
            false => COMP2VALUER_A::NotEqual,
            true => COMP2VALUER_A::Equal,
        }
    }
    #[doc = "Checks if the value of the field is `NotEqual`"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        *self == COMP2VALUER_A::NotEqual
    }
    #[doc = "Checks if the value of the field is `Equal`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == COMP2VALUER_A::Equal
    }
}
#[doc = "Field `COMP2LOCK` reader - COMP2_CSR register lock bit"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK_A>;
#[doc = "COMP2_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK_A {
    #[doc = "0: COMP2_CSR\\[31:0\\]
for comparator 2 are read/write"]
    ReadWrite = 0,
    #[doc = "1: COMP2_CSR\\[31:0\\]
for comparator 2 are read-only"]
    ReadOnly = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::ReadWrite,
            true => COMP2LOCK_A::ReadOnly,
        }
    }
    #[doc = "Checks if the value of the field is `ReadWrite`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == COMP2LOCK_A::ReadWrite
    }
    #[doc = "Checks if the value of the field is `ReadOnly`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == COMP2LOCK_A::ReadOnly
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&self) -> COMP2SPEED_R {
        COMP2SPEED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&self) -> COMP2INNSEL_R {
        COMP2INNSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&self) -> COMP2LPTIMIN2_R {
        COMP2LPTIMIN2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&self) -> COMP2LPTIMIN1_R {
        COMP2LPTIMIN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&self) -> COMP2POLARITY_R {
        COMP2POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2value(&self) -> COMP2VALUE_R {
        COMP2VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<0> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2speed(&mut self) -> COMP2SPEED_W<3> {
        COMP2SPEED_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2innsel(&mut self) -> COMP2INNSEL_W<4> {
        COMP2INNSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W<8> {
        COMP2INPSEL_W::new(self)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lptimin2(&mut self) -> COMP2LPTIMIN2_W<12> {
        COMP2LPTIMIN2_W::new(self)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lptimin1(&mut self) -> COMP2LPTIMIN1_W<13> {
        COMP2LPTIMIN1_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2polarity(&mut self) -> COMP2POLARITY_W<15> {
        COMP2POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
