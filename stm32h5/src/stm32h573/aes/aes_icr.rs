#[doc = "Register `AES_ICR` writer"]
pub struct W(crate::W<AES_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_ICR_SPEC>;
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
impl From<crate::W<AES_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCF` writer - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_SR and AES_ISR registers."]
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AES_ICR_SPEC, bool, O>;
#[doc = "Field `RWEIF` writer - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and both RDERR and WRERR flags in the AES_SR register."]
pub type RWEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AES_ICR_SPEC, bool, O>;
#[doc = "Field `KEIF` writer - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register."]
pub type KEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AES_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_SR and AES_ISR registers."]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<0> {
        CCF_W::new(self)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and both RDERR and WRERR flags in the AES_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rweif(&mut self) -> RWEIF_W<1> {
        RWEIF_W::new(self)
    }
    #[doc = "Bit 2 - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register."]
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
#[doc = "AES interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_icr](index.html) module"]
pub struct AES_ICR_SPEC;
impl crate::RegisterSpec for AES_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_icr::W](W) writer structure"]
impl crate::Writable for AES_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_ICR to value 0"]
impl crate::Resettable for AES_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
