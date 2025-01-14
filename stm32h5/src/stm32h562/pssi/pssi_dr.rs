#[doc = "Register `PSSI_DR` reader"]
pub struct R(crate::R<PSSI_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSI_DR` writer"]
pub struct W(crate::W<PSSI_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSI_DR_SPEC>;
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
impl From<crate::W<PSSI_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSI_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE0` reader - Data byte 0"]
pub type BYTE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE0` writer - Data byte 0"]
pub type BYTE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_DR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BYTE1` reader - Data byte 1"]
pub type BYTE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE1` writer - Data byte 1"]
pub type BYTE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_DR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BYTE2` reader - Data byte 2"]
pub type BYTE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE2` writer - Data byte 2"]
pub type BYTE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_DR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BYTE3` reader - Data byte 3"]
pub type BYTE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE3` writer - Data byte 3"]
pub type BYTE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSI_DR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn byte0(&mut self) -> BYTE0_W<0> {
        BYTE0_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn byte1(&mut self) -> BYTE1_W<8> {
        BYTE1_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn byte2(&mut self) -> BYTE2_W<16> {
        BYTE2_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn byte3(&mut self) -> BYTE3_W<24> {
        BYTE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSSI data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi_dr](index.html) module"]
pub struct PSSI_DR_SPEC;
impl crate::RegisterSpec for PSSI_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi_dr::R](R) reader structure"]
impl crate::Readable for PSSI_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssi_dr::W](W) writer structure"]
impl crate::Writable for PSSI_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSSI_DR to value 0"]
impl crate::Resettable for PSSI_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
