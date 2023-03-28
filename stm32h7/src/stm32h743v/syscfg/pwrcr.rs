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
pub type ODEN_R = crate::BitReader<bool>;
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type ODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCR_SPEC, bool, O>;
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
