#[doc = "Register `I3C_TIMINGR0` reader"]
pub struct R(crate::R<I3C_TIMINGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_TIMINGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_TIMINGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_TIMINGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_TIMINGR0` writer"]
pub struct W(crate::W<I3C_TIMINGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_TIMINGR0_SPEC>;
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
impl From<crate::W<I3C_TIMINGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_TIMINGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL_PP` reader - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL_PP` writer - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TIMINGR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLH_I3C` reader - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH_I3C` writer - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TIMINGR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLL_OD` reader - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL_OD` writer - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TIMINGR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLH_I2C` reader - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH_I2C` writer - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TIMINGR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    pub fn scll_pp(&self) -> SCLL_PP_R {
        SCLL_PP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    pub fn sclh_i3c(&self) -> SCLH_I3C_R {
        SCLH_I3C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    pub fn scll_od(&self) -> SCLL_OD_R {
        SCLL_OD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    pub fn sclh_i2c(&self) -> SCLH_I2C_R {
        SCLH_I2C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    #[must_use]
    pub fn scll_pp(&mut self) -> SCLL_PP_W<0> {
        SCLL_PP_W::new(self)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    #[must_use]
    pub fn sclh_i3c(&mut self) -> SCLH_I3C_W<8> {
        SCLH_I3C_W::new(self)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    #[must_use]
    pub fn scll_od(&mut self) -> SCLL_OD_W<16> {
        SCLL_OD_W::new(self)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    #[must_use]
    pub fn sclh_i2c(&mut self) -> SCLH_I2C_W<24> {
        SCLH_I2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_timingr0](index.html) module"]
pub struct I3C_TIMINGR0_SPEC;
impl crate::RegisterSpec for I3C_TIMINGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_timingr0::R](R) reader structure"]
impl crate::Readable for I3C_TIMINGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_timingr0::W](W) writer structure"]
impl crate::Writable for I3C_TIMINGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_TIMINGR0 to value 0"]
impl crate::Resettable for I3C_TIMINGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
