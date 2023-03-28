#[doc = "Register `I2C_TIMINGR` reader"]
pub struct R(crate::R<I2C_TIMINGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TIMINGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TIMINGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TIMINGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TIMINGR` writer"]
pub struct W(crate::W<I2C_TIMINGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TIMINGR_SPEC>;
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
impl From<crate::W<I2C_TIMINGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TIMINGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - SCLL"]
pub type SCLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL` writer - SCLL"]
pub type SCLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_TIMINGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLH` reader - SCLH"]
pub type SCLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH` writer - SCLH"]
pub type SCLH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_TIMINGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDADEL` reader - SDADEL"]
pub type SDADEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDADEL` writer - SDADEL"]
pub type SDADEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_TIMINGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCLDEL` reader - SCLDEL"]
pub type SCLDEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLDEL` writer - SCLDEL"]
pub type SCLDEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_TIMINGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRESC` reader - PRESC"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - PRESC"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2C_TIMINGR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<0> {
        SCLL_W::new(self)
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<8> {
        SCLH_W::new(self)
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SDADEL_W<16> {
        SDADEL_W::new(self)
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> SCLDEL_W<20> {
        SCLDEL_W::new(self)
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<28> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timingr](index.html) module"]
pub struct I2C_TIMINGR_SPEC;
impl crate::RegisterSpec for I2C_TIMINGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_timingr::R](R) reader structure"]
impl crate::Readable for I2C_TIMINGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_timingr::W](W) writer structure"]
impl crate::Writable for I2C_TIMINGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_TIMINGR to value 0"]
impl crate::Resettable for I2C_TIMINGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
