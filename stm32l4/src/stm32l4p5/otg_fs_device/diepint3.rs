#[doc = "Register `DIEPINT3` reader"]
pub struct R(crate::R<DIEPINT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT3` writer"]
pub struct W(crate::W<DIEPINT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT3_SPEC>;
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
impl From<crate::W<DIEPINT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::BitReader<bool>;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type ITTXFE_R = crate::BitReader<bool>;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type ITTXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `INEPNM` reader - IN token received with EP mismatch"]
pub type INEPNM_R = crate::BitReader<bool>;
#[doc = "Field `INEPNM` writer - IN token received with EP mismatch"]
pub type INEPNM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type INEPNE_R = crate::BitReader<bool>;
#[doc = "Field `INEPNE` writer - INEPNE"]
pub type INEPNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub type PKTDRPSTS_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub type PKTDRPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK input"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK input"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch"]
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<3> {
        TOC_W::new(self)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<4> {
        ITTXFE_W::new(self)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn inepnm(&mut self) -> INEPNM_W<5> {
        INEPNM_W::new(self)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    #[must_use]
    pub fn inepne(&mut self) -> INEPNE_W<6> {
        INEPNE_W::new(self)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<11> {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<13> {
        NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint3](index.html) module"]
pub struct DIEPINT3_SPEC;
impl crate::RegisterSpec for DIEPINT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint3::R](R) reader structure"]
impl crate::Readable for DIEPINT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint3::W](W) writer structure"]
impl crate::Writable for DIEPINT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT3 to value 0x80"]
impl crate::Resettable for DIEPINT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}