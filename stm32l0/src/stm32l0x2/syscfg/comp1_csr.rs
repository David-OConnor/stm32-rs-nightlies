#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable bit"]
pub type COMP1EN_R = crate::BitReader<COMP1EN_A>;
#[doc = "Comparator 1 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1EN_A {
    #[doc = "0: Comparator 1 switched OFF"]
    Disabled = 0,
    #[doc = "1: Comparator 1 switched ON"]
    Enabled = 1,
}
impl From<COMP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1EN_A {
        match self.bits {
            false => COMP1EN_A::Disabled,
            true => COMP1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN_A::Enabled
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable bit"]
pub type COMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, COMP1EN_A, O>;
impl<'a, const O: u8> COMP1EN_W<'a, O> {
    #[doc = "Comparator 1 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::Disabled)
    }
    #[doc = "Comparator 1 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::Enabled)
    }
}
#[doc = "Field `COMP1INNSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1INNSEL_R = crate::FieldReader<u8, COMP1INNSEL_A>;
#[doc = "Comparator 1 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1INNSEL_A {
    #[doc = "0: VREFINT"]
    Vrefint = 0,
    #[doc = "1: PA0"]
    Pa0 = 1,
    #[doc = "2: PA4"]
    Pa4 = 2,
    #[doc = "3: PA5"]
    Pa5 = 3,
}
impl From<COMP1INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INNSEL_A) -> Self {
        variant as _
    }
}
impl COMP1INNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1INNSEL_A {
        match self.bits {
            0 => COMP1INNSEL_A::Vrefint,
            1 => COMP1INNSEL_A::Pa0,
            2 => COMP1INNSEL_A::Pa4,
            3 => COMP1INNSEL_A::Pa5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Vrefint`"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        *self == COMP1INNSEL_A::Vrefint
    }
    #[doc = "Checks if the value of the field is `Pa0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == COMP1INNSEL_A::Pa0
    }
    #[doc = "Checks if the value of the field is `Pa4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == COMP1INNSEL_A::Pa4
    }
    #[doc = "Checks if the value of the field is `Pa5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == COMP1INNSEL_A::Pa5
    }
}
#[doc = "Field `COMP1INNSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1INNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP1_CSR_SPEC, u8, COMP1INNSEL_A, 2, O>;
impl<'a, const O: u8> COMP1INNSEL_W<'a, O> {
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::Vrefint)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::Pa0)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(COMP1INNSEL_A::Pa5)
    }
}
#[doc = "Field `COMP1WM` reader - Comparator 1 window mode selection bit"]
pub type COMP1WM_R = crate::BitReader<COMP1WM_A>;
#[doc = "Comparator 1 window mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1WM_A {
    #[doc = "0: Plus input of comparator 1 connected to PA1"]
    Pa1 = 0,
    #[doc = "1: Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    Comp2plus = 1,
}
impl From<COMP1WM_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1WM_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1WM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1WM_A {
        match self.bits {
            false => COMP1WM_A::Pa1,
            true => COMP1WM_A::Comp2plus,
        }
    }
    #[doc = "Checks if the value of the field is `Pa1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == COMP1WM_A::Pa1
    }
    #[doc = "Checks if the value of the field is `Comp2plus`"]
    #[inline(always)]
    pub fn is_comp2plus(&self) -> bool {
        *self == COMP1WM_A::Comp2plus
    }
}
#[doc = "Field `COMP1WM` writer - Comparator 1 window mode selection bit"]
pub type COMP1WM_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, COMP1WM_A, O>;
impl<'a, const O: u8> COMP1WM_W<'a, O> {
    #[doc = "Plus input of comparator 1 connected to PA1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(COMP1WM_A::Pa1)
    }
    #[doc = "Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    #[inline(always)]
    pub fn comp2plus(self) -> &'a mut W {
        self.variant(COMP1WM_A::Comp2plus)
    }
}
#[doc = "Field `COMP1LPTIMIN1` reader - Comparator 1 LPTIM input propagation bit"]
pub type COMP1LPTIMIN1_R = crate::BitReader<COMP1LPTIMIN1_A>;
#[doc = "Comparator 1 LPTIM input propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LPTIMIN1_A {
    #[doc = "0: Comparator 1 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 1 output sent to LPTIM input 1"]
    NotGated = 1,
}
impl From<COMP1LPTIMIN1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LPTIMIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1LPTIMIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LPTIMIN1_A {
        match self.bits {
            false => COMP1LPTIMIN1_A::Gated,
            true => COMP1LPTIMIN1_A::NotGated,
        }
    }
    #[doc = "Checks if the value of the field is `Gated`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP1LPTIMIN1_A::Gated
    }
    #[doc = "Checks if the value of the field is `NotGated`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP1LPTIMIN1_A::NotGated
    }
}
#[doc = "Field `COMP1LPTIMIN1` writer - Comparator 1 LPTIM input propagation bit"]
pub type COMP1LPTIMIN1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP1_CSR_SPEC, COMP1LPTIMIN1_A, O>;
impl<'a, const O: u8> COMP1LPTIMIN1_W<'a, O> {
    #[doc = "Comparator 1 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(COMP1LPTIMIN1_A::Gated)
    }
    #[doc = "Comparator 1 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(COMP1LPTIMIN1_A::NotGated)
    }
}
#[doc = "Field `COMP1POLARITY` reader - Comparator 1 polarity selection bit"]
pub type COMP1POLARITY_R = crate::BitReader<COMP1POLARITY_A>;
#[doc = "Comparator 1 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1POLARITY_A {
    #[doc = "0: Comparator 1 output value not inverted"]
    NotInverted = 0,
    #[doc = "1: Comparator 1 output value inverted"]
    Inverted = 1,
}
impl From<COMP1POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1POLARITY_A {
        match self.bits {
            false => COMP1POLARITY_A::NotInverted,
            true => COMP1POLARITY_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POLARITY_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POLARITY_A::Inverted
    }
}
#[doc = "Field `COMP1POLARITY` writer - Comparator 1 polarity selection bit"]
pub type COMP1POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP1_CSR_SPEC, COMP1POLARITY_A, O>;
impl<'a, const O: u8> COMP1POLARITY_W<'a, O> {
    #[doc = "Comparator 1 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP1POLARITY_A::NotInverted)
    }
    #[doc = "Comparator 1 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP1POLARITY_A::Inverted)
    }
}
#[doc = "Field `COMP1VALUE` reader - Comparator 1 output status bit"]
pub type COMP1VALUE_R = crate::BitReader<COMP1VALUER_A>;
#[doc = "Comparator 1 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1VALUER_A {
    #[doc = "0: Comparator values are not equal"]
    NotEqual = 0,
    #[doc = "1: Comparator values are equal"]
    Equal = 1,
}
impl From<COMP1VALUER_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1VALUER_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1VALUER_A {
        match self.bits {
            false => COMP1VALUER_A::NotEqual,
            true => COMP1VALUER_A::Equal,
        }
    }
    #[doc = "Checks if the value of the field is `NotEqual`"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        *self == COMP1VALUER_A::NotEqual
    }
    #[doc = "Checks if the value of the field is `Equal`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == COMP1VALUER_A::Equal
    }
}
#[doc = "Field `COMP1LOCK` reader - COMP1_CSR register lock bit"]
pub type COMP1LOCK_R = crate::BitReader<COMP1LOCK_A>;
#[doc = "COMP1_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LOCK_A {
    #[doc = "0: COMP1_CSR\\[31:0\\]
for comparator 1 are read/write"]
    ReadWrite = 0,
    #[doc = "1: COMP1_CSR\\[31:0\\]
for comparator 1 are read-only"]
    ReadOnly = 1,
}
impl From<COMP1LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP1LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LOCK_A {
        match self.bits {
            false => COMP1LOCK_A::ReadWrite,
            true => COMP1LOCK_A::ReadOnly,
        }
    }
    #[doc = "Checks if the value of the field is `ReadWrite`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == COMP1LOCK_A::ReadWrite
    }
    #[doc = "Checks if the value of the field is `ReadOnly`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == COMP1LOCK_A::ReadOnly
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<0> {
        COMP1EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W<4> {
        COMP1INNSEL_W::new(self)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1wm(&mut self) -> COMP1WM_W<8> {
        COMP1WM_W::new(self)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W<12> {
        COMP1LPTIMIN1_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W<15> {
        COMP1POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
