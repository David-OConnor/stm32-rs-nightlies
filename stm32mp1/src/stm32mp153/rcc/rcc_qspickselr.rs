#[doc = "Register `RCC_QSPICKSELR` reader"]
pub struct R(crate::R<RCC_QSPICKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_QSPICKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_QSPICKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_QSPICKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_QSPICKSELR` writer"]
pub struct W(crate::W<RCC_QSPICKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_QSPICKSELR_SPEC>;
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
impl From<crate::W<RCC_QSPICKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_QSPICKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPISRC` reader - QSPISRC"]
pub type QSPISRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QSPISRC` writer - QSPISRC"]
pub type QSPISRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_QSPICKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    #[must_use]
    pub fn qspisrc(&mut self) -> QSPISRC_W<0> {
        QSPISRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_qspickselr](index.html) module"]
pub struct RCC_QSPICKSELR_SPEC;
impl crate::RegisterSpec for RCC_QSPICKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_qspickselr::R](R) reader structure"]
impl crate::Readable for RCC_QSPICKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_qspickselr::W](W) writer structure"]
impl crate::Writable for RCC_QSPICKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_QSPICKSELR to value 0"]
impl crate::Resettable for RCC_QSPICKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
