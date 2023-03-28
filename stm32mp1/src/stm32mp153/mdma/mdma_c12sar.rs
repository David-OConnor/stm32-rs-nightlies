#[doc = "Register `MDMA_C12SAR` reader"]
pub struct R(crate::R<MDMA_C12SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C12SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C12SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C12SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C12SAR` writer"]
pub struct W(crate::W<MDMA_C12SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C12SAR_SPEC>;
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
impl From<crate::W<MDMA_C12SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C12SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR` reader - SAR"]
pub type SAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR` writer - SAR"]
pub type SAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C12SAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<0> {
        SAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12sar](index.html) module"]
pub struct MDMA_C12SAR_SPEC;
impl crate::RegisterSpec for MDMA_C12SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c12sar::R](R) reader structure"]
impl crate::Readable for MDMA_C12SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c12sar::W](W) writer structure"]
impl crate::Writable for MDMA_C12SAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C12SAR to value 0"]
impl crate::Resettable for MDMA_C12SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
