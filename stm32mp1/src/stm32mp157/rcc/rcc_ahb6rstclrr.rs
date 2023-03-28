#[doc = "Register `RCC_AHB6RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB6RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB6RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB6RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB6RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB6RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB6RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB6RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB6RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB6RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETHMACRST` reader - ETHMACRST"]
pub type ETHMACRST_R = crate::BitReader<bool>;
#[doc = "Field `ETHMACRST` writer - ETHMACRST"]
pub type ETHMACRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `FMCRST` reader - FMCRST"]
pub type FMCRST_R = crate::BitReader<bool>;
#[doc = "Field `FMCRST` writer - FMCRST"]
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `QSPIRST` reader - QSPIRST"]
pub type QSPIRST_R = crate::BitReader<bool>;
#[doc = "Field `QSPIRST` writer - QSPIRST"]
pub type QSPIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SDMMC1RST` reader - SDMMC1RST"]
pub type SDMMC1RST_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1RST` writer - SDMMC1RST"]
pub type SDMMC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `SDMMC2RST` reader - SDMMC2RST"]
pub type SDMMC2RST_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2RST` writer - SDMMC2RST"]
pub type SDMMC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `CRC1RST` reader - CRC1RST"]
pub type CRC1RST_R = crate::BitReader<bool>;
#[doc = "Field `CRC1RST` writer - CRC1RST"]
pub type CRC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
#[doc = "Field `USBHRST` reader - USBHRST"]
pub type USBHRST_R = crate::BitReader<bool>;
#[doc = "Field `USBHRST` writer - USBHRST"]
pub type USBHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB6RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&self) -> CRC1RST_R {
        CRC1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&self) -> USBHRST_R {
        USBHRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    #[must_use]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<10> {
        ETHMACRST_W::new(self)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<12> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<14> {
        QSPIRST_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<16> {
        SDMMC1RST_W::new(self)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<17> {
        SDMMC2RST_W::new(self)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn crc1rst(&mut self) -> CRC1RST_W<20> {
        CRC1RST_W::new(self)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbhrst(&mut self) -> USBHRST_W<24> {
        USBHRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb6rstclrr](index.html) module"]
pub struct RCC_AHB6RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB6RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb6rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB6RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb6rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB6RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB6RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB6RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
