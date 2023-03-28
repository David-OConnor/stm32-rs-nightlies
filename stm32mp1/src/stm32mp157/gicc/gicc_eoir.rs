#[doc = "Register `GICC_EOIR` writer"]
pub struct W(crate::W<GICC_EOIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_EOIR_SPEC>;
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
impl From<crate::W<GICC_EOIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_EOIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOIINTID` writer - EOIINTID"]
pub type EOIINTID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICC_EOIR_SPEC, u16, u16, 10, O>;
#[doc = "Field `CPUID` writer - CPUID"]
pub type CPUID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_EOIR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:9 - EOIINTID"]
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<0> {
        EOIINTID_W::new(self)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<10> {
        CPUID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC end of interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_eoir](index.html) module"]
pub struct GICC_EOIR_SPEC;
impl crate::RegisterSpec for GICC_EOIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicc_eoir::W](W) writer structure"]
impl crate::Writable for GICC_EOIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_EOIR to value 0"]
impl crate::Resettable for GICC_EOIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
