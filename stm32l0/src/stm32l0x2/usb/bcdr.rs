#[doc = "Register `BCDR` reader"]
pub struct R(crate::R<BCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDR` writer"]
pub struct W(crate::W<BCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDR_SPEC>;
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
impl From<crate::W<BCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDEN` reader - BCDEN"]
pub type BCDEN_R = crate::BitReader<BCDEN_A>;
#[doc = "BCDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDEN_A {
    #[doc = "0: disable the BCD support"]
    Disabled = 0,
    #[doc = "1: enable the BCD support within the USB device"]
    Enabled = 1,
}
impl From<BCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDEN_A {
        match self.bits {
            false => BCDEN_A::Disabled,
            true => BCDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCDEN_A::Enabled
    }
}
#[doc = "Field `BCDEN` writer - BCDEN"]
pub type BCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, BCDEN_A, O>;
impl<'a, const O: u8> BCDEN_W<'a, O> {
    #[doc = "disable the BCD support"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCDEN_A::Disabled)
    }
    #[doc = "enable the BCD support within the USB device"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCDEN_A::Enabled)
    }
}
#[doc = "Field `DCDEN` reader - DCDEN"]
pub type DCDEN_R = crate::BitReader<DCDEN_A>;
#[doc = "DCDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDEN_A {
    #[doc = "0: Data contact detection (DCD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Data contact detection (DCD) mode enabled"]
    Enabled = 1,
}
impl From<DCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDEN_A {
        match self.bits {
            false => DCDEN_A::Disabled,
            true => DCDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN_A::Enabled
    }
}
#[doc = "Field `DCDEN` writer - DCDEN"]
pub type DCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, DCDEN_A, O>;
impl<'a, const O: u8> DCDEN_W<'a, O> {
    #[doc = "Data contact detection (DCD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDEN_A::Disabled)
    }
    #[doc = "Data contact detection (DCD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDEN_A::Enabled)
    }
}
#[doc = "Field `PDEN` reader - PDEN"]
pub type PDEN_R = crate::BitReader<PDEN_A>;
#[doc = "PDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_A {
    #[doc = "0: Primary detection (PD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Primary detection (PD) mode enabled"]
    Enabled = 1,
}
impl From<PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_A {
        match self.bits {
            false => PDEN_A::Disabled,
            true => PDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDEN_A::Enabled
    }
}
#[doc = "Field `PDEN` writer - PDEN"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, PDEN_A, O>;
impl<'a, const O: u8> PDEN_W<'a, O> {
    #[doc = "Primary detection (PD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDEN_A::Disabled)
    }
    #[doc = "Primary detection (PD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDEN_A::Enabled)
    }
}
#[doc = "Field `SDEN` reader - SDEN"]
pub type SDEN_R = crate::BitReader<SDEN_A>;
#[doc = "SDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDEN_A {
    #[doc = "0: Secondary detection (SD) mode disabled"]
    Disabled = 0,
    #[doc = "1: Secondary detection (SD) mode enabled"]
    Enabled = 1,
}
impl From<SDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDEN_A {
        match self.bits {
            false => SDEN_A::Disabled,
            true => SDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDEN_A::Enabled
    }
}
#[doc = "Field `SDEN` writer - SDEN"]
pub type SDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, SDEN_A, O>;
impl<'a, const O: u8> SDEN_W<'a, O> {
    #[doc = "Secondary detection (SD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDEN_A::Disabled)
    }
    #[doc = "Secondary detection (SD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDEN_A::Enabled)
    }
}
#[doc = "Field `DCDET` reader - DCDET"]
pub type DCDET_R = crate::BitReader<DCDET_A>;
#[doc = "DCDET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDET_A {
    #[doc = "0: data lines contact not detected"]
    NotDetected = 0,
    #[doc = "1: data lines contact detected"]
    Detected = 1,
}
impl From<DCDET_A> for bool {
    #[inline(always)]
    fn from(variant: DCDET_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDET_A {
        match self.bits {
            false => DCDET_A::NotDetected,
            true => DCDET_A::Detected,
        }
    }
    #[doc = "Checks if the value of the field is `NotDetected`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DCDET_A::NotDetected
    }
    #[doc = "Checks if the value of the field is `Detected`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DCDET_A::Detected
    }
}
#[doc = "Field `PDET` reader - PDET"]
pub type PDET_R = crate::BitReader<PDET_A>;
#[doc = "PDET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDET_A {
    #[doc = "0: no BCD support detected"]
    NoBcd = 0,
    #[doc = "1: BCD support detected"]
    Bcd = 1,
}
impl From<PDET_A> for bool {
    #[inline(always)]
    fn from(variant: PDET_A) -> Self {
        variant as u8 != 0
    }
}
impl PDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDET_A {
        match self.bits {
            false => PDET_A::NoBcd,
            true => PDET_A::Bcd,
        }
    }
    #[doc = "Checks if the value of the field is `NoBcd`"]
    #[inline(always)]
    pub fn is_no_bcd(&self) -> bool {
        *self == PDET_A::NoBcd
    }
    #[doc = "Checks if the value of the field is `Bcd`"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == PDET_A::Bcd
    }
}
#[doc = "Field `SDET` reader - SDET"]
pub type SDET_R = crate::BitReader<SDET_A>;
#[doc = "SDET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDET_A {
    #[doc = "0: CDP detected"]
    Cdp = 0,
    #[doc = "1: DCP detected"]
    Dcp = 1,
}
impl From<SDET_A> for bool {
    #[inline(always)]
    fn from(variant: SDET_A) -> Self {
        variant as u8 != 0
    }
}
impl SDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDET_A {
        match self.bits {
            false => SDET_A::Cdp,
            true => SDET_A::Dcp,
        }
    }
    #[doc = "Checks if the value of the field is `Cdp`"]
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        *self == SDET_A::Cdp
    }
    #[doc = "Checks if the value of the field is `Dcp`"]
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        *self == SDET_A::Dcp
    }
}
#[doc = "Field `PS2DET` reader - PS2DET"]
pub type PS2DET_R = crate::BitReader<PS2DET_A>;
#[doc = "PS2DET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS2DET_A {
    #[doc = "0: Normal port detected"]
    Normal = 0,
    #[doc = "1: PS2 port or proprietary charger detected"]
    Ps2 = 1,
}
impl From<PS2DET_A> for bool {
    #[inline(always)]
    fn from(variant: PS2DET_A) -> Self {
        variant as u8 != 0
    }
}
impl PS2DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS2DET_A {
        match self.bits {
            false => PS2DET_A::Normal,
            true => PS2DET_A::Ps2,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PS2DET_A::Normal
    }
    #[doc = "Checks if the value of the field is `Ps2`"]
    #[inline(always)]
    pub fn is_ps2(&self) -> bool {
        *self == PS2DET_A::Ps2
    }
}
#[doc = "Field `DPPU` reader - DPPU"]
pub type DPPU_R = crate::BitReader<DPPU_A>;
#[doc = "DPPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPPU_A {
    #[doc = "0: signalize disconnect to the host when needed by the user software"]
    Disabled = 0,
    #[doc = "1: enable the embedded pull-up on the DP line"]
    Enabled = 1,
}
impl From<DPPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_A) -> Self {
        variant as u8 != 0
    }
}
impl DPPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPU_A {
        match self.bits {
            false => DPPU_A::Disabled,
            true => DPPU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPPU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPPU_A::Enabled
    }
}
#[doc = "Field `DPPU` writer - DPPU"]
pub type DPPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCDR_SPEC, DPPU_A, O>;
impl<'a, const O: u8> DPPU_W<'a, O> {
    #[doc = "signalize disconnect to the host when needed by the user software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DPPU_A::Disabled)
    }
    #[doc = "enable the embedded pull-up on the DP line"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DPPU_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - BCDEN"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDEN"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDEN"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCDET"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDET"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SDET"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PS2DET"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - DPPU"]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<0> {
        BCDEN_W::new(self)
    }
    #[doc = "Bit 1 - DCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<1> {
        DCDEN_W::new(self)
    }
    #[doc = "Bit 2 - PDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<2> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 3 - SDEN"]
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<3> {
        SDEN_W::new(self)
    }
    #[doc = "Bit 15 - DPPU"]
    #[inline(always)]
    #[must_use]
    pub fn dppu(&mut self) -> DPPU_W<15> {
        DPPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery charging detector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdr](index.html) module"]
pub struct BCDR_SPEC;
impl crate::RegisterSpec for BCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdr::R](R) reader structure"]
impl crate::Readable for BCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdr::W](W) writer structure"]
impl crate::Writable for BCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
