#[doc = "Register `SECCCR` writer"]
pub struct W(crate::W<SECCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCCR_SPEC>;
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
impl From<crate::W<SECCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_EOP` writer - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_SECSR register."]
pub type CLR_EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_WRPERR` writer - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_SECSR register."]
pub type CLR_WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_PGSERR` writer - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_SECSR register."]
pub type CLR_PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_STRBERR` writer - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_SECSR register."]
pub type CLR_STRBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_INCERR` writer - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_SECSR register."]
pub type CLR_INCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_OBKERR` writer - OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
pub type CLR_OBKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
#[doc = "Field `CLR_OBKWERR` writer - OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
pub type CLR_OBKWERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 16 - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_eop(&mut self) -> CLR_EOP_W<16> {
        CLR_EOP_W::new(self)
    }
    #[doc = "Bit 17 - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W<17> {
        CLR_WRPERR_W::new(self)
    }
    #[doc = "Bit 18 - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W<18> {
        CLR_PGSERR_W::new(self)
    }
    #[doc = "Bit 19 - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W<19> {
        CLR_STRBERR_W::new(self)
    }
    #[doc = "Bit 20 - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W<20> {
        CLR_INCERR_W::new(self)
    }
    #[doc = "Bit 21 - OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_obkerr(&mut self) -> CLR_OBKERR_W<21> {
        CLR_OBKERR_W::new(self)
    }
    #[doc = "Bit 22 - OBKWERR flag clear bit Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register."]
    #[inline(always)]
    #[must_use]
    pub fn clr_obkwerr(&mut self) -> CLR_OBKWERR_W<22> {
        CLR_OBKWERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure clear control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secccr](index.html) module"]
pub struct SECCCR_SPEC;
impl crate::RegisterSpec for SECCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [secccr::W](W) writer structure"]
impl crate::Writable for SECCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCCR to value 0"]
impl crate::Resettable for SECCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
