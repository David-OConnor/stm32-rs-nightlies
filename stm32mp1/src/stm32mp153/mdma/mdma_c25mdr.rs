#[doc = "Register `MDMA_C25MDR` reader"]
pub struct R(crate::R<MDMA_C25MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C25MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C25MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C25MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C25MDR` writer"]
pub struct W(crate::W<MDMA_C25MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C25MDR_SPEC>;
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
impl From<crate::W<MDMA_C25MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C25MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDR` reader - MDR"]
pub type MDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDR` writer - MDR"]
pub type MDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C25MDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    #[must_use]
    pub fn mdr(&mut self) -> MDR_W<0> {
        MDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c25mdr](index.html) module"]
pub struct MDMA_C25MDR_SPEC;
impl crate::RegisterSpec for MDMA_C25MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c25mdr::R](R) reader structure"]
impl crate::Readable for MDMA_C25MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c25mdr::W](W) writer structure"]
impl crate::Writable for MDMA_C25MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C25MDR to value 0"]
impl crate::Resettable for MDMA_C25MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
