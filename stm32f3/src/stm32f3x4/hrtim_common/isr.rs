#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub type FLT1_R = crate::BitReader<FLT1R_A>;
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1R_A {
    #[doc = "0: No fault interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Fault interrupt occurred"]
    Event = 1,
}
impl From<FLT1R_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1R_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1R_A {
        match self.bits {
            false => FLT1R_A::NoEvent,
            true => FLT1R_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == FLT1R_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == FLT1R_A::Event
    }
}
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1W_AW {
    #[doc = "1: Clear fault interrupt"]
    Clear = 1,
}
impl From<FLT1W_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1` writer - Fault 1 Interrupt Flag"]
pub type FLT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, FLT1W_AW, O>;
impl<'a, const O: u8> FLT1_W<'a, O> {
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT1W_AW::Clear)
    }
}
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub use FLT1_R as FLT2_R;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub use FLT1_R as FLT3_R;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub use FLT1_R as FLT4_R;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub use FLT1_R as FLT5_R;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub use FLT1_R as SYSFLT_R;
#[doc = "Field `FLT2` writer - Fault 2 Interrupt Flag"]
pub use FLT1_W as FLT2_W;
#[doc = "Field `FLT3` writer - Fault 3 Interrupt Flag"]
pub use FLT1_W as FLT3_W;
#[doc = "Field `FLT4` writer - Fault 4 Interrupt Flag"]
pub use FLT1_W as FLT4_W;
#[doc = "Field `FLT5` writer - Fault 5 Interrupt Flag"]
pub use FLT1_W as FLT5_W;
#[doc = "Field `SYSFLT` writer - System Fault Interrupt Flag"]
pub use FLT1_W as SYSFLT_W;
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub type DLLRDY_R = crate::BitReader<DLLRDYR_A>;
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYR_A {
    #[doc = "0: No DLL calibration ready interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: DLL calibration ready interrupt occurred"]
    Event = 1,
}
impl From<DLLRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl DLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRDYR_A {
        match self.bits {
            false => DLLRDYR_A::NoEvent,
            true => DLLRDYR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == DLLRDYR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLLRDYR_A::Event
    }
}
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYW_AW {
    #[doc = "1: Clear DLL calibration interrupt"]
    Clear = 1,
}
impl From<DLLRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDY` writer - DLL Ready Interrupt Flag"]
pub type DLLRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, DLLRDYW_AW, O>;
impl<'a, const O: u8> DLLRDY_W<'a, O> {
    #[doc = "Clear DLL calibration interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLLRDYW_AW::Clear)
    }
}
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub type BMPER_R = crate::BitReader<BMPERR_A>;
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERR_A {
    #[doc = "0: No burst mode period interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Burst mode period interrupt occured"]
    Event = 1,
}
impl From<BMPERR_A> for bool {
    #[inline(always)]
    fn from(variant: BMPERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BMPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPERR_A {
        match self.bits {
            false => BMPERR_A::NoEvent,
            true => BMPERR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BMPERR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == BMPERR_A::Event
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERW_AW {
    #[doc = "1: Clear burst mode period interrupt"]
    Clear = 1,
}
impl From<BMPERW_AW> for bool {
    #[inline(always)]
    fn from(variant: BMPERW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPER` writer - Burst mode Period Interrupt Flag"]
pub type BMPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, BMPERW_AW, O>;
impl<'a, const O: u8> BMPER_W<'a, O> {
    #[doc = "Clear burst mode period interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BMPERW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt1(&mut self) -> FLT1_W<0> {
        FLT1_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt2(&mut self) -> FLT2_W<1> {
        FLT2_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt3(&mut self) -> FLT3_W<2> {
        FLT3_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt4(&mut self) -> FLT4_W<3> {
        FLT4_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt5(&mut self) -> FLT5_W<4> {
        FLT5_W::new(self)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sysflt(&mut self) -> SYSFLT_W<5> {
        SYSFLT_W::new(self)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdy(&mut self) -> DLLRDY_W<16> {
        DLLRDY_W::new(self)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BMPER_W<17> {
        BMPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
