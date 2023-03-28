#[doc = "Register `CONFR2` reader"]
pub struct R(crate::R<CONFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFR2` writer"]
pub struct W(crate::W<CONFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR2_SPEC>;
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
impl From<crate::W<CONFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMCU` reader - Number of MCU"]
pub type NMCU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NMCU` writer - Number of MCU"]
pub type NMCU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR2_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - Number of MCU"]
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU"]
    #[inline(always)]
    #[must_use]
    pub fn nmcu(&mut self) -> NMCU_W<0> {
        NMCU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr2](index.html) module"]
pub struct CONFR2_SPEC;
impl crate::RegisterSpec for CONFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confr2::R](R) reader structure"]
impl crate::Readable for CONFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confr2::W](W) writer structure"]
impl crate::Writable for CONFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR2 to value 0"]
impl crate::Resettable for CONFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
