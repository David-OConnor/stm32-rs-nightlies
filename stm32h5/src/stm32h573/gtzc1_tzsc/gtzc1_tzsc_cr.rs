#[doc = "Register `GTZC1_TZSC_CR` reader"]
pub struct R(crate::R<GTZC1_TZSC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_CR` writer"]
pub struct W(crate::W<GTZC1_TZSC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_CR_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCK` reader - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
pub type LCK_R = crate::BitReader<bool>;
#[doc = "Field `LCK` writer - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
pub type LCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
    #[inline(always)]
    #[must_use]
    pub fn lck(&mut self) -> LCK_W<0> {
        LCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_cr](index.html) module"]
pub struct GTZC1_TZSC_CR_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_cr::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_cr::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_CR to value 0"]
impl crate::Resettable for GTZC1_TZSC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
