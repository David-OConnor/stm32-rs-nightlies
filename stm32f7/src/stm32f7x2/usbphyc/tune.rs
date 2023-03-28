#[doc = "Register `TUNE` reader"]
pub struct R(crate::R<TUNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TUNE` writer"]
pub struct W(crate::W<TUNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUNE_SPEC>;
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
impl From<crate::W<TUNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCURREN` reader - Controls the current boosting function"]
pub type INCURREN_R = crate::BitReader<bool>;
#[doc = "Field `INCURREN` writer - Controls the current boosting function"]
pub type INCURREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `INCURRINT` reader - Controls PHY current boosting"]
pub type INCURRINT_R = crate::BitReader<bool>;
#[doc = "Field `INCURRINT` writer - Controls PHY current boosting"]
pub type INCURRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `LFSCAPEN` reader - : Enables the Low Full Speed feedback capacitor"]
pub type LFSCAPEN_R = crate::BitReader<bool>;
#[doc = "Field `LFSCAPEN` writer - : Enables the Low Full Speed feedback capacitor"]
pub type LFSCAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVSLEW` reader - Controls the HS driver slew rate"]
pub type HSDRVSLEW_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVSLEW` writer - Controls the HS driver slew rate"]
pub type HSDRVSLEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVDCCUR` reader - Decreases the HS driver DC level"]
pub type HSDRVDCCUR_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVDCCUR` writer - Decreases the HS driver DC level"]
pub type HSDRVDCCUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVDCLEV` reader - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer"]
pub type HSDRVDCLEV_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVDCLEV` writer - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer"]
pub type HSDRVDCLEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVCURINCR` reader - Enable the HS driver current increase feature"]
pub type HSDRVCURINCR_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVCURINCR` writer - Enable the HS driver current increase feature"]
pub type HSDRVCURINCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `FSDRVRFADJ` reader - Tuning pin to adjust the full speed rise/fall time"]
pub type FSDRVRFADJ_R = crate::BitReader<bool>;
#[doc = "Field `FSDRVRFADJ` writer - Tuning pin to adjust the full speed rise/fall time"]
pub type FSDRVRFADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVRFRED` reader - High Speed rise-fall reduction enable"]
pub type HSDRVRFRED_R = crate::BitReader<bool>;
#[doc = "Field `HSDRVRFRED` writer - High Speed rise-fall reduction enable"]
pub type HSDRVRFRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSDRVCHKITRM` reader - HS Driver current trimming pins for choke compensation"]
pub type HSDRVCHKITRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSDRVCHKITRM` writer - HS Driver current trimming pins for choke compensation"]
pub type HSDRVCHKITRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_SPEC, u8, u8, 4, O>;
#[doc = "Field `HSDRVCHKZTRM` reader - Controls the PHY bus HS driver impedance tuning for choke compensation"]
pub type HSDRVCHKZTRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSDRVCHKZTRM` writer - Controls the PHY bus HS driver impedance tuning for choke compensation"]
pub type HSDRVCHKZTRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_SPEC, u8, u8, 2, O>;
#[doc = "Field `SQLCHCTL` reader - Adjust the squelch DC threshold value"]
pub type SQLCHCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQLCHCTL` writer - Adjust the squelch DC threshold value"]
pub type SQLCHCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_SPEC, u8, u8, 2, O>;
#[doc = "Field `HDRXGNEQEN` reader - Enables the HS Rx Gain Equalizer"]
pub type HDRXGNEQEN_R = crate::BitReader<bool>;
#[doc = "Field `HDRXGNEQEN` writer - Enables the HS Rx Gain Equalizer"]
pub type HDRXGNEQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `STAGSEL` reader - HS Tx staggering enable"]
pub type STAGSEL_R = crate::BitReader<bool>;
#[doc = "Field `STAGSEL` writer - HS Tx staggering enable"]
pub type STAGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSFALLPREEM` reader - HS Fall time control of single ended signals during pre-emphasis"]
pub type HSFALLPREEM_R = crate::BitReader<bool>;
#[doc = "Field `HSFALLPREEM` writer - HS Fall time control of single ended signals during pre-emphasis"]
pub type HSFALLPREEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `HSRXOFF` reader - : HS Receiver Offset adjustment"]
pub type HSRXOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSRXOFF` writer - : HS Receiver Offset adjustment"]
pub type HSRXOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_SPEC, u8, u8, 2, O>;
#[doc = "Field `SHTCCTCTLPROT` reader - Enables the short circuit protection circuitry in LS/FS driver"]
pub type SHTCCTCTLPROT_R = crate::BitReader<bool>;
#[doc = "Field `SHTCCTCTLPROT` writer - Enables the short circuit protection circuitry in LS/FS driver"]
pub type SHTCCTCTLPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
#[doc = "Field `SQLBYP` reader - This pin is used to bypass the squelch inter-locking circuitry"]
pub type SQLBYP_R = crate::BitReader<bool>;
#[doc = "Field `SQLBYP` writer - This pin is used to bypass the squelch inter-locking circuitry"]
pub type SQLBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TUNE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Controls the current boosting function"]
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls PHY current boosting"]
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - : Enables the Low Full Speed feedback capacitor"]
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the HS driver slew rate"]
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Decreases the HS driver DC level"]
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer"]
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the HS driver current increase feature"]
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tuning pin to adjust the full speed rise/fall time"]
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Speed rise-fall reduction enable"]
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - HS Driver current trimming pins for choke compensation"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - Controls the PHY bus HS driver impedance tuning for choke compensation"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Adjust the squelch DC threshold value"]
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Enables the HS Rx Gain Equalizer"]
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HS Tx staggering enable"]
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HS Fall time control of single ended signals during pre-emphasis"]
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - : HS Receiver Offset adjustment"]
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Enables the short circuit protection circuitry in LS/FS driver"]
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This pin is used to bypass the squelch inter-locking circuitry"]
    #[inline(always)]
    pub fn sqlbyp(&self) -> SQLBYP_R {
        SQLBYP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the current boosting function"]
    #[inline(always)]
    #[must_use]
    pub fn incurren(&mut self) -> INCURREN_W<0> {
        INCURREN_W::new(self)
    }
    #[doc = "Bit 1 - Controls PHY current boosting"]
    #[inline(always)]
    #[must_use]
    pub fn incurrint(&mut self) -> INCURRINT_W<1> {
        INCURRINT_W::new(self)
    }
    #[doc = "Bit 2 - : Enables the Low Full Speed feedback capacitor"]
    #[inline(always)]
    #[must_use]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<2> {
        LFSCAPEN_W::new(self)
    }
    #[doc = "Bit 3 - Controls the HS driver slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<3> {
        HSDRVSLEW_W::new(self)
    }
    #[doc = "Bit 4 - Decreases the HS driver DC level"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<4> {
        HSDRVDCCUR_W::new(self)
    }
    #[doc = "Bit 5 - Increases the HS Driver DC level. Not applicable during the HS Test J and Test K data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<5> {
        HSDRVDCLEV_W::new(self)
    }
    #[doc = "Bit 6 - Enable the HS driver current increase feature"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<6> {
        HSDRVCURINCR_W::new(self)
    }
    #[doc = "Bit 7 - Tuning pin to adjust the full speed rise/fall time"]
    #[inline(always)]
    #[must_use]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<7> {
        FSDRVRFADJ_W::new(self)
    }
    #[doc = "Bit 8 - High Speed rise-fall reduction enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<8> {
        HSDRVRFRED_W::new(self)
    }
    #[doc = "Bits 9:12 - HS Driver current trimming pins for choke compensation"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<9> {
        HSDRVCHKITRM_W::new(self)
    }
    #[doc = "Bits 13:14 - Controls the PHY bus HS driver impedance tuning for choke compensation"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<13> {
        HSDRVCHKZTRM_W::new(self)
    }
    #[doc = "Bits 15:16 - Adjust the squelch DC threshold value"]
    #[inline(always)]
    #[must_use]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<15> {
        SQLCHCTL_W::new(self)
    }
    #[doc = "Bit 17 - Enables the HS Rx Gain Equalizer"]
    #[inline(always)]
    #[must_use]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<17> {
        HDRXGNEQEN_W::new(self)
    }
    #[doc = "Bit 18 - HS Tx staggering enable"]
    #[inline(always)]
    #[must_use]
    pub fn stagsel(&mut self) -> STAGSEL_W<18> {
        STAGSEL_W::new(self)
    }
    #[doc = "Bit 19 - HS Fall time control of single ended signals during pre-emphasis"]
    #[inline(always)]
    #[must_use]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<19> {
        HSFALLPREEM_W::new(self)
    }
    #[doc = "Bits 20:21 - : HS Receiver Offset adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<20> {
        HSRXOFF_W::new(self)
    }
    #[doc = "Bit 22 - Enables the short circuit protection circuitry in LS/FS driver"]
    #[inline(always)]
    #[must_use]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<22> {
        SHTCCTCTLPROT_W::new(self)
    }
    #[doc = "Bit 23 - This pin is used to bypass the squelch inter-locking circuitry"]
    #[inline(always)]
    #[must_use]
    pub fn sqlbyp(&mut self) -> SQLBYP_W<23> {
        SQLBYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBPHYC tuning control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tune](index.html) module"]
pub struct TUNE_SPEC;
impl crate::RegisterSpec for TUNE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tune::R](R) reader structure"]
impl crate::Readable for TUNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tune::W](W) writer structure"]
impl crate::Writable for TUNE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNE to value 0x04"]
impl crate::Resettable for TUNE_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
