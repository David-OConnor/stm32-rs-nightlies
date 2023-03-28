#[doc = "Register `CONF2R` reader"]
pub struct R(crate::R<CONF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF2R` writer"]
pub struct W(crate::W<CONF2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF2R_SPEC>;
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
impl From<crate::W<CONF2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET2` reader - Twelve-bit calibration offset for configuration 2"]
pub type OFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET2` writer - Twelve-bit calibration offset for configuration 2"]
pub type OFFSET2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF2R_SPEC, u16, u16, 12, O>;
#[doc = "Field `GAIN2` reader - Gain setting for configuration 2"]
pub type GAIN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN2` writer - Gain setting for configuration 2"]
pub type GAIN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF2R_SPEC, u8, u8, 3, O>;
#[doc = "Field `SE2` reader - Single-ended mode for configuration 2"]
pub type SE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SE2` writer - Single-ended mode for configuration 2"]
pub type SE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF2R_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMMON2` reader - Common mode for configuration 2"]
pub type COMMON2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMON2` writer - Common mode for configuration 2"]
pub type COMMON2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF2R_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&self) -> COMMON2_R {
        COMMON2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<0> {
        OFFSET2_W::new(self)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn gain2(&mut self) -> GAIN2_W<20> {
        GAIN2_W::new(self)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn se2(&mut self) -> SE2_W<26> {
        SE2_W::new(self)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn common2(&mut self) -> COMMON2_W<30> {
        COMMON2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2r](index.html) module"]
pub struct CONF2R_SPEC;
impl crate::RegisterSpec for CONF2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf2r::R](R) reader structure"]
impl crate::Readable for CONF2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf2r::W](W) writer structure"]
impl crate::Writable for CONF2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF2R to value 0"]
impl crate::Resettable for CONF2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
