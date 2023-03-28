#[doc = "Register `LPTIM_ICR` writer"]
pub struct W(crate::W<LPTIM_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_ICR_SPEC>;
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
impl From<crate::W<LPTIM_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPMCF` writer - CMPMCF"]
pub type CMPMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `ARRMCF` writer - ARRMCF"]
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `EXTTRIGCF` writer - EXTTRIGCF"]
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `CMPOKCF` writer - CMPOKCF"]
pub type CMPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `ARROKCF` writer - ARROKCF"]
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `UPCF` writer - UPCF"]
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
#[doc = "Field `DOWNCF` writer - DOWNCF"]
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CMPMCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<0> {
        CMPMCF_W::new(self)
    }
    #[doc = "Bit 1 - ARRMCF"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    #[doc = "Bit 2 - EXTTRIGCF"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    #[doc = "Bit 3 - CMPOKCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<3> {
        CMPOKCF_W::new(self)
    }
    #[doc = "Bit 4 - ARROKCF"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    #[doc = "Bit 5 - UPCF"]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    #[doc = "Bit 6 - DOWNCF"]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_icr](index.html) module"]
pub struct LPTIM_ICR_SPEC;
impl crate::RegisterSpec for LPTIM_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lptim_icr::W](W) writer structure"]
impl crate::Writable for LPTIM_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_ICR to value 0"]
impl crate::Resettable for LPTIM_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
