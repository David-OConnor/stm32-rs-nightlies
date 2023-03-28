#[doc = "Register `CLKCR` reader"]
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCR` writer"]
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - Clock divide factor"]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - Clock divide factor"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `PWRSAV` reader - Power saving configuration bit"]
pub type PWRSAV_R = crate::BitReader<bool>;
#[doc = "Field `PWRSAV` writer - Power saving configuration bit"]
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit"]
pub type WIDBUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit"]
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NEGEDGE` reader - SDMMC_CK dephasing selection bit for data and command"]
pub type NEGEDGE_R = crate::BitReader<bool>;
#[doc = "Field `NEGEDGE` writer - SDMMC_CK dephasing selection bit for data and command"]
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `HWFC_EN` reader - Hardware flow control enable"]
pub type HWFC_EN_R = crate::BitReader<bool>;
#[doc = "Field `HWFC_EN` writer - Hardware flow control enable"]
pub type HWFC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `DDR` reader - Data rate signaling selection"]
pub type DDR_R = crate::BitReader<bool>;
#[doc = "Field `DDR` writer - Data rate signaling selection"]
pub type DDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `BUSSPEED` reader - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
pub type BUSSPEED_R = crate::BitReader<bool>;
#[doc = "Field `BUSSPEED` writer - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
pub type BUSSPEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `SELCLKRX` reader - Receive clock selection"]
pub type SELCLKRX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELCLKRX` writer - Receive clock selection"]
pub type SELCLKRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and command"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data rate signaling selection"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Receive clock selection"]
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 12 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<12> {
        PWRSAV_W::new(self)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<14> {
        WIDBUS_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and command"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<16> {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 17 - Hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<17> {
        HWFC_EN_W::new(self)
    }
    #[doc = "Bit 18 - Data rate signaling selection"]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<18> {
        DDR_W::new(self)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50,DDR50"]
    #[inline(always)]
    #[must_use]
    pub fn busspeed(&mut self) -> BUSSPEED_W<19> {
        BUSSPEED_W::new(self)
    }
    #[doc = "Bits 20:21 - Receive clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn selclkrx(&mut self) -> SELCLKRX_W<20> {
        SELCLKRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](index.html) module"]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcr::R](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcr::W](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
