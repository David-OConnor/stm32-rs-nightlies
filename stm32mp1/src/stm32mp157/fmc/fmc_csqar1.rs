#[doc = "Register `FMC_CSQAR1` reader"]
pub struct R(crate::R<FMC_CSQAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQAR1` writer"]
pub struct W(crate::W<FMC_CSQAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQAR1_SPEC>;
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
impl From<crate::W<FMC_CSQAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDC1` reader - ADDC1"]
pub type ADDC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDC1` writer - ADDC1"]
pub type ADDC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDC2` reader - ADDC2"]
pub type ADDC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDC2` writer - ADDC2"]
pub type ADDC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDC3` reader - ADDC3"]
pub type ADDC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDC3` writer - ADDC3"]
pub type ADDC3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDC4` reader - ADDC4"]
pub type ADDC4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDC4` writer - ADDC4"]
pub type ADDC4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    pub fn addc1(&self) -> ADDC1_R {
        ADDC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    pub fn addc2(&self) -> ADDC2_R {
        ADDC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    pub fn addc3(&self) -> ADDC3_R {
        ADDC3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    pub fn addc4(&self) -> ADDC4_R {
        ADDC4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    #[must_use]
    pub fn addc1(&mut self) -> ADDC1_W<0> {
        ADDC1_W::new(self)
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    #[must_use]
    pub fn addc2(&mut self) -> ADDC2_W<8> {
        ADDC2_W::new(self)
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    #[must_use]
    pub fn addc3(&mut self) -> ADDC3_W<16> {
        ADDC3_W::new(self)
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    #[must_use]
    pub fn addc4(&mut self) -> ADDC4_W<24> {
        ADDC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to define the value of address cycles 1 to 4 to be issued by the command sequencer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqar1](index.html) module"]
pub struct FMC_CSQAR1_SPEC;
impl crate::RegisterSpec for FMC_CSQAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqar1::R](R) reader structure"]
impl crate::Readable for FMC_CSQAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqar1::W](W) writer structure"]
impl crate::Writable for FMC_CSQAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQAR1 to value 0"]
impl crate::Resettable for FMC_CSQAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
