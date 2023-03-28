#[doc = "Register `ADC_CFGR2` reader"]
pub struct R(crate::R<ADC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFGR2` writer"]
pub struct W(crate::W<ADC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFGR2_SPEC>;
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
impl From<crate::W<ADC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROVSE` reader - ROVSE"]
pub type ROVSE_R = crate::BitReader<bool>;
#[doc = "Field `ROVSE` writer - ROVSE"]
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `JOVSE` reader - JOVSE"]
pub type JOVSE_R = crate::BitReader<bool>;
#[doc = "Field `JOVSE` writer - JOVSE"]
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `OVSS` reader - OVSS"]
pub type OVSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVSS` writer - OVSS"]
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TROVS` reader - TROVS"]
pub type TROVS_R = crate::BitReader<bool>;
#[doc = "Field `TROVS` writer - TROVS"]
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `ROVSM` reader - ROVSM"]
pub type ROVSM_R = crate::BitReader<bool>;
#[doc = "Field `ROVSM` writer - ROVSM"]
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `RSHIFT1` reader - RSHIFT1"]
pub type RSHIFT1_R = crate::BitReader<bool>;
#[doc = "Field `RSHIFT1` writer - RSHIFT1"]
pub type RSHIFT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `RSHIFT2` reader - RSHIFT2"]
pub type RSHIFT2_R = crate::BitReader<bool>;
#[doc = "Field `RSHIFT2` writer - RSHIFT2"]
pub type RSHIFT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `RSHIFT3` reader - RSHIFT3"]
pub type RSHIFT3_R = crate::BitReader<bool>;
#[doc = "Field `RSHIFT3` writer - RSHIFT3"]
pub type RSHIFT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `RSHIFT4` reader - RSHIFT4"]
pub type RSHIFT4_R = crate::BitReader<bool>;
#[doc = "Field `RSHIFT4` writer - RSHIFT4"]
pub type RSHIFT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFGR2_SPEC, bool, O>;
#[doc = "Field `OSVR` reader - OSVR"]
pub type OSVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OSVR` writer - OSVR"]
pub type OSVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CFGR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `LSHIFT` reader - LSHIFT"]
pub type LSHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSHIFT` writer - LSHIFT"]
pub type LSHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_CFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - ROVSE"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - JOVSE"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:8 - OVSS"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - TROVS"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ROVSM"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RSHIFT1"]
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSHIFT2"]
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RSHIFT3"]
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RSHIFT4"]
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - OSVR"]
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - LSHIFT"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ROVSE"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    #[doc = "Bit 1 - JOVSE"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    #[doc = "Bits 5:8 - OVSS"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - TROVS"]
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<9> {
        TROVS_W::new(self)
    }
    #[doc = "Bit 10 - ROVSM"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<10> {
        ROVSM_W::new(self)
    }
    #[doc = "Bit 11 - RSHIFT1"]
    #[inline(always)]
    #[must_use]
    pub fn rshift1(&mut self) -> RSHIFT1_W<11> {
        RSHIFT1_W::new(self)
    }
    #[doc = "Bit 12 - RSHIFT2"]
    #[inline(always)]
    #[must_use]
    pub fn rshift2(&mut self) -> RSHIFT2_W<12> {
        RSHIFT2_W::new(self)
    }
    #[doc = "Bit 13 - RSHIFT3"]
    #[inline(always)]
    #[must_use]
    pub fn rshift3(&mut self) -> RSHIFT3_W<13> {
        RSHIFT3_W::new(self)
    }
    #[doc = "Bit 14 - RSHIFT4"]
    #[inline(always)]
    #[must_use]
    pub fn rshift4(&mut self) -> RSHIFT4_W<14> {
        RSHIFT4_W::new(self)
    }
    #[doc = "Bits 16:25 - OSVR"]
    #[inline(always)]
    #[must_use]
    pub fn osvr(&mut self) -> OSVR_W<16> {
        OSVR_W::new(self)
    }
    #[doc = "Bits 28:31 - LSHIFT"]
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LSHIFT_W<28> {
        LSHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr2](index.html) module"]
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfgr2::R](R) reader structure"]
impl crate::Readable for ADC_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfgr2::W](W) writer structure"]
impl crate::Writable for ADC_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for ADC_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
