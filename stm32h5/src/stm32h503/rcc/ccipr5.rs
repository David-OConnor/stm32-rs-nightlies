#[doc = "Register `CCIPR5` reader"]
pub struct R(crate::R<CCIPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR5` writer"]
pub struct W(crate::W<CCIPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR5_SPEC>;
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
impl From<crate::W<CCIPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCDACSEL` reader - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCDACSEL` writer - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 3, O>;
#[doc = "Field `DACSEL` reader - DAC hold clock"]
pub type DACSEL_R = crate::BitReader<bool>;
#[doc = "Field `DACSEL` writer - DAC hold clock"]
pub type DACSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR5_SPEC, bool, O>;
#[doc = "Field `RNGSEL` reader - RNG kernel clock source selection"]
pub type RNGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNGSEL` writer - RNG kernel clock source selection"]
pub type RNGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
#[doc = "Field `FDCAN1SEL` reader - FDCAN1 kernel clock source selection"]
pub type FDCAN1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDCAN1SEL` writer - FDCAN1 kernel clock source selection"]
pub type FDCAN1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKPERSEL` reader - per_ck clock source selection"]
pub type CKPERSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKPERSEL` writer - per_ck clock source selection"]
pub type CKPERSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR5_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn adcdacsel(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DAC hold clock"]
    #[inline(always)]
    pub fn dacsel(&self) -> DACSEL_R {
        DACSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    pub fn fdcan1sel(&self) -> FDCAN1SEL_R {
        FDCAN1SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn adcdacsel(&mut self) -> ADCDACSEL_W<0> {
        ADCDACSEL_W::new(self)
    }
    #[doc = "Bit 3 - DAC hold clock"]
    #[inline(always)]
    #[must_use]
    pub fn dacsel(&mut self) -> DACSEL_W<3> {
        DACSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<4> {
        RNGSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sel(&mut self) -> FDCAN1SEL_W<8> {
        FDCAN1SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<30> {
        CKPERSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr5](index.html) module"]
pub struct CCIPR5_SPEC;
impl crate::RegisterSpec for CCIPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr5::R](R) reader structure"]
impl crate::Readable for CCIPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr5::W](W) writer structure"]
impl crate::Writable for CCIPR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR5 to value 0"]
impl crate::Resettable for CCIPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
