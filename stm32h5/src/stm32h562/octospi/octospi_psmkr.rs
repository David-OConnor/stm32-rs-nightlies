#[doc = "Register `OCTOSPI_PSMKR` reader"]
pub struct R(crate::R<OCTOSPI_PSMKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_PSMKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_PSMKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_PSMKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_PSMKR` writer"]
pub struct W(crate::W<OCTOSPI_PSMKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_PSMKR_SPEC>;
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
impl From<crate::W<OCTOSPI_PSMKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_PSMKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
pub type MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MASK` writer - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_PSMKR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_psmkr](index.html) module"]
pub struct OCTOSPI_PSMKR_SPEC;
impl crate::RegisterSpec for OCTOSPI_PSMKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_psmkr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_PSMKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_psmkr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_PSMKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_PSMKR to value 0"]
impl crate::Resettable for OCTOSPI_PSMKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
