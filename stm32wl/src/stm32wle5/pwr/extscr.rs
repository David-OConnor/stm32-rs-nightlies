#[doc = "Register `EXTSCR` reader"]
pub struct R(crate::R<EXTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTSCR` writer"]
pub struct W(crate::W<EXTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSCR_SPEC>;
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
impl From<crate::W<EXTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear CPU1 Stop Standby flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1CSSFW_AW {
    #[doc = "1: Setting this bit clears the C1STOPF and C1SBF bits"]
    Clear = 1,
}
impl From<C1CSSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1CSSFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1CSSF` writer - Clear CPU1 Stop Standby flags"]
pub type C1CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTSCR_SPEC, C1CSSFW_AW, O>;
impl<'a, const O: u8> C1CSSF_W<'a, O> {
    #[doc = "Setting this bit clears the C1STOPF and C1SBF bits"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(C1CSSFW_AW::Clear)
    }
}
#[doc = "Field `C1SBF` reader - System Standby flag for CPU1. (no core states retained)"]
pub type C1SBF_R = crate::BitReader<C1SBF_A>;
#[doc = "System Standby flag for CPU1. (no core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1SBF_A {
    #[doc = "0: System has not been in Standby mode"]
    NoStandby = 0,
    #[doc = "1: System has been in Standby mode"]
    Standby = 1,
}
impl From<C1SBF_A> for bool {
    #[inline(always)]
    fn from(variant: C1SBF_A) -> Self {
        variant as u8 != 0
    }
}
impl C1SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1SBF_A {
        match self.bits {
            false => C1SBF_A::NoStandby,
            true => C1SBF_A::Standby,
        }
    }
    #[doc = "Checks if the value of the field is `NoStandby`"]
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        *self == C1SBF_A::NoStandby
    }
    #[doc = "Checks if the value of the field is `Standby`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == C1SBF_A::Standby
    }
}
#[doc = "Field `C1STOP2F` reader - System Stop2 flag for CPU1. (partial core states retained)"]
pub type C1STOP2F_R = crate::BitReader<C1STOP2F_A>;
#[doc = "System Stop2 flag for CPU1. (partial core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOP2F_A {
    #[doc = "0: System has not been in Stop 2 mode"]
    NoStop = 0,
    #[doc = "1: System has been in Stop 2 mode"]
    Stop = 1,
}
impl From<C1STOP2F_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOP2F_A) -> Self {
        variant as u8 != 0
    }
}
impl C1STOP2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1STOP2F_A {
        match self.bits {
            false => C1STOP2F_A::NoStop,
            true => C1STOP2F_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `NoStop`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOP2F_A::NoStop
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOP2F_A::Stop
    }
}
#[doc = "Field `C1STOPF` reader - System Stop0, 1 flag for CPU1. (All core states retained)"]
pub type C1STOPF_R = crate::BitReader<C1STOPF_A>;
#[doc = "System Stop0, 1 flag for CPU1. (All core states retained)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOPF_A {
    #[doc = "0: System has not been in Stop 0 or 1 mode"]
    NoStop = 0,
    #[doc = "1: System has been in Stop 0 or 1 mode"]
    Stop = 1,
}
impl From<C1STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl C1STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1STOPF_A {
        match self.bits {
            false => C1STOPF_A::NoStop,
            true => C1STOPF_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `NoStop`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOPF_A::NoStop
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOPF_A::Stop
    }
}
#[doc = "Field `C1DS` reader - CPU1 deepsleep mode"]
pub type C1DS_R = crate::BitReader<C1DS_A>;
#[doc = "CPU1 deepsleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1DS_A {
    #[doc = "0: CPU is running or in sleep"]
    RunningOrSleep = 0,
    #[doc = "1: CPU is in Deep-Sleep"]
    DeepSleep = 1,
}
impl From<C1DS_A> for bool {
    #[inline(always)]
    fn from(variant: C1DS_A) -> Self {
        variant as u8 != 0
    }
}
impl C1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1DS_A {
        match self.bits {
            false => C1DS_A::RunningOrSleep,
            true => C1DS_A::DeepSleep,
        }
    }
    #[doc = "Checks if the value of the field is `RunningOrSleep`"]
    #[inline(always)]
    pub fn is_running_or_sleep(&self) -> bool {
        *self == C1DS_A::RunningOrSleep
    }
    #[doc = "Checks if the value of the field is `DeepSleep`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == C1DS_A::DeepSleep
    }
}
impl R {
    #[doc = "Bit 8 - System Standby flag for CPU1. (no core states retained)"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System Stop2 flag for CPU1. (partial core states retained)"]
    #[inline(always)]
    pub fn c1stop2f(&self) -> C1STOP2F_R {
        C1STOP2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System Stop0, 1 flag for CPU1. (All core states retained)"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c1cssf(&mut self) -> C1CSSF_W<0> {
        C1CSSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power extended status and status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extscr](index.html) module"]
pub struct EXTSCR_SPEC;
impl crate::RegisterSpec for EXTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extscr::R](R) reader structure"]
impl crate::Readable for EXTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extscr::W](W) writer structure"]
impl crate::Writable for EXTSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTSCR to value 0"]
impl crate::Resettable for EXTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
