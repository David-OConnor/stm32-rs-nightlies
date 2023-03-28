#[doc = "Register `HLCR` reader"]
pub struct R(crate::R<HLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HLCR` writer"]
pub struct W(crate::W<HLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HLCR_SPEC>;
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
impl From<crate::W<HLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LM` reader - Latency mode"]
pub type LM_R = crate::BitReader<LM_A>;
#[doc = "Latency mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM_A {
    #[doc = "0: Variable initial latency"]
    Variable = 0,
    #[doc = "1: Fixed latency"]
    Fixed = 1,
}
impl From<LM_A> for bool {
    #[inline(always)]
    fn from(variant: LM_A) -> Self {
        variant as u8 != 0
    }
}
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LM_A {
        match self.bits {
            false => LM_A::Variable,
            true => LM_A::Fixed,
        }
    }
    #[doc = "Checks if the value of the field is `Variable`"]
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == LM_A::Variable
    }
    #[doc = "Checks if the value of the field is `Fixed`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == LM_A::Fixed
    }
}
#[doc = "Field `LM` writer - Latency mode"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HLCR_SPEC, LM_A, O>;
impl<'a, const O: u8> LM_W<'a, O> {
    #[doc = "Variable initial latency"]
    #[inline(always)]
    pub fn variable(self) -> &'a mut W {
        self.variant(LM_A::Variable)
    }
    #[doc = "Fixed latency"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(LM_A::Fixed)
    }
}
#[doc = "Field `WZL` reader - Write zero latency"]
pub type WZL_R = crate::BitReader<WZL_A>;
#[doc = "Write zero latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WZL_A {
    #[doc = "0: Latency on write accesses"]
    Enabled = 0,
    #[doc = "1: No latency on write accesses"]
    Disabled = 1,
}
impl From<WZL_A> for bool {
    #[inline(always)]
    fn from(variant: WZL_A) -> Self {
        variant as u8 != 0
    }
}
impl WZL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WZL_A {
        match self.bits {
            false => WZL_A::Enabled,
            true => WZL_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WZL_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WZL_A::Disabled
    }
}
#[doc = "Field `WZL` writer - Write zero latency"]
pub type WZL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HLCR_SPEC, WZL_A, O>;
impl<'a, const O: u8> WZL_W<'a, O> {
    #[doc = "Latency on write accesses"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WZL_A::Enabled)
    }
    #[doc = "No latency on write accesses"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WZL_A::Disabled)
    }
}
#[doc = "Field `TACC` reader - Access time"]
pub type TACC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TACC` writer - Access time"]
pub type TACC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HLCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRWR` reader - Read write recovery time"]
pub type TRWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRWR` writer - Read write recovery time"]
pub type TRWR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HLCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<0> {
        LM_W::new(self)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<1> {
        WZL_W::new(self)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<8> {
        TACC_W::new(self)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<16> {
        TRWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HyperBusTM latency configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlcr](index.html) module"]
pub struct HLCR_SPEC;
impl crate::RegisterSpec for HLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hlcr::R](R) reader structure"]
impl crate::Readable for HLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hlcr::W](W) writer structure"]
impl crate::Writable for HLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HLCR to value 0"]
impl crate::Resettable for HLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
