#[doc = "Register `CH2AWSCDR` reader"]
pub struct R(crate::R<CH2AWSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2AWSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2AWSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2AWSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2AWSCDR` writer"]
pub struct W(crate::W<CH2AWSCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2AWSCDR_SPEC>;
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
impl From<crate::W<CH2AWSCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2AWSCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT` reader - SCDT"]
pub type SCDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDT` writer - SCDT"]
pub type SCDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2AWSCDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKSCD` reader - BKSCD"]
pub type BKSCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKSCD` writer - BKSCD"]
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2AWSCDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWFOSR` reader - AWFOSR"]
pub type AWFOSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFOSR` writer - AWFOSR"]
pub type AWFOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2AWSCDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `AWFORD` reader - AWFORD"]
pub type AWFORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFORD` writer - AWFORD"]
pub type AWFORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2AWSCDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<0> {
        SCDT_W::new(self)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<12> {
        BKSCD_W::new(self)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<16> {
        AWFOSR_W::new(self)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<22> {
        AWFORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AWSCD2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2awscdr](index.html) module"]
pub struct CH2AWSCDR_SPEC;
impl crate::RegisterSpec for CH2AWSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2awscdr::R](R) reader structure"]
impl crate::Readable for CH2AWSCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2awscdr::W](W) writer structure"]
impl crate::Writable for CH2AWSCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2AWSCDR to value 0"]
impl crate::Resettable for CH2AWSCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
