#[doc = "Register `RCC_TZAHB6RSTCLRR` reader"]
pub struct R(crate::R<RCC_TZAHB6RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_TZAHB6RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_TZAHB6RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_TZAHB6RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_TZAHB6RSTCLRR` writer"]
pub struct W(crate::W<RCC_TZAHB6RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_TZAHB6RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_TZAHB6RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_TZAHB6RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMARST` reader - MDMARST"]
pub type MDMARST_R = crate::BitReader<bool>;
#[doc = "Field `MDMARST` writer - MDMARST"]
pub type MDMARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_TZAHB6RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    #[must_use]
    pub fn mdmarst(&mut self) -> MDMARST_W<0> {
        MDMARST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzahb6rstclrr](index.html) module"]
pub struct RCC_TZAHB6RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_TZAHB6RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_tzahb6rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_TZAHB6RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_tzahb6rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_TZAHB6RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_TZAHB6RSTCLRR to value 0"]
impl crate::Resettable for RCC_TZAHB6RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
