#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIF` writer - Clears the Line Interrupt Flag"]
pub type CLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag"]
pub type CFUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag"]
pub type CTERRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CRRIF` writer - Clears Register Reload Interrupt Flag"]
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clears the Line Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<0> {
        CLIF_W::new(self)
    }
    #[doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<1> {
        CFUIF_W::new(self)
    }
    #[doc = "Bit 2 - Clears the Transfer Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<2> {
        CTERRIF_W::new(self)
    }
    #[doc = "Bit 3 - Clears Register Reload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<3> {
        CRRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
