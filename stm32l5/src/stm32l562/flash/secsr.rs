#[doc = "Register `SECSR` reader"]
pub struct R(crate::R<SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECSR` writer"]
pub struct W(crate::W<SECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECSR_SPEC>;
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
impl From<crate::W<SECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECEOP` reader - SECEOP"]
pub type SECEOP_R = crate::BitReader<bool>;
#[doc = "Field `SECEOP` writer - SECEOP"]
pub type SECEOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECOPERR` reader - SECOPERR"]
pub type SECOPERR_R = crate::BitReader<bool>;
#[doc = "Field `SECOPERR` writer - SECOPERR"]
pub type SECOPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECPROGERR` reader - SECPROGERR"]
pub type SECPROGERR_R = crate::BitReader<bool>;
#[doc = "Field `SECPROGERR` writer - SECPROGERR"]
pub type SECPROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECWRPERR` reader - SECWRPERR"]
pub type SECWRPERR_R = crate::BitReader<bool>;
#[doc = "Field `SECWRPERR` writer - SECWRPERR"]
pub type SECWRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECPGAERR` reader - SECPGAERR"]
pub type SECPGAERR_R = crate::BitReader<bool>;
#[doc = "Field `SECPGAERR` writer - SECPGAERR"]
pub type SECPGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECSIZERR` reader - SECSIZERR"]
pub type SECSIZERR_R = crate::BitReader<bool>;
#[doc = "Field `SECSIZERR` writer - SECSIZERR"]
pub type SECSIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECPGSERR` reader - SECPGSERR"]
pub type SECPGSERR_R = crate::BitReader<bool>;
#[doc = "Field `SECPGSERR` writer - SECPGSERR"]
pub type SECPGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECRDERR` reader - Secure read protection error"]
pub type SECRDERR_R = crate::BitReader<bool>;
#[doc = "Field `SECRDERR` writer - Secure read protection error"]
pub type SECRDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SECBSY` reader - SECBusy"]
pub type SECBSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&self) -> SECOPERR_R {
        SECOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&self) -> SECPROGERR_R {
        SECPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&self) -> SECPGAERR_R {
        SECPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&self) -> SECSIZERR_R {
        SECSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&self) -> SECRDERR_R {
        SECRDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SECBusy"]
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    #[must_use]
    pub fn seceop(&mut self) -> SECEOP_W<0> {
        SECEOP_W::new(self)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    #[must_use]
    pub fn secoperr(&mut self) -> SECOPERR_W<1> {
        SECOPERR_W::new(self)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    #[must_use]
    pub fn secprogerr(&mut self) -> SECPROGERR_W<3> {
        SECPROGERR_W::new(self)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    #[must_use]
    pub fn secwrperr(&mut self) -> SECWRPERR_W<4> {
        SECWRPERR_W::new(self)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    #[must_use]
    pub fn secpgaerr(&mut self) -> SECPGAERR_W<5> {
        SECPGAERR_W::new(self)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    #[must_use]
    pub fn secsizerr(&mut self) -> SECSIZERR_W<6> {
        SECSIZERR_W::new(self)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    #[must_use]
    pub fn secpgserr(&mut self) -> SECPGSERR_W<7> {
        SECPGSERR_W::new(self)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    #[must_use]
    pub fn secrderr(&mut self) -> SECRDERR_W<14> {
        SECRDERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](index.html) module"]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secsr::R](R) reader structure"]
impl crate::Readable for SECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secsr::W](W) writer structure"]
impl crate::Writable for SECSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
