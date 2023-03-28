#[doc = "Register `GADPCTL` reader"]
pub struct R(crate::R<GADPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GADPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GADPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GADPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GADPCTL` writer"]
pub struct W(crate::W<GADPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GADPCTL_SPEC>;
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
impl From<crate::W<GADPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GADPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRBDSCHG` reader - Probe discharge"]
pub type PRBDSCHG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRBDSCHG` writer - Probe discharge"]
pub type PRBDSCHG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GADPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRBPER` reader - Probe period"]
pub type PRBPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRBPER` writer - Probe period"]
pub type PRBPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GADPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRBDELTA` reader - Probe delta"]
pub type PRBDELTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRBDELTA` writer - Probe delta"]
pub type PRBDELTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GADPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTIM` reader - Ramp time"]
pub type RTIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTIM` writer - Ramp time"]
pub type RTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GADPCTL_SPEC, u16, u16, 11, O>;
#[doc = "Field `ENAPRB` reader - Enable probe"]
pub type ENAPRB_R = crate::BitReader<bool>;
#[doc = "Field `ENAPRB` writer - Enable probe"]
pub type ENAPRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ENASNS` reader - Enable sense"]
pub type ENASNS_R = crate::BitReader<bool>;
#[doc = "Field `ENASNS` writer - Enable sense"]
pub type ENASNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPRST` reader - ADP reset"]
pub type ADPRST_R = crate::BitReader<bool>;
#[doc = "Field `ADPRST` writer - ADP reset"]
pub type ADPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPEN` reader - ADP enable"]
pub type ADPEN_R = crate::BitReader<bool>;
#[doc = "Field `ADPEN` writer - ADP enable"]
pub type ADPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPPRBIF` reader - ADP probe interrupt flag"]
pub type ADPPRBIF_R = crate::BitReader<bool>;
#[doc = "Field `ADPPRBIF` writer - ADP probe interrupt flag"]
pub type ADPPRBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPSNSIF` reader - ADP sense interrupt flag"]
pub type ADPSNSIF_R = crate::BitReader<bool>;
#[doc = "Field `ADPSNSIF` writer - ADP sense interrupt flag"]
pub type ADPSNSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPTOIF` reader - ADP timeout interrupt flag"]
pub type ADPTOIF_R = crate::BitReader<bool>;
#[doc = "Field `ADPTOIF` writer - ADP timeout interrupt flag"]
pub type ADPTOIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPPRBIM` reader - ADP probe interrupt mask"]
pub type ADPPRBIM_R = crate::BitReader<bool>;
#[doc = "Field `ADPPRBIM` writer - ADP probe interrupt mask"]
pub type ADPPRBIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPSNSIM` reader - ADP sense interrupt mask"]
pub type ADPSNSIM_R = crate::BitReader<bool>;
#[doc = "Field `ADPSNSIM` writer - ADP sense interrupt mask"]
pub type ADPSNSIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `ADPTOIM` reader - ADP timeout interrupt mask"]
pub type ADPTOIM_R = crate::BitReader<bool>;
#[doc = "Field `ADPTOIM` writer - ADP timeout interrupt mask"]
pub type ADPTOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GADPCTL_SPEC, bool, O>;
#[doc = "Field `AR` reader - Access request"]
pub type AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AR` writer - Access request"]
pub type AR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GADPCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    pub fn prbdschg(&self) -> PRBDSCHG_R {
        PRBDSCHG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 0:3 - Probe period"]
    #[inline(always)]
    pub fn prbper(&self) -> PRBPER_R {
        PRBPER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    pub fn prbdelta(&self) -> PRBDELTA_R {
        PRBDELTA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:16 - Ramp time"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    pub fn enaprb(&self) -> ENAPRB_R {
        ENAPRB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    pub fn enasns(&self) -> ENASNS_R {
        ENASNS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADP reset"]
    #[inline(always)]
    pub fn adprst(&self) -> ADPRST_R {
        ADPRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    pub fn adpen(&self) -> ADPEN_R {
        ADPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprbif(&self) -> ADPPRBIF_R {
        ADPPRBIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnsif(&self) -> ADPSNSIF_R {
        ADPSNSIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptoif(&self) -> ADPTOIF_R {
        ADPTOIF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    pub fn adpprbim(&self) -> ADPPRBIM_R {
        ADPPRBIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    pub fn adpsnsim(&self) -> ADPSNSIM_R {
        ADPSNSIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    pub fn adptoim(&self) -> ADPTOIM_R {
        ADPTOIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    #[must_use]
    pub fn prbdschg(&mut self) -> PRBDSCHG_W<0> {
        PRBDSCHG_W::new(self)
    }
    #[doc = "Bits 0:3 - Probe period"]
    #[inline(always)]
    #[must_use]
    pub fn prbper(&mut self) -> PRBPER_W<0> {
        PRBPER_W::new(self)
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    #[must_use]
    pub fn prbdelta(&mut self) -> PRBDELTA_W<2> {
        PRBDELTA_W::new(self)
    }
    #[doc = "Bits 6:16 - Ramp time"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<6> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    #[must_use]
    pub fn enaprb(&mut self) -> ENAPRB_W<17> {
        ENAPRB_W::new(self)
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    #[must_use]
    pub fn enasns(&mut self) -> ENASNS_W<18> {
        ENASNS_W::new(self)
    }
    #[doc = "Bit 19 - ADP reset"]
    #[inline(always)]
    #[must_use]
    pub fn adprst(&mut self) -> ADPRST_W<19> {
        ADPRST_W::new(self)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpen(&mut self) -> ADPEN_W<20> {
        ADPEN_W::new(self)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpprbif(&mut self) -> ADPPRBIF_W<21> {
        ADPPRBIF_W::new(self)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnsif(&mut self) -> ADPSNSIF_W<22> {
        ADPSNSIF_W::new(self)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adptoif(&mut self) -> ADPTOIF_W<23> {
        ADPTOIF_W::new(self)
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adpprbim(&mut self) -> ADPPRBIM_W<24> {
        ADPPRBIM_W::new(self)
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnsim(&mut self) -> ADPSNSIM_W<25> {
        ADPSNSIM_W::new(self)
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adptoim(&mut self) -> ADPTOIM_W<26> {
        ADPTOIM_W::new(self)
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<27> {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG ADP timer, control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gadpctl](index.html) module"]
pub struct GADPCTL_SPEC;
impl crate::RegisterSpec for GADPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gadpctl::R](R) reader structure"]
impl crate::Readable for GADPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gadpctl::W](W) writer structure"]
impl crate::Writable for GADPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GADPCTL to value 0"]
impl crate::Resettable for GADPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
