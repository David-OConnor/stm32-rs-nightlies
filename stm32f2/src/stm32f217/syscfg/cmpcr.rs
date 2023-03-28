#[doc = "Register `CMPCR` reader"]
pub struct R(crate::R<CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCR` writer"]
pub struct W(crate::W<CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCR_SPEC>;
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
impl From<crate::W<CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_PD` reader - Compensation cell power-down"]
pub type CMP_PD_R = crate::BitReader<bool>;
#[doc = "Field `CMP_PD` writer - Compensation cell power-down"]
pub type CMP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCR_SPEC, bool, O>;
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_pd(&mut self) -> CMP_PD_W<0> {
        CMP_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcr](index.html) module"]
pub struct CMPCR_SPEC;
impl crate::RegisterSpec for CMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcr::R](R) reader structure"]
impl crate::Readable for CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpcr::W](W) writer structure"]
impl crate::Writable for CMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
