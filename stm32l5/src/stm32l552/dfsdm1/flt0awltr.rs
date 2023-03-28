#[doc = "Register `FLT0AWLTR` reader"]
pub struct R(crate::R<FLT0AWLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT0AWLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT0AWLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT0AWLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT0AWLTR` writer"]
pub struct W(crate::W<FLT0AWLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT0AWLTR_SPEC>;
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
impl From<crate::W<FLT0AWLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT0AWLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT0AWLTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWLT` reader - Analog watchdog low threshold"]
pub type AWLT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold"]
pub type AWLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT0AWLTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<0> {
        BKAWL_W::new(self)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<8> {
        AWLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt0awltr](index.html) module"]
pub struct FLT0AWLTR_SPEC;
impl crate::RegisterSpec for FLT0AWLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt0awltr::R](R) reader structure"]
impl crate::Readable for FLT0AWLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt0awltr::W](W) writer structure"]
impl crate::Writable for FLT0AWLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLT0AWLTR to value 0"]
impl crate::Resettable for FLT0AWLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
