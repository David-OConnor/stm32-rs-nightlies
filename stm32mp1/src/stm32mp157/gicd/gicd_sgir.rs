#[doc = "Register `GICD_SGIR` writer"]
pub struct W(crate::W<GICD_SGIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SGIR_SPEC>;
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
impl From<crate::W<GICD_SGIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SGIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGIINTID` writer - SGIINTID"]
pub type SGIINTID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_SGIR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NSATT` writer - NSATT"]
pub type NSATT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SGIR_SPEC, bool, O>;
#[doc = "Field `CPUTARGETLIST` writer - CPUTARGETLIST"]
pub type CPUTARGETLIST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SGIR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TARGETLISTFILTER` writer - TARGETLISTFILTER"]
pub type TARGETLISTFILTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SGIR_SPEC, u8, u8, 2, O>;
impl W {
    #[doc = "Bits 0:3 - SGIINTID"]
    #[inline(always)]
    #[must_use]
    pub fn sgiintid(&mut self) -> SGIINTID_W<0> {
        SGIINTID_W::new(self)
    }
    #[doc = "Bit 15 - NSATT"]
    #[inline(always)]
    #[must_use]
    pub fn nsatt(&mut self) -> NSATT_W<15> {
        NSATT_W::new(self)
    }
    #[doc = "Bits 16:17 - CPUTARGETLIST"]
    #[inline(always)]
    #[must_use]
    pub fn cputargetlist(&mut self) -> CPUTARGETLIST_W<16> {
        CPUTARGETLIST_W::new(self)
    }
    #[doc = "Bits 24:25 - TARGETLISTFILTER"]
    #[inline(always)]
    #[must_use]
    pub fn targetlistfilter(&mut self) -> TARGETLISTFILTER_W<24> {
        TARGETLISTFILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD software generated interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_sgir](index.html) module"]
pub struct GICD_SGIR_SPEC;
impl crate::RegisterSpec for GICD_SGIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicd_sgir::W](W) writer structure"]
impl crate::Writable for GICD_SGIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SGIR to value 0"]
impl crate::Resettable for GICD_SGIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
