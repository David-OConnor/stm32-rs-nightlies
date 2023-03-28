#[doc = "Register `HSEM_CR` writer"]
pub struct W(crate::W<HSEM_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_CR_SPEC>;
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
impl From<crate::W<HSEM_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COREID` writer - COREID"]
pub type COREID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `KEY` writer - KEY"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_CR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<8> {
        COREID_W::new(self)
    }
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_cr](index.html) module"]
pub struct HSEM_CR_SPEC;
impl crate::RegisterSpec for HSEM_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hsem_cr::W](W) writer structure"]
impl crate::Writable for HSEM_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_CR to value 0"]
impl crate::Resettable for HSEM_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
