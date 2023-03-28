#[doc = "Register `USBPHYC_TUNE1` reader"]
pub struct R(crate::R<USBPHYC_TUNE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_TUNE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_TUNE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_TUNE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYC_TUNE1` writer"]
pub struct W(crate::W<USBPHYC_TUNE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_TUNE1_SPEC>;
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
impl From<crate::W<USBPHYC_TUNE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_TUNE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCURREN` reader - INCURREN"]
pub type INCURREN_R = crate::BitReader<bool>;
#[doc = "Field `INCURREN` writer - INCURREN"]
pub type INCURREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `INCURRINT` reader - INCURRINT"]
pub type INCURRINT_R = crate::BitReader<bool>;
#[doc = "Field `INCURRINT` writer - INCURRINT"]
pub type INCURRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `LFSCAPEN` reader - LFSCAPEN"]
pub type LFSCAPEN_R = crate::BitReader<bool>;
#[doc = "Field `LFSCAPEN` writer - LFSCAPEN"]
pub type LFSCAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVSLEW` reader - HSDRVSLEW"]
pub type HSDRVSLEW_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVSLEW` writer - HSDRVSLEW"]
pub type HSDRVSLEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVDCCUR` reader - HSDRVDCCUR"]
pub type HSDRVDCCUR_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVDCCUR` writer - HSDRVDCCUR"]
pub type HSDRVDCCUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVDCLEV` reader - HSDRVDCLEV"]
pub type HSDRVDCLEV_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVDCLEV` writer - HSDRVDCLEV"]
pub type HSDRVDCLEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVCURINCR` reader - HSDRVCURINCR"]
pub type HSDRVCURINCR_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVCURINCR` writer - HSDRVCURINCR"]
pub type HSDRVCURINCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `FSDRVRFADJ` reader - FSDRVRFADJ"]
pub type FSDRVRFADJ_R = crate::BitReader<bool>;
#[doc = "Field `FSDRVRFADJ` writer - FSDRVRFADJ"]
pub type FSDRVRFADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVRFRED` reader - HSDRVRFRED"]
pub type HSDRVRFRED_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVRFRED` writer - HSDRVRFRED"]
pub type HSDRVRFRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSDRVCHKITRM` reader - HSDRVCHKITRM"]
pub type HSDRVCHKITRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSDRVCHKITRM` writer - HSDRVCHKITRM"]
pub type HSDRVCHKITRM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 4, O>;
#[doc = "Field `HSDRVCHKZTRM` reader - HSDRVCHKZTRM"]
pub type HSDRVCHKZTRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSDRVCHKZTRM` writer - HSDRVCHKZTRM"]
pub type HSDRVCHKZTRM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
#[doc = "Field `OTPCOMP` reader - OTPCOMP"]
pub type OTPCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTPCOMP` writer - OTPCOMP"]
pub type OTPCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQLCHCTL` reader - SQLCHCTL"]
pub type SQLCHCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQLCHCTL` writer - SQLCHCTL"]
pub type SQLCHCTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HDRXGNEQEN` reader - HDRXGNEQEN"]
pub type HDRXGNEQEN_R = crate::BitReader<bool>;
#[doc = "Field `HDRXGNEQEN` writer - HDRXGNEQEN"]
pub type HDRXGNEQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `HSRXOFF` reader - HSRXOFF"]
pub type HSRXOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSRXOFF` writer - HSRXOFF"]
pub type HSRXOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSFALLPREEM` reader - HSFALLPREEM"]
pub type HSFALLPREEM_R = crate::BitReader<bool>;
#[doc = "Field `HSFALLPREEM` writer - HSFALLPREEM"]
pub type HSFALLPREEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `SHTCCTCTLPROT` reader - SHTCCTCTLPROT"]
pub type SHTCCTCTLPROT_R = crate::BitReader<bool>;
#[doc = "Field `SHTCCTCTLPROT` writer - SHTCCTCTLPROT"]
pub type SHTCCTCTLPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
#[doc = "Field `STAGSEL` reader - STAGSEL"]
pub type STAGSEL_R = crate::BitReader<bool>;
#[doc = "Field `STAGSEL` writer - STAGSEL"]
pub type STAGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    #[must_use]
    pub fn incurren(&mut self) -> INCURREN_W<0> {
        INCURREN_W::new(self)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    #[must_use]
    pub fn incurrint(&mut self) -> INCURRINT_W<1> {
        INCURRINT_W::new(self)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<2> {
        LFSCAPEN_W::new(self)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<3> {
        HSDRVSLEW_W::new(self)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<4> {
        HSDRVDCCUR_W::new(self)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<5> {
        HSDRVDCLEV_W::new(self)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<6> {
        HSDRVCURINCR_W::new(self)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    #[must_use]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<7> {
        FSDRVRFADJ_W::new(self)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<8> {
        HSDRVRFRED_W::new(self)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<9> {
        HSDRVCHKITRM_W::new(self)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<13> {
        HSDRVCHKZTRM_W::new(self)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    #[must_use]
    pub fn otpcomp(&mut self) -> OTPCOMP_W<15> {
        OTPCOMP_W::new(self)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    #[must_use]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<20> {
        SQLCHCTL_W::new(self)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<22> {
        HDRXGNEQEN_W::new(self)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    #[must_use]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<23> {
        HSRXOFF_W::new(self)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    #[must_use]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<25> {
        HSFALLPREEM_W::new(self)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    #[must_use]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<26> {
        SHTCCTCTLPROT_W::new(self)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    #[must_use]
    pub fn stagsel(&mut self) -> STAGSEL_W<27> {
        STAGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the tune interface of the HS PHY, port #x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_tune1](index.html) module"]
pub struct USBPHYC_TUNE1_SPEC;
impl crate::RegisterSpec for USBPHYC_TUNE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphyc_tune1::R](R) reader structure"]
impl crate::Readable for USBPHYC_TUNE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyc_tune1::W](W) writer structure"]
impl crate::Writable for USBPHYC_TUNE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHYC_TUNE1 to value 0x0407_0004"]
impl crate::Resettable for USBPHYC_TUNE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0407_0004;
}
