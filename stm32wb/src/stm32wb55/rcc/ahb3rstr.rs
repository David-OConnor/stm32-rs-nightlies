#[doc = "Register `AHB3RSTR` reader"]
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3RSTR` writer"]
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPIRST` reader - Quad SPI memory interface reset"]
pub type QSPIRST_R = crate::BitReader<bool>;
#[doc = "Field `QSPIRST` writer - Quad SPI memory interface reset"]
pub type QSPIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `PKARST` reader - PKA interface reset"]
pub type PKARST_R = crate::BitReader<bool>;
#[doc = "Field `PKARST` writer - PKA interface reset"]
pub type PKARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `AES2RST` reader - AES2 interface reset"]
pub type AES2RST_R = crate::BitReader<bool>;
#[doc = "Field `AES2RST` writer - AES2 interface reset"]
pub type AES2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `RNGRST` reader - RNG interface reset"]
pub type RNGRST_R = crate::BitReader<bool>;
#[doc = "Field `RNGRST` writer - RNG interface reset"]
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `HSEMRST` reader - HSEM interface reset"]
pub type HSEMRST_R = crate::BitReader<bool>;
#[doc = "Field `HSEMRST` writer - HSEM interface reset"]
pub type HSEMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `IPCCRST` reader - IPCC interface reset"]
pub type IPCCRST_R = crate::BitReader<bool>;
#[doc = "Field `IPCCRST` writer - IPCC interface reset"]
pub type IPCCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
#[doc = "Field `FLASHRST` reader - Flash interface reset"]
pub type FLASHRST_R = crate::BitReader<bool>;
#[doc = "Field `FLASHRST` writer - Flash interface reset"]
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - PKA interface reset"]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES2 interface reset"]
    #[inline(always)]
    pub fn aes2rst(&self) -> AES2RST_R {
        AES2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG interface reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSEM interface reset"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IPCC interface reset"]
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Quad SPI memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<8> {
        QSPIRST_W::new(self)
    }
    #[doc = "Bit 16 - PKA interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<16> {
        PKARST_W::new(self)
    }
    #[doc = "Bit 17 - AES2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn aes2rst(&mut self) -> AES2RST_W<17> {
        AES2RST_W::new(self)
    }
    #[doc = "Bit 18 - RNG interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 19 - HSEM interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<19> {
        HSEMRST_W::new(self)
    }
    #[doc = "Bit 20 - IPCC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn ipccrst(&mut self) -> IPCCRST_W<20> {
        IPCCRST_W::new(self)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<25> {
        FLASHRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](index.html) module"]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3rstr::R](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
