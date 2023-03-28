#[doc = "Register `PMC` reader"]
pub struct R(crate::R<PMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC` writer"]
pub struct W(crate::W<PMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SPEC>;
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
impl From<crate::W<PMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1_FMP I2C1 Fast Mode + Enable"]
pub type I2C1_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_FMP` writer - I2C1_FMP I2C1 Fast Mode + Enable"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `I2C2_FMP` reader - I2C2_FMP I2C2 Fast Mode + Enable"]
pub type I2C2_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C2_FMP` writer - I2C2_FMP I2C2 Fast Mode + Enable"]
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP I2C3 Fast Mode + Enable"]
pub type I2C3_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP I2C3 Fast Mode + Enable"]
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `PB6_FMP` reader - PB6_FMP Fast Mode"]
pub type PB6_FMP_R = crate::BitReader<bool>;
#[doc = "Field `PB6_FMP` writer - PB6_FMP Fast Mode"]
pub type PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `PB7_FMP` reader - PB7_FMP Fast Mode + Enable"]
pub type PB7_FMP_R = crate::BitReader<bool>;
#[doc = "Field `PB7_FMP` writer - PB7_FMP Fast Mode + Enable"]
pub type PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `PB8_FMP` reader - PB8_FMP Fast Mode + Enable"]
pub type PB8_FMP_R = crate::BitReader<bool>;
#[doc = "Field `PB8_FMP` writer - PB8_FMP Fast Mode + Enable"]
pub type PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `PB9_FMP` reader - Fast Mode + Enable"]
pub type PB9_FMP_R = crate::BitReader<bool>;
#[doc = "Field `PB9_FMP` writer - Fast Mode + Enable"]
pub type PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `ADC1DC2` reader - ADC3DC2"]
pub type ADC1DC2_R = crate::BitReader<bool>;
#[doc = "Field `ADC1DC2` writer - ADC3DC2"]
pub type ADC1DC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `ADC2DC2` reader - ADC2DC2"]
pub type ADC2DC2_R = crate::BitReader<bool>;
#[doc = "Field `ADC2DC2` writer - ADC2DC2"]
pub type ADC2DC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `ADC3DC2` reader - ADC3DC2"]
pub type ADC3DC2_R = crate::BitReader<bool>;
#[doc = "Field `ADC3DC2` writer - ADC3DC2"]
pub type ADC3DC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC3DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<0> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<1> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<2> {
        I2C3_FMP_W::new(self)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<4> {
        PB6_FMP_W::new(self)
    }
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<5> {
        PB7_FMP_W::new(self)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<6> {
        PB8_FMP_W::new(self)
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W<7> {
        PB9_FMP_W::new(self)
    }
    #[doc = "Bit 16 - ADC3DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W<16> {
        ADC1DC2_W::new(self)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W<17> {
        ADC2DC2_W::new(self)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W<18> {
        ADC3DC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc](index.html) module"]
pub struct PMC_SPEC;
impl crate::RegisterSpec for PMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc::R](R) reader structure"]
impl crate::Readable for PMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc::W](W) writer structure"]
impl crate::Writable for PMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
