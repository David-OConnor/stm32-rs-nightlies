#[doc = "Register `RNG_HTCR` reader"]
pub struct R(crate::R<RNG_HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_HTCR` writer"]
pub struct W(crate::W<RNG_HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_HTCR_SPEC>;
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
impl From<crate::W<RNG_HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTCFG` reader - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTCFG` writer - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNG_HTCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<0> {
        HTCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG health test control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_htcr](index.html) module"]
pub struct RNG_HTCR_SPEC;
impl crate::RegisterSpec for RNG_HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_htcr::R](R) reader structure"]
impl crate::Readable for RNG_HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_htcr::W](W) writer structure"]
impl crate::Writable for RNG_HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNG_HTCR to value 0x72ac"]
impl crate::Resettable for RNG_HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x72ac;
}
