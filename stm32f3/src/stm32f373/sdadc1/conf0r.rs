#[doc = "Register `CONF0R` reader"]
pub struct R(crate::R<CONF0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0R` writer"]
pub struct W(crate::W<CONF0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0R_SPEC>;
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
impl From<crate::W<CONF0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET0` reader - Twelve-bit calibration offset for configuration 0"]
pub type OFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET0` writer - Twelve-bit calibration offset for configuration 0"]
pub type OFFSET0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF0R_SPEC, u16, u16, 12, O>;
#[doc = "Field `GAIN0` reader - Gain setting for configuration 0"]
pub type GAIN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN0` writer - Gain setting for configuration 0"]
pub type GAIN0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF0R_SPEC, u8, u8, 3, O>;
#[doc = "Field `SE0` reader - Single-ended mode for configuration 0"]
pub type SE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SE0` writer - Single-ended mode for configuration 0"]
pub type SE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF0R_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMMON0` reader - Common mode for configuration 0"]
pub type COMMON0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMON0` writer - Common mode for configuration 0"]
pub type COMMON0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF0R_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&self) -> OFFSET0_R {
        OFFSET0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&self) -> COMMON0_R {
        COMMON0_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn offset0(&mut self) -> OFFSET0_W<0> {
        OFFSET0_W::new(self)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn gain0(&mut self) -> GAIN0_W<20> {
        GAIN0_W::new(self)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> SE0_W<26> {
        SE0_W::new(self)
    }
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn common0(&mut self) -> COMMON0_W<30> {
        COMMON0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0r](index.html) module"]
pub struct CONF0R_SPEC;
impl crate::RegisterSpec for CONF0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0r::R](R) reader structure"]
impl crate::Readable for CONF0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0r::W](W) writer structure"]
impl crate::Writable for CONF0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0R to value 0"]
impl crate::Resettable for CONF0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
