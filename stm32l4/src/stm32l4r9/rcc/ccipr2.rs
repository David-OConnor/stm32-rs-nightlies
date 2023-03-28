#[doc = "Register `CCIPR2` reader"]
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR2` writer"]
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
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
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub type I2C4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub type I2C4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DFSDMSEL` reader - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMSEL` writer - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, bool, O>;
#[doc = "Field `ADFSDMSEL` reader - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADFSDMSEL` writer - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub type SAI2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub type SAI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DSISEL` reader - clock selection"]
pub type DSISEL_R = crate::BitReader<bool>;
#[doc = "Field `DSISEL` writer - clock selection"]
pub type DSISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, bool, O>;
#[doc = "Field `SDMMCSEL` reader - SDMMC clock selection"]
pub type SDMMCSEL_R = crate::BitReader<bool>;
#[doc = "Field `SDMMCSEL` writer - SDMMC clock selection"]
pub type SDMMCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, bool, O>;
#[doc = "Field `PLLSAI2DIVR` reader - division factor for LTDC clock"]
pub type PLLSAI2DIVR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI2DIVR` writer - division factor for LTDC clock"]
pub type PLLSAI2DIVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSPISEL` reader - Octospi clock source selection"]
pub type OSPISEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSPISEL` writer - Octospi clock source selection"]
pub type OSPISEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<0> {
        I2C4SEL_W::new(self)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<2> {
        DFSDMSEL_W::new(self)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W<3> {
        ADFSDMSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<5> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<8> {
        SAI2SEL_W::new(self)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsisel(&mut self) -> DSISEL_W<12> {
        DSISEL_W::new(self)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<14> {
        SDMMCSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W<16> {
        PLLSAI2DIVR_W::new(self)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ospisel(&mut self) -> OSPISEL_W<20> {
        OSPISEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr2::R](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr2::W](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
