#[doc = "Register `I2SCFGR` reader"]
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCFGR` writer"]
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader<bool>;
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode others, not used"]
pub type I2SCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SCFG` writer - I2S configuration mode others, not used"]
pub type I2SCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to"]
pub type I2SSTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to"]
pub type I2SSTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `DATLEN` reader - data length to be transferred"]
pub type DATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLEN` writer - data length to be transferred"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHLEN` reader - channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<bool>;
#[doc = "Field `CHLEN` writer - channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `CKPOL` reader - serial audio clock polarity"]
pub type CKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CKPOL` writer - serial audio clock polarity"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `FIXCH` reader - fixed channel length in slave"]
pub type FIXCH_R = crate::BitReader<bool>;
#[doc = "Field `FIXCH` writer - fixed channel length in slave"]
pub type FIXCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `WSINV` reader - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_R = crate::BitReader<bool>;
#[doc = "Field `WSINV` writer - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `DATFMT` reader - data format"]
pub type DATFMT_R = crate::BitReader<bool>;
#[doc = "Field `DATFMT` writer - data format"]
pub type DATFMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `ODD` reader - odd factor for the prescaler Refer to for details"]
pub type ODD_R = crate::BitReader<bool>;
#[doc = "Field `ODD` writer - odd factor for the prescaler Refer to for details"]
pub type ODD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
#[doc = "Field `MCKOE` reader - master clock output enable"]
pub type MCKOE_R = crate::BitReader<bool>;
#[doc = "Field `MCKOE` writer - master clock output enable"]
pub type MCKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<0> {
        I2SMOD_W::new(self)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<1> {
        I2SCFG_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<8> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<10> {
        CHLEN_W::new(self)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<11> {
        CKPOL_W::new(self)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    #[must_use]
    pub fn fixch(&mut self) -> FIXCH_W<12> {
        FIXCH_W::new(self)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    #[must_use]
    pub fn wsinv(&mut self) -> WSINV_W<13> {
        WSINV_W::new(self)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    #[must_use]
    pub fn datfmt(&mut self) -> DATFMT_W<14> {
        DATFMT_W::new(self)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<16> {
        I2SDIV_W::new(self)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<24> {
        ODD_W::new(self)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<25> {
        MCKOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2scfgr::R](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
