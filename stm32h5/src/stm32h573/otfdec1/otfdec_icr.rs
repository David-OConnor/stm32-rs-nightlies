#[doc = "Register `OTFDEC_ICR` writer"]
pub struct W(crate::W<OTFDEC_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_ICR_SPEC>;
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
impl From<crate::W<OTFDEC_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEIF` writer - Security error interrupt flag clear This bit is written by application, and always read as 0."]
pub type SEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_ICR_SPEC, bool, O>;
#[doc = "Field `XONEIF` writer - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0."]
pub type XONEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_ICR_SPEC, bool, O>;
#[doc = "Field `KEIF` writer - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again."]
pub type KEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Security error interrupt flag clear This bit is written by application, and always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn seif(&mut self) -> SEIF_W<0> {
        SEIF_W::new(self)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn xoneif(&mut self) -> XONEIF_W<1> {
        XONEIF_W::new(self)
    }
    #[doc = "Bit 2 - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again."]
    #[inline(always)]
    #[must_use]
    pub fn keif(&mut self) -> KEIF_W<2> {
        KEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_icr](index.html) module"]
pub struct OTFDEC_ICR_SPEC;
impl crate::RegisterSpec for OTFDEC_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [otfdec_icr::W](W) writer structure"]
impl crate::Writable for OTFDEC_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_ICR to value 0"]
impl crate::Resettable for OTFDEC_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
