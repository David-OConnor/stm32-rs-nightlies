#[doc = "Register `RCC_STGENCKSELR` reader"]
pub struct R(crate::R<RCC_STGENCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_STGENCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_STGENCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_STGENCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_STGENCKSELR` writer"]
pub struct W(crate::W<RCC_STGENCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_STGENCKSELR_SPEC>;
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
impl From<crate::W<RCC_STGENCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_STGENCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STGENSRC` reader - STGENSRC"]
pub type STGENSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STGENSRC` writer - STGENSRC"]
pub type STGENSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_STGENCKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&self) -> STGENSRC_R {
        STGENSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    #[must_use]
    pub fn stgensrc(&mut self) -> STGENSRC_W<0> {
        STGENSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_stgenckselr](index.html) module"]
pub struct RCC_STGENCKSELR_SPEC;
impl crate::RegisterSpec for RCC_STGENCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_stgenckselr::R](R) reader structure"]
impl crate::Readable for RCC_STGENCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_stgenckselr::W](W) writer structure"]
impl crate::Writable for RCC_STGENCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_STGENCKSELR to value 0"]
impl crate::Resettable for RCC_STGENCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
