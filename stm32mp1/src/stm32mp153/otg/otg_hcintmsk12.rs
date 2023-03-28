#[doc = "Register `OTG_HCINTMSK12` reader"]
pub struct R(crate::R<OTG_HCINTMSK12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCINTMSK12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCINTMSK12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCINTMSK12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCINTMSK12` writer"]
pub struct W(crate::W<OTG_HCINTMSK12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCINTMSK12_SPEC>;
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
impl From<crate::W<OTG_HCINTMSK12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCINTMSK12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - XFRCM"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - XFRCM"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `CHHM` reader - CHHM"]
pub type CHHM_R = crate::BitReader<bool>;
#[doc = "Field `CHHM` writer - CHHM"]
pub type CHHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub type AHBERRM_R = crate::BitReader<bool>;
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub type AHBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `STALLM` reader - STALLM"]
pub type STALLM_R = crate::BitReader<bool>;
#[doc = "Field `STALLM` writer - STALLM"]
pub type STALLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `NAKM` reader - NAKM"]
pub type NAKM_R = crate::BitReader<bool>;
#[doc = "Field `NAKM` writer - NAKM"]
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `ACKM` reader - ACKM"]
pub type ACKM_R = crate::BitReader<bool>;
#[doc = "Field `ACKM` writer - ACKM"]
pub type ACKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `NYET` reader - NYET"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - NYET"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `TXERRM` reader - TXERRM"]
pub type TXERRM_R = crate::BitReader<bool>;
#[doc = "Field `TXERRM` writer - TXERRM"]
pub type TXERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `BBERRM` reader - BBERRM"]
pub type BBERRM_R = crate::BitReader<bool>;
#[doc = "Field `BBERRM` writer - BBERRM"]
pub type BBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `FRMORM` reader - FRMORM"]
pub type FRMORM_R = crate::BitReader<bool>;
#[doc = "Field `FRMORM` writer - FRMORM"]
pub type FRMORM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `DTERRM` reader - DTERRM"]
pub type DTERRM_R = crate::BitReader<bool>;
#[doc = "Field `DTERRM` writer - DTERRM"]
pub type DTERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `BNAMSK` reader - BNAMSK"]
pub type BNAMSK_R = crate::BitReader<bool>;
#[doc = "Field `BNAMSK` writer - BNAMSK"]
pub type BNAMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
#[doc = "Field `DESCLSTROLLMSK` reader - DESCLSTROLLMSK"]
pub type DESCLSTROLLMSK_R = crate::BitReader<bool>;
#[doc = "Field `DESCLSTROLLMSK` writer - DESCLSTROLLMSK"]
pub type DESCLSTROLLMSK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_HCINTMSK12_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNAMSK"]
    #[inline(always)]
    pub fn bnamsk(&self) -> BNAMSK_R {
        BNAMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - DESCLSTROLLMSK"]
    #[inline(always)]
    pub fn desclstrollmsk(&self) -> DESCLSTROLLMSK_R {
        DESCLSTROLLMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    #[must_use]
    pub fn chhm(&mut self) -> CHHM_W<1> {
        CHHM_W::new(self)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<2> {
        AHBERRM_W::new(self)
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    #[must_use]
    pub fn stallm(&mut self) -> STALLM_W<3> {
        STALLM_W::new(self)
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<4> {
        NAKM_W::new(self)
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    #[must_use]
    pub fn ackm(&mut self) -> ACKM_W<5> {
        ACKM_W::new(self)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    #[must_use]
    pub fn txerrm(&mut self) -> TXERRM_W<7> {
        TXERRM_W::new(self)
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn bberrm(&mut self) -> BBERRM_W<8> {
        BBERRM_W::new(self)
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    #[must_use]
    pub fn frmorm(&mut self) -> FRMORM_W<9> {
        FRMORM_W::new(self)
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    #[must_use]
    pub fn dterrm(&mut self) -> DTERRM_W<10> {
        DTERRM_W::new(self)
    }
    #[doc = "Bit 11 - BNAMSK"]
    #[inline(always)]
    #[must_use]
    pub fn bnamsk(&mut self) -> BNAMSK_W<11> {
        BNAMSK_W::new(self)
    }
    #[doc = "Bit 13 - DESCLSTROLLMSK"]
    #[inline(always)]
    #[must_use]
    pub fn desclstrollmsk(&mut self) -> DESCLSTROLLMSK_W<13> {
        DESCLSTROLLMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk12](index.html) module"]
pub struct OTG_HCINTMSK12_SPEC;
impl crate::RegisterSpec for OTG_HCINTMSK12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcintmsk12::R](R) reader structure"]
impl crate::Readable for OTG_HCINTMSK12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk12::W](W) writer structure"]
impl crate::Writable for OTG_HCINTMSK12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HCINTMSK12 to value 0"]
impl crate::Resettable for OTG_HCINTMSK12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
