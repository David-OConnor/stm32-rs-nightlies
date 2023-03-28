#[doc = "Register `GLPMCFG` reader"]
pub struct R(crate::R<GLPMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLPMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLPMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLPMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLPMCFG` writer"]
pub struct W(crate::W<GLPMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLPMCFG_SPEC>;
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
impl From<crate::W<GLPMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLPMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader<bool>;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LPMACK_R = crate::BitReader<bool>;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LPMACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `BESL` reader - Best effort service latency"]
pub type BESL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BESL` writer - Best effort service latency"]
pub type BESL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub type REMWAKE_R = crate::BitReader<bool>;
#[doc = "Field `REMWAKE` writer - bRemoteWake value"]
pub type REMWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `L1SSEN` reader - L1 Shallow Sleep enable"]
pub type L1SSEN_R = crate::BitReader<bool>;
#[doc = "Field `L1SSEN` writer - L1 Shallow Sleep enable"]
pub type L1SSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `BESLTHRS` reader - BESL threshold"]
pub type BESLTHRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BESLTHRS` writer - BESL threshold"]
pub type BESLTHRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `L1DSEN` reader - L1 deep sleep enable"]
pub type L1DSEN_R = crate::BitReader<bool>;
#[doc = "Field `L1DSEN` writer - L1 deep sleep enable"]
pub type L1DSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMRST` reader - LPM response"]
pub type LPMRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLPSTS` reader - Port sleep status"]
pub type SLPSTS_R = crate::BitReader<bool>;
#[doc = "Field `L1RSMOK` reader - Sleep State Resume OK"]
pub type L1RSMOK_R = crate::BitReader<bool>;
#[doc = "Field `LPMCHIDX` reader - LPM Channel Index"]
pub type LPMCHIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMCHIDX` writer - LPM Channel Index"]
pub type LPMCHIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPMRCNT` reader - LPM retry count"]
pub type LPMRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMRCNT` writer - LPM retry count"]
pub type LPMRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SNDLPM` reader - Send LPM transaction"]
pub type SNDLPM_R = crate::BitReader<bool>;
#[doc = "Field `SNDLPM` writer - Send LPM transaction"]
pub type SNDLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
#[doc = "Field `LPMRCNTSTS` reader - LPM retry count status"]
pub type LPMRCNTSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENBESL` reader - Enable best effort service latency"]
pub type ENBESL_R = crate::BitReader<bool>;
#[doc = "Field `ENBESL` writer - Enable best effort service latency"]
pub type ENBESL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - LPM response"]
    #[inline(always)]
    pub fn lpmrst(&self) -> LPMRST_R {
        LPMRST_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port sleep status"]
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sleep State Resume OK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<0> {
        LPMEN_W::new(self)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<1> {
        LPMACK_W::new(self)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<2> {
        BESL_W::new(self)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    #[must_use]
    pub fn remwake(&mut self) -> REMWAKE_W<6> {
        REMWAKE_W::new(self)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1ssen(&mut self) -> L1SSEN_W<7> {
        L1SSEN_W::new(self)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    #[must_use]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<8> {
        BESLTHRS_W::new(self)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1dsen(&mut self) -> L1DSEN_W<12> {
        L1DSEN_W::new(self)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    #[must_use]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<17> {
        LPMCHIDX_W::new(self)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W<21> {
        LPMRCNT_W::new(self)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn sndlpm(&mut self) -> SNDLPM_W<24> {
        SNDLPM_W::new(self)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    #[must_use]
    pub fn enbesl(&mut self) -> ENBESL_W<28> {
        ENBESL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glpmcfg](index.html) module"]
pub struct GLPMCFG_SPEC;
impl crate::RegisterSpec for GLPMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glpmcfg::R](R) reader structure"]
impl crate::Readable for GLPMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glpmcfg::W](W) writer structure"]
impl crate::Writable for GLPMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLPMCFG to value 0"]
impl crate::Resettable for GLPMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
