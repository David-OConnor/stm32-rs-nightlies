#[doc = "Register `I3C_DCR` reader"]
pub struct R(crate::R<I3C_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_DCR` writer"]
pub struct W(crate::W<I3C_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_DCR_SPEC>;
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
impl From<crate::W<I3C_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCR` reader - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCR` writer - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
pub type DCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_DCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_I3C_device_characteristics_register"]
    #[inline(always)]
    #[must_use]
    pub fn dcr(&mut self) -> DCR_W<0> {
        DCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C device characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_dcr](index.html) module"]
pub struct I3C_DCR_SPEC;
impl crate::RegisterSpec for I3C_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_dcr::R](R) reader structure"]
impl crate::Readable for I3C_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_dcr::W](W) writer structure"]
impl crate::Writable for I3C_DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_DCR to value 0"]
impl crate::Resettable for I3C_DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
