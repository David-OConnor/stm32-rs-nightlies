#[doc = "Register `PWRCR` reader"]
pub struct R(crate::R<PWRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCR` writer"]
pub struct W(crate::W<PWRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCR_SPEC>;
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
impl From<crate::W<PWRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub type ODEN_R = crate::BitReader<ODEN_A>;
#[doc = "Overdrive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODEN_A {
    #[doc = "0: Overdrive mode disabled"]
    Disabled = 0,
    #[doc = "1: Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    Enabled = 1,
}
impl From<ODEN_A> for bool {
    #[inline(always)]
    fn from(variant: ODEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODEN_A {
        match self.bits {
            false => ODEN_A::Disabled,
            true => ODEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ODEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODEN_A::Enabled
    }
}
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type ODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCR_SPEC, ODEN_A, O>;
impl<'a, const O: u8> ODEN_W<'a, O> {
    #[doc = "Overdrive mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ODEN_A::Disabled)
    }
    #[doc = "Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<0> {
        ODEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcr](index.html) module"]
pub struct PWRCR_SPEC;
impl crate::RegisterSpec for PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcr::R](R) reader structure"]
impl crate::Readable for PWRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcr::W](W) writer structure"]
impl crate::Writable for PWRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
