#[doc = "Register `PRGKEYR` writer"]
pub struct W(crate::W<PRGKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRGKEYR_SPEC>;
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
impl From<crate::W<PRGKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRGKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRGKEYR` writer - Program memory key"]
pub type PRGKEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRGKEYR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Program memory key"]
    #[inline(always)]
    #[must_use]
    pub fn prgkeyr(&mut self) -> PRGKEYR_W<0> {
        PRGKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program memory key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prgkeyr](index.html) module"]
pub struct PRGKEYR_SPEC;
impl crate::RegisterSpec for PRGKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prgkeyr::W](W) writer structure"]
impl crate::Writable for PRGKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRGKEYR to value 0"]
impl crate::Resettable for PRGKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
