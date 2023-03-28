#[doc = "Register `RCC_LPTIM1CKSELR` reader"]
pub struct R(crate::R<RCC_LPTIM1CKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_LPTIM1CKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_LPTIM1CKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_LPTIM1CKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_LPTIM1CKSELR` writer"]
pub struct W(crate::W<RCC_LPTIM1CKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_LPTIM1CKSELR_SPEC>;
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
impl From<crate::W<RCC_LPTIM1CKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_LPTIM1CKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM1SRC` reader - LPTIM1SRC"]
pub type LPTIM1SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM1SRC` writer - LPTIM1SRC"]
pub type LPTIM1SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_LPTIM1CKSELR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - LPTIM1SRC"]
    #[inline(always)]
    pub fn lptim1src(&self) -> LPTIM1SRC_R {
        LPTIM1SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM1SRC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1src(&mut self) -> LPTIM1SRC_W<0> {
        LPTIM1SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_lptim1ckselr](index.html) module"]
pub struct RCC_LPTIM1CKSELR_SPEC;
impl crate::RegisterSpec for RCC_LPTIM1CKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_lptim1ckselr::R](R) reader structure"]
impl crate::Readable for RCC_LPTIM1CKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_lptim1ckselr::W](W) writer structure"]
impl crate::Writable for RCC_LPTIM1CKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_LPTIM1CKSELR to value 0"]
impl crate::Resettable for RCC_LPTIM1CKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
