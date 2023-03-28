#[doc = "Register `ADC_CR` reader"]
pub struct R(crate::R<ADC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CR` writer"]
pub struct W(crate::W<ADC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CR_SPEC>;
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
impl From<crate::W<ADC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader<bool>;
#[doc = "Field `ADEN` writer - ADEN"]
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader<bool>;
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type ADDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader<bool>;
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type ADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `JADSTART` reader - JADSTART"]
pub type JADSTART_R = crate::BitReader<bool>;
#[doc = "Field `JADSTART` writer - JADSTART"]
pub type JADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader<bool>;
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `JADSTP` reader - JADSTP"]
pub type JADSTP_R = crate::BitReader<bool>;
#[doc = "Field `JADSTP` writer - JADSTP"]
pub type JADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `BOOST` reader - BOOST"]
pub type BOOST_R = crate::BitReader<bool>;
#[doc = "Field `BOOST` writer - BOOST"]
pub type BOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADCALLIN` reader - ADCALLIN"]
pub type ADCALLIN_R = crate::BitReader<bool>;
#[doc = "Field `ADCALLIN` writer - ADCALLIN"]
pub type ADCALLIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW1` reader - LINCALRDYW1"]
pub type LINCALRDYW1_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW1` writer - LINCALRDYW1"]
pub type LINCALRDYW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW2` reader - LINCALRDYW2"]
pub type LINCALRDYW2_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW2` writer - LINCALRDYW2"]
pub type LINCALRDYW2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW3` reader - LINCALRDYW3"]
pub type LINCALRDYW3_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW3` writer - LINCALRDYW3"]
pub type LINCALRDYW3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW4` reader - LINCALRDYW4"]
pub type LINCALRDYW4_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW4` writer - LINCALRDYW4"]
pub type LINCALRDYW4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW5` reader - LINCALRDYW5"]
pub type LINCALRDYW5_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW5` writer - LINCALRDYW5"]
pub type LINCALRDYW5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `LINCALRDYW6` reader - LINCALRDYW6"]
pub type LINCALRDYW6_R = crate::BitReader<bool>;
#[doc = "Field `LINCALRDYW6` writer - LINCALRDYW6"]
pub type LINCALRDYW6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::BitReader<bool>;
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `DEEPPWD` reader - DEEPPWD"]
pub type DEEPPWD_R = crate::BitReader<bool>;
#[doc = "Field `DEEPPWD` writer - DEEPPWD"]
pub type DEEPPWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub type ADCALDIF_R = crate::BitReader<bool>;
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub type ADCALDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader<bool>;
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type ADCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<1> {
        ADDIS_W::new(self)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<2> {
        ADSTART_W::new(self)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<3> {
        JADSTART_W::new(self)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<4> {
        ADSTP_W::new(self)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<5> {
        JADSTP_W::new(self)
    }
    #[doc = "Bit 8 - BOOST"]
    #[inline(always)]
    #[must_use]
    pub fn boost(&mut self) -> BOOST_W<8> {
        BOOST_W::new(self)
    }
    #[doc = "Bit 16 - ADCALLIN"]
    #[inline(always)]
    #[must_use]
    pub fn adcallin(&mut self) -> ADCALLIN_W<16> {
        ADCALLIN_W::new(self)
    }
    #[doc = "Bit 22 - LINCALRDYW1"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W<22> {
        LINCALRDYW1_W::new(self)
    }
    #[doc = "Bit 23 - LINCALRDYW2"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W<23> {
        LINCALRDYW2_W::new(self)
    }
    #[doc = "Bit 24 - LINCALRDYW3"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W<24> {
        LINCALRDYW3_W::new(self)
    }
    #[doc = "Bit 25 - LINCALRDYW4"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W<25> {
        LINCALRDYW4_W::new(self)
    }
    #[doc = "Bit 26 - LINCALRDYW5"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W<26> {
        LINCALRDYW5_W::new(self)
    }
    #[doc = "Bit 27 - LINCALRDYW6"]
    #[inline(always)]
    #[must_use]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W<27> {
        LINCALRDYW6_W::new(self)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<28> {
        ADVREGEN_W::new(self)
    }
    #[doc = "Bit 29 - DEEPPWD"]
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<29> {
        DEEPPWD_W::new(self)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<30> {
        ADCALDIF_W::new(self)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<31> {
        ADCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cr](index.html) module"]
pub struct ADC_CR_SPEC;
impl crate::RegisterSpec for ADC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cr::R](R) reader structure"]
impl crate::Readable for ADC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cr::W](W) writer structure"]
impl crate::Writable for ADC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CR to value 0x2000_0000"]
impl crate::Resettable for ADC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
